use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use crate::app::state::AppState;
use crate::features::auth::jwt::{verify_token, Claims};
use crate::http::error::AppError;

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: i64,
    pub email: String,
}

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let authorization = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("missing authorization header".to_string()))?;

        let token = authorization
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("invalid authorization scheme".to_string()))?;

        let claims = verify_token(token, &state.settings.auth.jwt_secret)
            .map_err(|_| AppError::Unauthorized("invalid or expired token".to_string()))?;

        Ok(map_claims_to_current_user(claims))
    }
}

fn map_claims_to_current_user(claims: Claims) -> CurrentUser {
    CurrentUser {
        id: claims.sub,
        email: claims.email
    }
}