/// Integration tests for the Reference Data CRUD API.
///
/// Prerequisites:
/// - PostgreSQL running on localhost:5433 (docker compose up -d db)
/// - DATABASE_URL env var set (or smiled-server/.env file)
/// - Migrations applied (sqlx migrate run) — seed data from 007-009 required
///
/// Run with: cargo test --test reference_test
use axum::http::{HeaderValue, StatusCode};
use serde_json::{json, Value};
use uuid::Uuid;

// ─── Common Test Helpers ──────────────────────────────────────────────────────

mod common {
    use axum::Router;
    use axum_test::TestServer;
    use serde_json::Value;
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

    pub async fn test_pool() -> PgPool {
        dotenvy::dotenv().ok();
        let url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set for integration tests");
        create_pool(&url).await.expect("Failed to connect to test database")
    }

    pub fn build_app(state: AppState) -> Router {
        let cors = CorsLayer::new().allow_origin(Any);
        Router::new()
            .merge(v1::router())
            .with_state(state)
            .layer(cors)
    }

    pub async fn create_test_cabinet(pool: &PgPool) -> Uuid {
        let id = Uuid::new_v4();
        sqlx::query!(
            "INSERT INTO cabinet (id, nom) VALUES ($1, $2)",
            id,
            "Cabinet Test Reference",
        )
        .execute(pool)
        .await
        .expect("Failed to create test cabinet");
        id
    }

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

    pub async fn cleanup(pool: &PgPool, cabinet_id: Uuid) {
        // Delete tarif_cabinet entries for the test cabinet
        sqlx::query!(
            "DELETE FROM tarif_cabinet WHERE cabinet_id = $1",
            cabinet_id
        )
        .execute(pool)
        .await
        .ok();

        // Delete cabinet-level actes
        sqlx::query!(
            "DELETE FROM acte WHERE cabinet_id = $1",
            cabinet_id
        )
        .execute(pool)
        .await
        .ok();

        // Delete cabinet-level materiaux
        sqlx::query!(
            "DELETE FROM materiau WHERE cabinet_id = $1",
            cabinet_id
        )
        .execute(pool)
        .await
        .ok();

        // Delete cabinet-level teintes
        sqlx::query!(
            "DELETE FROM teinte WHERE cabinet_id = $1",
            cabinet_id
        )
        .execute(pool)
        .await
        .ok();

        // Delete cabinet-level motifs
        sqlx::query!(
            "DELETE FROM motif_consultation WHERE cabinet_id = $1",
            cabinet_id
        )
        .execute(pool)
        .await
        .ok();

        // Delete utilisateurs
        sqlx::query!(
            "DELETE FROM password_reset_token WHERE user_id IN (SELECT id FROM utilisateur WHERE cabinet_id = $1)",
            cabinet_id
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

    pub fn test_config() -> Config {
        dotenvy::dotenv().ok();
        Config::from_env().expect("Config must be valid for integration tests")
    }

    /// Login and return the access token.
    pub async fn login(server: &TestServer, email: &str, password: &str) -> String {
        let resp: Value = server
            .post("/api/v1/auth/login")
            .json(&serde_json::json!({ "email": email, "password": password }))
            .await
            .json();
        resp["access_token"]
            .as_str()
            .expect("access_token missing in login response")
            .to_string()
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn auth_header(token: &str) -> HeaderValue {
    format!("Bearer {token}").parse().unwrap()
}

// ─── GET /actes — Seed data ───────────────────────────────────────────────────

#[tokio::test]
async fn list_actes_returns_seed_data() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("ref_acte_list_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    let response = server
        .get("/api/v1/actes")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());

    let actes = body["data"].as_array().unwrap();
    assert!(
        actes.len() > 100,
        "Expected > 100 seed actes, got {}",
        actes.len()
    );

    // Verify structure of first acte
    let first = &actes[0];
    assert!(first["id"].is_string());
    assert!(first["libelle"].is_string());
    assert!(first["nomenclature"].is_string());
    assert!(first["prix_defaut"].is_number());

    common::cleanup(&pool, cabinet_id).await;
}

// ─── POST /actes — Create cabinet acte ───────────────────────────────────────

#[tokio::test]
async fn create_custom_acte_returns_201() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("ref_acte_create_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    // Use a known seed category id (CONS category from migration 007)
    let response = server
        .post("/api/v1/actes")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "categorie_id": "c1000000-0000-0000-0000-000000000001",
            "nomenclature": "hors_nomenclature",
            "libelle": "Consultation cabinet spéciale test",
            "prix_defaut": 95.0,
            "panier_rac": "libre"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["libelle"], "Consultation cabinet spéciale test");
    assert_eq!(body["data"]["niveau"], "cabinet");
    assert_eq!(body["data"]["prix_defaut"], 95.0);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── PUT /actes/:id/tarif — Override system acte price ───────────────────────

#[tokio::test]
async fn override_tarif_on_system_acte() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("ref_tarif_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    // Get a system acte id from the list
    let list_body: Value = server
        .get("/api/v1/actes")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await
        .json();

    let actes = list_body["data"].as_array().unwrap();
    let system_acte = actes
        .iter()
        .find(|a| a["niveau"] == "systeme")
        .expect("No system acte found in seed data");
    let acte_id = system_acte["id"].as_str().unwrap();

    // Override the tarif
    let response = server
        .put(&format!("/api/v1/actes/{acte_id}/tarif"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "prix": 42.50 }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());

    // Verify the override appears in the list
    let list_after: Value = server
        .get("/api/v1/actes")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await
        .json();

    let overridden = list_after["data"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["id"] == acte_id)
        .expect("Acte not found after tarif override");

    assert_eq!(
        overridden["prix_cabinet"].as_f64().unwrap(),
        42.50,
        "prix_cabinet should be the overridden value"
    );

    common::cleanup(&pool, cabinet_id).await;
}

// ─── PUT /actes/:id/toggle — Toggle cabinet acte ─────────────────────────────

#[tokio::test]
async fn toggle_cabinet_acte_flips_actif() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("ref_toggle_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    // Create a cabinet acte first
    let created: Value = server
        .post("/api/v1/actes")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "categorie_id": "c1000000-0000-0000-0000-000000000001",
            "nomenclature": "hors_nomenclature",
            "libelle": "Acte toggle test",
            "prix_defaut": 50.0
        }))
        .await
        .json();

    let acte_id = created["data"]["id"].as_str().unwrap();
    let initial_actif = created["data"]["actif"].as_bool().unwrap_or(true);

    // Toggle
    let response = server
        .put(&format!("/api/v1/actes/{acte_id}/toggle"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(
        body["data"]["actif"].as_bool().unwrap(),
        !initial_actif,
        "actif should be toggled"
    );

    common::cleanup(&pool, cabinet_id).await;
}

// ─── GET /materiaux — Returns seed with categories ────────────────────────────

#[tokio::test]
async fn list_materiaux_returns_with_categories() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("ref_mat_list_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    let response = server
        .get("/api/v1/materiaux")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());

    let materiaux = body["data"].as_array().unwrap();
    assert!(
        !materiaux.is_empty(),
        "Expected seed materiaux, got 0"
    );

    // Verify structure includes category info
    let first = &materiaux[0];
    assert!(first["id"].is_string());
    assert!(first["libelle"].is_string());
    assert!(first["categorie_materiau_id"].is_string());
    // categorie_code and categorie_libelle should be populated from JOIN
    assert!(first["categorie_libelle"].is_string());

    common::cleanup(&pool, cabinet_id).await;
}

// ─── GET /teintes — Verify seed data (VITA Classical + 3D-Master) ─────────────

#[tokio::test]
async fn list_teintes_returns_vita_seed_data() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("ref_teinte_list_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    let response = server
        .get("/api/v1/teintes")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());

    let teintes = body["data"].as_array().unwrap();
    assert!(
        teintes.len() >= 45,
        "Expected >= 45 seed teintes (VITA Classical + 3D-Master), got {}",
        teintes.len()
    );

    // Verify both VITA systems are present (seed uses underscores: VITA_Classical, VITA_3D_Master)
    let systemes: Vec<&str> = teintes
        .iter()
        .filter_map(|t| t["systeme"].as_str())
        .collect();

    assert!(
        systemes.iter().any(|s| s.contains("Classical")),
        "VITA Classical teintes missing — found: {:?}",
        {
            let unique: std::collections::HashSet<&str> = systemes.iter().copied().collect();
            unique
        }
    );
    assert!(
        systemes.iter().any(|s| s.contains("3D")),
        "VITA 3D-Master teintes missing"
    );

    common::cleanup(&pool, cabinet_id).await;
}

// ─── POST /teintes — Create custom shade ──────────────────────────────────────

#[tokio::test]
async fn create_custom_teinte_returns_201() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("ref_teinte_create_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    let response = server
        .post("/api/v1/teintes")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "systeme": "Custom Cabinet",
            "code": "CC-01",
            "libelle": "Teinte personnalisée test",
            "ordre": 99
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["code"], "CC-01");
    assert_eq!(body["data"]["systeme"], "Custom Cabinet");
    assert_eq!(body["data"]["niveau"], "cabinet");

    common::cleanup(&pool, cabinet_id).await;
}

// ─── GET /troubles — Returns seed data ───────────────────────────────────────

#[tokio::test]
async fn list_troubles_returns_seed_data() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("ref_trouble_list_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    let response = server
        .get("/api/v1/troubles")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"].is_array());

    common::cleanup(&pool, cabinet_id).await;
}

// ─── GET /motifs — Returns seed data ─────────────────────────────────────────

#[tokio::test]
async fn list_motifs_returns_seed_data() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("ref_motif_list_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    let response = server
        .get("/api/v1/motifs")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"].is_array());

    common::cleanup(&pool, cabinet_id).await;
}

// ─── Auth guard — 401 without token ──────────────────────────────────────────

#[tokio::test]
async fn reference_endpoints_require_auth() {
    let pool = common::test_pool().await;
    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    for path in ["/api/v1/actes", "/api/v1/materiaux", "/api/v1/teintes"] {
        let response = server.get(path).await;
        assert_eq!(
            response.status_code(),
            StatusCode::UNAUTHORIZED,
            "Expected 401 for {path} without auth"
        );
    }
}
