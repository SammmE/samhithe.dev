use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};

/// Middleware to require admin password for protected endpoints
pub async fn require_admin_password(
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let admin_password = std::env::var("ADMIN_PASSWORD").unwrap_or_default();

    match headers.get("x-admin-password") {
        Some(password) if password.to_str().unwrap_or("") == admin_password => {
            Ok(next.run(request).await)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}
