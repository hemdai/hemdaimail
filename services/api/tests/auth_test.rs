use api;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use sqlx::PgPool;
use std::env;
use serde_json::json;

#[tokio::test]
async fn test_auth_flow() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.unwrap();

    // Clean up test user
    sqlx::query("DELETE FROM users WHERE email = $1").bind("test@example.com").execute(&pool).await.unwrap();
    sqlx::query("DELETE FROM domains WHERE name = $1").bind("example.com").execute(&pool).await.unwrap();

    let app = api::app(pool).await;

    // 1. Register
    let register_payload = json!({
        "email": "test@example.com",
        "password": "password123",
        "domain_name": "example.com"
    });

    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/register")
                .header("Content-Type", "application/json")
                .body(Body::from(register_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // 2. Login
    let login_payload = json!({
        "email": "test@example.com",
        "password": "password123"
    });

    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("Content-Type", "application/json")
                .body(Body::from(login_payload.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    // Verify we got a token
    let body = axum::body::to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(body["access_token"].is_string());
}
