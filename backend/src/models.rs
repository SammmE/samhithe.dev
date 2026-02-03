use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a portfolio project
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub repo_url: String,
    pub readme_content: String,
    pub demo_url: Option<String>,
    pub view_count: u32,
    pub priority: u8,
    pub created_at: DateTime<Utc>,
}

/// Represents a log entry
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    pub id: u32,
    pub content: String,
    pub view_count: u32,
    pub created_at: DateTime<Utc>,
}

/// System statistics response
#[derive(Serialize)]
pub struct SystemStats {
    pub uptime_seconds: u64,
    pub memory_usage_mb: u32,
    pub engine: String,
    pub persistence: String,
    pub buffered_views_size: usize,
}

/// Request body for creating a new project
#[derive(Deserialize)]
pub struct CreateProject {
    pub title: String,
    pub repo_url: String,
    pub demo_url: Option<String>,
    pub priority: Option<u8>,
}

/// Request body for updating a project
#[derive(Deserialize)]
pub struct UpdateProject {
    pub title: Option<String>,
    pub repo_url: Option<String>,
    pub demo_url: Option<String>,
    pub readme_content: Option<String>,
    pub priority: Option<u8>,
}

/// Request body for creating a new log
#[derive(Deserialize)]
pub struct CreateLog {
    pub content: String,
}

/// Request body for updating a log
#[derive(Deserialize)]
pub struct UpdateLog {
    pub content: Option<String>,
}

/// GitHub API response for README content
#[derive(Serialize, Deserialize, Debug)]
pub struct GithubReadme {
    pub content: String,
    pub encoding: String,
}
