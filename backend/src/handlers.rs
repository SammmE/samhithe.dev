use std::{net::SocketAddr, sync::Arc};

use axum::{
    Json, Router,
    extract::{ConnectInfo, Path, State},
    http::{HeaderMap, StatusCode},
    middleware,
    routing::{get, patch, post},
};
use sha2::{Digest, Sha256};
use tower::ServiceBuilder;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};

use crate::{
    auth::require_firebase_auth,
    error::AppError,
    markdown,
    models::{
        AdminArticleInput, ArticleContent, ArticleMeta, HealthResponse, HitResponse, ProjectInput,
        ProjectPatch,
    },
    state::AppState,
};

pub fn router(state: Arc<AppState>) -> Router {
    let governor_conf = {
        let mut builder = GovernorConfigBuilder::default();
        builder.per_second(60).burst_size(10);
        Arc::new(builder.finish().expect("governor config should be valid"))
    };

    let admin_routes = Router::new()
        .route("/articles", post(create_article))
        .route("/stats", get(stats))
        .route("/charts", get(charts))
        .route("/projects", post(create_project))
        .route("/projects/{id}", patch(update_project))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_firebase_auth,
        ));

    let cors = state
        .config
        .cors_layer()
        .expect("CORS origins are validated during startup");

    Router::new()
        .route("/health", get(health))
        .route("/projects", get(projects))
        .route("/articles", get(articles))
        .route("/articles/{id}", get(article))
        .route(
            "/hit/{id}",
            post(hit).layer(GovernorLayer::new(governor_conf)),
        )
        .nest("/admin", admin_routes)
        .with_state(state)
        .layer(ServiceBuilder::new().layer(cors))
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

async fn projects(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<crate::models::Project>>, AppError> {
    Ok(Json(state.firestore.list_projects().await?))
}

async fn articles(State(state): State<Arc<AppState>>) -> Result<Json<Vec<ArticleMeta>>, AppError> {
    Ok(Json(state.firestore.list_published_articles().await?))
}

async fn article(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::models::Article>, AppError> {
    Ok(Json(state.firestore.get_article(&id).await?))
}

async fn hit(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Json<HitResponse>, AppError> {
    let ip = headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|value| value.to_str().ok())
        })
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| addr.ip().to_string());
    let ip_hash = hash_ip(&ip);
    let counted = state.firestore.record_hit(&id, &ip_hash).await?;

    Ok(Json(HitResponse { counted }))
}

async fn create_article(
    State(state): State<Arc<AppState>>,
    Json(input): Json<AdminArticleInput>,
) -> Result<(StatusCode, Json<ArticleMeta>), AppError> {
    if input.id.trim().is_empty() || input.title.trim().is_empty() {
        return Err(AppError::bad_request("id and title are required"));
    }

    let audit = markdown::audit(&input.content);
    let meta = ArticleMeta {
        id: input.id.trim().to_string(),
        title: input.title,
        description: input.description,
        date: input.date.unwrap_or_else(chrono::Utc::now),
        word_count: audit.word_count,
        image_count: audit.image_count,
        heading_count: audit.heading_count,
        views: 0,
        is_published: input.is_published,
    };
    let content = ArticleContent {
        id: meta.id.clone(),
        abstract_markdown: input.abstract_markdown,
        content: input.content,
    };

    state.firestore.upsert_article(&meta, &content).await?;
    Ok((StatusCode::CREATED, Json(meta)))
}

async fn stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::models::StatsResponse>, AppError> {
    Ok(Json(state.firestore.stats().await?))
}

async fn charts(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<crate::models::ChartPoint>>, AppError> {
    Ok(Json(state.firestore.chart_points().await?))
}

async fn create_project(
    State(state): State<Arc<AppState>>,
    Json(input): Json<ProjectInput>,
) -> Result<(StatusCode, Json<crate::models::Project>), AppError> {
    if input.id.trim().is_empty() || input.name.trim().is_empty() {
        return Err(AppError::bad_request("id and name are required"));
    }

    Ok((
        StatusCode::CREATED,
        Json(state.firestore.create_project(input).await?),
    ))
}

async fn update_project(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(input): Json<ProjectPatch>,
) -> Result<Json<crate::models::Project>, AppError> {
    Ok(Json(state.firestore.update_project(&id, input).await?))
}

fn hash_ip(ip: &str) -> String {
    let digest = Sha256::digest(ip.as_bytes());
    base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, digest)
}
