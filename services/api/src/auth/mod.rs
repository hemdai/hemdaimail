pub mod models;

use axum::{
    async_trait,
    extract::{State, Json, FromRequestParts},
    response::IntoResponse,
    http::{StatusCode, request::Parts},
};
use axum_extra::extract::cookie::{Cookie, CookieJar};
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
use rand::{distributions::Alphanumeric, Rng};
use crate::db::audit;
use axum::http::HeaderMap;

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
    headers: HeaderMap,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
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

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    let result = sqlx::query!(
        "INSERT INTO users (email, password_hash, domain_id) VALUES ($1, $2, $3) RETURNING id",
        payload.email,
        password_hash,
        domain_id
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(user) => {
            audit::log_event(
                &pool,
                Some(user.id),
                "user_registered",
                "user",
                Some(&user.id.to_string()),
                None,
                headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()),
                headers.get("user-agent").and_then(|v| v.to_str().ok()),
            ).await;
            StatusCode::CREATED.into_response()
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

pub async fn login_user(
    State(pool): State<PgPool>,
    jar: CookieJar,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = sqlx::query_as!(
        User,
        "SELECT id, email, password_hash, domain_id, created_at FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await;

    let user = match user {
        Ok(Some(u)) => u,
        Ok(None) => return (StatusCode::UNAUTHORIZED, jar, "Invalid credentials").into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, jar, "Database error").into_response(),
    };

    let parsed_hash = PasswordHash::new(&user.password_hash).expect("Invalid password hash in DB");
    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        audit::log_event(
            &pool,
            Some(user.id),
            "login_failed",
            "user",
            Some(&user.id.to_string()),
            None,
            headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()),
            headers.get("user-agent").and_then(|v| v.to_str().ok()),
        ).await;
        return (StatusCode::UNAUTHORIZED, jar, "Invalid credentials").into_response();
    }

    audit::log_event(
        &pool,
        Some(user.id),
        "user_logged_in",
        "user",
        Some(&user.id.to_string()),
        None,
        headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()),
        headers.get("user-agent").and_then(|v| v.to_str().ok()),
    ).await;

    let access_token = generate_access_token(user.id, &user.email);
    let refresh_token = generate_refresh_token();
    
    let expires_at = Utc::now() + Duration::days(7);
    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES ($1, $2, $3)",
        user.id,
        refresh_token,
        expires_at
    )
    .execute(&pool)
    .await
    .expect("Failed to save refresh token");

    let jar = jar.add(
        Cookie::build(("refresh_token", refresh_token))
            .path("/")
            .http_only(true)
            .secure(true) // Should be true in production
            .same_site(axum_extra::extract::cookie::SameSite::Strict)
            .expires(Some(std::time::SystemTime::from(expires_at).into()))
            .build(),
    );

    (jar, Json(AuthResponse {
        access_token,
        token_type: "Bearer".to_string(),
    })).into_response()
}

pub async fn refresh_token(
    State(pool): State<PgPool>,
    jar: CookieJar,
) -> impl IntoResponse {
    let token = match jar.get("refresh_token") {
        Some(cookie) => cookie.value(),
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    let row = sqlx::query!(
        "SELECT user_id, email FROM refresh_tokens JOIN users ON users.id = user_id WHERE token = $1 AND expires_at > NOW()",
        token
    )
    .fetch_optional(&pool)
    .await;

    match row {
        Ok(Some(row)) => {
            let access_token = generate_access_token(row.user_id, &row.email);
            Json(AuthResponse {
                access_token,
                token_type: "Bearer".to_string(),
            }).into_response()
        }
        _ => StatusCode::UNAUTHORIZED.into_response(),
    }
}

fn generate_access_token(user_id: Uuid, email: &str) -> String {
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).expect("Failed to generate token")
}

fn generate_refresh_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}
