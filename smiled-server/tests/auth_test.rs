/// Integration tests for the authentication system.
///
/// Prerequisites:
/// - PostgreSQL running on localhost:5433 (docker compose up -d db)
/// - DATABASE_URL env var set (or smiled-server/.env file)
/// - Migrations applied (sqlx migrate run)
///
/// Run with: cargo test --test auth_test
use axum::http::{HeaderValue, StatusCode};
use axum_test::TestServer;
use serde_json::{json, Value};
use sqlx::PgPool;
use uuid::Uuid;

mod common {
    use axum::{
        http::{header, Method},
        routing::get,
        Json, Router,
    };
    use axum_test::TestServer;
    use serde::Serialize;
    use sqlx::PgPool;
    use tower_http::cors::{Any, CorsLayer};
    use uuid::Uuid;

    use smiled_server::{
        api::v1,
        auth::password::hash_password,
        config::Config,
        db::pool::create_pool,
        state::AppState,
    };

    /// Create a test database pool using the DATABASE_URL env var.
    pub async fn test_pool() -> PgPool {
        dotenvy::dotenv().ok();
        let url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for integration tests");
        create_pool(&url).await.expect("Failed to connect to test database")
    }

    /// Build the full application router wired to `state`.
    pub fn build_app(state: AppState) -> Router {
        let cors = CorsLayer::new().allow_origin(Any);
        Router::new()
            .merge(v1::router())
            .with_state(state)
            .layer(cors)
    }

    /// Create a test cabinet and return its id.
    pub async fn create_test_cabinet(pool: &PgPool) -> Uuid {
        let id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO cabinet (id, nom) VALUES ($1, $2)",
            id,
            "Cabinet Test",
        )
        .execute(pool)
        .await
        .expect("Failed to create test cabinet");
        id
    }

    /// Create a test user and return their id.
    pub async fn create_test_user(
        pool: &PgPool,
        cabinet_id: Uuid,
        email: &str,
        password: &str,
        role: &str,
    ) -> Uuid {
        let id = Uuid::new_v4();
        let hash = hash_password(password).expect("Failed to hash password");

        sqlx::query!(
            r#"
            INSERT INTO utilisateur (id, cabinet_id, role, nom, prenom, email, password_hash)
            VALUES ($1, $2, $3, 'Test', 'User', $4, $5)
            "#,
            id,
            cabinet_id,
            role,
            email,
            hash,
        )
        .execute(pool)
        .await
        .expect("Failed to create test user");

        id
    }

    /// Clean up test data by deleting the cabinet (cascades to users via FK).
    pub async fn cleanup(pool: &PgPool, cabinet_id: Uuid) {
        // Clean up password reset tokens for users in this cabinet
        sqlx::query!(
            "DELETE FROM password_reset_token WHERE user_id IN (SELECT id FROM utilisateur WHERE cabinet_id = $1)",
            cabinet_id,
        )
        .execute(pool)
        .await
        .ok();

        sqlx::query!("DELETE FROM utilisateur WHERE cabinet_id = $1", cabinet_id)
            .execute(pool)
            .await
            .ok();

        sqlx::query!("DELETE FROM cabinet WHERE id = $1", cabinet_id)
            .execute(pool)
            .await
            .ok();
    }

    /// Build a minimal test config.
    pub fn test_config() -> Config {
        dotenvy::dotenv().ok();
        Config::from_env().expect("Config must be valid for integration tests")
    }
}

// ─── Login Tests ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn login_with_valid_credentials_returns_tokens() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("valid_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "S3cur3P@ss!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = TestServer::new(common::build_app(state)).unwrap();

    let response = server
        .post("/api/v1/auth/login")
        .json(&json!({ "email": email, "password": "S3cur3P@ss!" }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["access_token"].is_string(), "access_token missing");
    assert!(body["refresh_token"].is_string(), "refresh_token missing");
    assert_eq!(body["user"]["email"], email);
    assert_eq!(body["user"]["role"], "titulaire");

    common::cleanup(&pool, cabinet_id).await;
}

#[tokio::test]
async fn login_with_wrong_password_returns_401() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("wrong_pw_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "correct_pw", "secretaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = TestServer::new(common::build_app(state)).unwrap();

    let response = server
        .post("/api/v1/auth/login")
        .json(&json!({ "email": email, "password": "wrong_pw" }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);

    common::cleanup(&pool, cabinet_id).await;
}

#[tokio::test]
async fn login_with_unknown_email_returns_401() {
    let pool = common::test_pool().await;
    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = TestServer::new(common::build_app(state)).unwrap();

    let response = server
        .post("/api/v1/auth/login")
        .json(&json!({ "email": "nobody@test.smiled.io", "password": "any" }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

// ─── /me Tests ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn me_with_valid_token_returns_profile() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("me_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ssword!", "associe").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = TestServer::new(common::build_app(state)).unwrap();

    // Get a token
    let login_resp: Value = server
        .post("/api/v1/auth/login")
        .json(&json!({ "email": email, "password": "P@ssword!" }))
        .await
        .json();

    let token = login_resp["access_token"].as_str().unwrap();

    // Use token to call /me
    let me_resp = server
        .get("/api/v1/me")
        .add_header(
            axum::http::header::AUTHORIZATION,
            format!("Bearer {token}").parse::<HeaderValue>().unwrap(),
        )
        .await;

    assert_eq!(me_resp.status_code(), StatusCode::OK);

    let me_body: Value = me_resp.json();
    assert_eq!(me_body["email"], email);
    assert_eq!(me_body["role"], "associe");

    common::cleanup(&pool, cabinet_id).await;
}

#[tokio::test]
async fn me_without_token_returns_401() {
    let pool = common::test_pool().await;
    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = TestServer::new(common::build_app(state)).unwrap();

    let response = server.get("/api/v1/me").await;
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn me_with_invalid_token_returns_401() {
    let pool = common::test_pool().await;
    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = TestServer::new(common::build_app(state)).unwrap();

    let response = server
        .get("/api/v1/me")
        .add_header(
            axum::http::header::AUTHORIZATION,
            "Bearer not.a.valid.jwt".parse::<HeaderValue>().unwrap(),
        )
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

// ─── Refresh Tests ────────────────────────────────────────────────────────────

#[tokio::test]
async fn refresh_with_valid_token_returns_new_tokens() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("refresh_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "Refr3sh!", "collaborateur").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = TestServer::new(common::build_app(state)).unwrap();

    // Login first
    let login_resp: Value = server
        .post("/api/v1/auth/login")
        .json(&json!({ "email": email, "password": "Refr3sh!" }))
        .await
        .json();

    let refresh_token = login_resp["refresh_token"].as_str().unwrap();

    // Refresh
    let refresh_resp = server
        .post("/api/v1/auth/refresh")
        .json(&json!({ "refresh_token": refresh_token }))
        .await;

    assert_eq!(refresh_resp.status_code(), StatusCode::OK);

    let body: Value = refresh_resp.json();
    assert!(body["access_token"].is_string());
    assert!(body["refresh_token"].is_string());

    common::cleanup(&pool, cabinet_id).await;
}

#[tokio::test]
async fn refresh_with_access_token_returns_401() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("refresh_at_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "Refr3sh!", "secretaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = TestServer::new(common::build_app(state)).unwrap();

    let login_resp: Value = server
        .post("/api/v1/auth/login")
        .json(&json!({ "email": email, "password": "Refr3sh!" }))
        .await
        .json();

    // Deliberately pass access_token as refresh_token
    let access_token = login_resp["access_token"].as_str().unwrap();

    let response = server
        .post("/api/v1/auth/refresh")
        .json(&json!({ "refresh_token": access_token }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── JWT unit tests ───────────────────────────────────────────────────────────

#[tokio::test]
async fn jwt_create_and_validate_roundtrip() {
    use smiled_server::auth::jwt::{create_token_pair, validate_token};

    let user_id = Uuid::new_v4();
    let cabinet_id = Uuid::new_v4();
    let secret = "test_jwt_secret_for_dev_only";

    let pair = create_token_pair(user_id, cabinet_id, "titulaire", secret, 24).unwrap();

    let data = validate_token(&pair.access_token, secret).unwrap();
    assert_eq!(data.claims.sub, user_id.to_string());
    assert_eq!(data.claims.cabinet_id, cabinet_id.to_string());
    assert_eq!(data.claims.role, "titulaire");
    assert_eq!(data.claims.token_type, "access");
}
