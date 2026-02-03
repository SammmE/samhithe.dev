mod db;
mod handlers;
mod middleware;
mod models;
mod state;

use axum::{
    middleware as axum_middleware,
    routing::{delete, get, post, put},
    Router,
};
use rusqlite::Connection;
use state::{AppState, ViewBuffer};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .with_level(true)
        .compact()
        .init();

    info!("Portfolio API starting");

    // Load environment variables
    dotenvy::dotenv().ok();

    let admin_password = std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "file:portfolio.db".to_string());
    let turso_url = std::env::var("TURSO_URL").expect("TURSO_URL must be set");
    let turso_token = std::env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");

    // Initialize local SQLite
    let db_path = db_url.trim_start_matches("file:");
    let conn = Connection::open(db_path).expect("Failed to open local database");

    if let Err(e) = db::local::init_db(&conn) {
        error!("Failed to initialize database: {}", e);
        std::process::exit(1);
    }

    // Load data from local DB into memory
    let projects = match db::local::load_all_projects(&conn) {
        Ok(data) => {
            info!("Loaded {} projects", data.len());
            data
        }
        Err(e) => {
            error!("Failed to load projects: {}", e);
            std::process::exit(1);
        }
    };

    let logs = match db::local::load_all_logs(&conn) {
        Ok(data) => {
            info!("Loaded {} logs", data.len());
            data
        }
        Err(e) => {
            error!("Failed to load logs: {}", e);
            std::process::exit(1);
        }
    };

    // Initialize Turso connection
    let turso_db = match libsql::Builder::new_remote(turso_url.clone(), turso_token.clone())
        .build()
        .await
    {
        Ok(db) => {
            info!("Connected to Turso");
            db
        }
        Err(e) => {
            warn!("Turso connection failed (will retry on sync): {}", e);
            libsql::Builder::new_remote(turso_url, turso_token)
                .build()
                .await
                .expect("Failed to create Turso client")
        }
    };

    // Create app state
    let state = AppState {
        projects: Arc::new(RwLock::new(projects)),
        logs: Arc::new(RwLock::new(logs)),
        view_buffer: Arc::new(RwLock::new(ViewBuffer::new())),
        local_db: Arc::new(Mutex::new(conn)),
        turso_db: Arc::new(turso_db),
        admin_password: admin_password.clone(),
        start_time: Instant::now(),
    };

    // Start background sync task (every 30 minutes)
    let sync_state = state.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(30 * 60));
        loop {
            interval.tick().await;
            sync_view_counts(&sync_state).await;
        }
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let public_routes = Router::new()
        .route("/projects", get(handlers::projects::list_projects))
        .route("/projects/:id", get(handlers::projects::get_project))
        .route("/logs", get(handlers::logs::list_logs))
        .route("/logs/:id", get(handlers::logs::get_log))
        .route("/stats", get(handlers::system::get_stats));

    let admin_routes = Router::new()
        .route("/admin/projects", post(handlers::admin::create_project))
        .route("/admin/projects/:id", put(handlers::admin::update_project))
        .route("/admin/projects/:id", delete(handlers::admin::delete_project))
        .route(
            "/admin/projects/:id/refresh-readme",
            post(handlers::admin::refresh_readme),
        )
        .route("/admin/logs", post(handlers::admin::create_log))
        .route("/admin/logs/:id", put(handlers::admin::update_log))
        .route("/admin/logs/:id", delete(handlers::admin::delete_log))
        .route("/admin/sync", post(handlers::admin::force_sync))
        .layer(axum_middleware::from_fn(
            middleware::auth::require_admin_password,
        ));

    let app = Router::new()
        .merge(public_routes)
        .merge(admin_routes)
        .with_state(state)
        .layer(cors);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    info!("Server listening on http://0.0.0.0:3000");

    axum::serve(listener, app).await.expect("Server failed");
}

/// Sync buffered view counts to databases
pub async fn sync_view_counts(state: &AppState) {
    let (project_views, log_views) = {
        let mut buffer = state.view_buffer.write().await;
        buffer.drain()
    };

    if project_views.is_empty() && log_views.is_empty() {
        return;
    }

    info!(
        "Syncing view counts: {} projects, {} logs",
        project_views.len(),
        log_views.len()
    );

    // Update local SQLite
    {
        let conn = state.local_db.lock().await;
        if !project_views.is_empty() {
            if let Err(e) = db::local::batch_update_project_views(&conn, &project_views) {
                error!("Failed to update project views in local DB: {}", e);
            }
        }
        if !log_views.is_empty() {
            if let Err(e) = db::local::batch_update_log_views(&conn, &log_views) {
                error!("Failed to update log views in local DB: {}", e);
            }
        }
    }

    // Update in-memory counts
    {
        let mut projects = state.projects.write().await;
        for (id, count) in &project_views {
            if let Some(project) = projects.get_mut(id) {
                project.view_count += count;
            }
        }
    }
    {
        let mut logs = state.logs.write().await;
        for (id, count) in &log_views {
            if let Some(log) = logs.get_mut(id) {
                log.view_count += count;
            }
        }
    }

    // Async sync to Turso (with error handling and buffering on failure)
    let turso_db = state.turso_db.clone();
    let view_buffer = state.view_buffer.clone();

    tokio::spawn(async move {
        let mut failed_project_views = Vec::new();
        let mut failed_log_views = Vec::new();

        if !project_views.is_empty() {
            match db::turso::sync_project_views(&turso_db, &project_views).await {
                Ok(_) => {}
                Err(e) => {
                    warn!("Turso sync failed (buffering for retry): {}", e);
                    failed_project_views = project_views.into_iter().collect();
                }
            }
        }

        if !log_views.is_empty() {
            match db::turso::sync_log_views(&turso_db, &log_views).await {
                Ok(_) => {}
                Err(e) => {
                    warn!("Turso sync failed (buffering for retry): {}", e);
                    failed_log_views = log_views.into_iter().collect();
                }
            }
        }

        // Re-buffer failed syncs for next attempt
        if !failed_project_views.is_empty() || !failed_log_views.is_empty() {
            let mut buffer = view_buffer.write().await;
            for (id, count) in failed_project_views {
                *buffer.project_views.entry(id).or_insert(0) += count;
            }
            for (id, count) in failed_log_views {
                *buffer.log_views.entry(id).or_insert(0) += count;
            }
        }
    });
}
