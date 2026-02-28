use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};

use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    Internal(String),
    Validation(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub success: bool,
    pub error: String,
}

pub type ApiResult<T> = Result<T, AppError>;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Internal(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            AppError::Validation(message) => (StatusCode::BAD_REQUEST, message),
            AppError::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
            AppError::Forbidden(message) => (StatusCode::FORBIDDEN, message),
            AppError::Conflict(message) => (StatusCode::CONFLICT, message),
        };

        let body = Json(ErrorBody {
            success: false,
            error: message,
        });

        (status, body).into_response()
    }
}