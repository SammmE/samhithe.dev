use crate::db::local;
use crate::models::Log;
use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use serde::Deserialize;
use tracing::error;

#[derive(Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_page() -> u32 {
    1
}

fn default_limit() -> u32 {
    20
}

/// List all logs (sorted by creation date, newest first) with pagination
pub async fn list_logs(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<Log>>, StatusCode> {
    let page = if params.page < 1 { 1 } else { params.page };
    let limit = if params.limit < 1 { 20 } else { params.limit };
    let offset = (page - 1) * limit;

    let conn = state.local_db.lock().await;
    let mut logs = local::load_logs_page(&conn, limit, offset).map_err(|e| {
        error!("Failed to load logs: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    drop(conn);

    // Include buffered view counts in response
    let buffer = state.view_buffer.read().await;
    for log in &mut logs {
        let buffered = buffer.get_log_buffered(log.id);
        log.view_count += buffered;
    }

    Ok(Json(logs))
}

/// Get a single log by ID (increments view count)
pub async fn get_log(
    Path(id): Path<u32>,
    State(state): State<AppState>,
) -> Result<Json<Log>, StatusCode> {
    // Increment view buffer
    {
        let mut buffer = state.view_buffer.write().await;
        buffer.increment_log(id);
    }

    let logs = state.logs.read().await;
    let buffer = state.view_buffer.read().await;

    match logs.get(&id) {
        Some(log) => {
            let mut result = log.clone();
            result.view_count += buffer.get_log_buffered(id);
            Ok(Json(result))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}
