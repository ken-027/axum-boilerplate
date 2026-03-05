use crate::repositories::user::{AuthResponse, CreateUserRequest, LoginRequest, UserResponse};
use crate::repositories::user::UserRepository;
use crate::utils::{jwt::JwtService, password, validation, AppError, Result};
use axum::{extract::State, http::StatusCode, response::Json, routing::post, Router};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
}

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    validation::validate_request(&payload)?;

    let user_repo = UserRepository::new(pool.clone());

    if user_repo.email_exists(&payload.email).await? {
        return Err(AppError::Conflict("Email already exists".to_string()));
    }

    if user_repo.username_exists(&payload.username).await? {
        return Err(AppError::Conflict("Username already exists".to_string()));
    }

    let password_hash = password::hash_password(&payload.password)?;

    let user = user_repo
        .create(&payload.email, &payload.username, &password_hash)
        .await?;

    let jwt_service = JwtService::new("your-secret-key", 24);
    let token = jwt_service.create_token(user.id, &user.email)?;

    let response = AuthResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    validation::validate_request(&payload)?;

    let user_repo = UserRepository::new(pool);

    let user = user_repo
        .find_by_email(&payload.email)
        .await?
        .ok_or_else(|| AppError::Authentication("Invalid credentials".to_string()))?;

    if !password::verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Authentication("Invalid credentials".to_string()));
    }

    let jwt_service = JwtService::new("your-secret-key", 24);
    let token = jwt_service.create_token(user.id, &user.email)?;

    let response = AuthResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok(Json(response))
}