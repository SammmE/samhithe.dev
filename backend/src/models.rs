use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadmeType {
    Url,
    Raw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    Unknown,
    Healthy,
    Broken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub demo_link: Option<String>,
    pub repo_link: Option<String>,
    pub readme_type: ReadmeType,
    pub readme_content: String,
    pub importance: u32,
    pub portfolio_entry: String,
    pub tags: Vec<String>,
    pub health_status: HealthStatus,
    pub last_health_check: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectPatch {
    pub name: Option<String>,
    pub description: Option<String>,
    pub demo_link: Option<Option<String>>,
    pub repo_link: Option<Option<String>>,
    pub readme_type: Option<ReadmeType>,
    pub readme_content: Option<String>,
    pub importance: Option<u32>,
    pub portfolio_entry: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInput {
    pub id: String,
    pub name: String,
    pub description: String,
    pub demo_link: Option<String>,
    pub repo_link: Option<String>,
    pub readme_type: ReadmeType,
    pub readme_content: String,
    pub importance: u32,
    pub portfolio_entry: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleMeta {
    pub id: String,
    pub title: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub word_count: u64,
    pub image_count: u64,
    pub heading_count: u64,
    pub views: u64,
    pub is_published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleContent {
    pub id: String,
    pub abstract_markdown: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    #[serde(flatten)]
    pub meta: ArticleMeta,
    pub abstract_markdown: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminArticleInput {
    pub id: String,
    pub title: String,
    pub description: String,
    pub abstract_markdown: String,
    pub content: String,
    pub date: Option<DateTime<Utc>>,
    pub is_published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsEvent {
    pub id: String,
    pub article_id: String,
    pub timestamp: DateTime<Utc>,
    pub ip_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub global_views: u64,
    pub best_performer: Option<ArticleMeta>,
    pub trending: u64,
    pub broken_projects: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartPoint {
    pub date: String,
    pub views: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitResponse {
    pub counted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: &'static str,
}
