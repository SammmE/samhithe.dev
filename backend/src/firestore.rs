use std::{collections::HashMap, sync::Arc};

use chrono::{DateTime, Duration, NaiveDate, Utc};
use reqwest::{Client, Method, Url};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value, json};

use crate::{
    config::Config,
    error::AppError,
    models::{
        AnalyticsEvent, Article, ArticleContent, ArticleMeta, ChartPoint, HealthStatus, Project,
        ProjectInput, ProjectPatch, ReadmeType, StatsResponse,
    },
    service_account::ServiceAccountTokenProvider,
};

#[derive(Clone)]
pub struct Firestore {
    config: Arc<Config>,
    http: Client,
    base_url: Url,
    token_provider: Option<ServiceAccountTokenProvider>,
}

impl Firestore {
    pub fn new(config: Arc<Config>) -> Result<Self, AppError> {
        let root = if let Some(host) = &config.firestore_emulator_host {
            format!(
                "http://{host}/v1/projects/{}/databases/{}/documents/",
                config.firebase_project_id, config.firestore_database
            )
        } else {
            format!(
                "https://firestore.googleapis.com/v1/projects/{}/databases/{}/documents/",
                config.firebase_project_id, config.firestore_database
            )
        };

        let token_provider = config
            .service_account
            .clone()
            .map(ServiceAccountTokenProvider::new);

        Ok(Self {
            config,
            http: Client::new(),
            base_url: Url::parse(&root).map_err(AppError::internal)?,
            token_provider,
        })
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>, AppError> {
        let docs = self.list_collection("projects").await?;
        docs.into_iter().map(Project::try_from).collect()
    }

    pub async fn list_published_articles(&self) -> Result<Vec<ArticleMeta>, AppError> {
        let mut articles: Vec<ArticleMeta> = self
            .list_collection("articles_meta")
            .await?
            .into_iter()
            .map(ArticleMeta::try_from)
            .collect::<Result<_, _>>()?;

        articles.retain(|article| article.is_published);
        articles.sort_by(|left, right| right.date.cmp(&left.date));
        Ok(articles)
    }

    pub async fn create_project(&self, input: ProjectInput) -> Result<Project, AppError> {
        let project = Project {
            id: input.id,
            name: input.name,
            description: input.description,
            demo_link: input.demo_link,
            repo_link: input.repo_link,
            readme_type: input.readme_type,
            readme_content: input.readme_content,
            health_status: HealthStatus::Unknown,
            last_health_check: None,
        };

        self.put_document("projects", &project.id, &project_fields(&project))
            .await?;
        Ok(project)
    }

    pub async fn get_article(&self, id: &str) -> Result<Article, AppError> {
        let (meta, content) = tokio::join!(
            self.get_document::<ArticleMeta>("articles_meta", id),
            self.get_document::<ArticleContent>("articles_content", id)
        );

        let meta = meta?.ok_or(AppError::NotFound)?;
        let content = content?.ok_or(AppError::NotFound)?;

        if !meta.is_published {
            return Err(AppError::NotFound);
        }

        Ok(Article {
            meta,
            abstract_markdown: content.abstract_markdown,
            content: content.content,
        })
    }

    pub async fn upsert_article(
        &self,
        meta: &ArticleMeta,
        content: &ArticleContent,
    ) -> Result<(), AppError> {
        let writes = json!({
            "writes": [
                { "update": self.document_payload("articles_meta", &meta.id, &article_meta_fields(meta))? },
                { "update": self.document_payload("articles_content", &content.id, &article_content_fields(content))? }
            ]
        });

        self.request(
            Method::POST,
            self.base_url.join(":commit").map_err(AppError::internal)?,
            Some(writes),
        )
        .await?;
        Ok(())
    }

    pub async fn update_project(&self, id: &str, patch: ProjectPatch) -> Result<Project, AppError> {
        let mut project = self
            .get_document::<Project>("projects", id)
            .await?
            .ok_or(AppError::NotFound)?;

        if let Some(value) = patch.name {
            project.name = value;
        }
        if let Some(value) = patch.description {
            project.description = value;
        }
        if let Some(value) = patch.demo_link {
            project.demo_link = value;
        }
        if let Some(value) = patch.repo_link {
            project.repo_link = value;
        }
        if let Some(value) = patch.readme_type {
            project.readme_type = value;
        }
        if let Some(value) = patch.readme_content {
            project.readme_content = value;
        }

        self.put_document("projects", id, &project_fields(&project))
            .await?;
        Ok(project)
    }

    pub async fn record_hit(&self, article_id: &str, ip_hash: &str) -> Result<bool, AppError> {
        let cutoff = Utc::now() - Duration::minutes(30);
        let events = self.events_since(cutoff).await?;
        let duplicate = events
            .iter()
            .any(|event| event.article_id == article_id && event.ip_hash == ip_hash);

        if duplicate {
            return Ok(false);
        }

        let mut meta = self
            .get_document::<ArticleMeta>("articles_meta", article_id)
            .await?
            .ok_or(AppError::NotFound)?;

        meta.views += 1;
        let event = AnalyticsEvent {
            id: uuid::Uuid::new_v4().to_string(),
            article_id: article_id.to_string(),
            timestamp: Utc::now(),
            ip_hash: ip_hash.to_string(),
        };

        let writes = json!({
            "writes": [
                { "update": self.document_payload("articles_meta", &meta.id, &article_meta_fields(&meta))? },
                { "update": self.document_payload("analytics_events", &event.id, &analytics_event_fields(&event))? }
            ]
        });

        self.request(
            Method::POST,
            self.base_url.join(":commit").map_err(AppError::internal)?,
            Some(writes),
        )
        .await?;
        Ok(true)
    }

    pub async fn stats(&self) -> Result<StatsResponse, AppError> {
        let articles: Vec<ArticleMeta> = self
            .list_collection("articles_meta")
            .await?
            .into_iter()
            .map(ArticleMeta::try_from)
            .collect::<Result<_, _>>()?;
        let projects: Vec<Project> = self
            .list_collection("projects")
            .await?
            .into_iter()
            .map(Project::try_from)
            .collect::<Result<_, _>>()?;

        let global_views = articles.iter().map(|article| article.views).sum();
        let best_performer = articles.into_iter().max_by_key(|article| article.views);
        let trending = self
            .events_since(Utc::now() - Duration::hours(48))
            .await?
            .len() as u64;
        let broken_projects = projects
            .iter()
            .filter(|project| matches!(project.health_status, HealthStatus::Broken))
            .count() as u64;

        Ok(StatsResponse {
            global_views,
            best_performer,
            trending,
            broken_projects,
        })
    }

    pub async fn chart_points(&self) -> Result<Vec<ChartPoint>, AppError> {
        let start = Utc::now() - Duration::days(29);
        let events = self.events_since(start).await?;
        let mut counts: HashMap<NaiveDate, u64> = HashMap::new();

        for event in events {
            *counts.entry(event.timestamp.date_naive()).or_default() += 1;
        }

        let today = Utc::now().date_naive();
        let mut points = Vec::with_capacity(30);
        for offset in (0..30).rev() {
            let date = today - Duration::days(offset);
            points.push(ChartPoint {
                date: date.to_string(),
                views: *counts.get(&date).unwrap_or(&0),
            });
        }

        Ok(points)
    }

    pub async fn update_project_health(
        &self,
        project: &Project,
        status: HealthStatus,
    ) -> Result<(), AppError> {
        let mut next = project.clone();
        next.health_status = status;
        next.last_health_check = Some(Utc::now());
        self.put_document("projects", &next.id, &project_fields(&next))
            .await
    }

    async fn events_since(&self, cutoff: DateTime<Utc>) -> Result<Vec<AnalyticsEvent>, AppError> {
        let mut events: Vec<AnalyticsEvent> = self
            .list_collection("analytics_events")
            .await?
            .into_iter()
            .map(AnalyticsEvent::try_from)
            .collect::<Result<_, _>>()?;
        events.retain(|event| event.timestamp >= cutoff);
        Ok(events)
    }

    async fn get_document<T>(&self, collection: &str, id: &str) -> Result<Option<T>, AppError>
    where
        T: TryFrom<Document, Error = AppError>,
    {
        let url = self.doc_url(collection, id)?;
        let response = self.request(Method::GET, url, None).await?;
        if response
            .get("error")
            .and_then(|err| err.get("code"))
            .and_then(Value::as_i64)
            == Some(404)
        {
            return Ok(None);
        }
        match Document::from_value(response) {
            Ok(document) => T::try_from(document).map(Some),
            Err(err) => Err(err),
        }
    }

    async fn list_collection(&self, collection: &str) -> Result<Vec<Document>, AppError> {
        let url = self.base_url.join(collection).map_err(AppError::internal)?;
        let response = self.request(Method::GET, url, None).await?;
        let docs = response
            .get("documents")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();

        docs.into_iter().map(Document::from_value).collect()
    }

    async fn put_document(
        &self,
        collection: &str,
        id: &str,
        fields: &Map<String, Value>,
    ) -> Result<(), AppError> {
        let payload = self.document_payload(collection, id, fields)?;
        self.request(Method::PATCH, self.doc_url(collection, id)?, Some(payload))
            .await?;
        Ok(())
    }

    fn document_payload(
        &self,
        collection: &str,
        id: &str,
        fields: &Map<String, Value>,
    ) -> Result<Value, AppError> {
        Ok(json!({
            "name": self.doc_name(collection, id),
            "fields": fields,
        }))
    }

    fn doc_name(&self, collection: &str, id: &str) -> String {
        format!(
            "projects/{}/databases/{}/documents/{collection}/{id}",
            self.config.firebase_project_id, self.config.firestore_database
        )
    }

    fn doc_url(&self, collection: &str, id: &str) -> Result<Url, AppError> {
        self.base_url
            .join(&format!("{collection}/{id}"))
            .map_err(AppError::internal)
    }

    async fn request(
        &self,
        method: Method,
        url: Url,
        body: Option<Value>,
    ) -> Result<Value, AppError> {
        let mut request = self.http.request(method, url);
        if self.config.firestore_emulator_host.is_none() {
            if let Some(provider) = &self.token_provider {
                let token = provider.access_token().await?;
                request = request.bearer_auth(token);
            } else {
                return Err(AppError::configuration(
                    "Firestore production access requires a firebase-adminsdk service account JSON",
                ));
            }
        }
        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await.map_err(AppError::upstream)?;
        let status = response.status();
        let text = response.text().await.map_err(AppError::upstream)?;

        if status == reqwest::StatusCode::NOT_FOUND {
            return Ok(json!({ "error": { "code": 404 } }));
        }

        if !status.is_success() {
            tracing::error!(status = %status, body = %text, "firestore request failed");
            return Err(AppError::Upstream);
        }

        if text.trim().is_empty() {
            return Ok(json!({}));
        }

        serde_json::from_str(&text).map_err(AppError::upstream)
    }
}

#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub fields: Map<String, Value>,
}

impl Document {
    fn from_value(value: Value) -> Result<Self, AppError> {
        let name = value
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| AppError::upstream("firestore document missing name"))?;
        let fields = value
            .get("fields")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        let id = name.rsplit('/').next().unwrap_or(name).to_string();

        Ok(Self { id, fields })
    }
}

impl TryFrom<Document> for Project {
    type Error = AppError;

    fn try_from(doc: Document) -> Result<Self, Self::Error> {
        Ok(Self {
            id: str_field(&doc, "id").unwrap_or_else(|| doc.id.clone()),
            name: required_str(&doc, "name")?,
            description: required_str(&doc, "description")?,
            demo_link: str_field(&doc, "demo_link"),
            repo_link: str_field(&doc, "repo_link"),
            readme_type: enum_field(&doc, "readme_type", ReadmeType::Raw)?,
            readme_content: str_field(&doc, "readme_content").unwrap_or_default(),
            health_status: enum_field(&doc, "health_status", HealthStatus::Unknown)?,
            last_health_check: timestamp_field(&doc, "last_health_check"),
        })
    }
}

impl TryFrom<Document> for ArticleMeta {
    type Error = AppError;

    fn try_from(doc: Document) -> Result<Self, Self::Error> {
        Ok(Self {
            id: str_field(&doc, "id").unwrap_or_else(|| doc.id.clone()),
            title: required_str(&doc, "title")?,
            description: required_str(&doc, "description")?,
            date: timestamp_field(&doc, "date").unwrap_or_else(Utc::now),
            word_count: int_field(&doc, "word_count"),
            image_count: int_field(&doc, "image_count"),
            heading_count: int_field(&doc, "heading_count"),
            views: int_field(&doc, "views"),
            is_published: bool_field(&doc, "is_published"),
        })
    }
}

impl TryFrom<Document> for ArticleContent {
    type Error = AppError;

    fn try_from(doc: Document) -> Result<Self, Self::Error> {
        Ok(Self {
            id: str_field(&doc, "id").unwrap_or_else(|| doc.id.clone()),
            abstract_markdown: str_field(&doc, "abstract_markdown")
                .or_else(|| str_field(&doc, "abstract"))
                .unwrap_or_default(),
            content: str_field(&doc, "content").unwrap_or_default(),
        })
    }
}

impl TryFrom<Document> for AnalyticsEvent {
    type Error = AppError;

    fn try_from(doc: Document) -> Result<Self, Self::Error> {
        Ok(Self {
            id: str_field(&doc, "id").unwrap_or_else(|| doc.id.clone()),
            article_id: required_str(&doc, "article_id")?,
            timestamp: timestamp_field(&doc, "timestamp").unwrap_or_else(Utc::now),
            ip_hash: required_str(&doc, "ip_hash")?,
        })
    }
}

fn project_fields(project: &Project) -> Map<String, Value> {
    fields([
        ("id", string_value(&project.id)),
        ("name", string_value(&project.name)),
        ("description", string_value(&project.description)),
        (
            "demo_link",
            nullable_string_value(project.demo_link.as_deref()),
        ),
        (
            "repo_link",
            nullable_string_value(project.repo_link.as_deref()),
        ),
        (
            "readme_type",
            string_value(enum_to_str(&project.readme_type)),
        ),
        ("readme_content", string_value(&project.readme_content)),
        (
            "health_status",
            string_value(enum_to_str(&project.health_status)),
        ),
        (
            "last_health_check",
            nullable_timestamp_value(project.last_health_check),
        ),
    ])
}

pub fn article_meta_fields(article: &ArticleMeta) -> Map<String, Value> {
    fields([
        ("id", string_value(&article.id)),
        ("title", string_value(&article.title)),
        ("description", string_value(&article.description)),
        ("date", timestamp_value(article.date)),
        ("word_count", int_value(article.word_count)),
        ("image_count", int_value(article.image_count)),
        ("heading_count", int_value(article.heading_count)),
        ("views", int_value(article.views)),
        ("is_published", bool_value(article.is_published)),
    ])
}

pub fn article_content_fields(article: &ArticleContent) -> Map<String, Value> {
    fields([
        ("id", string_value(&article.id)),
        ("abstract", string_value(&article.abstract_markdown)),
        (
            "abstract_markdown",
            string_value(&article.abstract_markdown),
        ),
        ("content", string_value(&article.content)),
    ])
}

fn analytics_event_fields(event: &AnalyticsEvent) -> Map<String, Value> {
    fields([
        ("id", string_value(&event.id)),
        ("article_id", string_value(&event.article_id)),
        ("timestamp", timestamp_value(event.timestamp)),
        ("ip_hash", string_value(&event.ip_hash)),
    ])
}

fn fields<const N: usize>(values: [(&'static str, Value); N]) -> Map<String, Value> {
    values
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect()
}

fn string_value(value: &str) -> Value {
    json!({ "stringValue": value })
}

fn nullable_string_value(value: Option<&str>) -> Value {
    value
        .map(string_value)
        .unwrap_or_else(|| json!({ "nullValue": null }))
}

fn int_value(value: u64) -> Value {
    json!({ "integerValue": value.to_string() })
}

fn bool_value(value: bool) -> Value {
    json!({ "booleanValue": value })
}

fn timestamp_value(value: DateTime<Utc>) -> Value {
    json!({ "timestampValue": value.to_rfc3339() })
}

fn nullable_timestamp_value(value: Option<DateTime<Utc>>) -> Value {
    value
        .map(timestamp_value)
        .unwrap_or_else(|| json!({ "nullValue": null }))
}

fn enum_to_str<T: serde::Serialize>(value: &T) -> &'static str {
    match serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_owned))
        .as_deref()
    {
        Some("url") => "url",
        Some("raw") => "raw",
        Some("healthy") => "healthy",
        Some("broken") => "broken",
        _ => "unknown",
    }
}

fn required_str(doc: &Document, key: &str) -> Result<String, AppError> {
    str_field(doc, key).ok_or_else(|| AppError::upstream(format!("missing field {key}")))
}

fn str_field(doc: &Document, key: &str) -> Option<String> {
    doc.fields
        .get(key)?
        .get("stringValue")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
}

fn int_field(doc: &Document, key: &str) -> u64 {
    doc.fields
        .get(key)
        .and_then(|value| value.get("integerValue"))
        .and_then(Value::as_str)
        .and_then(|value| value.parse().ok())
        .unwrap_or(0)
}

fn bool_field(doc: &Document, key: &str) -> bool {
    doc.fields
        .get(key)
        .and_then(|value| value.get("booleanValue"))
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn timestamp_field(doc: &Document, key: &str) -> Option<DateTime<Utc>> {
    doc.fields
        .get(key)?
        .get("timestampValue")
        .and_then(Value::as_str)
        .and_then(|value| value.parse().ok())
}

fn enum_field<T>(doc: &Document, key: &str, fallback: T) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    match str_field(doc, key) {
        Some(value) => serde_json::from_value(Value::String(value)).map_err(AppError::upstream),
        None => Ok(fallback),
    }
}
