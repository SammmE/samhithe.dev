use std::{env, fmt, path::PathBuf};

use axum::http::{HeaderValue, Method};
use tower_http::cors::{AllowOrigin, CorsLayer};

use crate::{error::AppError, service_account::ServiceAccountCredentials};

#[derive(Clone)]
pub struct Config {
    pub bind_host: String,
    pub port: u16,
    pub firebase_project_id: String,
    pub firestore_database: String,
    pub service_account: Option<ServiceAccountCredentials>,
    pub firestore_emulator_host: Option<String>,
    pub allowed_origins: Vec<String>,
    pub link_health_interval_secs: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        let firestore_emulator_host = env::var("FIRESTORE_EMULATOR_HOST").ok();
        let service_account = if firestore_emulator_host.is_some() {
            service_account_from_env()?
        } else {
            Some(service_account_from_env()?.ok_or_else(|| {
                AppError::configuration(
                    "set GOOGLE_APPLICATION_CREDENTIALS or FIREBASE_ADMINSDK_JSON to your firebase-adminsdk JSON file",
                )
            })?)
        };
        let firebase_project_id = service_account
            .as_ref()
            .map(|credentials| credentials.project_id.clone())
            .or_else(|| env::var("FIREBASE_PROJECT_ID").ok())
            .ok_or_else(|| {
                AppError::configuration(
                    "FIREBASE_PROJECT_ID is required when no service account JSON is configured",
                )
            })?;

        Ok(Self {
            bind_host: env::var("BIND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|port| port.parse().ok())
                .unwrap_or(3000),
            firebase_project_id,
            firestore_database: env::var("FIRESTORE_DATABASE")
                .unwrap_or_else(|_| "(default)".to_string()),
            service_account,
            firestore_emulator_host,
            allowed_origins: env::var("ALLOWED_ORIGINS")
                .unwrap_or_else(|_| {
                    "http://localhost:3000,http://localhost:3001,http://localhost:5173".to_string()
                })
                .split(',')
                .map(str::trim)
                .map(|origin| origin.trim_end_matches('/'))
                .filter(|origin| !origin.is_empty())
                .map(ToOwned::to_owned)
                .collect(),
            link_health_interval_secs: env::var("LINK_HEALTH_INTERVAL_SECS")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(7 * 24 * 60 * 60),
        })
    }

    pub fn cors_layer(&self) -> Result<CorsLayer, AppError> {
        let origins = self
            .allowed_origins
            .iter()
            .map(|origin| HeaderValue::from_str(origin))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| {
                AppError::configuration(format!("invalid ALLOWED_ORIGINS value: {err}"))
            })?;

        Ok(CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins))
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::OPTIONS])
            .allow_headers([
                axum::http::header::AUTHORIZATION,
                axum::http::header::CONTENT_TYPE,
            ]))
    }
}

fn service_account_from_env() -> Result<Option<ServiceAccountCredentials>, AppError> {
    let path = env_value("FIREBASE_ADMINSDK_JSON")
        .or_else(|| env_value("GOOGLE_APPLICATION_CREDENTIALS"))
        .map(PathBuf::from)
        .or_else(default_service_account_path);

    path.map(ServiceAccountCredentials::from_file).transpose()
}

fn env_value(key: &'static str) -> Option<String> {
    env::var(key)
        .ok()
        .map(|value| {
            value
                .trim()
                .trim_matches('"')
                .trim_matches('\'')
                .to_string()
        })
        .filter(|value| !value.is_empty())
}

fn default_service_account_path() -> Option<PathBuf> {
    [
        "firebase-adminsdk.json",
        "service-account.json",
        "serviceAccountKey.json",
    ]
    .into_iter()
    .map(PathBuf::from)
    .find(|path| path.exists())
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "project={}, database={}, origins={}",
            self.firebase_project_id,
            self.firestore_database,
            self.allowed_origins.join("|")
        )
    }
}
