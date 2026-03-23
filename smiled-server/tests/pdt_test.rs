/// Integration tests for the Diagnostic and Treatment Plan API.
///
/// Prerequisites:
/// - PostgreSQL running on localhost:5433 (docker compose up -d db)
/// - DATABASE_URL env var set (or smiled-server/.env file)
/// - Migrations applied (sqlx migrate run)
///
/// Run with: cargo test --test pdt_test
use axum::http::{HeaderValue, StatusCode};
use serde_json::{json, Value};
use uuid::Uuid;

// ─── Common Test Helpers ──────────────────────────────────────────────────────

mod common {
    use axum::{http::HeaderValue, Router};
    use axum_test::TestServer;
    use serde_json::{json, Value};
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
            "Cabinet PDT Test",
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
        role: &str,
    ) -> Uuid {
        let id = Uuid::new_v4();
        let hash = hash_password("P@ss1234!").expect("Failed to hash password");
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
        // ligne_pdt cascades from plan_traitement, finding cascades from diagnostic
        sqlx::query!("DELETE FROM plan_traitement WHERE cabinet_id = $1", cabinet_id)
            .execute(pool)
            .await
            .ok();

        sqlx::query!("DELETE FROM diagnostic WHERE cabinet_id = $1", cabinet_id)
            .execute(pool)
            .await
            .ok();

        sqlx::query!("DELETE FROM evenement_dent WHERE cabinet_id = $1", cabinet_id)
            .execute(pool)
            .await
            .ok();

        sqlx::query!("DELETE FROM schema_dentaire WHERE cabinet_id = $1", cabinet_id)
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

    pub async fn login(server: &TestServer, email: &str) -> String {
        let resp: Value = server
            .post("/api/v1/auth/login")
            .json(&json!({ "email": email, "password": "P@ss1234!" }))
            .await
            .json();
        resp["access_token"]
            .as_str()
            .expect("access_token missing in login response")
            .to_string()
    }

    pub fn auth_header(token: &str) -> HeaderValue {
        format!("Bearer {token}").parse().unwrap()
    }

    pub async fn create_patient(server: &TestServer, token: &str) -> String {
        let body = json!({
            "nom": "Durand",
            "prenom": "Marie",
            "sexe": "F",
            "date_naissance": "1985-03-12",
            "couverture": "mutuelle",
        });
        let resp: Value = server
            .post("/api/v1/patients")
            .add_header(axum::http::header::AUTHORIZATION, auth_header(token))
            .json(&body)
            .await
            .json();
        resp["data"]["id"].as_str().unwrap().to_string()
    }
}

fn auth_header(token: &str) -> HeaderValue {
    format!("Bearer {token}").parse().unwrap()
}

// ─── POST /patients/:id/diagnostics — Create with findings ────────────────────

#[tokio::test]
async fn create_diagnostic_with_findings_returns_201() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("diag_create_{}@test.smiled.io", Uuid::new_v4());
    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    let response = server
        .post(&format!("/api/v1/patients/{patient_id}/diagnostics"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "schema_version": 1,
            "date": "2026-03-20",
            "findings": [
                { "type": "carie", "dent_fdi": 16, "severite": 3, "source": "praticien", "details": { "face": "O" } },
                { "type": "paro", "dent_fdi": 36, "severite": 4, "source": "praticien" },
                { "type": "absence", "dent_fdi": 48, "severite": 2, "source": "praticien" }
            ]
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());

    let data = &body["data"];
    assert_eq!(data["patient_id"].as_str().unwrap(), patient_id);
    assert_eq!(data["schema_version"], 1);

    let findings = data["findings"].as_array().unwrap();
    assert_eq!(findings.len(), 3);

    // Verify finding fields
    let carie = findings.iter().find(|f| f["type"] == "carie").unwrap();
    assert_eq!(carie["dent_fdi"], 16);
    assert_eq!(carie["severite"], 3);
    assert_eq!(carie["source"], "praticien");
    assert!(carie["details"].is_object());

    common::cleanup(&pool, cabinet_id).await;
}

// ─── GET /patients/:id/diagnostics — List ─────────────────────────────────────

#[tokio::test]
async fn list_diagnostics_returns_all() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("diag_list_{}@test.smiled.io", Uuid::new_v4());
    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    let base_body = json!({
        "schema_version": 1,
        "findings": []
    });

    // Create 2 diagnostics
    for _ in 0..2 {
        server
            .post(&format!("/api/v1/patients/{patient_id}/diagnostics"))
            .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
            .json(&base_body)
            .await;
    }

    let response = server
        .get(&format!("/api/v1/patients/{patient_id}/diagnostics"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    let diagnostics = body["data"].as_array().unwrap();
    assert_eq!(diagnostics.len(), 2);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── GET /patients/:id/diagnostics/:diag_id — Single ─────────────────────────

#[tokio::test]
async fn get_diagnostic_returns_findings() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("diag_get_{}@test.smiled.io", Uuid::new_v4());
    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    // Create diagnostic
    let create_resp: Value = server
        .post(&format!("/api/v1/patients/{patient_id}/diagnostics"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "schema_version": 1,
            "findings": [
                { "type": "fracture", "dent_fdi": 11, "severite": 5, "source": "praticien" }
            ]
        }))
        .await
        .json();

    let diag_id = create_resp["data"]["id"].as_str().unwrap().to_string();

    // GET single
    let response = server
        .get(&format!("/api/v1/patients/{patient_id}/diagnostics/{diag_id}"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert_eq!(body["data"]["id"], diag_id);

    let findings = body["data"]["findings"].as_array().unwrap();
    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0]["type"], "fracture");
    assert_eq!(findings[0]["severite"], 5);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── POST /patients/:id/pdts — Create PDT (lab) ────────────────────────────────

#[tokio::test]
async fn create_pdt_lab_calculates_prix_total() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pdt_lab_{}@test.smiled.io", Uuid::new_v4());
    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    // Create diagnostic first
    let diag_resp: Value = server
        .post(&format!("/api/v1/patients/{patient_id}/diagnostics"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "schema_version": 1, "findings": [] }))
        .await
        .json();
    let diag_id = diag_resp["data"]["id"].as_str().unwrap().to_string();

    // Create PDT lab with 3 lines
    let response = server
        .post(&format!("/api/v1/patients/{patient_id}/pdts"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "diagnostic_id": diag_id,
            "formule": "lab",
            "lignes": [
                { "dent_fdi": 16, "prix_praticien": 850.00, "ordre": 1 },
                { "dent_fdi": 26, "prix_praticien": 950.00, "ordre": 2 },
                { "dent_fdi": 36, "prix_praticien": 1200.00, "ordre": 3 }
            ]
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());

    let pdt = &body["data"];
    assert_eq!(pdt["formule"], "lab");
    assert_eq!(pdt["statut"], "brouillon");

    // prix_total = 850 + 950 + 1200 = 3000
    let prix_total: f64 = pdt["prix_total"].as_str().unwrap().parse().unwrap();
    assert!((prix_total - 3000.0).abs() < 0.01, "Expected prix_total = 3000, got {prix_total}");

    let lignes = pdt["lignes"].as_array().unwrap();
    assert_eq!(lignes.len(), 3);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── Create second PDT (budget) for same diagnostic ───────────────────────────

#[tokio::test]
async fn create_two_pdts_for_same_diagnostic_succeeds() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pdt_two_{}@test.smiled.io", Uuid::new_v4());
    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    let diag_resp: Value = server
        .post(&format!("/api/v1/patients/{patient_id}/diagnostics"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "schema_version": 1, "findings": [] }))
        .await
        .json();
    let diag_id = diag_resp["data"]["id"].as_str().unwrap().to_string();

    // Create lab
    let lab_resp = server
        .post(&format!("/api/v1/patients/{patient_id}/pdts"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "diagnostic_id": diag_id, "formule": "lab", "lignes": [
            { "prix_praticien": 500.0, "ordre": 1 }
        ]}))
        .await;
    assert_eq!(lab_resp.status_code(), StatusCode::CREATED);

    // Create budget
    let budget_resp = server
        .post(&format!("/api/v1/patients/{patient_id}/pdts"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "diagnostic_id": diag_id, "formule": "budget", "lignes": [
            { "prix_praticien": 300.0, "ordre": 1 }
        ]}))
        .await;
    assert_eq!(budget_resp.status_code(), StatusCode::CREATED);

    let budget_body: Value = budget_resp.json();
    assert_eq!(budget_body["data"]["formule"], "budget");

    // List PDTs — should have 2
    let list_resp = server
        .get(&format!("/api/v1/patients/{patient_id}/pdts"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    let list_body: Value = list_resp.json();
    assert_eq!(list_body["data"].as_array().unwrap().len(), 2);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── Reject duplicate formule ─────────────────────────────────────────────────

#[tokio::test]
async fn duplicate_formule_returns_409() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pdt_dup_{}@test.smiled.io", Uuid::new_v4());
    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    let diag_resp: Value = server
        .post(&format!("/api/v1/patients/{patient_id}/diagnostics"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "schema_version": 1, "findings": [] }))
        .await
        .json();
    let diag_id = diag_resp["data"]["id"].as_str().unwrap().to_string();

    let body = json!({ "diagnostic_id": diag_id, "formule": "lab", "lignes": [] });

    // First create succeeds
    let first = server
        .post(&format!("/api/v1/patients/{patient_id}/pdts"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&body)
        .await;
    assert_eq!(first.status_code(), StatusCode::CREATED);

    // Duplicate formule → 409
    let second = server
        .post(&format!("/api/v1/patients/{patient_id}/pdts"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&body)
        .await;
    assert_eq!(second.status_code(), StatusCode::CONFLICT);

    let err_body: Value = second.json();
    assert!(!err_body["success"].as_bool().unwrap());

    common::cleanup(&pool, cabinet_id).await;
}

// ─── PUT /patients/:id/pdts/:pdt_id — Update PDT status ──────────────────────

#[tokio::test]
async fn update_pdt_status_persists() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pdt_upd_{}@test.smiled.io", Uuid::new_v4());
    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    let diag_resp: Value = server
        .post(&format!("/api/v1/patients/{patient_id}/diagnostics"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "schema_version": 1, "findings": [] }))
        .await
        .json();
    let diag_id = diag_resp["data"]["id"].as_str().unwrap().to_string();

    let create_resp: Value = server
        .post(&format!("/api/v1/patients/{patient_id}/pdts"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "diagnostic_id": diag_id,
            "formule": "lab",
            "lignes": [{ "prix_praticien": 600.0, "ordre": 1 }]
        }))
        .await
        .json();

    let pdt_id = create_resp["data"]["id"].as_str().unwrap().to_string();

    // Update status to "presente"
    let update_resp = server
        .put(&format!("/api/v1/patients/{patient_id}/pdts/{pdt_id}"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "statut": "presente" }))
        .await;

    assert_eq!(update_resp.status_code(), StatusCode::OK);

    let body: Value = update_resp.json();
    assert_eq!(body["data"]["statut"], "presente");

    common::cleanup(&pool, cabinet_id).await;
}

// ─── Update ligne statut ───────────────────────────────────────────────────────

#[tokio::test]
async fn update_ligne_statut_recalculates_prix_total() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pdt_ligne_{}@test.smiled.io", Uuid::new_v4());
    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    let diag_resp: Value = server
        .post(&format!("/api/v1/patients/{patient_id}/diagnostics"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "schema_version": 1, "findings": [] }))
        .await
        .json();
    let diag_id = diag_resp["data"]["id"].as_str().unwrap().to_string();

    let create_resp: Value = server
        .post(&format!("/api/v1/patients/{patient_id}/pdts"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "diagnostic_id": diag_id,
            "formule": "compromis_1",
            "lignes": [
                { "dent_fdi": 14, "prix_praticien": 200.0, "ordre": 1 },
                { "dent_fdi": 15, "prix_praticien": 300.0, "ordre": 2 }
            ]
        }))
        .await
        .json();

    let pdt_id = create_resp["data"]["id"].as_str().unwrap().to_string();
    let ligne_id = create_resp["data"]["lignes"][0]["id"].as_str().unwrap().to_string();

    // Update ligne statut
    let update_resp = server
        .put(&format!("/api/v1/patients/{patient_id}/pdts/{pdt_id}"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "lignes": [{ "id": ligne_id, "statut": "fait", "prix_praticien": 250.0 }]
        }))
        .await;

    assert_eq!(update_resp.status_code(), StatusCode::OK);

    let body: Value = update_resp.json();
    let updated_ligne = body["data"]["lignes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|l| l["id"] == ligne_id)
        .cloned()
        .unwrap();

    assert_eq!(updated_ligne["statut"], "fait");

    // prix_total should be recalculated: 250 + 300 = 550
    let prix_total: f64 = body["data"]["prix_total"].as_str().unwrap().parse().unwrap();
    assert!((prix_total - 550.0).abs() < 0.01, "Expected prix_total = 550, got {prix_total}");

    common::cleanup(&pool, cabinet_id).await;
}

// ─── POST /patients/:id/pdts/:pdt_id/pdf — PDF stub ──────────────────────────

#[tokio::test]
async fn generate_pdf_returns_stub_url() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pdt_pdf_{}@test.smiled.io", Uuid::new_v4());
    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    let diag_resp: Value = server
        .post(&format!("/api/v1/patients/{patient_id}/diagnostics"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "schema_version": 1, "findings": [] }))
        .await
        .json();
    let diag_id = diag_resp["data"]["id"].as_str().unwrap().to_string();

    let create_resp: Value = server
        .post(&format!("/api/v1/patients/{patient_id}/pdts"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "diagnostic_id": diag_id, "formule": "lab", "lignes": [] }))
        .await
        .json();
    let pdt_id = create_resp["data"]["id"].as_str().unwrap().to_string();

    // Generate PDF stub
    let pdf_resp = server
        .post(&format!("/api/v1/patients/{patient_id}/pdts/{pdt_id}/pdf"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(pdf_resp.status_code(), StatusCode::OK);

    let body: Value = pdf_resp.json();
    let pdf_url = body["data"]["pdf_url"].as_str().unwrap();

    assert!(
        pdf_url.contains(&pdt_id),
        "pdf_url should contain pdt_id, got: {pdf_url}"
    );
    assert!(
        pdf_url.ends_with("/pdf/download"),
        "pdf_url should end with /pdf/download, got: {pdf_url}"
    );

    common::cleanup(&pool, cabinet_id).await;
}

// ─── Auth guard — 401 without token ───────────────────────────────────────────

#[tokio::test]
async fn create_diagnostic_without_token_returns_401() {
    let pool = common::test_pool().await;
    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let response = server
        .post(&format!(
            "/api/v1/patients/{}/diagnostics",
            Uuid::new_v4()
        ))
        .json(&json!({ "schema_version": 1, "findings": [] }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_pdt_without_token_returns_401() {
    let pool = common::test_pool().await;
    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let response = server
        .post(&format!("/api/v1/patients/{}/pdts", Uuid::new_v4()))
        .json(&json!({ "diagnostic_id": Uuid::new_v4(), "formule": "lab", "lignes": [] }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

// ─── Invalid diagnostic → 404 ─────────────────────────────────────────────────

#[tokio::test]
async fn create_pdt_with_unknown_diagnostic_returns_404() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pdt_404_{}@test.smiled.io", Uuid::new_v4());
    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    let response = server
        .post(&format!("/api/v1/patients/{patient_id}/pdts"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "diagnostic_id": Uuid::new_v4(),
            "formule": "lab",
            "lignes": []
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);

    common::cleanup(&pool, cabinet_id).await;
}
