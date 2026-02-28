use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn generate_token(
    user_id: i64,
    email: &str,
    secret: &str,
    ttl_minutes: i64,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expires_at = now + Duration::minutes(ttl_minutes);

    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        iat: now.timestamp(),
        exp: expires_at.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes())
    )
}

pub fn verify_token(
    token: &str,
    secret: &str,
) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}

