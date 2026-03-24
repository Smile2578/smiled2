# Smiled.IO Phase 2 — Security Hardening + Feature Complete + UX Polish

> **Date:** 2026-03-24
> **Status:** Approved (reviewed, all issues resolved)
> **Author:** Claude (brainstorming session with Simon)
> **Scope:** Phase A (Security + Better Auth migration) + Phase B (Feature complete + UX polish)
> **Prerequisite:** Phase 1 Core Clinique complete (12/12 tasks done)

---

## 1. Context & Motivation

Phase 1 delivered the Core Clinique MVP: dental charting, questionnaire, diagnostic, treatment plans, documents, and reference data. A comprehensive audit (5 agents) + deep research (4 agents) revealed:

- **12 critical security issues** (no rate limiting, JWT in localStorage, no audit log, RBAC not enforced)
- **6 RGPD/HDS compliance gaps** (no erasure, no audit trail, consent optional)
- **3 missing API modules** (Cabinet, Motifs, Prothese amovible — 13 endpoints)
- **15 UX issues** (no auto-save, silent errors, no keyboard nav, no validation)
- **Unique market position**: no French dental software is open source + self-hosted

Phase 2 addresses security and feature completeness in parallel to produce a **demo-ready, security-hardened product**.

---

## 2. Architecture Changes

### 2.1 Better Auth Migration

**Decision:** Replace custom JWT auth with Better Auth running in Nuxt's Nitro server.

**Why:**
- Native httpOnly cookies (eliminates XSS token theft — audit issue C2)
- Built-in rate limiting on auth endpoints (audit issue C1)
- Session revocation on logout (audit issue C10 — logout was silently 404ing)
- Password reset flow with email (replaces custom Argon2-hashed reset tokens — audit issue C6/H6)
- SSR-safe authentication (fixes audit issue H8 — auth middleware was client-only)
- 2FA ready for HDS compliance (Horizon 2)

**Architecture:**

```
Browser
  ├── Better Auth Client (authClient)
  │   └── POST /api/auth/* → Nuxt Nitro Server
  │       └── Better Auth handler (sessions, cookies, email)
  │           └── PostgreSQL (auth schema: user, session, account, verification)
  │
  └── API calls with session cookie
      └── GET/POST /api/v1/* → Rust Axum Server
          └── Session validation middleware
              └── Query auth.session + public.utilisateur
              └── Inject AuthUser (user_id, cabinet_id, role)
              └── Handler executes with tenant context
```

**Database layout:**

```sql
-- Better Auth tables in separate schema
CREATE SCHEMA IF NOT EXISTS auth;

-- Better Auth creates: auth.user, auth.session, auth.account, auth.verification
-- We link auth.user to our utilisateur table:
-- auth.user.id = public.utilisateur.auth_user_id (new FK column)

-- Our tables remain in public schema, untouched except:
ALTER TABLE public.utilisateur ADD COLUMN auth_user_id UUID UNIQUE REFERENCES auth.user(id);
```

**RBAC strategy:** Keep our existing `role_permission` + `user_permission_override` tables (14 roles × 23 permissions = 178 mappings). Better Auth only handles authentication, not authorization. The Rust middleware reads the user's role from `utilisateur`, then queries `role_permission` with moka cache (5-min TTL).

### 2.2 Unified Error Handling (Rust)

Replace 10 duplicate `XxxApiError` enums with a single `ApiError` (~500 lines removed):

```rust
// src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Not found")]
    NotFound,
    #[error("Validation: {0}")]
    Validation(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Forbidden")]
    Forbidden,
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("Storage: {0}")]
    Storage(String),
    #[error("Internal")]
    Internal(String),
}
```

Single `IntoResponse` impl. All handlers return `Result<Json<ApiResponse<T>>, ApiError>`.

### 2.3 Middleware Stack (Rust)

```rust
ServiceBuilder::new()
    .layer(SetRequestIdLayer::new(x_request_id, MakeRequestUuid))
    .layer(PropagateRequestIdLayer::new(x_request_id))
    .layer(TraceLayer::new_for_http())
    .layer(cors)
    .layer(TimeoutLayer::new(Duration::from_secs(30)))
    .layer(GovernorLayer { config: global_governor })  // 100 req/10s per IP
    .layer(DefaultBodyLimit::max(10 * 1024 * 1024))    // 10 MB
    .layer(security_headers)
    .layer(from_fn_with_state(state, audit_middleware))  // auto audit log
```

---

## 3. Phase A — Security Hardening

### A1. Unified ApiError
- Create `src/error.rs` with single `ApiError` enum + `IntoResponse`
- Migrate all handlers from `XxxApiError` to `ApiError`
- Delete 10 module-specific error enums (~500 lines removed)
- **Must complete before B1a/B1b/B1c** so new modules use ApiError from the start

### A2. Rate Limiting
- Add `tower-governor` to Cargo.toml
- Global: 100 req/10s per IP on all `/api/v1/*` routes
- Auth-specific rate limiting handled by Better Auth

### A3. Audit Log Middleware
- Create `src/audit/middleware.rs`
- Tower middleware: intercept POST/PUT/PATCH/DELETE responses with 2xx status
- Fire-and-forget INSERT into `audit_log` (tokio::spawn, don't block response)
- Capture: user_id, cabinet_id, method, path, status, request_id
- Add `GET /api/v1/audit-log` endpoint (titulaire/admin only, paginated)

### A4. RBAC Enforcement
- Create `src/auth/rbac.rs`
- `has_permission(pool, user_id, cabinet_id, permission_name) -> bool`
- Cache permissions per user in moka (5-min TTL, invalidate on role change)
- Replace all `can_write_clinical()` / `can_manage_settings()` with specific permission checks
- Middleware extractor: `RequirePermission("patient.write_clinical")`

### A5. Better Auth Migration
- **Nuxt side:**
  - Install `better-auth` + `@better-auth/nuxt`
  - Create `server/auth.ts`: PostgreSQL adapter, email+password plugin, admin plugin
  - Create `server/api/auth/[...all].ts`: catch-all route
  - Rewrite `stores/auth.ts`: use `authClient` methods
  - Rewrite `middleware/auth.global.ts`: check `useSession()` (SSR-safe)
  - Update login.vue, forgot-password.vue, reset-password.vue to use Better Auth forms
  - Delete `composables/useAuth.ts` (dead wrapper)
- **Rust side:**
  - Delete `src/auth/jwt.rs`, `src/auth/password.rs`, `src/api/v1/auth.rs`
  - Rewrite `src/auth/middleware.rs`: validate session cookie by querying `auth.session` table
  - Add `auth_user_id` FK column to `utilisateur` table (new migration)
  - Session validation (detailed below)
- **Database:**
  - New migration 014: `CREATE SCHEMA IF NOT EXISTS auth` (schema only — Better Auth creates its own tables on first Nuxt startup via its PostgreSQL adapter)
  - New migration 015: `ALTER TABLE utilisateur ADD COLUMN auth_user_id UUID`
  - Seed script: create auth.user entries linked to existing utilisateur records
- **Password migration strategy:** Existing passwords are Argon2id-hashed. Better Auth uses its own hashing (bcrypt by default). Options: (a) force password reset for all users on first login post-migration, or (b) write a custom Better Auth password verifier that checks Argon2 first, then re-hashes to Better Auth format on successful login. Option (b) recommended for zero-downtime migration.

#### A5.1 Session Validation Protocol (Rust side)

Better Auth stores sessions with these columns in `auth.session`:
```sql
-- Better Auth auto-created table (DO NOT create manually)
auth.session (
  id          TEXT PRIMARY KEY,
  "userId"    TEXT NOT NULL REFERENCES auth."user"(id),
  token       TEXT NOT NULL UNIQUE,  -- session token (raw, unhashed)
  "expiresAt" TIMESTAMPTZ NOT NULL,
  "ipAddress" TEXT,
  "userAgent" TEXT,
  "createdAt" TIMESTAMPTZ DEFAULT now(),
  "updatedAt" TIMESTAMPTZ DEFAULT now()
)
```

The Rust middleware reads the `better-auth.session_token` cookie (raw value = the `token` column directly, no hashing needed) and validates:

```rust
// src/auth/session.rs — conceptual query
let row = sqlx::query(
    r#"SELECT s.token, s."expiresAt", u.id as auth_user_id,
              ut.id as user_id, ut.cabinet_id, ut.role, ut.actif
       FROM auth.session s
       JOIN auth."user" u ON u.id = s."userId"
       JOIN public.utilisateur ut ON ut.auth_user_id = u.id
       WHERE s.token = $1 AND s."expiresAt" > NOW()"#
)
.bind(&session_token)
.fetch_optional(&state.pool)
.await?;
```

Note: Uses `sqlx::query()` (runtime, not macro) because `auth.*` tables are created by Better Auth at runtime, not by sqlx migrations. No compile-time schema verification for auth tables.

#### A5.2 SSR Auth Middleware (Nuxt side)

```typescript
// middleware/auth.global.ts — rewritten for Better Auth
export default defineNuxtRouteMiddleware(async (to) => {
  const { data: session } = await useSession()  // SSR-safe, reads cookie
  const publicPaths = ['/login', '/forgot-password', '/reset-password']
  if (!session.value && !publicPaths.includes(to.path)) {
    return navigateTo('/login')
  }
})
```

### A6. Graceful Shutdown
- `axum::serve(...).with_graceful_shutdown(shutdown_signal())`
- Handle SIGTERM + Ctrl+C

### A7. Body Size Limit
- `DefaultBodyLimit::max(10 * 1024 * 1024)` global
- Document upload keeps its own 50MB limit via multipart config

### A8. Magic Byte Validation
- Add `infer` crate to Cargo.toml
- Validate actual file content in `document/handlers.rs` upload handler
- Log MIME mismatch (Content-Type vs detected) as warning

### A9. Request ID Tracing
- `SetRequestIdLayer` + `PropagateRequestIdLayer` from tower-http
- Request ID included in all log entries and error responses
- JSON structured logging in production (`RUST_ENV=production`)

### A10. Deep Health Check
- `GET /api/v1/health` — shallow (existing, for load balancers)
- `GET /api/v1/health/deep` — checks DB connectivity + S3 bucket access + latency

### A11. Async SMTP
- Replace `SmtpTransport` with `AsyncSmtpTransport<Tokio1Executor>` in email sending
- Note: password reset email may move to Better Auth's built-in email system

---

## 4. Phase B — Feature Complete + UX Polish

### B1. Missing Backend Modules

#### B1a. Cabinet Management
```
GET    /api/v1/cabinet              → cabinet info
PUT    /api/v1/cabinet              → update cabinet settings
GET    /api/v1/cabinet/users        → list cabinet users
POST   /api/v1/cabinet/users/invite → invite user (sends email via Better Auth)
PUT    /api/v1/cabinet/users/:id    → update user role/status
```
Permission: `settings.manage`

#### B1b. Motifs de Consultation (partial — GET/POST already exist)
```
PUT    /api/v1/motifs/:id    → update motif    (MISSING)
DELETE /api/v1/motifs/:id    → soft delete      (MISSING)
```
GET and POST already exist in `src/reference/motif.rs`. Table `motif_consultation` already exists (migration 005). Only add PUT and DELETE handlers to the existing module. No new migration needed. Permission: `settings.manage` for write, `patient.read_admin` for read.

#### B1c. Prothese Amovible
```
GET    /api/v1/patients/:id/protheses-amovibles       → list
POST   /api/v1/patients/:id/protheses-amovibles       → create
PUT    /api/v1/patients/:id/protheses-amovibles/:pa_id → update
DELETE /api/v1/patients/:id/protheses-amovibles/:pa_id → soft delete
```
New module `src/core/prothese/`. Table `prothese_amovible` already exists (migration 003). No new migration needed — only API module. Permission: `patient.write_clinical`.

### B2. Backend Performance Fixes

#### B2a. N+1 Documents
Replace loop in `document/queries.rs` with single query:
```sql
SELECT d.*, COALESCE(array_agg(dd.dent_fdi) FILTER (WHERE dd.dent_fdi IS NOT NULL), '{}') as linked_teeth
FROM document d
LEFT JOIN document_dent dd ON dd.document_id = d.id
WHERE d.patient_id = $1
GROUP BY d.id
```

#### B2b. N+1 Teeth Init
Replace 96 sequential INSERTs with 3 batch INSERTs using UNNEST for teeth, faces, and paro sites.

#### B2c. Moka Cache for Reference Data
Cache actes, materiaux, teintes per cabinet_id with 5-min TTL. Invalidate on write.

#### B2d. pg_trgm Search Index
```sql
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE INDEX idx_patient_search_trgm ON patient
  USING gin ((lower(nom) || ' ' || lower(prenom)) gin_trgm_ops)
  WHERE deleted_at IS NULL;
```

#### B2e. Enum Consistency
Fix `temporaire` → `lacteale` in **frontend only** (types.ts, schema.vue, dental chart components). The database and backend already use `lacteale` correctly (migration 003 CHECK constraint). No migration needed.

### B3. Frontend UX

#### B3a. Auto-Save Composable
```typescript
// composables/useAutoSave.ts
export function useAutoSave<T>(data: Ref<T>, saveFn: (d: T) => Promise<void>, delay = 1500)
```
- Debounced watch on data changes
- Visual indicator: spinner → checkmark with timestamp
- Apply to: questionnaire, paro, fiche patient, tooth detail

#### B3b. Dental Chart Popover Rapide
Replace `ToothDetailPanel.vue` (side panel) with a contextual `Popover` positioned at click coordinates:
- Quick status buttons (1 click = change + auto-save)
- Face quick-select (toggle face state)
- Inline notes textarea (auto-save on blur)
- "Details complets" link opens full panel if needed

#### B3c. Medical Alert Banner
Persistent banner on all patient sub-pages showing critical questionnaire flags:
- AVK, Endocardite, Bisphosphonates, Allergies, Radiotherapie
- Derived from questionnaire data, visible without switching tabs
- Amber background, compact single line

#### B3d. Zod Validation
Add Zod schemas for all form inputs:
- Patient create/edit (nom, prenom, date_naissance, num_ss format, email)
- FDI validation (valid numbers only: quadrant 1-4, tooth 1-8)
- Paro values (profondeur 0-15, recession 0-10)
- Treatment plan lines (prix >= 0, acte_id required)

#### B3e. Fix Silent Errors
Replace all empty `catch {}` blocks with user-visible toast notifications:
- Use shadcn-vue `Toast` component
- Pattern: `catch (e) { toast({ title: 'Erreur', description: extractMessage(e), variant: 'destructive' }) }`

#### B3f. Keyboard Navigation Dental Chart
- Arrow keys: navigate between teeth (Left/Right = adjacent, Up/Down = opposite arch)
- Enter/Space: open popover on selected tooth
- Escape: close popover, deselect
- Tab: cycle through face selectors in popover

#### B3g. PDT Autocomplete Actes
Replace manual `acte_libelle` + `prix_unitaire` text input with searchable dropdown:
- Search actes catalog by code or libelle
- Auto-fill: code, nomenclature, prix from tarif_cabinet or acte.prix_unitaire
- Optional materiau + teinte selectors
- Uses cached actes data (moka cache backend / client-side cache frontend)

#### B3h. Complete Sidebar Roles (14 roles)
Map all 14 roles in `default.vue` sidebar:
- titulaire, associe, collaborateur, remplacant
- specialiste_odf, specialiste_co, specialiste_mbd
- assistant, assistant_formation, aspbd
- secretaire, comptable, prothesiste, admin

#### B3i. RBAC UI
- Load user permissions on login (from API)
- `v-if="hasPermission('patient.read_clinical')"` on menu items and page sections
- Redirect unauthorized users to dashboard with message
- Composable `usePermissions()` with `hasPermission(name: string): boolean`

#### B3j. Canvas Accessibility
- Add `role="img"` and `aria-label` to canvas element
- Fallback HTML table inside `<canvas>` for screen readers
- `tabindex="0"` for keyboard focus
- Announce selected tooth via `aria-live="polite"` region

#### B3k. OffscreenCanvas Layer Cache
- Background layer: all teeth rendered once, cached until data changes
- Foreground: only selected tooth highlight redrawn per frame
- Hash-based invalidation: recompute only when teeth data changes

### B4. Frontend Cleanup

- Extract `formatDate`, `formatPrice`, `couvertureLabel`, `couvertureBadgeVariant` → `utils/display.ts`
- Remove dead `apiFetch` imports from `useDocument.ts`, `usePdt.ts`
- Remove `console.error` from `useDentalChart.ts`
- Remove unused deps: `radix-vue`, `@nuxtjs/color-mode`
  - **Keep** `@vueuse/core` and `@vueuse/nuxt` — used by shadcn-vue UI components (Textarea, Input, Label, TabsTrigger etc.)
- Remove unused Rust crate: `anyhow` (confirmed zero imports in src/)
- Fix double navigation in login.vue
- Fix double patient fetch in `[id].vue` (use provide/inject)
- Remove `#[allow(unused_imports)]` suppressions, fix underlying issues
- Delete `src/core/historique/mod.rs` (dead re-export)

---

## 5. New Dependencies

### Rust (Cargo.toml)
| Crate | Version | Purpose |
|-------|---------|---------|
| `tower-governor` | 0.4 | Rate limiting |
| `infer` | 0.16 | Magic byte file validation |
| `tokio-util` | 0.7 (features: io) | Streaming S3 downloads |

### Remove from Rust
| Crate | Reason |
|-------|--------|
| `anyhow` | Unused (thiserror used everywhere) |
| `jsonwebtoken` | Replaced by Better Auth sessions |
| `argon2` | Replaced by Better Auth password hashing |

### Frontend (package.json)
| Package | Purpose |
|---------|---------|
| `better-auth` | Auth framework |
| `@better-auth/nuxt` | Nuxt integration |
| `zod` | Already in package.json but unused — now actually used for form validation (B3d) |

### Remove from Frontend
| Package | Reason |
|---------|--------|
| `radix-vue` | Unused (shadcn-vue uses reka-ui) |
| `@nuxtjs/color-mode` | Not wired up, no UI toggle |

**Keep:** `@vueuse/core`, `@vueuse/nuxt` — required by shadcn-vue generated components (useVModel, reactiveOmit in 20+ UI files).

---

## 6. Database Migrations

| # | Migration | Description |
|---|-----------|-------------|
| 014 | `create_auth_schema.sql` | `CREATE SCHEMA IF NOT EXISTS auth` (schema only — Better Auth creates its own tables via PostgreSQL adapter on first Nuxt startup) |
| 015 | `link_utilisateur_auth.sql` | Add `auth_user_id UUID` column to `utilisateur`, add UNIQUE constraint |
| 016 | `add_pg_trgm.sql` | `CREATE EXTENSION IF NOT EXISTS pg_trgm` + trigram search index on patient |
| 017 | `audit_log_columns.sql` | Add `method`, `path`, `status_code`, `request_id` columns to existing `audit_log` table for middleware-based logging |

Note: Motifs and Prothese amovible tables already exist (migrations 005, 003). No new migrations needed for those modules.

---

## 7. Parallel Execution Strategy

Phase A and Phase B run simultaneously on separate concerns:

```
Phase A (Backend Security)          Phase B (Features + UX)
─────────────────────────           ─────────────────────────
A1. Unified ApiError                B1a. Cabinet module
A2. Rate limiting                   B1b. Motifs module
A3. Audit middleware                B1c. Prothese amovible module
A4. RBAC enforcement                B2. Backend perf fixes
A5. Better Auth (backend)           B3. Frontend UX (10 items)
A6. Graceful shutdown               B4. Frontend cleanup
A7. Body size limit
A8. Magic byte validation           B5. Better Auth (frontend)
A9. Request ID tracing                  ↑ depends on A5
A10. Deep health check
A11. Async SMTP
```

**Dependencies:**
- B5 (frontend Better Auth) depends on A5 (backend Better Auth)
- B1a/B1b/B1c (new backend modules) depend on A1 (Unified ApiError) — new modules must use `ApiError`, not create new per-module error enums
- A1 should be the **first** Phase A task completed

**Agent allocation:**
- 2-3 agents on Phase A (security + Better Auth backend)
- 2-3 agents on Phase B (missing modules + UX + cleanup)
- 1 agent on Better Auth frontend (starts after A5 completes)

---

## 8. Success Criteria

- [ ] All 12 critical audit issues resolved
- [ ] Better Auth login flow works end-to-end (email/password + httpOnly cookies)
- [ ] Session revocation works (logout invalidates server-side)
- [ ] RBAC enforced: comptable cannot access clinical data
- [ ] Audit log captures all mutations with user/cabinet/action
- [ ] Rate limiting active: global 100 req/10s per IP on Rust API + Better Auth's built-in auth rate limiting
- [ ] All 3 missing API modules implemented with tests
- [ ] Auto-save works on questionnaire, paro, dental chart
- [ ] Dental chart popover replaces side panel
- [ ] Medical alert banner visible on all patient tabs
- [ ] Zod validation on all form inputs
- [ ] No silent error swallowing (all catch blocks show toast)
- [ ] Keyboard navigation works on dental chart
- [ ] Build passes (`pnpm run build` + `cargo build`)
- [ ] All existing tests pass + new tests for new modules

## 9. Anti-Goals

- No offline-first (Phase 3)
- No IA integration (Phase 3)
- No SESAM-Vitale (Horizon 3-4)
- No i18n (Phase 3)
- No dark mode (Phase 3)
- No patient portal (Phase 3)
- No voice input (Phase 3)
