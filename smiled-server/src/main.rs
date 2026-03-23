use axum::{
    http::{header, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::{net::SocketAddr, time::Duration};
use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Re-use all modules from the library crate
use smiled_server::{
    api,
    config::Config,
    db::pool::create_pool,
    state::AppState,
};

#[tokio::main]
async fn main() {
    // Initialize structured logging
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = Config::from_env().unwrap_or_else(|e| {
        eprintln!("Configuration error: {e}");
        std::process::exit(1);
    });

    let pool = create_pool(&cfg.database_url)
        .await
        .unwrap_or_else(|e| {
            eprintln!("Database connection error: {e}");
            std::process::exit(1);
        });

    let state = AppState::new(pool, cfg.clone());
    let cors = build_cors(&cfg);
    let app = build_router(state, cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.server_port));
    info!("Smiled.IO server listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

fn build_cors(cfg: &Config) -> CorsLayer {
    let allowed_headers = [
        header::AUTHORIZATION,
        header::CONTENT_TYPE,
        header::ACCEPT,
        header::ORIGIN,
        header::HeaderName::from_static("x-cabinet-id"),
    ];

    let allowed_methods = [
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
        Method::OPTIONS,
    ];

    if cfg.cors_origins.is_empty() || cfg.cors_origins == ["*"] {
        return CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(allowed_methods)
            .allow_headers(Any);
    }

    CorsLayer::new()
        .allow_origin(cfg.cors_origins_parsed.clone())
        .allow_methods(allowed_methods)
        .allow_headers(allowed_headers)
        .allow_credentials(true)
}

fn build_router(state: AppState, cors: CorsLayer) -> Router {
    Router::new()
        .route("/api/v1/health", get(health_handler))
        .merge(api::v1::router())
        .with_state(state)
        .layer(cors)
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(TraceLayer::new_for_http())
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
    service: &'static str,
}

async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(HealthResponse {
            status: "ok",
            version: env!("CARGO_PKG_VERSION"),
            service: "smiled-server",
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    fn test_cors() -> CorsLayer {
        CorsLayer::new().allow_origin(Any)
    }

    #[tokio::test]
    async fn health_returns_ok() {
        let app = Router::new()
            .route("/api/v1/health", get(health_handler))
            .layer(test_cors());

        let server = TestServer::new(app).unwrap();
        let response = server.get("/api/v1/health").await;
        assert_eq!(response.status_code(), StatusCode::OK);

        let body: serde_json::Value = response.json();
        assert_eq!(body["status"], "ok");
        assert_eq!(body["service"], "smiled-server");
    }

    #[tokio::test]
    async fn unknown_route_returns_404() {
        let app = Router::new()
            .route("/api/v1/health", get(health_handler))
            .layer(test_cors());

        let server = TestServer::new(app).unwrap();
        let response = server.get("/api/v1/unknown").await;
        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
    }
}
