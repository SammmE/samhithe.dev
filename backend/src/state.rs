use crate::models::{Log, Project};
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{Mutex, RwLock};

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    /// In-memory storage of all projects
    pub projects: Arc<RwLock<HashMap<u32, Project>>>,
    /// In-memory storage of all logs
    pub logs: Arc<RwLock<HashMap<u32, Log>>>,
    /// Buffer for view counts (synced periodically to database)
    pub view_buffer: Arc<RwLock<ViewBuffer>>,
    /// Local SQLite database connection
    pub local_db: Arc<Mutex<Connection>>,
    /// Turso database client (for backup/sync)
    pub turso_db: Arc<libsql::Database>,
    /// Admin password for protected endpoints
    pub admin_password: String,
    /// Server start time (for uptime calculation)
    pub start_time: Instant,
}

/// Buffer for accumulating view counts before batch write to database
pub struct ViewBuffer {
    /// Project ID -> accumulated view count
    pub project_views: HashMap<u32, u32>,
    /// Log ID -> accumulated view count
    pub log_views: HashMap<u32, u32>,
}

impl ViewBuffer {
    /// Create a new empty view buffer
    pub fn new() -> Self {
        Self {
            project_views: HashMap::new(),
            log_views: HashMap::new(),
        }
    }

    /// Increment view count for a project
    pub fn increment_project(&mut self, id: u32) {
        *self.project_views.entry(id).or_insert(0) += 1;
    }

    /// Increment view count for a log
    pub fn increment_log(&mut self, id: u32) {
        *self.log_views.entry(id).or_insert(0) += 1;
    }

    /// Get buffered view count for a project
    pub fn get_project_buffered(&self, id: u32) -> u32 {
        *self.project_views.get(&id).unwrap_or(&0)
    }

    /// Get buffered view count for a log
    pub fn get_log_buffered(&self, id: u32) -> u32 {
        *self.log_views.get(&id).unwrap_or(&0)
    }

    /// Get total number of buffered entries
    pub fn total_buffered_count(&self) -> usize {
        self.project_views.len() + self.log_views.len()
    }

    /// Drain all buffered view counts and return them
    pub fn drain(&mut self) -> (HashMap<u32, u32>, HashMap<u32, u32>) {
        let projects = std::mem::take(&mut self.project_views);
        let logs = std::mem::take(&mut self.log_views);
        (projects, logs)
    }
}
