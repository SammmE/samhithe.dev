use crate::db::{local, turso};
use crate::models::{CreateLog, CreateProject, GithubReadme, Log, Project, UpdateLog, UpdateProject};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use base64::Engine;
use chrono::Utc;
use tracing::{error, warn};

/// Fetch README content from a GitHub repository
async fn fetch_github_readme(repo_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = repo_url.trim_end_matches('/').split('/').collect();
    if parts.len() < 2 {
        return Err("Invalid GitHub URL format".into());
    }
    let owner = parts[parts.len() - 2];
    let repo = parts[parts.len() - 1];

    let url = format!("https://api.github.com/repos/{}/{}/readme", owner, repo);
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header("User-Agent", "Portfolio-API")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(format!("GitHub API error: {}", response.status()).into());
    }

    let readme: GithubReadme = response.json().await?;
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(&readme.content.replace("\n", ""))?;

    Ok(String::from_utf8(decoded)?)
}

/// Create a new project
pub async fn create_project(
    State(state): State<AppState>,
    Json(input): Json<CreateProject>,
) -> Result<Json<Project>, StatusCode> {
    let readme_content = match fetch_github_readme(&input.repo_url).await {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to fetch README: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    let conn = state.local_db.lock().await;
    let id = local::get_next_project_id(&conn).map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let project = Project {
        id,
        title: input.title.clone(),
        repo_url: input.repo_url.clone(),
        readme_content,
        demo_url: input.demo_url.clone(),
        view_count: 0,
        priority: input.priority.unwrap_or(0),
        created_at: Utc::now(),
    };

    local::insert_project(&conn, &project).map_err(|e| {
        error!("Failed to insert project: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    drop(conn);

    state.projects.write().await.insert(id, project.clone());

    // Async sync to Turso
    let turso_db = state.turso_db.clone();
    let p = project.clone();
    tokio::spawn(async move {
        if let Err(e) = turso::sync_project(
            &turso_db,
            p.id,
            &p.title,
            &p.repo_url,
            p.demo_url.as_deref(),
            p.view_count,
            p.priority,
            &p.created_at.to_rfc3339(),
        )
        .await
        {
            warn!("Turso sync failed: {}", e);
        }
    });

    Ok(Json(project))
}

/// Update an existing project
pub async fn update_project(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(input): Json<UpdateProject>,
) -> Result<Json<Project>, StatusCode> {
    let mut projects = state.projects.write().await;
    let project = projects.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(title) = input.title {
        project.title = title;
    }
    if let Some(repo_url) = input.repo_url {
        project.repo_url = repo_url;
    }
    if let Some(demo_url) = input.demo_url {
        project.demo_url = Some(demo_url);
    }
    if let Some(readme_content) = input.readme_content {
        project.readme_content = readme_content;
    }
    if let Some(priority) = input.priority {
        project.priority = priority;
    }

    let updated = project.clone();

    let conn = state.local_db.lock().await;
    local::update_project(&conn, &updated).map_err(|e| {
        error!("Failed to update project: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    drop(conn);
    drop(projects);

    // Async sync to Turso
    let turso_db = state.turso_db.clone();
    let p = updated.clone();
    tokio::spawn(async move {
        if let Err(e) = turso::sync_project(
            &turso_db,
            p.id,
            &p.title,
            &p.repo_url,
            p.demo_url.as_deref(),
            p.view_count,
            p.priority,
            &p.created_at.to_rfc3339(),
        )
        .await
        {
            warn!("Turso sync failed: {}", e);
        }
    });

    Ok(Json(updated))
}

/// Delete a project
pub async fn delete_project(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let mut projects = state.projects.write().await;

    if projects.remove(&id).is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let conn = state.local_db.lock().await;
    local::delete_project(&conn, id).map_err(|e| {
        error!("Failed to delete project: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    drop(conn);
    drop(projects);

    // Async sync to Turso
    let turso_db = state.turso_db.clone();
    tokio::spawn(async move {
        if let Err(e) = turso::delete_project_turso(&turso_db, id).await {
            warn!("Turso sync failed: {}", e);
        }
    });

    Ok(StatusCode::NO_CONTENT)
}

/// Refresh README content from GitHub
pub async fn refresh_readme(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<Project>, StatusCode> {
    let mut projects = state.projects.write().await;
    let project = projects.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    let readme_content = match fetch_github_readme(&project.repo_url).await {
        Ok(content) => content,
        Err(e) => {
            error!("Failed to fetch README: {}", e);
            return Err(StatusCode::BAD_REQUEST);
        }
    };

    project.readme_content = readme_content;
    let updated = project.clone();

    let conn = state.local_db.lock().await;
    local::update_project(&conn, &updated).map_err(|e| {
        error!("Failed to update project: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    drop(conn);
    drop(projects);

    // Async sync to Turso
    let turso_db = state.turso_db.clone();
    let p = updated.clone();
    tokio::spawn(async move {
        if let Err(e) = turso::sync_project(
            &turso_db,
            p.id,
            &p.title,
            &p.repo_url,
            p.demo_url.as_deref(),
            p.view_count,
            p.priority,
            &p.created_at.to_rfc3339(),
        )
        .await
        {
            warn!("Turso sync failed: {}", e);
        }
    });

    Ok(Json(updated))
}

/// Create a new log entry
pub async fn create_log(
    State(state): State<AppState>,
    Json(input): Json<CreateLog>,
) -> Result<Json<Log>, StatusCode> {
    let conn = state.local_db.lock().await;
    let id = local::get_next_log_id(&conn).map_err(|e| {
        error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let log = Log {
        id,
        content: input.content.clone(),
        view_count: 0,
        created_at: Utc::now(),
    };

    local::insert_log(&conn, &log).map_err(|e| {
        error!("Failed to insert log: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    drop(conn);

    state.logs.write().await.insert(id, log.clone());

    // Async sync to Turso
    let turso_db = state.turso_db.clone();
    let l = log.clone();
    tokio::spawn(async move {
        if let Err(e) = turso::sync_log(
            &turso_db,
            l.id,
            &l.content,
            l.view_count,
            &l.created_at.to_rfc3339(),
        )
        .await
        {
            warn!("Turso sync failed: {}", e);
        }
    });

    Ok(Json(log))
}

/// Update an existing log entry
pub async fn update_log(
    Path(id): Path<u32>,
    State(state): State<AppState>,
    Json(input): Json<UpdateLog>,
) -> Result<Json<Log>, StatusCode> {
    let mut logs = state.logs.write().await;
    let log = logs.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(content) = input.content {
        log.content = content;
    }

    let updated = log.clone();

    let conn = state.local_db.lock().await;
    local::update_log(&conn, &updated).map_err(|e| {
        error!("Failed to update log: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    drop(conn);
    drop(logs);

    // Async sync to Turso
    let turso_db = state.turso_db.clone();
    let l = updated.clone();
    tokio::spawn(async move {
        if let Err(e) = turso::sync_log(
            &turso_db,
            l.id,
            &l.content,
            l.view_count,
            &l.created_at.to_rfc3339(),
        )
        .await
        {
            warn!("Turso sync failed: {}", e);
        }
    });

    Ok(Json(updated))
}

/// Delete a log entry
pub async fn delete_log(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<StatusCode, StatusCode> {
    let mut logs = state.logs.write().await;

    if logs.remove(&id).is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    let conn = state.local_db.lock().await;
    local::delete_log(&conn, id).map_err(|e| {
        error!("Failed to delete log: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    drop(conn);
    drop(logs);

    // Async sync to Turso
    let turso_db = state.turso_db.clone();
    tokio::spawn(async move {
        if let Err(e) = turso::delete_log_turso(&turso_db, id).await {
            warn!("Turso sync failed: {}", e);
        }
    });

    Ok(StatusCode::NO_CONTENT)
}

/// Force immediate sync of view counts
pub async fn force_sync(State(state): State<AppState>) -> Result<Json<String>, StatusCode> {
    crate::sync_view_counts(&state).await;
    Ok(Json("Sync completed".to_string()))
}
