use crate::models::Project;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};

/// List all projects
pub async fn list_projects(State(state): State<AppState>) -> Json<Vec<Project>> {
    let projects = state.projects.read().await;
    let buffer = state.view_buffer.read().await;

    let mut result: Vec<Project> = projects.values().cloned().collect();

    // Include buffered view counts in response
    for project in &mut result {
        let buffered = buffer.get_project_buffered(project.id);
        project.view_count += buffered;
    }

    // Sort by priority (descending), then by id (ascending)
    result.sort_by(|a, b| {
        b.priority.cmp(&a.priority)
            .then_with(|| a.id.cmp(&b.id))
    });
    Json(result)
}

/// Get a single project by ID (increments view count)
pub async fn get_project(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<Project>, StatusCode> {
    // Increment view buffer
    {
        let mut buffer = state.view_buffer.write().await;
        buffer.increment_project(id);
    }

    let projects = state.projects.read().await;
    let buffer = state.view_buffer.read().await;

    match projects.get(&id) {
        Some(project) => {
            let mut result = project.clone();
            result.view_count += buffer.get_project_buffered(id);
            Ok(Json(result))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
