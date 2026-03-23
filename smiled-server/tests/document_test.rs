/// Integration tests for the Document management and S3 integration.
///
/// Prerequisites:
/// - PostgreSQL running on localhost:5433 (docker compose up -d db)
/// - DATABASE_URL env var set (or smiled-server/.env file)
/// - Migrations applied (sqlx migrate run)
/// - S3 is optional: tests run without it (url_storage will be the key path)
///
/// Run with: cargo test --test document_test
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

    pub fn test_config() -> Config {
        dotenvy::dotenv().ok();
        Config::from_env().expect("Config must be valid for integration tests")
    }

    /// Build the test app — without S3 (s3: None) so uploads only store metadata.
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
            "Cabinet Document Test",
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
        // document_dent cascades from document
        sqlx::query!("DELETE FROM document WHERE cabinet_id = $1", cabinet_id)
            .execute(pool)
            .await
            .ok();

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
        let resp: Value = server
            .post("/api/v1/patients")
            .add_header(axum::http::header::AUTHORIZATION, auth_header(token))
            .json(&json!({
                "nom": "Lefevre",
                "prenom": "Sophie",
                "sexe": "F",
                "date_naissance": "1990-03-22",
                "couverture": "mutuelle"
            }))
            .await
            .json();
        resp["data"]["id"]
            .as_str()
            .expect("Patient id missing")
            .to_string()
    }
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn auth_header(token: &str) -> HeaderValue {
    format!("Bearer {token}").parse().unwrap()
}

/// Build a minimal multipart body for a document upload.
/// Returns (body_bytes, boundary_string).
fn build_multipart_body(doc_type: &str, filename: &str, content: &[u8]) -> (Vec<u8>, String) {
    let boundary = "testboundary1234567890";
    let mut body = Vec::new();

    // File field
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(
        format!(
            "Content-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\n"
        )
        .as_bytes(),
    );
    body.extend_from_slice(b"Content-Type: application/pdf\r\n\r\n");
    body.extend_from_slice(content);
    body.extend_from_slice(b"\r\n");

    // Type field
    body.extend_from_slice(format!("--{boundary}\r\n").as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"type\"\r\n\r\n");
    body.extend_from_slice(doc_type.as_bytes());
    body.extend_from_slice(b"\r\n");

    body.extend_from_slice(format!("--{boundary}--\r\n").as_bytes());

    (body, boundary.to_string())
}

// ─── POST /patients/:id/documents/upload ─────────────────────────────────────

#[tokio::test]
async fn upload_document_returns_201_with_metadata() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("doc_upload_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    let (body, boundary) = build_multipart_body("panoramique", "pano.pdf", b"PDFCONTENT");
    let content_type = format!("multipart/form-data; boundary={boundary}");

    let response = server
        .post(&format!("/api/v1/patients/{patient_id}/documents/upload"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .add_header(
            axum::http::header::CONTENT_TYPE,
            content_type.parse::<HeaderValue>().unwrap(),
        )
        .bytes(body.into())
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["type"], "panoramique");
    assert_eq!(body["data"]["filename"], "pano.pdf");
    assert_eq!(body["data"]["mime_type"], "application/pdf");
    assert!(body["data"]["id"].is_string());
    assert!(body["data"]["url_storage"].as_str().unwrap().contains(&patient_id));

    common::cleanup(&pool, cabinet_id).await;
}

// ─── GET /patients/:id/documents ─────────────────────────────────────────────

#[tokio::test]
async fn list_documents_returns_uploaded_doc() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("doc_list_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    // Upload one document first
    let (body, boundary) = build_multipart_body("photo", "photo.jpg", b"JPGDATA");
    let content_type = format!("multipart/form-data; boundary={boundary}");
    server
        .post(&format!("/api/v1/patients/{patient_id}/documents/upload"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .add_header(
            axum::http::header::CONTENT_TYPE,
            content_type.parse::<HeaderValue>().unwrap(),
        )
        .bytes(body.into())
        .await;

    // Now list
    let response = server
        .get(&format!("/api/v1/patients/{patient_id}/documents"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    let docs = body["data"].as_array().unwrap();
    assert_eq!(docs.len(), 1);
    assert_eq!(docs[0]["type"], "photo");
    assert_eq!(docs[0]["filename"], "photo.jpg");
    assert!(docs[0]["dents_fdi"].as_array().unwrap().is_empty());

    common::cleanup(&pool, cabinet_id).await;
}

// ─── POST /patients/:id/documents/:doc_id/link-dent/:fdi ─────────────────────

#[tokio::test]
async fn link_document_to_tooth_returns_201() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("doc_link_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;
    let patient_id = common::create_patient(&server, &token).await;

    // Upload a document
    let (body, boundary) = build_multipart_body("radio_retro", "radio.jpg", b"RADIODATA");
    let content_type = format!("multipart/form-data; boundary={boundary}");
    let upload_resp: Value = server
        .post(&format!("/api/v1/patients/{patient_id}/documents/upload"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .add_header(
            axum::http::header::CONTENT_TYPE,
            content_type.parse::<HeaderValue>().unwrap(),
        )
        .bytes(body.into())
        .await
        .json();

    let doc_id = upload_resp["data"]["id"].as_str().unwrap().to_string();

    // Link to tooth 11
    let response = server
        .post(&format!("/api/v1/patients/{patient_id}/documents/{doc_id}/link-dent/11"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await;

    assert_eq!(response.status_code(), StatusCode::CREATED);

    let body: Value = response.json();
    assert!(body["success"].as_bool().unwrap());
    assert_eq!(body["data"]["dent_fdi"], 11);

    // Verify the link appears in the document list
    let list_resp: Value = server
        .get(&format!("/api/v1/patients/{patient_id}/documents"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .await
        .json();

    let docs = list_resp["data"].as_array().unwrap();
    let dents = docs[0]["dents_fdi"].as_array().unwrap();
    assert_eq!(dents.len(), 1);
    assert_eq!(dents[0], 11);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── Auth guard tests ─────────────────────────────────────────────────────────

#[tokio::test]
async fn upload_without_token_returns_401() {
    let pool = common::test_pool().await;
    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let fake_patient_id = Uuid::new_v4();
    let (body, boundary) = build_multipart_body("photo", "test.jpg", b"DATA");
    let content_type = format!("multipart/form-data; boundary={boundary}");

    let response = server
        .post(&format!("/api/v1/patients/{fake_patient_id}/documents/upload"))
        .add_header(
            axum::http::header::CONTENT_TYPE,
            content_type.parse::<HeaderValue>().unwrap(),
        )
        .bytes(body.into())
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn list_documents_without_token_returns_401() {
    let pool = common::test_pool().await;
    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let fake_patient_id = Uuid::new_v4();
    let response = server
        .get(&format!("/api/v1/patients/{fake_patient_id}/documents"))
        .await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

// ─── Upload to non-existent patient → 404 ────────────────────────────────────

#[tokio::test]
async fn upload_to_nonexistent_patient_returns_404() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let email = format!("doc_404_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &email, "titulaire").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let token = common::login(&server, &email).await;

    let nonexistent_id = Uuid::new_v4();
    let (body, boundary) = build_multipart_body("photo", "test.jpg", b"DATA");
    let content_type = format!("multipart/form-data; boundary={boundary}");

    let response = server
        .post(&format!("/api/v1/patients/{nonexistent_id}/documents/upload"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&token))
        .add_header(
            axum::http::header::CONTENT_TYPE,
            content_type.parse::<HeaderValue>().unwrap(),
        )
        .bytes(body.into())
        .await;

    assert_eq!(response.status_code(), StatusCode::NOT_FOUND);

    common::cleanup(&pool, cabinet_id).await;
}

// ─── RBAC: assistant cannot upload ───────────────────────────────────────────

#[tokio::test]
async fn upload_by_assistant_returns_403() {
    let pool = common::test_pool().await;
    let cabinet_id = common::create_test_cabinet(&pool).await;
    let dr_email = format!("doc_rbac_dr_{}@test.smiled.io", Uuid::new_v4());
    let asst_email = format!("doc_rbac_asst_{}@test.smiled.io", Uuid::new_v4());

    common::create_test_user(&pool, cabinet_id, &dr_email, "titulaire").await;
    common::create_test_user(&pool, cabinet_id, &asst_email, "assistant").await;

    let config = common::test_config();
    let state = smiled_server::state::AppState::new(pool.clone(), config);
    let server = axum_test::TestServer::new(common::build_app(state)).unwrap();

    let dr_token = common::login(&server, &dr_email).await;
    let patient_id = common::create_patient(&server, &dr_token).await;

    let asst_token = common::login(&server, &asst_email).await;

    let (body, boundary) = build_multipart_body("photo", "test.jpg", b"DATA");
    let content_type = format!("multipart/form-data; boundary={boundary}");

    let response = server
        .post(&format!("/api/v1/patients/{patient_id}/documents/upload"))
        .add_header(axum::http::header::AUTHORIZATION, auth_header(&asst_token))
        .add_header(
            axum::http::header::CONTENT_TYPE,
            content_type.parse::<HeaderValue>().unwrap(),
        )
        .bytes(body.into())
        .await;

    assert_eq!(response.status_code(), StatusCode::FORBIDDEN);

    common::cleanup(&pool, cabinet_id).await;
}
