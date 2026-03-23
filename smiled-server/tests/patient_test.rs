/// Integration tests for the Patient CRUD and Questionnaire API.
///
/// Prerequisites:
/// - PostgreSQL running on localhost:5433 (docker compose up -d db)
/// - DATABASE_URL env var set (or smiled-server/.env file)
/// - Migrations applied (sqlx migrate run)
///
/// Run with: cargo test --test patient_test
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
            "Cabinet Test Patient",
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
        // Delete questionnaires (FK: patient_id -> patient)
        sqlx::query!(
            "DELETE FROM questionnaire_medical WHERE cabinet_id = $1",
            cabinet_id,
        )
        .execute(pool)
        .await
        .ok();

        // Delete patients
        sqlx::query!("DELETE FROM patient WHERE cabinet_id = $1", cabinet_id)
            .execute(pool)
            .await
            .ok();

        // Delete password reset tokens
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

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn auth_header(token: &str) -> HeaderValue {
    format!("Bearer {token}").parse().unwrap()
}

fn sample_patient_body() -> Value {
    json!({
        "nom": "Dupont",
        "prenom": "Jean",
        "sexe": "M",
        "date_naissance": "1985-06-15",
        "couverture": "mutuelle",
        "mutuelle_nom": "MGEN",
        "email": "jean.dupont@example.com",
        "telephone": "0612345678"
    })
}

// ─── POST /patients — Create ──────────────────────────────────────────────────

#[tokio::test]
async fn create_patient_returns_201_with_data() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pat_create_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    let response = server
        .post("/api/v1/patients")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&sample_patient_body())
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["nom"], "Dupont");
    assert_eq!(body["data"]["prenom"], "Jean");
    assert_eq!(body["data"]["sexe"], "M");
    assert_eq!(body["data"]["couverture"], "mutuelle");
    assert!(body["data"]["id"].is_string());

    common::cleanup(&pool, cabinet_id).await;
}

#[tokio::test]
async fn create_patient_without_token_returns_401() {
    let pool = common::test_pool().await;
    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let response = server
        .post("/api/v1/patients")
        .json(&sample_patient_body())
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_patient_with_invalid_data_returns_422() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pat_invalid_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    // nom is empty — validation should fail
    let response = server
        .post("/api/v1/patients")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "nom": "",
            "prenom": "Jean",
            "sexe": "M",
            "date_naissance": "1985-06-15",
            "couverture": "mutuelle"
        }))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── GET /patients — List ─────────────────────────────────────────────────────

#[tokio::test]
async fn list_patients_returns_paginated_results() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pat_list_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "secretaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    // Create two patients
    for (nom, prenom) in [("Martin", "Alice"), ("Bernard", "Bob")] {
        server
            .post("/api/v1/patients")
            .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
            .json(&json!({
                "nom": nom,
                "prenom": prenom,
                "sexe": "F",
                "date_naissance": "1990-01-01",
                "couverture": "aucune"
            }))
            .await;
    }

    let response = server
        .get("/api/v1/patients")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert!(body["data"].is_array());
    assert!(body["meta"]["total"].as_i64().unwrap() >= 2);
    assert!(body["meta"]["page"].as_i64().is_some());
    assert!(body["meta"]["limit"].as_i64().is_some());

    common::cleanup(&pool, cabinet_id).await;
}

#[tokio::test]
async fn search_patients_filters_by_nom() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let unique = Uuid::new_v4().to_string().replace('-', "");
    let nom = format!("Ztest{}", &unique[..8]);
    let email = format!("pat_search_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    // Create one uniquely-named patient
    server
        .post("/api/v1/patients")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({
            "nom": nom,
            "prenom": "SearchTarget",
            "sexe": "M",
            "date_naissance": "1975-03-20",
            "couverture": "aucune"
        }))
        .await;

    // Search for that unique nom
    let response = server
        .get(&format!("/api/v1/patients?search={nom}"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    let data = body["data"].as_array().unwrap();
    assert!(!data.is_empty(), "Expected at least one result for search={nom}");
    assert!(data.iter().all(|p| p["nom"]
        .as_str()
        .unwrap()
        .to_lowercase()
        .contains(&nom.to_lowercase())));

    common::cleanup(&pool, cabinet_id).await;
}

// ─── GET /patients/:id — Single ───────────────────────────────────────────────

#[tokio::test]
async fn get_patient_by_id_returns_patient() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pat_get_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    // Create patient
    let created: Value = server
        .post("/api/v1/patients")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&sample_patient_body())
        .await
        .json();

    let patient_id = created["data"]["id"].as_str().unwrap();

    // Fetch by ID
    let response = server
        .get(&format!("/api/v1/patients/{patient_id}"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["id"], patient_id);
    assert_eq!(body["data"]["nom"], "Dupont");

    common::cleanup(&pool, cabinet_id).await;
}

#[tokio::test]
async fn get_patient_not_found_returns_404() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pat_404_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    let response = server
        .get(&format!("/api/v1/patients/{}", Uuid::new_v4()))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── PUT /patients/:id — Update ───────────────────────────────────────────────

#[tokio::test]
async fn update_patient_applies_partial_changes() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pat_update_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    // Create patient
    let created: Value = server
        .post("/api/v1/patients")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&sample_patient_body())
        .await
        .json();

    let patient_id = created["data"]["id"].as_str().unwrap();

    // Update only telephone
    let response = server
        .put(&format!("/api/v1/patients/{patient_id}"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "telephone": "0699999999" }))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["telephone"], "0699999999");
    // nom unchanged
    assert_eq!(body["data"]["nom"], "Dupont");

    common::cleanup(&pool, cabinet_id).await;
}

// ─── Questionnaire ────────────────────────────────────────────────────────────

#[tokio::test]
async fn upsert_questionnaire_saves_all_risk_sections() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pat_qst_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    // Create patient
    let created: Value = server
        .post("/api/v1/patients")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&sample_patient_body())
        .await
        .json();

    let patient_id = created["data"]["id"].as_str().unwrap();

    // Submit comprehensive questionnaire
    let questionnaire_body = json!({
        "date_signature": "2026-03-23",
        "signe_par": "patient",
        "nom_signataire": "Jean Dupont",
        "prochaine_maj": "2027-03-23",
        // P0 haemorrhage risk
        "avk": { "molecule": "warfarine", "inr": 2.5, "controle_date": "2026-03-01" },
        "aod_molecule": "apixaban",
        "antiagregants": { "molecules": ["aspirin"], "dose": 100 },
        "hemostase": { "troubles": ["hemophilie_a"], "details": "Von Willebrand type 2" },
        // P0 infectious risk
        "endocardite": { "haut_risque": true, "molecules": ["amoxicilline"] },
        "immunodepression": { "type": "VIH", "cd4": 350 },
        "protheses_articulaires": { "localisation": ["hanche_gauche"], "date_pose": "2020-05-10" },
        // P0 drug risk
        "bisphosphonates": { "molecule": "alendronate", "voie": "orale", "duree_ans": 3 },
        "antiresorptifs": { "molecule": "denosumab" },
        "radiotherapie": { "localisation": "cervico-faciale", "date": "2022-01", "grays": 60 },
        // Medical troubles
        "troubles": [
            { "categorie": "cardiovasculaire", "libelle": "Hypertension", "details": "traitée" }
        ],
        // Medications & allergies
        "medicaments": ["metformine", "ramipril"],
        "allergies": [
            { "substance": "pénicilline", "type": "medicament", "reaction": "urticaire" }
        ],
        // Lifestyle
        "tabac": "actif_15_paquets",
        "alcool": "occasionnel",
        "drogues": { "cannabis": true },
        "grossesse_mois": null,
        "allaitement": false,
        "activite_physique": "reguliere",
        "bruxisme": "nocturne",
        "sahos": { "traitement": "PPC", "type": "modere" },
        "rgo": true,
        "tca": "anorexie",
        // Dental history
        "dernier_rdv_date": "2025-09-15",
        "brossage_quotidien": 2,
        "auxiliaires": ["brossette", "fil"],
        "historique_connu": ["extractions", "bridges"],
        "apprehension": "moderee",
        // RGPD
        "notice_information_date": "2026-03-23"
    });

    let response = server
        .put(&format!("/api/v1/patients/{patient_id}/questionnaire"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&questionnaire_body)
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    let q = &body["data"];
    assert_eq!(q["patient_id"], patient_id);
    assert_eq!(q["version"], 1);
    assert_eq!(q["signe_par"], "patient");
    assert_eq!(q["aod_molecule"], "apixaban");
    assert_eq!(q["tabac"], "actif_15_paquets");
    assert_eq!(q["rgo"], true);
    assert_eq!(q["brossage_quotidien"], 2);
    assert!(q["avk"].is_object());
    assert!(q["troubles"].is_array());
    assert!(q["allergies"].is_array());
    assert!(q["medicaments"].is_array());

    common::cleanup(&pool, cabinet_id).await;
}

#[tokio::test]
async fn get_questionnaire_returns_latest_version() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pat_qst_get_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    // Create patient
    let created: Value = server
        .post("/api/v1/patients")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&sample_patient_body())
        .await
        .json();

    let patient_id = created["data"]["id"].as_str().unwrap();

    // Version 1
    server
        .put(&format!("/api/v1/patients/{patient_id}/questionnaire"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "tabac": "non" }))
        .await;

    // Version 2
    server
        .put(&format!("/api/v1/patients/{patient_id}/questionnaire"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&json!({ "tabac": "sevrage" }))
        .await;

    // GET should return version 2
    let response = server
        .get(&format!("/api/v1/patients/{patient_id}/questionnaire"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["version"], 2);
    assert_eq!(body["data"]["tabac"], "sevrage");

    common::cleanup(&pool, cabinet_id).await;
}

#[tokio::test]
async fn get_questionnaire_not_found_returns_404() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("pat_qst_404_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email, "P@ss1234!").await;

    // Create patient with no questionnaire
    let created: Value = server
        .post("/api/v1/patients")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .json(&sample_patient_body())
        .await
        .json();

    let patient_id = created["data"]["id"].as_str().unwrap();

    let response = server
        .get(&format!("/api/v1/patients/{patient_id}/questionnaire"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── RLS Isolation ────────────────────────────────────────────────────────────

/// Verify RLS tenant isolation: Cabinet B cannot access Cabinet A's patients.
///
/// NOTE: This test requires a non-superuser DB connection to enforce RLS.
/// The `smiled` user in Docker Compose is a superuser (bypasses RLS by design
/// for migrations and dev). In production, the application connects as a
/// restricted role (no BYPASSRLS). This test skips automatically when the
/// connecting user is a superuser.
#[tokio::test]
async fn cabinet_cannot_see_patients_from_other_cabinet() {
    let pool = common::test_pool().await;

    // Skip if the DB user is a superuser (RLS is bypassed)
    let is_superuser: bool = sqlx::query_scalar!(
        "SELECT usesuper FROM pg_user WHERE usename = current_user"
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to check superuser status")
    .unwrap_or(false);

    if is_superuser {
        eprintln!("SKIP: cabinet_cannot_see_patients_from_other_cabinet — DB user is superuser, RLS bypassed");
        return;
    }

    // Cabinet A
    let cabinet_a_id = common::create_test_cabinet(&pool).await;
    let email_a = format!("pat_rls_a_{}@test.smiled.io", Uuid::new_v4());
    common::create_test_user(&pool, cabinet_a_id, &email_a, "P@ss1234!", "titulaire").await;

    // Cabinet B
    let cabinet_b_id = common::create_test_cabinet(&pool).await;
    let email_b = format!("pat_rls_b_{}@test.smiled.io", Uuid::new_v4());
    common::create_test_user(&pool, cabinet_b_id, &email_b, "P@ss1234!", "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token_a = common::login(&server, &email_a, "P@ss1234!").await;
    let token_b = common::login(&server, &email_b, "P@ss1234!").await;

    // Cabinet A creates a patient
    let created: Value = server
        .post("/api/v1/patients")
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token_a))
        .json(&sample_patient_body())
        .await
        .json();

    let patient_id = created["data"]["id"].as_str().unwrap();

    // Cabinet B tries to access Cabinet A's patient
    let response = server
        .get(&format!("/api/v1/patients/{patient_id}"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token_b))
        .await;

    assert_eq!(
        response.status_code(),
        StatusCode::NOT_FOUND,
        "Cabinet B should not see Cabinet A's patient"
    );

    common::cleanup(&pool, cabinet_a_id).await;
    common::cleanup(&pool, cabinet_b_id).await;
}
