use crate::models::SystemStats;
use crate::state::AppState;
use axum::{extract::State, response::Json};

/// Get system statistics
pub async fn get_stats(State(state): State<AppState>) -> Json<SystemStats> {
    let uptime = state.start_time.elapsed().as_secs();
    let buffer = state.view_buffer.read().await;
    let buffered_size = buffer.total_buffered_count();

    let projects_count = state.projects.read().await.len();
    let logs_count = state.logs.read().await.len();

    // Rough memory estimate in MB
    let memory_estimate = (projects_count * 2 + logs_count * 1 + buffered_size) as u32;

    Json(SystemStats {
        uptime_seconds: uptime,
        memory_usage_mb: memory_estimate,
        engine: "Axum + Tokio".to_string(),
        persistence: "SQLite (local) + Turso (backup)".to_string(),
        buffered_views_size: buffered_size,
    })
}
