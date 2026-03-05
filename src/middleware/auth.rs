use crate::utils::{jwt::JwtService, AppError, Result};
use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub email: String,
}

pub async fn auth_middleware(
    State(jwt_service): State<Arc<JwtService>>,
    mut request: Request,
    next: Next,
) -> Result<Response> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or_else(|| AppError::Authentication("Missing authorization header".to_string()))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| AppError::Authentication("Invalid authorization header".to_string()))?;

    if !auth_str.starts_with("Bearer ") {
        return Err(AppError::Authentication(
            "Authorization header must start with 'Bearer '".to_string(),
        ));
    }

    let token = &auth_str[7..];
    let claims = jwt_service.validate_token(token)?;

    let auth_user = AuthUser {
        id: claims.user_id,
        email: claims.sub,
    };

    request.extensions_mut().insert(auth_user);

    Ok(next.run(request).await)
}