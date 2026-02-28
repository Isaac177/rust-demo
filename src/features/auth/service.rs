use crate::{
    features::auth::{
        dto::{AuthResponse, LoginRequest, RegisterRequest}
        ,
        password::{hash_password, verify_password},
        repository::{create_user, find_user_by_email},
    },
    http::error::AppError
};

use crate::features::auth::jwt::generate_token;
use crate::features::auth::repository::UserAuthRecord;
use sqlx::PgPool;

const ACCESS_TOKEN_TTL_MINUTES: i64 = 60;

pub async fn register(pool:&PgPool, input: RegisterRequest, jwt_secret: &str) -> Result<AuthResponse, AppError> {
    validate_register_input(&input)?;

    let existing_user = find_user_by_email(pool, &input.email)
        .await
        .map_err(internal_error)?;

    if existing_user.is_some() {
        return Err(AppError::Conflict("email is already registered".to_string()));
    }

    let password_hash = hash_password(&input.password)
        .map_err(|err| AppError::Internal(format!("failed to hash password: {err}")))?;

    let user = create_user(pool, &input.email, &input.full_name, &password_hash)
        .await
        .map_err(internal_error)?;

    build_auth_response(&user, jwt_secret)
}

pub async fn login(
    pool: &PgPool,
    input: LoginRequest,
    jwt_secret: &str,
) -> Result<AuthResponse, AppError> {
    validate_login_input(&input)?;

    let user = find_user_by_email(pool, &input.email)
        .await
        .map_err(internal_error)?
        .ok_or_else(| | AppError::Unauthorized("invalid credentials".to_string()))?;

    if !user.is_active {
        return Err(AppError::Forbidden("user account is inactive".to_string()));
    }

    let password_is_valid = verify_password(&input.password, &user.password_hash)
        .map_err(|err| AppError::Internal(format!("failed to verify password: {err}")))?;

    if !password_is_valid {
        return Err(AppError::Unauthorized("invalid credentials".to_string()));
    }

    build_auth_response(&user, jwt_secret)
}

fn build_auth_response(user: &UserAuthRecord, jwt_secret: &str) -> Result<AuthResponse, AppError> {
    let access_token = generate_token(user.id, &user.email, jwt_secret, ACCESS_TOKEN_TTL_MINUTES)
        .map_err(|err| AppError::Internal(format!("failed to generate token: {err}")))?;

    Ok(AuthResponse {
        access_token,
        token_type: "Bearer",
    })
}

fn validate_register_input(input: &RegisterRequest) -> Result<(), AppError> {
    if input.email.trim().is_empty() {
        return Err(AppError::Validation("email is required".to_string()));
    }

    if input.full_name.trim().is_empty() {
        return Err(AppError::Validation("full_name is required".to_string()));
    }

    if input.password.len() < 8 {
        return Err(AppError::Validation(
            "password must be at least 8 characters".to_string(),
        ));
    }

    Ok(())
}

fn validate_login_input(input: &LoginRequest) -> Result<(), AppError> {
    if input.email.trim().is_empty() {
        return Err(AppError::Validation("password is required".to_string()));
    }

    Ok(())
}

fn internal_error(err: sqlx::Error) -> AppError {
    AppError::Internal(format!("database operation failed {err}"))
}