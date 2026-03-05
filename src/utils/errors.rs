use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("JWT error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("Bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("UUID error: {0}")]
    Uuid(#[from] uuid::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::Validation(ref message) => (StatusCode::BAD_REQUEST, message.as_str()),
            AppError::Authentication(ref message) => (StatusCode::UNAUTHORIZED, message.as_str()),
            AppError::Authorization(ref message) => (StatusCode::FORBIDDEN, message.as_str()),
            AppError::NotFound(ref message) => (StatusCode::NOT_FOUND, message.as_str()),
            AppError::Conflict(ref message) => (StatusCode::CONFLICT, message.as_str()),
            AppError::Internal(ref message) => {
                tracing::error!("Internal error: {}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
            AppError::Jwt(_) => (StatusCode::UNAUTHORIZED, "Invalid token"),
            AppError::Bcrypt(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            AppError::Parse(_) => (StatusCode::BAD_REQUEST, "Invalid input"),
            AppError::Uuid(_) => (StatusCode::BAD_REQUEST, "Invalid UUID format"),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16()
        }));

        (status, body).into_response()
    }
}