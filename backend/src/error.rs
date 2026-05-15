use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("configuration error: {message}")]
    Configuration { message: String },
    #[error("bad request: {message}")]
    BadRequest { message: String },
    #[error("unauthorized")]
    Unauthorized,
    #[error("not found")]
    NotFound,
    #[error("upstream error")]
    Upstream,
    #[error("internal error")]
    Internal,
}

impl AppError {
    pub fn configuration(message: impl Into<String>) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest {
            message: message.into(),
        }
    }

    pub fn upstream(err: impl std::fmt::Display) -> Self {
        tracing::error!("upstream service error: {err}");
        Self::Upstream
    }

    pub fn internal(err: impl std::fmt::Display) -> Self {
        tracing::error!("internal service error: {err}");
        Self::Internal
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Configuration { .. } | AppError::Internal => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            AppError::BadRequest { ref message } => (StatusCode::BAD_REQUEST, message.as_str()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found"),
            AppError::Upstream => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
