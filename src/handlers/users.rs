use crate::middleware::auth::AuthUser;
use crate::repositories::user::UserResponse;
use crate::repositories::user::UserRepository;
use crate::utils::{AppError, Result};
use axum::{
    extract::{Extension, Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get},
    Router,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/users", get(list_users))
        .route("/users/me", get(get_current_user))
        .route("/users/:id", get(get_user))
        .route("/users/:id", delete(delete_user))
}

pub async fn list_users(
    State(pool): State<PgPool>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<Vec<UserResponse>>> {
    let user_repo = UserRepository::new(pool);
    let users = user_repo
        .list_users(pagination.limit, pagination.offset)
        .await?;

    let user_responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    Ok(Json(user_responses))
}

pub async fn get_current_user(
    Extension(auth_user): Extension<AuthUser>,
    State(pool): State<PgPool>,
) -> Result<Json<UserResponse>> {
    let user_repo = UserRepository::new(pool);
    let user = user_repo
        .find_by_id(auth_user.id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(UserResponse::from(user)))
}

pub async fn get_user(
    Path(user_id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> Result<Json<UserResponse>> {
    let user_repo = UserRepository::new(pool);
    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(UserResponse::from(user)))
}

pub async fn delete_user(
    Path(user_id): Path<Uuid>,
    Extension(auth_user): Extension<AuthUser>,
    State(pool): State<PgPool>,
) -> Result<StatusCode> {
    if auth_user.id != user_id {
        return Err(AppError::Authorization(
            "You can only delete your own account".to_string(),
        ));
    }

    let user_repo = UserRepository::new(pool);
    let deleted = user_repo.delete_by_id(user_id).await?;

    if !deleted {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}