pub mod models;

use axum::{
    async_trait,
    extract::{State, Json, FromRequestParts},
    response::IntoResponse,
    http::{StatusCode, request::Parts},
};
use sqlx::PgPool;
use models::{User, RegisterRequest, LoginRequest, AuthResponse, Claims};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use uuid::Uuid;
use std::env;
use chrono::{Utc, Duration};

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = if let Some(auth_header) = parts.headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
        {
            if !auth_header.starts_with("Bearer ") {
                return Err((StatusCode::UNAUTHORIZED, "Invalid Authorization header".to_string()));
            }
            auth_header[7..].to_string()
        } else {
            // Try to get token from query parameter
            parts.uri.query()
                .and_then(|q| q.split('&').find(|p| p.starts_with("token=")))
                .map(|p| p[6..].to_string())
                .ok_or((StatusCode::UNAUTHORIZED, "Missing authentication".to_string()))?
        };

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        ).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        Ok(token_data.claims)
    }
}

pub async fn register_user(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    // 1. Check if domain exists, if not create it (simplified for now)
    let domain = sqlx::query!(
        "INSERT INTO domains (name) VALUES ($1) ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name RETURNING id",
        payload.domain_name
    )
    .fetch_one(&pool)
    .await;

    let domain_id = match domain {
        Ok(d) => d.id,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // 2. Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    // 3. Create user
    let result = sqlx::query!(
        "INSERT INTO users (email, password_hash, domain_id) VALUES ($1, $2, $3)",
        payload.email,
        password_hash,
        domain_id
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn login_user(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    // 1. Find user
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, domain_id, created_at FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await;

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    // 2. Verify password
    let parsed_hash = PasswordHash::new(&user.password_hash).expect("Invalid password hash in DB");
    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        return StatusCode::UNAUTHORIZED.into_response();
    }

    // 3. Generate JWT
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id,
        email: user.email,
        exp: expiration as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).expect("Failed to generate token");

    Json(AuthResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
    }).into_response()
}
