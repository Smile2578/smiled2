use axum::{
    extract::{DefaultBodyLimit, State},
    http::{header, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tower_cookies::CookieManagerLayer;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::{
    cors::{Any, CorsLayer},
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    set_header::SetResponseHeaderLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// Re-use all modules from the library crate
use smiled_server::{
    api,
    config::Config,
    core::document::storage::S3Storage,
    db::pool::create_pool,
    state::AppState,
};

#[tokio::main]
async fn main() {
    // Initialize structured logging (JSON in production, human-readable otherwise)
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    let is_production = std::env::var("RUST_ENV")
        .map(|v| v == "production")
        .unwrap_or(false);

    if is_production {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .init();
    }

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

    let s3 = S3Storage::new(
        &cfg.s3_endpoint,
        &cfg.s3_bucket,
        &cfg.s3_access_key,
        &cfg.s3_secret_key,
    )
    .await;

    if let Err(e) = s3.ensure_bucket().await {
        tracing::warn!("S3 bucket setup failed (continuing without S3): {e}");
    }

    let state = AppState::with_s3(pool, cfg.clone(), s3);
    let cors = build_cors(&cfg);
    let app = build_router(state, cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.server_port));
    info!("Smiled.IO server listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Server error");
}

/// Wait for Ctrl+C (all platforms) or SIGTERM (Unix) to trigger graceful shutdown.
async fn shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    {
        let mut sigterm =
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("Failed to register SIGTERM handler");
        tokio::select! {
            _ = ctrl_c => info!("Received Ctrl+C, shutting down gracefully"),
            _ = sigterm.recv() => info!("Received SIGTERM, shutting down gracefully"),
        }
    }

    #[cfg(not(unix))]
    {
        ctrl_c.await.expect("Failed to listen for Ctrl+C");
        info!("Received Ctrl+C, shutting down gracefully");
    }
}

fn build_cors(cfg: &Config) -> CorsLayer {
    let allowed_headers = [
        header::AUTHORIZATION,
        header::CONTENT_TYPE,
        header::ACCEPT,
        header::ORIGIN,
        header::HeaderName::from_static("x-cabinet-id"),
        header::HeaderName::from_static("x-requested-with"),
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
        tracing::warn!(
            "CORS_ORIGINS is empty or wildcard -- using permissive defaults. \
             Set CORS_ORIGINS explicitly in production."
        );
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
    // Global rate limit: 100 requests per 10 seconds per IP
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(10)
            .burst_size(100)
            .finish()
            .expect("Invalid rate limit config"),
    );

    let audit_state = state.clone();

    let api_routes = api::v1::router()
        .layer(GovernorLayer {
            config: governor_conf,
        })
        .with_state(state.clone());

    let health_routes = Router::new()
        .route("/api/v1/health/deep", get(deep_health_handler))
        .with_state(state);

    let x_request_id = header::HeaderName::from_static("x-request-id");

    Router::new()
        .route("/api/v1/health", get(health_handler))
        .merge(health_routes)
        .merge(api_routes)
        .layer(axum::middleware::from_fn_with_state(
            audit_state,
            smiled_server::audit::middleware::audit_layer,
        ))
        .layer(CookieManagerLayer::new())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10 MB
        .layer(cors)
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .layer(TraceLayer::new_for_http())
        .layer(SetRequestIdLayer::new(x_request_id, MakeRequestUuid))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
}

// ─── Health Check ────────────────────────────────────────────────────────────

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

// ─── Deep Health Check ───────────────────────────────────────────────────────

#[derive(Serialize)]
struct DeepHealthResponse {
    status: &'static str,
    version: &'static str,
    checks: DeepHealthChecks,
}

#[derive(Serialize)]
struct DeepHealthChecks {
    database: CheckResult,
    s3: CheckResult,
}

#[derive(Serialize)]
struct CheckResult {
    status: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

/// GET /api/v1/health/deep -- checks DB connectivity and S3 bucket access.
/// Returns 200 if all healthy, 503 if DB is down.
async fn deep_health_handler(State(state): State<AppState>) -> impl IntoResponse {
    let db_check = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.pool)
        .await;

    let database = match db_check {
        Ok(_) => CheckResult {
            status: "ok",
            error: None,
        },
        Err(e) => CheckResult {
            status: "error",
            error: Some(e.to_string()),
        },
    };

    let s3 = match &state.s3 {
        Some(storage) => match storage.check_bucket_access().await {
            Ok(()) => CheckResult {
                status: "ok",
                error: None,
            },
            Err(e) => CheckResult {
                status: "error",
                error: Some(e.to_string()),
            },
        },
        None => CheckResult {
            status: "skipped",
            error: Some("S3 not configured".to_string()),
        },
    };

    let status = if database.status == "ok" {
        "ok"
    } else {
        "degraded"
    };
    let http_status = if database.status == "ok" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        http_status,
        Json(DeepHealthResponse {
            status,
            version: env!("CARGO_PKG_VERSION"),
            checks: DeepHealthChecks { database, s3 },
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
