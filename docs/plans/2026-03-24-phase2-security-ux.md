# Smiled.IO Phase 2 — Security + Feature Complete + UX Polish

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Harden security (Better Auth, rate limiting, audit log, RBAC enforcement) and complete features (3 missing API modules, 10 UX improvements) to produce a demo-ready product.

**Architecture:** Better Auth in Nuxt Nitro handles auth (sessions, cookies, password). Rust Axum becomes a pure API server validating sessions by reading Better Auth's `auth.session` table. RBAC uses existing 14-role permission matrix with moka cache.

**Tech Stack:** Rust (Axum, sqlx, tower-governor, moka, infer), Nuxt 4 (Better Auth, Zod, shadcn-vue), PostgreSQL 17 (RLS, pg_trgm)

**Spec:** `docs/specs/2026-03-24-phase2-security-ux-design.md`

---

## Dependency Graph

```
A1 (Unified ApiError) ──────┬──→ B1a (Cabinet module)
                             ├──→ B1b (Motifs PUT/DELETE)
                             ├──→ B1c (Prothese amovible)
                             └──→ A2-A4, A6-A11 (other security tasks)

A5 (Better Auth backend) ───→ B5 (Better Auth frontend)

Independent (start immediately):
  B2 (Backend perf), B3 (Frontend UX), B4 (Frontend cleanup)
```

---

### Task 1: Unified ApiError (A1) — MUST BE FIRST

**Files:**
- Create: `smiled-server/src/error.rs`
- Modify: `smiled-server/src/lib.rs`
- Modify: `smiled-server/src/core/patient/handlers.rs`
- Modify: `smiled-server/src/core/patient/types.rs` (move ApiResponse here → `src/types.rs`)
- Modify: ALL handler files (acte, materiau, diagnostic, pdt, document, schema_dentaire, reference/*)
- Modify: `smiled-server/src/api/v1/auth.rs`

- [ ] Create `src/error.rs` with unified `ApiError` enum:

```rust
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Ressource introuvable")]
    NotFound,
    #[error("{0}")]
    Validation(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("Action non autorisee")]
    Forbidden,
    #[error("{0}")]
    Conflict(String),
    #[error("Erreur base de donnees")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    Storage(String),
    #[error("Erreur interne")]
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            ApiError::Validation(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg.clone()),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, self.to_string()),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            ApiError::Database(e) => {
                tracing::error!(error = %e, "Database error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur serveur".to_string())
            }
            ApiError::Storage(msg) => {
                tracing::error!(error = %msg, "Storage error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur de stockage".to_string())
            }
            ApiError::Internal(msg) => {
                tracing::error!(error = %msg, "Internal error");
                (StatusCode::INTERNAL_SERVER_ERROR, "Erreur serveur".to_string())
            }
        };
        (status, Json(json!({ "success": false, "error": message }))).into_response()
    }
}
```

- [ ] Create `src/types.rs` — move `ApiResponse<T>` from `core/patient/types.rs`:

```rust
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<PaginationMeta>,
}

#[derive(Debug, Serialize)]
pub struct PaginationMeta {
    pub total: i64,
    pub page: i64,
    pub limit: i64,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self { success: true, data: Some(data), error: None, meta: None }
    }
    pub fn with_meta(data: T, meta: PaginationMeta) -> Self {
        Self { success: true, data: Some(data), error: None, meta: Some(meta) }
    }
    pub fn error(msg: impl Into<String>) -> Self {
        Self { success: false, data: None, error: Some(msg.into()), meta: None }
    }
}
```

- [ ] Update `src/lib.rs` to export new modules: `pub mod error; pub mod types;`
- [ ] Migrate each handler module: replace local `XxxApiError` with `use crate::error::ApiError` and `use crate::types::ApiResponse`
  - Start with `patient/handlers.rs` as template
  - Then: acte, materiau, diagnostic, pdt, document, schema_dentaire
  - Then: reference/teinte.rs, reference/motif.rs, reference/trouble.rs
  - Then: api/v1/auth.rs
- [ ] Delete all `XxxApiError` enums from each module's types.rs or handlers.rs
- [ ] Delete `ApiResponse<T>` from `patient/types.rs` (now in `src/types.rs`)
- [ ] Verify: `cargo build` passes
- [ ] Verify: `cargo test` passes
- [ ] Commit: `refactor: unify error handling with single ApiError enum`

---

### Task 2: Security Infrastructure (A2, A6, A7, A8, A9, A10, A11)

**Files:**
- Modify: `smiled-server/Cargo.toml` (add `infer`, `tokio-util`)
- Modify: `smiled-server/src/main.rs`
- Modify: `smiled-server/src/core/document/handlers.rs`
- Modify: `smiled-server/src/api/v1/auth.rs` (async SMTP)
- Modify: `smiled-server/src/db/pool.rs` (pool tuning)

- [ ] Add crates to Cargo.toml: `infer = "0.16"`, `tokio-util = { version = "0.7", features = ["io"] }`
- [ ] Remove `anyhow = "1"` from Cargo.toml (confirmed zero imports)

- [ ] Add graceful shutdown to `main.rs`:

```rust
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! { _ = ctrl_c => {}, _ = terminate => {} }
    tracing::info!("Shutdown signal received");
}
```

Update serve call: `axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await`

- [ ] Add `DefaultBodyLimit` to middleware stack: `.layer(axum::extract::DefaultBodyLimit::max(10 * 1024 * 1024))`

- [ ] Add `SetRequestIdLayer` + `PropagateRequestIdLayer`:

```rust
use tower_http::request_id::{MakeRequestUuid, SetRequestIdLayer, PropagateRequestIdLayer};
let x_request_id = axum::http::HeaderName::from_static("x-request-id");
// Add to ServiceBuilder:
.layer(SetRequestIdLayer::new(x_request_id.clone(), MakeRequestUuid))
.layer(PropagateRequestIdLayer::new(x_request_id))
```

- [ ] Add rate limiting with tower-governor (already in Cargo.toml):

```rust
use tower_governor::{GovernorConfigBuilder, GovernorLayer};
let governor_conf = GovernorConfigBuilder::default()
    .per_second(10)
    .burst_size(100)
    .finish()
    .unwrap();
// Apply to api routes: .layer(GovernorLayer { config: governor_conf })
```

- [ ] Add deep health check endpoint:

```rust
async fn health_deep_handler(State(state): State<AppState>) -> impl IntoResponse {
    let db_ok = sqlx::query_scalar!("SELECT 1 as \"v!: i32\"")
        .fetch_one(&state.pool).await.is_ok();
    let s3_ok = match &state.s3 {
        Some(s) => s.client.head_bucket().bucket(&s.bucket).send().await.is_ok(),
        None => true,
    };
    let status = if db_ok { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    (status, Json(json!({
        "status": if db_ok { "ok" } else { "degraded" },
        "checks": { "database": db_ok, "s3": s3_ok }
    })))
}
```

Add route: `.route("/api/v1/health/deep", get(health_deep_handler))`

- [ ] Add magic byte validation in `document/handlers.rs`:

```rust
fn validate_file_content(data: &[u8], claimed_mime: &str) -> Result<(), ApiError> {
    let detected = infer::get(data);
    let mime = detected.map(|k| k.mime_type()).unwrap_or("application/octet-stream");
    let allowed = matches!(mime, "image/jpeg" | "image/png" | "image/webp" | "application/pdf");
    if !allowed {
        return Err(ApiError::Validation(format!("Type de fichier non autorise: {}", mime)));
    }
    if detected.is_some() && detected.unwrap().mime_type() != claimed_mime {
        tracing::warn!(claimed = claimed_mime, detected = mime, "MIME type mismatch");
    }
    Ok(())
}
```

Call after reading bytes in upload handler, before S3 upload.

- [ ] Fix async SMTP in `api/v1/auth.rs`: replace `SmtpTransport` with `AsyncSmtpTransport::<lettre::Tokio1Executor>` and `.await` on send

- [ ] Tune connection pool in `db/pool.rs`:

```rust
PgPoolOptions::new()
    .max_connections(max_connections)
    .min_connections(2)
    .acquire_timeout(Duration::from_secs(5))
    .idle_timeout(Duration::from_secs(600))
    .max_lifetime(Duration::from_secs(1800))
    .connect(database_url)
    .await
```

- [ ] Add JSON structured logging for production in `main.rs`:

```rust
let is_production = std::env::var("RUST_ENV").map(|v| v == "production").unwrap_or(false);
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
```

- [ ] Verify: `cargo build` passes
- [ ] Verify: `cargo test` passes
- [ ] Commit: `feat: add security infrastructure (rate limiting, graceful shutdown, request ID, magic bytes, deep health check)`

---

### Task 3: Audit Log Middleware (A3)

**Files:**
- Create: `smiled-server/src/audit/mod.rs`
- Create: `smiled-server/src/audit/middleware.rs`
- Create: `smiled-server/src/audit/handlers.rs`
- Create: `smiled-server/migrations/20260324000017_audit_log_columns.sql`
- Modify: `smiled-server/src/main.rs` (add middleware layer)
- Modify: `smiled-server/src/lib.rs` (add `pub mod audit`)
- Modify: `smiled-server/src/api/v1/mod.rs` (add audit routes)

- [ ] Write migration `017_audit_log_columns.sql`:

```sql
ALTER TABLE audit_log ADD COLUMN IF NOT EXISTS method TEXT;
ALTER TABLE audit_log ADD COLUMN IF NOT EXISTS path TEXT;
ALTER TABLE audit_log ADD COLUMN IF NOT EXISTS status_code INTEGER;
ALTER TABLE audit_log ADD COLUMN IF NOT EXISTS request_id TEXT;
```

- [ ] Write `src/audit/mod.rs`:

```rust
pub mod middleware;
pub mod handlers;
```

- [ ] Write `src/audit/middleware.rs`:

```rust
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use crate::state::AppState;
use crate::auth::middleware::AuthUser;

pub async fn audit_layer(
    State(state): State<AppState>,
    auth_user: Option<AuthUser>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let request_id = request.headers()
        .get("x-request-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let response = next.run(request).await;
    let status_code = response.status().as_u16() as i32;

    // Only log mutations with successful responses
    if method != "GET" && method != "OPTIONS" && (200..300).contains(&(status_code as usize)) {
        if let Some(user) = auth_user {
            let pool = state.pool.clone();
            tokio::spawn(async move {
                let _ = sqlx::query(
                    r#"INSERT INTO audit_log (cabinet_id, user_id, action, entity_type, method, path, status_code, request_id)
                       VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#
                )
                .bind(user.cabinet_id)
                .bind(user.user_id)
                .bind(&method)
                .bind(&path)
                .bind(&method)
                .bind(&path)
                .bind(status_code)
                .bind(&request_id)
                .execute(&pool)
                .await;
            });
        }
    }

    response
}
```

- [ ] Write `src/audit/handlers.rs`:

```rust
use axum::{extract::{Query, State}, Json};
use crate::{error::ApiError, types::ApiResponse, state::AppState, auth::middleware::AuthUser};

#[derive(serde::Deserialize)]
pub struct AuditQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub async fn list_audit_logs(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<AuditQuery>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, ApiError> {
    if !["titulaire", "admin"].contains(&auth_user.role.as_str()) {
        return Err(ApiError::Forbidden);
    }
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = (query.page.unwrap_or(1) - 1) * limit;

    let logs = sqlx::query_as!(
        serde_json::Value,
        "SELECT row_to_json(al.*) as \"log!\" FROM audit_log al WHERE al.cabinet_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        auth_user.cabinet_id, limit, offset
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(ApiResponse::success(logs)))
}
```

- [ ] Add audit middleware to router in `main.rs`: `.layer(axum::middleware::from_fn_with_state(state.clone(), audit::middleware::audit_layer))`
- [ ] Add audit route in `api/v1/mod.rs`: `.route("/audit-log", get(audit::handlers::list_audit_logs))`
- [ ] Run migration: `sqlx migrate run`
- [ ] Verify: `cargo build` + `cargo test`
- [ ] Commit: `feat: add audit log middleware with automatic mutation logging`

---

### Task 4: RBAC Enforcement (A4)

**Files:**
- Create: `smiled-server/src/auth/rbac.rs`
- Modify: `smiled-server/src/auth/mod.rs`
- Modify: `smiled-server/src/auth/middleware.rs`
- Modify: ALL handler files (replace hardcoded role checks)
- Modify: `smiled-server/src/state.rs` (add permission cache)

- [ ] Add permission cache to `state.rs`:

```rust
use moka::future::Cache;
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
    pub s3: Option<S3Storage>,
    pub permission_cache: Cache<(Uuid, String), bool>,  // (user_id, permission) -> allowed
}
```

Initialize with `Cache::builder().max_capacity(10_000).time_to_live(Duration::from_secs(300)).build()`

- [ ] Write `src/auth/rbac.rs`:

```rust
use sqlx::PgPool;
use uuid::Uuid;
use crate::error::ApiError;

pub async fn has_permission(pool: &PgPool, user_id: Uuid, cabinet_id: Uuid, permission: &str) -> Result<bool, ApiError> {
    // Check user override first
    let override_row = sqlx::query!(
        r#"SELECT autorise FROM user_permission_override
           WHERE utilisateur_id = $1 AND permission_id = (SELECT id FROM permission WHERE nom = $2)"#,
        user_id, permission
    )
    .fetch_optional(pool)
    .await?;

    if let Some(row) = override_row {
        return Ok(row.autorise);
    }

    // Fallback to role-based permission
    let has = sqlx::query_scalar!(
        r#"SELECT EXISTS(
            SELECT 1 FROM role_permission rp
            JOIN permission p ON p.id = rp.permission_id
            JOIN utilisateur u ON u.role = rp.role
            WHERE u.id = $1 AND u.cabinet_id = $2 AND p.nom = $3
        ) as "exists!: bool""#,
        user_id, cabinet_id, permission
    )
    .fetch_one(pool)
    .await?;

    Ok(has)
}
```

- [ ] Add `RequirePermission` extractor to `auth/middleware.rs`:

```rust
pub struct RequirePermission(pub &'static str);

// Usage in handlers:
// let _perm = RequirePermission("patient.write_clinical").check(&state, &auth_user).await?;
impl RequirePermission {
    pub async fn check(&self, state: &AppState, user: &AuthUser) -> Result<(), ApiError> {
        let allowed = state.permission_cache
            .try_get_with((user.user_id, self.0.to_string()), async {
                rbac::has_permission(&state.pool, user.user_id, user.cabinet_id, self.0).await
            })
            .await
            .map_err(|e| ApiError::Internal(e.to_string()))?;

        if !allowed {
            Err(ApiError::Forbidden)
        } else {
            Ok(())
        }
    }
}
```

- [ ] Replace all `can_write_clinical()` / `can_manage_settings()` calls in handlers with specific permission checks:
  - patient handlers: `RequirePermission("patient.write_clinical")`
  - schema_dentaire handlers: `RequirePermission("patient.write_clinical")`
  - diagnostic handlers: `RequirePermission("diagnostic.create")`
  - pdt handlers: `RequirePermission("pdt.create")`
  - document handlers: `RequirePermission("document.upload")`
  - acte/materiau/teinte handlers: `RequirePermission("settings.manage")`
  - Reference read endpoints: `RequirePermission("patient.read_admin")`
- [ ] Remove `can_write_clinical()` and `can_manage_settings()` from `AuthUser`
- [ ] Verify: `cargo build` + `cargo test`
- [ ] Commit: `feat: enforce granular RBAC permissions with moka cache`

---

### Task 5: Better Auth Backend (A5)

**Files:**
- Create: `smiled-server/migrations/20260324000014_create_auth_schema.sql`
- Create: `smiled-server/migrations/20260324000015_link_utilisateur_auth.sql`
- Create: `smiled-server/src/auth/session.rs`
- Modify: `smiled-server/src/auth/mod.rs`
- Modify: `smiled-server/src/auth/middleware.rs` (rewrite for session validation)
- Delete: `smiled-server/src/auth/jwt.rs`
- Delete: `smiled-server/src/auth/password.rs`
- Modify: `smiled-server/src/api/v1/mod.rs` (remove auth routes)
- Delete: `smiled-server/src/api/v1/auth.rs`
- Modify: `smiled-server/Cargo.toml` (remove jsonwebtoken, argon2)

- [ ] Write migration `014_create_auth_schema.sql`:

```sql
-- Better Auth will create its own tables (user, session, account, verification)
-- via its PostgreSQL adapter on first Nuxt startup.
-- We only create the schema here.
CREATE SCHEMA IF NOT EXISTS auth;
```

- [ ] Write migration `015_link_utilisateur_auth.sql`:

```sql
-- Link our utilisateur table to Better Auth's user table
-- This column will be populated after Better Auth creates its tables
ALTER TABLE utilisateur ADD COLUMN IF NOT EXISTS auth_user_id TEXT UNIQUE;
-- Note: Better Auth uses TEXT ids, not UUID
```

- [ ] Write `src/auth/session.rs`:

```rust
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use sqlx::PgPool;
use uuid::Uuid;
use crate::{error::ApiError, state::AppState};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub cabinet_id: Uuid,
    pub role: String,
    pub email: String,
    pub actif: bool,
}

pub async fn validate_session(pool: &PgPool, session_token: &str) -> Result<AuthUser, ApiError> {
    let row = sqlx::query_as!(
        AuthUserRow,
        r#"SELECT ut.id as "user_id!", ut.cabinet_id as "cabinet_id!",
                  ut.role as "role!", ut.email as "email!", ut.actif as "actif!"
           FROM auth.session s
           JOIN auth."user" u ON u.id = s."userId"
           JOIN public.utilisateur ut ON ut.auth_user_id = u.id
           WHERE s.token = $1 AND s."expiresAt" > NOW()"#,
        session_token
    )
    .fetch_optional(pool)
    .await
    .map_err(|_| ApiError::Unauthorized("Session invalide".into()))?
    .ok_or_else(|| ApiError::Unauthorized("Session expirée ou invalide".into()))?;

    if !row.actif {
        return Err(ApiError::Unauthorized("Compte désactivé".into()));
    }

    Ok(AuthUser {
        user_id: row.user_id,
        cabinet_id: row.cabinet_id,
        role: row.role,
        email: row.email,
        actif: row.actif,
    })
}

#[derive(Debug)]
struct AuthUserRow {
    user_id: Uuid,
    cabinet_id: Uuid,
    role: String,
    email: String,
    actif: bool,
}
```

- [ ] Rewrite `src/auth/middleware.rs` to use session validation:

```rust
use axum::{extract::FromRequestParts, http::request::Parts};
use crate::{state::AppState, error::ApiError};
use super::session::{self, AuthUser};

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &AppState) -> Result<Self, Self::Rejection> {
        // Extract session token from cookie
        let cookie_header = parts.headers.get("cookie")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let session_token = cookie_header.split(';')
            .find_map(|c| {
                let c = c.trim();
                c.strip_prefix("better-auth.session_token=")
            })
            .ok_or_else(|| ApiError::Unauthorized("Session manquante".into()))?;

        session::validate_session(&state.pool, session_token).await
    }
}
```

- [ ] Update `src/auth/mod.rs`:

```rust
pub mod middleware;
pub mod session;
pub mod rbac;
```

- [ ] Remove auth routes from `api/v1/mod.rs` (login, refresh, forgot-password, reset-password — all handled by Better Auth now)
- [ ] Delete `src/auth/jwt.rs`, `src/auth/password.rs`, `src/api/v1/auth.rs`
- [ ] Remove `jsonwebtoken` and `argon2` from Cargo.toml
- [ ] Update seed_demo.rs to note it needs updating post-Better Auth migration
- [ ] Run migrations: `sqlx migrate run`
- [ ] Verify: `cargo build` passes
- [ ] Commit: `feat: replace JWT auth with Better Auth session validation`

---

### Task 6: Better Auth Frontend (B5) — depends on Task 5

**Files:**
- Create: `smiled-web/server/auth.ts`
- Create: `smiled-web/server/api/auth/[...all].ts`
- Create: `smiled-web/app/composables/useAuthClient.ts`
- Modify: `smiled-web/package.json` (add better-auth)
- Modify: `smiled-web/nuxt.config.ts`
- Modify: `smiled-web/app/stores/auth.ts` (full rewrite)
- Modify: `smiled-web/app/middleware/auth.global.ts` (full rewrite)
- Modify: `smiled-web/app/pages/login.vue` (use Better Auth)
- Modify: `smiled-web/app/pages/forgot-password.vue`
- Modify: `smiled-web/app/pages/reset-password.vue`
- Modify: `smiled-web/app/layouts/default.vue` (user from session)

- [ ] Install: `pnpm add better-auth`

- [ ] Create `server/auth.ts` — Better Auth server config:

```typescript
import { betterAuth } from 'better-auth'

export const auth = betterAuth({
  database: {
    provider: 'pg',
    url: process.env.DATABASE_URL || 'postgresql://smiled:smiled@localhost:5433/smiled',
    schema: 'auth',
  },
  emailAndPassword: {
    enabled: true,
    minPasswordLength: 12,
  },
  session: {
    expiresIn: 60 * 60 * 24, // 24 hours
    updateAge: 60 * 60, // refresh every hour
  },
})
```

- [ ] Create `server/api/auth/[...all].ts`:

```typescript
import { auth } from '~/server/auth'
import { toWebHandler } from 'better-auth/node'

const handler = toWebHandler(auth)

export default defineEventHandler(async (event) => {
  return handler(event.node.req, event.node.res)
})
```

- [ ] Rewrite `stores/auth.ts`:

```typescript
import { defineStore } from 'pinia'
import { createAuthClient } from 'better-auth/vue'

const authClient = createAuthClient()

export const useAuthStore = defineStore('auth', () => {
  const session = authClient.useSession()
  const user = computed(() => session.value?.data?.user ?? null)
  const isAuthenticated = computed(() => !!user.value)

  async function login(email: string, password: string) {
    await authClient.signIn.email({ email, password })
    await navigateTo('/')
  }

  async function logout() {
    await authClient.signOut()
    await navigateTo('/login')
  }

  return { user, isAuthenticated, session, login, logout }
})
```

- [ ] Rewrite `middleware/auth.global.ts`:

```typescript
export default defineNuxtRouteMiddleware(async (to) => {
  const authStore = useAuthStore()
  const publicPaths = ['/login', '/forgot-password', '/reset-password']
  if (!authStore.isAuthenticated && !publicPaths.includes(to.path)) {
    return navigateTo('/login')
  }
})
```

- [ ] Rewrite `pages/login.vue` to use `authStore.login(email, password)` (remove double navigation)
- [ ] Rewrite `pages/forgot-password.vue` to use Better Auth password reset
- [ ] Rewrite `pages/reset-password.vue` to use Better Auth password reset
- [ ] Update `layouts/default.vue` to get user from `authStore.user` (session-based)
- [ ] Update `composables/useApi.ts` — remove manual Authorization header (cookies sent automatically)
- [ ] Delete localStorage token logic from stores/auth.ts
- [ ] Verify: `pnpm run build` passes
- [ ] Commit: `feat: migrate frontend auth to Better Auth with httpOnly cookies`

---

### Task 7: Missing Backend Modules (B1a, B1b, B1c) — depends on Task 1

**Files:**
- Create: `smiled-server/src/core/cabinet/mod.rs`
- Create: `smiled-server/src/core/cabinet/handlers.rs`
- Create: `smiled-server/src/core/cabinet/queries.rs`
- Create: `smiled-server/src/core/cabinet/types.rs`
- Create: `smiled-server/src/core/prothese/mod.rs`
- Create: `smiled-server/src/core/prothese/handlers.rs`
- Create: `smiled-server/src/core/prothese/queries.rs`
- Create: `smiled-server/src/core/prothese/types.rs`
- Modify: `smiled-server/src/reference/motif.rs` (add PUT, DELETE)
- Modify: `smiled-server/src/core/mod.rs` (add cabinet, prothese)
- Modify: `smiled-server/src/api/v1/mod.rs` (add routes)

- [ ] **Cabinet module**: Create handlers/queries/types following the same patterns as patient module
  - `GET /api/v1/cabinet` — return cabinet info for auth_user.cabinet_id
  - `PUT /api/v1/cabinet` — update cabinet name, address, phone, siret, rpps_cabinet
  - `GET /api/v1/cabinet/users` — list all users in cabinet
  - `POST /api/v1/cabinet/users/invite` — create user with role (password set via Better Auth)
  - `PUT /api/v1/cabinet/users/:id` — update role, actif status
  - All endpoints require `RequirePermission("settings.manage")`

- [ ] **Motifs CRUD completion**: Add to `reference/motif.rs`:
  - `PUT /api/v1/motifs/:id` — update motif libelle
  - `DELETE /api/v1/motifs/:id` — soft delete (set deleted_at)
  - Require `RequirePermission("settings.manage")`

- [ ] **Prothese amovible module**: Create following schema_dentaire patterns
  - `GET /api/v1/patients/:id/protheses-amovibles` — list for patient
  - `POST /api/v1/patients/:id/protheses-amovibles` — create (type, maxillaire/mandibulaire, nb_dents, materiau, notes)
  - `PUT /api/v1/patients/:id/protheses-amovibles/:pa_id` — update
  - `DELETE /api/v1/patients/:id/protheses-amovibles/:pa_id` — soft delete
  - Table `prothese_amovible` already exists (migration 003)
  - Require `RequirePermission("patient.write_clinical")`

- [ ] Wire all new routes in `api/v1/mod.rs`
- [ ] Verify: `cargo build` + `cargo test`
- [ ] Commit: `feat: add cabinet management, motifs CRUD, prothese amovible API`

---

### Task 8: Backend Performance Fixes (B2)

**Files:**
- Create: `smiled-server/migrations/20260324000016_add_pg_trgm.sql`
- Modify: `smiled-server/src/core/document/queries.rs` (N+1 fix)
- Modify: `smiled-server/src/core/schema_dentaire/queries.rs` (batch INSERT)
- Modify: `smiled-server/src/state.rs` (add ref data caches)
- Modify: `smiled-server/src/core/acte/handlers.rs` (use cache)
- Modify: `smiled-server/src/core/materiau/handlers.rs` (use cache)

- [ ] Write migration `016_add_pg_trgm.sql`:

```sql
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX IF NOT EXISTS idx_patient_search_trgm ON patient
  USING gin ((lower(nom) || ' ' || lower(prenom)) gin_trgm_ops)
  WHERE deleted_at IS NULL;
```

- [ ] Fix N+1 in `document/queries.rs` — replace loop with single query using `array_agg`:

```sql
SELECT d.*, COALESCE(
  array_agg(dd.dent_fdi) FILTER (WHERE dd.dent_fdi IS NOT NULL), '{}'
) as linked_teeth
FROM document d
LEFT JOIN document_dent dd ON dd.document_id = d.id
WHERE d.patient_id = $1 AND d.cabinet_id = $2
GROUP BY d.id
ORDER BY d.created_at DESC
```

- [ ] Fix N+1 in `schema_dentaire/queries.rs` `init_schema_teeth` — replace loop with batch UNNEST INSERT for teeth, faces, and paro sites (3 queries instead of 96)

- [ ] Add moka caches to `AppState` for actes, materiaux, teintes:

```rust
pub acte_cache: Cache<Uuid, Vec<Acte>>,       // cabinet_id -> actes
pub materiau_cache: Cache<Uuid, Vec<Materiau>>,
pub teinte_cache: Cache<String, Vec<Teinte>>,  // "all" -> teintes (not tenant-scoped)
```

- [ ] Use `try_get_with` pattern in acte/materiau list handlers to serve from cache with 5-min TTL
- [ ] Invalidate cache on write (create/update/delete handlers)
- [ ] Run migration: `sqlx migrate run`
- [ ] Verify: `cargo build` + `cargo test`
- [ ] Commit: `perf: fix N+1 queries, add pg_trgm search index, cache reference data`

---

### Task 9: Frontend UX — Auto-save + Error Handling (B3a, B3e)

**Files:**
- Create: `smiled-web/app/composables/useAutoSave.ts`
- Create: `smiled-web/app/composables/useToast.ts` (if shadcn-vue toast not yet set up)
- Modify: `smiled-web/app/pages/patients/[id]/questionnaire.vue` (add auto-save)
- Modify: `smiled-web/app/pages/patients/[id]/paro.vue` (add auto-save)
- Modify: ALL pages with empty catch blocks (diagnostic, pdts, actes, materiaux, teintes)

- [ ] Create `composables/useAutoSave.ts`:

```typescript
import type { Ref } from 'vue'

export function useAutoSave<T>(
  data: Ref<T>,
  saveFn: (d: T) => Promise<void>,
  delay = 1500,
) {
  const saving = ref(false)
  const lastSavedAt = ref<Date | null>(null)
  const error = ref<string | null>(null)
  const debouncedData = useDebounce(data, delay)

  watch(debouncedData, async (newData) => {
    if (!newData) return
    saving.value = true
    error.value = null
    try {
      await saveFn(toRaw(newData) as T)
      lastSavedAt.value = new Date()
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Erreur de sauvegarde'
    } finally {
      saving.value = false
    }
  }, { deep: true })

  return { saving: readonly(saving), lastSavedAt: readonly(lastSavedAt), error: readonly(error) }
}
```

- [ ] Add auto-save to questionnaire.vue: wrap form data ref with `useAutoSave(formData, saveQuestionnaire)`
- [ ] Add auto-save indicator to questionnaire template (spinner/checkmark)
- [ ] Add auto-save to paro.vue

- [ ] Fix ALL silent catch blocks — replace with toast:
  - diagnostic.vue: 3 locations
  - pdts.vue: 4 locations
  - actes/index.vue: 2 locations
  - materiaux/index.vue: 1 location
  - teintes/index.vue: 1 location

  Pattern: `catch (e) { useToast().toast({ title: 'Erreur', description: e instanceof Error ? e.message : 'Erreur inconnue', variant: 'destructive' }) }`

- [ ] Verify: `pnpm run build`
- [ ] Commit: `feat: add auto-save composable and fix silent error swallowing`

---

### Task 10: Frontend UX — Dental Chart Improvements (B3b, B3f, B3j, B3k)

**Files:**
- Modify: `smiled-web/app/pages/patients/[id]/schema.vue` (popover, keyboard)
- Modify: `smiled-web/app/components/dental-chart/DentalChart.vue` (keyboard, accessibility)
- Modify: `smiled-web/app/components/dental-chart/ToothRenderer.ts` (OffscreenCanvas)
- Modify: `smiled-web/app/components/dental-chart/ChartInteraction.ts` (keyboard support)
- Modify: `smiled-web/app/components/dental-chart/types.ts` (fix lacteale enum)

- [ ] Fix enum: replace `'temporaire'` with `'lacteale'` in `types.ts`, `schema.vue`, `DentalChart.vue`

- [ ] Add `tabindex="0"`, `role="img"`, `aria-label` to canvas in `DentalChart.vue`

- [ ] Add keyboard navigation handler to `DentalChart.vue`:

```typescript
function handleKeydown(e: KeyboardEvent) {
  if (!selectedFdi.value) { selectedFdi.value = 11; return }
  switch (e.key) {
    case 'ArrowRight': e.preventDefault(); selectNextTooth(); break
    case 'ArrowLeft': e.preventDefault(); selectPrevTooth(); break
    case 'ArrowDown': case 'ArrowUp': e.preventDefault(); selectOppositeTooth(); break
    case 'Enter': case ' ': e.preventDefault(); emit('tooth-click', selectedFdi.value); break
    case 'Escape': e.preventDefault(); selectedFdi.value = null; break
  }
}
```

- [ ] Add OffscreenCanvas layer cache to `ToothRenderer.ts`:
  - Cache background layer (all teeth) in OffscreenCanvas
  - Invalidate on teeth data hash change
  - Only redraw selected tooth highlight per frame

- [ ] Replace `ToothDetailPanel` with contextual Popover in `schema.vue`:
  - Position at click coordinates from canvas event
  - Quick status buttons (auto-save on click)
  - Face quick-select toggles
  - "Details complets" link for full panel

- [ ] Verify: `pnpm run build`
- [ ] Commit: `feat: dental chart popover, keyboard nav, canvas accessibility, OffscreenCanvas cache`

---

### Task 11: Frontend UX — Remaining Items (B3c, B3d, B3g, B3h, B3i)

**Files:**
- Create: `smiled-web/app/composables/usePermissions.ts`
- Create: `smiled-web/app/utils/display.ts`
- Modify: `smiled-web/app/pages/patients/[id].vue` (medical alert banner, provide/inject)
- Modify: `smiled-web/app/components/pdt/PdtBuilder.vue` (actes autocomplete)
- Modify: `smiled-web/app/layouts/default.vue` (14 roles, RBAC menu)
- Modify: ALL pages with form inputs (add Zod validation)

- [ ] Create `utils/display.ts` — extract shared formatters:

```typescript
export function formatDate(d: string | null): string {
  if (!d) return '-'
  return new Date(d).toLocaleDateString('fr-FR')
}
export function formatPrice(p: number): string {
  return new Intl.NumberFormat('fr-FR', { style: 'currency', currency: 'EUR' }).format(p)
}
export function couvertureLabel(c: string): string {
  const map: Record<string, string> = { mutuelle: 'Mutuelle', cmu: 'CMU-C', ame: 'AME', ald: 'ALD', aucune: 'Aucune' }
  return map[c] || c
}
```

- [ ] Create `composables/usePermissions.ts`:

```typescript
export function usePermissions() {
  const { user } = useAuthStore()
  const permissions = ref<string[]>([])

  async function loadPermissions() {
    if (!user.value) return
    const { apiGet } = useApi()
    const res = await apiGet<string[]>('/api/v1/me/permissions')
    if (res.success && res.data) permissions.value = res.data
  }

  function hasPermission(name: string): boolean {
    return permissions.value.includes(name)
  }

  return { permissions, loadPermissions, hasPermission }
}
```

- [ ] Add medical alert banner to `pages/patients/[id].vue`:
  - Compute alerts from questionnaire data (AVK, endocardite, bisphosphonates, allergies, radiotherapie)
  - Display amber banner below patient header, above tabs
  - Fix double fetch: use `provide('patient', patient)` and `inject` in child pages

- [ ] Add actes autocomplete to `PdtBuilder.vue`:
  - Searchable input that queries actes list (cached)
  - Auto-fill code, nomenclature, prix_unitaire on selection
  - Optional materiau and teinte selectors

- [ ] Complete sidebar roles in `default.vue` (all 14 roles)
- [ ] Add `v-if="hasPermission('...')"` on sidebar items and page sections

- [ ] Add Zod validation schemas for key forms:
  - Patient create/edit: `z.object({ nom: z.string().min(1), prenom: z.string().min(1), date_naissance: z.string(), ... })`
  - Paro values: `z.number().int().min(0).max(15)` for profondeur
  - PDT lines: `z.object({ acte_id: z.string().uuid(), prix_unitaire: z.number().min(0) })`

- [ ] Verify: `pnpm run build`
- [ ] Commit: `feat: medical alerts, RBAC UI, PDT autocomplete, Zod validation`

---

### Task 12: Frontend Cleanup (B4)

**Files:**
- Modify: `smiled-web/package.json` (remove unused deps)
- Modify: `smiled-web/app/composables/useDocument.ts` (remove dead import)
- Modify: `smiled-web/app/composables/usePdt.ts` (remove dead import)
- Modify: `smiled-web/app/composables/useDentalChart.ts` (remove console.error)
- Delete: `smiled-server/src/core/historique/mod.rs`
- Modify: Various pages (replace local formatDate with utils/display.ts import)

- [ ] Remove `radix-vue` and `@nuxtjs/color-mode` from package.json: `pnpm remove radix-vue @nuxtjs/color-mode`
- [ ] Remove dead `apiFetch` imports from `useDocument.ts` and `usePdt.ts`
- [ ] Remove `console.error` calls from `useDentalChart.ts` (lines 23, 90)
- [ ] Delete `smiled-server/src/core/historique/mod.rs` (dead re-export)
- [ ] Remove `#[allow(unused_imports)]` from `auth/mod.rs` and `db/mod.rs`, fix underlying issues
- [ ] Replace all local `formatDate`/`couvertureLabel` definitions with imports from `utils/display.ts`
- [ ] Fix double patient fetch in `[id].vue` / `[id]/index.vue` (use provide/inject pattern)
- [ ] Verify: `pnpm run build` + `cargo build`
- [ ] Commit: `chore: cleanup dead code, unused deps, duplicate utilities`

---

## Parallel Execution Strategy

```
WAVE 1 (start immediately, no dependencies):
  Agent A: Task 1 (Unified ApiError) — PRIORITY, blocks Wave 2
  Agent B: Task 8 (Backend perf fixes)
  Agent C: Task 9 (Auto-save + error handling)
  Agent D: Task 12 (Frontend cleanup)

WAVE 2 (after Task 1 completes):
  Agent A: Task 2 (Security infrastructure)
  Agent B: Task 3 (Audit log middleware)
  Agent C: Task 4 (RBAC enforcement)
  Agent D: Task 7 (Missing backend modules)

WAVE 3 (after Tasks 2-4 complete):
  Agent A: Task 5 (Better Auth backend)
  Agent B: Task 10 (Dental chart improvements)
  Agent C: Task 11 (Remaining UX)

WAVE 4 (after Task 5 completes):
  Agent A: Task 6 (Better Auth frontend)
```
