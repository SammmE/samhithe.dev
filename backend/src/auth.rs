use std::sync::Arc;

use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use serde::Deserialize;

use crate::{error::AppError, state::AppState};

#[derive(Debug, Deserialize)]
struct FirebaseClaims {
    aud: String,
    iss: String,
    sub: String,
}

pub async fn require_firebase_auth(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;

    let claims = verify_firebase_token(&state, token).await?;
    request.extensions_mut().insert(claims.sub);

    Ok(next.run(request).await)
}

async fn verify_firebase_token(state: &AppState, token: &str) -> Result<FirebaseClaims, AppError> {
    let header = decode_header(token).map_err(|_| AppError::Unauthorized)?;
    let kid = header.kid.ok_or(AppError::Unauthorized)?;

    let certs: serde_json::Value = state
        .http
        .get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com")
        .send()
        .await
        .map_err(AppError::upstream)?
        .json()
        .await
        .map_err(AppError::upstream)?;

    let pem = certs
        .get(&kid)
        .and_then(serde_json::Value::as_str)
        .ok_or(AppError::Unauthorized)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[state.config.firebase_project_id.as_str()]);
    validation.set_issuer(&[format!(
        "https://securetoken.google.com/{}",
        state.config.firebase_project_id
    )]);

    let data = decode::<FirebaseClaims>(
        token,
        &DecodingKey::from_rsa_pem(pem.as_bytes()).map_err(|_| AppError::Unauthorized)?,
        &validation,
    )
    .map_err(|_| AppError::Unauthorized)?;

    if data.claims.sub.is_empty()
        || data.claims.aud != state.config.firebase_project_id
        || data.claims.iss
            != format!(
                "https://securetoken.google.com/{}",
                state.config.firebase_project_id
            )
    {
        return Err(AppError::Unauthorized);
    }

    Ok(data.claims)
}
