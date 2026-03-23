/// Integration tests for the Dental Schema API (versioning, teeth, paro, occlusion, ATM).
///
/// Prerequisites:
/// - PostgreSQL running on localhost:5433 (docker compose up -d db)
/// - DATABASE_URL env var set (or smiled-server/.env file)
/// - Migrations applied (sqlx migrate run)
///
/// Run with: cargo test --test schema_dentaire_test
use axum::http::{HeaderValue, StatusCode};
use serde_json::{json, Value};
use uuid::Uuid;

// ─── Common Test Helpers ──────────────────────────────────────────────────────

mod common {
    use axum::{http::HeaderValue, Router};
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
            "Cabinet Schema Test",
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
        // evenement_dent depends on patient and cabinet
        sqlx::query!(
            "DELETE FROM evenement_dent WHERE cabinet_id = $1",
            cabinet_id
        )
        .execute(pool)
        .await
        .ok();

        // schema_dentaire cascades to dent -> face_dent, paro_site, occlusion, atm, paro_global
        sqlx::query!(
            "DELETE FROM schema_dentaire WHERE cabinet_id = $1",
            cabinet_id
        )
        .execute(pool)
        .await
        .ok();

        sqlx::query!("DELETE FROM questionnaire_medical WHERE cabinet_id = $1", cabinet_id)
            .execute(pool)
            .await
            .ok();

        sqlx::query!("DELETE FROM patient WHERE cabinet_id = $1", cabinet_id)
            .execute(pool)
            .await
            .ok();

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

    pub fn test_config() -> Config {
        dotenvy::dotenv().ok();
        Config::from_env().expect("Config must be valid for integration tests")
    }

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

    pub fn sample_patient_body() -> Value {
        serde_json::json!({
            "nom": "Moulin",
            "prenom": "Claire",
            "sexe": "F",
            "date_naissance": "1990-05-20",
            "couverture": "mutuelle",
        })
    }

    /// Create a patient and return its ID string.
    pub async fn create_patient(server: &TestServer, token: &str) -> String {
        let created: Value = server
            .post("/api/v1/patients")
            .add_header(
                axum::http::header::AUTHORIZATION,
                auth_header(token),
            )
            .json(&sample_patient_body())
            .await
            .json();
        created["data"]["id"].as_str().unwrap().to_string()
    }

    pub fn auth_header(token: &str) -> HeaderValue {
        format!("Bearer {token}").parse().unwrap()
    }
}

fn auth_header(token: &str) -> HeaderValue {
    format!("Bearer {token}").parse().unwrap()
}

// ─── POST /patients/:id/schema — Create v1 ───────────────────────────────────

#[tokio::test]
async fn create_schema_returns_201_with_32_teeth() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("schema_create_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;
    let patient_id = common::create_patient(&server, &token).await;

    let response = server
        .post(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "dentition": "permanente" }))
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());

    // Schema metadata
    let schema = &body["data"]["schema"];
    assert_eq!(schema["version"], 1);
    assert_eq!(schema["dentition"], "permanente");
    assert_eq!(schema["patient_id"], patient_id);

    // 32 teeth for permanent dentition
    let dents = body["data"]["dents"].as_array().unwrap();
    assert_eq!(dents.len(), 32, "Expected 32 teeth for permanent dentition");

    // Each tooth has 6 faces and 6 paro sites
    for tooth_entry in dents {
        let faces = tooth_entry["faces"].as_array().unwrap();
        let paro_sites = tooth_entry["paro_sites"].as_array().unwrap();
        assert_eq!(faces.len(), 6, "Each tooth should have 6 faces");
        assert_eq!(paro_sites.len(), 6, "Each tooth should have 6 paro sites");
    }

    // Occlusion, ATM, paro_global initialized
    assert!(body["data"]["occlusion"].is_object());
    assert!(body["data"]["atm"].is_object());
    assert!(body["data"]["paro_global"].is_object());

    common::cleanup(&pool, cabinet_id).await;
}

// ─── GET /patients/:id/schema — Full schema ───────────────────────────────────

#[tokio::test]
async fn get_schema_returns_full_nested_data() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("schema_get_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;
    let patient_id = common::create_patient(&server, &token).await;

    // Create schema
    server
        .post(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "dentition": "permanente" }))
        .await;

    // GET schema
    let response = server
        .get(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());

    let dents = body["data"]["dents"].as_array().unwrap();
    assert_eq!(dents.len(), 32);

    // Verify FDI numbers present (spot check: 11, 16, 36, 46)
    let fdis: Vec<i64> = dents
        .iter()
        .map(|d| d["tooth"]["numero_fdi"].as_i64().unwrap())
        .collect();
    for expected_fdi in [11, 16, 36, 46] {
        assert!(fdis.contains(&expected_fdi), "Missing FDI {expected_fdi}");
    }

    common::cleanup(&pool, cabinet_id).await;
}

// ─── PUT /patients/:id/schema/dent/:fdi — Update tooth ───────────────────────

#[tokio::test]
async fn update_tooth_state_persists() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("schema_tooth_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;
    let patient_id = common::create_patient(&server, &token).await;

    // Create schema
    server
        .post(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "dentition": "permanente" }))
        .await;

    // Update tooth 16 (upper right first molar)
    let update_response = server
        .put(&format!("/api/v1/patients/{patient_id}/schema/dent/16"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "statut": "absente",
            "paro_mobilite": 2
        }))
        .await;

    assert_eq!(update_response.status_code(), StatusCode::OK);

    let update_body: Value = update_response.json();
    assert_eq!(update_body["data"]["statut"], "absente");
    assert_eq!(update_body["data"]["paro_mobilite"], 2);

    // GET dent/16 and verify
    let get_response = server
        .get(&format!("/api/v1/patients/{patient_id}/schema/dent/16"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(get_response.status_code(), StatusCode::OK);

    let get_body: Value = get_response.json();
    assert_eq!(get_body["data"]["tooth"]["statut"], "absente");
    assert_eq!(get_body["data"]["tooth"]["paro_mobilite"], 2);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── PUT /patients/:id/schema/paro — Bulk paro update ────────────────────────

#[tokio::test]
async fn bulk_paro_update_persists() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("schema_paro_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;
    let patient_id = common::create_patient(&server, &token).await;

    // Create schema
    server
        .post(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "dentition": "permanente" }))
        .await;

    // Bulk update paro sites for tooth 36 (lower left first molar)
    let paro_response = server
        .put(&format!("/api/v1/patients/{patient_id}/schema/paro"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "sites": [
                { "dent_fdi": 36, "site": "MB", "profondeur_poche": 5, "recession": 1, "bop": true },
                { "dent_fdi": 36, "site": "B",  "profondeur_poche": 4, "recession": 0, "bop": false },
                { "dent_fdi": 36, "site": "DB", "profondeur_poche": 6, "recession": 2, "bop": true }
            ]
        }))
        .await;

    assert_eq!(paro_response.status_code(), StatusCode::OK);

    let paro_body: Value = paro_response.json();
    let updated_sites = paro_body["data"].as_array().unwrap();
    assert_eq!(updated_sites.len(), 3);

    // GET paro and verify
    let get_response = server
        .get(&format!("/api/v1/patients/{patient_id}/schema/paro"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(get_response.status_code(), StatusCode::OK);

    let get_body: Value = get_response.json();
    let all_sites = get_body["data"].as_array().unwrap();

    // Find MB site for tooth 36 and verify profondeur_poche = 5
    let mb_site = all_sites
        .iter()
        .find(|s| s["site"].as_str() == Some("MB") && s["profondeur_poche"].as_i64() == Some(5));
    assert!(mb_site.is_some(), "Expected MB site with profondeur_poche=5");
    assert_eq!(mb_site.unwrap()["bop"], true);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── PUT/GET occlusion ────────────────────────────────────────────────────────

#[tokio::test]
async fn update_occlusion_persists() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("schema_occ_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;
    let patient_id = common::create_patient(&server, &token).await;

    // Create schema
    server
        .post(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "dentition": "permanente" }))
        .await;

    // Update occlusion
    let put_response = server
        .put(&format!("/api/v1/patients/{patient_id}/schema/occlusion"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "angle_molaire_d": "classe_1",
            "angle_molaire_g": "classe_2",
            "overjet_mm": 3.5,
            "overbite_mm": 2.0
        }))
        .await;

    assert_eq!(put_response.status_code(), StatusCode::OK);

    let put_body: Value = put_response.json();
    assert_eq!(put_body["data"]["angle_molaire_d"], "classe_1");

    // GET occlusion
    let get_response = server
        .get(&format!("/api/v1/patients/{patient_id}/schema/occlusion"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(get_response.status_code(), StatusCode::OK);

    let get_body: Value = get_response.json();
    assert_eq!(get_body["data"]["angle_molaire_d"], "classe_1");
    assert_eq!(get_body["data"]["angle_molaire_g"], "classe_2");

    common::cleanup(&pool, cabinet_id).await;
}

// ─── Versioning ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn create_second_schema_increments_version() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("schema_v2_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;
    let patient_id = common::create_patient(&server, &token).await;

    // Version 1
    let v1_response = server
        .post(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "dentition": "permanente" }))
        .await;
    assert_eq!(v1_response.status_code(), StatusCode::CREATED);
    let v1_body: Value = v1_response.json();
    assert_eq!(v1_body["data"]["schema"]["version"], 1);

    // Version 2
    let v2_response = server
        .post(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "dentition": "permanente" }))
        .await;
    assert_eq!(v2_response.status_code(), StatusCode::CREATED);
    let v2_body: Value = v2_response.json();
    assert_eq!(v2_body["data"]["schema"]["version"], 2);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── GET versions ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn list_versions_returns_all() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("schema_versions_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;
    let patient_id = common::create_patient(&server, &token).await;

    // Create v1 and v2
    for _ in 0..2 {
        server
            .post(&format!("/api/v1/patients/{patient_id}/schema"))
            .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
            .json(&json!({ "dentition": "permanente" }))
            .await;
    }

    let response = server
        .get(&format!("/api/v1/patients/{patient_id}/schema/versions"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    let versions = body["data"].as_array().unwrap();
    assert_eq!(versions.len(), 2);
    assert_eq!(versions[0]["version"], 1);
    assert_eq!(versions[1]["version"], 2);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── GET schema?version=1 — Specific version ─────────────────────────────────

#[tokio::test]
async fn get_specific_schema_version_returns_correct_data() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("schema_ver_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;
    let patient_id = common::create_patient(&server, &token).await;

    // Create v1 and v2
    server
        .post(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "dentition": "permanente" }))
        .await;

    server
        .post(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "dentition": "permanente" }))
        .await;

    // GET ?version=1 explicitly
    let response = server
        .get(&format!("/api/v1/patients/{patient_id}/schema?version=1"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert_eq!(body["data"]["schema"]["version"], 1);

    // Without version param → returns latest (v2)
    let latest_response = server
        .get(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    let latest_body: Value = latest_response.json();
    assert_eq!(latest_body["data"]["schema"]["version"], 2);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── Auth guard ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn create_schema_without_token_returns_401() {
    let pool = common::test_pool().await;
    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let response = server
        .post(&format!("/api/v1/patients/{}/schema", Uuid::new_v4()))
        .json(&json!({ "dentition": "permanente" }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

// ─── 404 on missing patient ───────────────────────────────────────────────────

#[tokio::test]
async fn create_schema_for_missing_patient_returns_404() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("schema_404_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    let response = server
        .post(&format!("/api/v1/patients/{}/schema", Uuid::new_v4()))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "dentition": "permanente" }))
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── ATM ──────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn update_atm_persists() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("schema_atm_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;
    let patient_id = common::create_patient(&server, &token).await;

    server
        .post(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "dentition": "permanente" }))
        .await;

    let put_response = server
        .put(&format!("/api/v1/patients/{patient_id}/schema/atm"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "ouverture_max_mm": 42,
            "bruit_g": "crepitement",
            "score_helkimo": 2
        }))
        .await;

    assert_eq!(put_response.status_code(), StatusCode::OK);

    let body: Value = put_response.json();
    assert_eq!(body["data"]["ouverture_max_mm"], 42);
    assert_eq!(body["data"]["bruit_g"], "crepitement");
    assert_eq!(body["data"]["score_helkimo"], 2);

    // GET to verify persistence
    let get_response = server
        .get(&format!("/api/v1/patients/{patient_id}/schema/atm"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    let get_body: Value = get_response.json();
    assert_eq!(get_body["data"]["ouverture_max_mm"], 42);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── Paro Global ─────────────────────────────────────────────────────────────

#[tokio::test]
async fn update_paro_global_persists() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("schema_pg_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;
    let patient_id = common::create_patient(&server, &token).await;

    server
        .post(&format!("/api/v1/patients/{patient_id}/schema"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "dentition": "permanente" }))
        .await;

    let put_response = server
        .put(&format!("/api/v1/patients/{patient_id}/schema/paro-global"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "staging": "III",
            "grading": "B",
            "indice_plaque_pct": 45,
            "bop_global_pct": 30
        }))
        .await;

    assert_eq!(put_response.status_code(), StatusCode::OK);

    let body: Value = put_response.json();
    assert_eq!(body["data"]["staging"], "III");
    assert_eq!(body["data"]["grading"], "B");
    assert_eq!(body["data"]["indice_plaque_pct"], 45);
    assert_eq!(body["data"]["bop_global_pct"], 30);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── Tooth Timeline ───────────────────────────────────────────────────────────

#[tokio::test]
async fn tooth_timeline_returns_events() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("schema_tl_{}@test.smiled.io", Uuid::new_v4());

    let user_id =
        common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;
    let patient_id = common::create_patient(&server, &token).await;
    let patient_uuid = Uuid::parse_str(&patient_id).unwrap();

    // Insert a test event directly into the database
    sqlx::query!(
        r#"
        INSERT INTO evenement_dent (patient_id, cabinet_id, dent_fdi, date, type, praticien_id, description)
        VALUES ($1, $2, 46, '2026-01-15', 'traitement', $3, 'Couronne posée')
        "#,
        patient_uuid,
        cabinet_id,
        user_id,
    )
    .execute(&pool)
    .await
    .expect("Failed to insert test event");

    // GET timeline for tooth 46
    let response = server
        .get(&format!("/api/v1/patients/{patient_id}/dent/46/timeline"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    let events = body["data"].as_array().unwrap();
    assert!(!events.is_empty(), "Expected at least one event for tooth 46");

    let event = &events[0];
    assert_eq!(event["dent_fdi"], 46);
    assert_eq!(event["type"], "traitement");
    assert_eq!(event["description"], "Couronne posée");

    common::cleanup(&pool, cabinet_id).await;
}
