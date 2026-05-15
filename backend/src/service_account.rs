use std::{path::Path, sync::Arc};

use chrono::Utc;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::error::AppError;

const FIRESTORE_SCOPE: &str = "https://www.googleapis.com/auth/datastore";

#[derive(Clone, Deserialize)]
pub struct ServiceAccountCredentials {
    pub project_id: String,
    pub private_key: String,
    pub client_email: String,
    #[serde(default = "default_token_uri")]
    pub token_uri: String,
}

impl ServiceAccountCredentials {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, AppError> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path).map_err(|err| {
            AppError::configuration(format!(
                "failed to read service account JSON at {}: {err}",
                path.display()
            ))
        })?;
        serde_json::from_str(&contents).map_err(|err| {
            AppError::configuration(format!(
                "failed to parse service account JSON at {}: {err}",
                path.display()
            ))
        })
    }
}

#[derive(Clone)]
pub struct ServiceAccountTokenProvider {
    credentials: Arc<ServiceAccountCredentials>,
    http: Client,
    cached: Arc<Mutex<Option<CachedToken>>>,
}

impl ServiceAccountTokenProvider {
    pub fn new(credentials: ServiceAccountCredentials) -> Self {
        Self {
            credentials: Arc::new(credentials),
            http: Client::new(),
            cached: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn access_token(&self) -> Result<String, AppError> {
        let mut cached = self.cached.lock().await;
        let now = Utc::now().timestamp();

        if let Some(token) = cached.as_ref() {
            if token.expires_at > now + 60 {
                return Ok(token.access_token.clone());
            }
        }

        let token = self.fetch_access_token(now).await?;
        *cached = Some(token.clone());
        Ok(token.access_token)
    }

    async fn fetch_access_token(&self, now: i64) -> Result<CachedToken, AppError> {
        let claims = JwtClaims {
            iss: &self.credentials.client_email,
            scope: FIRESTORE_SCOPE,
            aud: &self.credentials.token_uri,
            iat: now,
            exp: now + 3600,
        };
        let assertion = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(self.credentials.private_key.as_bytes())
                .map_err(AppError::internal)?,
        )
        .map_err(AppError::internal)?;

        let response = self
            .http
            .post(&self.credentials.token_uri)
            .header(
                reqwest::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .body(
                serde_urlencoded::to_string([
                    ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                    ("assertion", assertion.as_str()),
                ])
                .map_err(AppError::internal)?,
            )
            .send()
            .await
            .map_err(AppError::upstream)?;

        let status = response.status();
        let body = response.text().await.map_err(AppError::upstream)?;
        if !status.is_success() {
            tracing::error!(status = %status, body = %body, "service account token exchange failed");
            return Err(AppError::Upstream);
        }

        let token: TokenResponse = serde_json::from_str(&body).map_err(AppError::upstream)?;
        Ok(CachedToken {
            access_token: token.access_token,
            expires_at: now + token.expires_in,
        })
    }
}

#[derive(Clone)]
struct CachedToken {
    access_token: String,
    expires_at: i64,
}

#[derive(Serialize)]
struct JwtClaims<'a> {
    iss: &'a str,
    scope: &'a str,
    aud: &'a str,
    iat: i64,
    exp: i64,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
}

fn default_token_uri() -> String {
    "https://oauth2.googleapis.com/token".to_string()
}
