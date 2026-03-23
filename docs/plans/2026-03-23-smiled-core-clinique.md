# Smiled.IO Core Clinique Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the Core Clinique module of Smiled.IO — a dental software with interactive dental charting, medical questionnaire, diagnosis, and treatment planning.

**Architecture:** Rust/Axum monolith backend with PostgreSQL (RLS multi-tenant) + Nuxt 4 frontend with Canvas 2D dental chart. Self-hostable via Docker Compose.

**Tech Stack:** Rust (Axum, sqlx, JWT), Nuxt 4 (Vue 3, TailwindCSS, shadcn-vue), PostgreSQL 17, S3/MinIO

**Spec:** `docs/superpowers/specs/2026-03-23-smiled-core-clinique-design.md`

---

### Task 1: Project Scaffold & Build System

**Files:**
- Create: `smiled-server/Cargo.toml`, `smiled-server/src/main.rs`, `smiled-server/src/config.rs`
- Create: `smiled-web/package.json`, `smiled-web/nuxt.config.ts`, `smiled-web/app/app.vue`
- Create: `docker-compose.yml`, `smiled-server/Dockerfile`
- Create: `smiled-server/.env.example`, `smiled-web/.env.example`

- [ ] Init Rust project: `cargo init smiled-server`, add all crates (axum, sqlx, tokio, serde, jsonwebtoken, argon2, validator, moka, lettre, aws-sdk-s3, tower-http, uuid)
- [ ] Write `main.rs` with basic Axum server (health check endpoint `GET /api/v1/health`)
- [ ] Write `config.rs` loading env vars (DATABASE_URL, JWT_SECRET, S3_ENDPOINT, TENANT_MODE, CORS_ORIGINS)
- [ ] Verify server starts: `cargo run` → `curl localhost:8080/api/v1/health`
- [ ] Init Nuxt 4 project: `npx nuxi@latest init smiled-web`, add tailwindcss, shadcn-vue, pinia, zod
- [ ] Create basic `app.vue` with "Smiled.IO" title
- [ ] Verify frontend starts: `npm run dev` → browser shows page
- [ ] Write `docker-compose.yml` with postgres:17 + minio services
- [ ] Write `smiled-server/Dockerfile` (multi-stage Rust build)
- [ ] Verify `docker compose up db minio` starts both services
- [ ] Commit

---

### Task 2: Database Schema & Migrations

**Files:**
- Create: `smiled-server/migrations/001_init.sql` through `010_seed_permissions.sql`
- Create: `smiled-server/src/db/mod.rs`, `smiled-server/src/db/pool.rs`

- [ ] Write migration `001_init.sql`: `set_updated_at()` trigger, `audit_log`, `cabinet`, `utilisateur`, `permission`, `role_permission`, `user_permission_override` tables
- [ ] Write migration `002_patient.sql`: `patient`, `questionnaire_medical` tables
- [ ] Write migration `003_schema_dentaire.sql`: `schema_dentaire`, `dent`, `face_dent`, `paro_site`, `occlusion`, `atm`, `paro_global`, `prothese_amovible`, `evenement_dent` tables
- [ ] Write migration `004_diagnostic_pdt.sql`: `diagnostic`, `finding`, `plan_traitement`, `ligne_pdt` tables
- [ ] Write migration `005_reference_data.sql`: `categorie_acte`, `acte`, `acte_materiau`, `tarif_cabinet`, `categorie_materiau`, `materiau`, `teinte`, `motif_consultation`, `trouble_medical`, `document`, `document_dent` tables
- [ ] Write migration `006_rls.sql`: Enable RLS + policies on all tenant-scoped tables
- [ ] Write migration `007_seed_actes.sql`: ~150 actes (14 categories, CCAM/NGAP/HN)
- [ ] Write migration `008_seed_materiaux.sql`: ~80 matériaux (full hierarchy)
- [ ] Write migration `009_seed_teintes.sql`: VITA Classical + 3D-Master
- [ ] Write migration `010_seed_permissions.sql`: permissions + role_permission matrix (14 roles)
- [ ] Write `db/pool.rs` and `db/mod.rs`: PgPool creation
- [ ] Run migrations: `sqlx migrate run` → verify all tables created
- [ ] Commit

---

### Task 3: Auth & Tenant Middleware

**Files:**
- Create: `smiled-server/src/auth/mod.rs`, `jwt.rs`, `middleware.rs`, `permissions.rs`, `password.rs`
- Create: `smiled-server/src/tenant/mod.rs`, `middleware.rs`
- Create: `smiled-server/src/api/mod.rs`, `smiled-server/src/api/v1/mod.rs`, `auth.rs`
- Test: `smiled-server/tests/auth_test.rs`

- [ ] Write `auth/jwt.rs`: create_token, validate_token
- [ ] Write `auth/password.rs`: hash_password (argon2), verify_password
- [ ] Write `auth/middleware.rs`: AuthUser extractor from JWT + set tenant
- [ ] Write `tenant/middleware.rs`: set_tenant via `SET app.current_tenant`
- [ ] Write `auth/permissions.rs`: check_permission querying role_permission + overrides
- [ ] Write `api/v1/auth.rs`: POST /login, POST /refresh, GET /me
- [ ] Write test: login flow, JWT validation, /me endpoint
- [ ] Run tests → PASS
- [ ] Add POST /forgot-password, POST /reset-password
- [ ] Commit

---

### Task 4: Patient CRUD + Questionnaire API

**Files:**
- Create: `smiled-server/src/core/patient/mod.rs`, `handlers.rs`, `queries.rs`, `types.rs`
- Create: `smiled-server/src/core/patient/questionnaire.rs`
- Test: `smiled-server/tests/patient_test.rs`

- [ ] Write types: Patient, CreatePatient, UpdatePatient, QuestionnaireInput
- [ ] Write queries: insert, get_by_id, list (search + pagination), update, soft_delete
- [ ] Write handlers: GET/POST/PUT /patients, GET/PUT /patients/:id/questionnaire
- [ ] Write tests: CRUD patient, questionnaire with all risk sections
- [ ] Run tests → PASS
- [ ] Wire routes in api/v1/mod.rs
- [ ] Commit

---

### Task 5: Reference Data CRUD (Actes, Matériaux, Teintes)

**Files:**
- Create: `smiled-server/src/core/acte/mod.rs`, `handlers.rs`, `queries.rs`, `types.rs`
- Create: `smiled-server/src/core/materiau/mod.rs`, `handlers.rs`, `queries.rs`, `types.rs`
- Create: `smiled-server/src/reference/teinte.rs`, `motif.rs`, `trouble.rs`
- Test: `smiled-server/tests/reference_test.rs`

- [ ] Write acte CRUD: list (merge système + cabinet), create, update, toggle, tarif override
- [ ] Write materiau CRUD: list with hierarchy, create, update
- [ ] Write teinte/motif/trouble CRUD
- [ ] Write tests: verify seed data, create custom acte, override tarif
- [ ] Run tests → PASS
- [ ] Commit

---

### Task 6: Dental Schema Backend (Versioning + API)

**Files:**
- Create: `smiled-server/src/core/schema_dentaire/mod.rs`, `handlers.rs`, `queries.rs`, `types.rs`
- Create: `smiled-server/src/core/historique/mod.rs`, `handlers.rs`
- Test: `smiled-server/tests/schema_dentaire_test.rs`

- [ ] Write types: DentalSchema, Tooth, ToothFace, ParoSite, Occlusion, Atm, ParoGlobal
- [ ] Write POST /patients/:id/schema: create version, init 32 teeth + faces + paro sites
- [ ] Write GET /patients/:id/schema: return full schema (teeth + faces + paro)
- [ ] Write PUT /patients/:id/schema/dent/:fdi: update tooth state
- [ ] Write GET/PUT paro, occlusion, atm, paro-global endpoints
- [ ] Write GET /patients/:id/schema/versions
- [ ] Write GET /patients/:id/dent/:fdi/timeline
- [ ] Write tests: create v1, modify, create v2, verify versioning
- [ ] Run tests → PASS
- [ ] Commit

---

### Task 7: Diagnostic & Treatment Plans Backend

**Files:**
- Create: `smiled-server/src/core/diagnostic/mod.rs`, `handlers.rs`, `queries.rs`, `types.rs`
- Create: `smiled-server/src/core/pdt/mod.rs`, `handlers.rs`, `queries.rs`, `types.rs`
- Test: `smiled-server/tests/pdt_test.rs`

- [ ] Write diagnostic CRUD with findings
- [ ] Write PDT CRUD with ligne_pdt (acte, dent, materiau, prix)
- [ ] Write 4-formule logic sharing same diagnostic
- [ ] Write PDF generation stub
- [ ] Write tests: diagnostic → 4 PDTs → prix_total, ligne statut
- [ ] Run tests → PASS
- [ ] Commit

---

### Task 8: Documents & S3 Integration

**Files:**
- Create: `smiled-server/src/core/document/mod.rs`, `handlers.rs`, `storage.rs`, `types.rs`
- Test: `smiled-server/tests/document_test.rs`

- [ ] Write storage.rs: S3 abstraction (upload, download, presigned URL)
- [ ] Write POST /patients/:id/documents/upload (multipart)
- [ ] Write GET /patients/:id/documents
- [ ] Write POST /documents/:id/link-dent/:fdi
- [ ] Write tests with MinIO
- [ ] Run tests → PASS
- [ ] Commit

---

### Task 9: Nuxt 4 Frontend — Auth & Layout

**Files:**
- Create: `smiled-web/app/layouts/default.vue`, `auth.vue`
- Create: `smiled-web/app/pages/login.vue`, `forgot-password.vue`, `reset-password.vue`
- Create: `smiled-web/app/composables/useAuth.ts`, `useApi.ts`
- Create: `smiled-web/app/middleware/auth.ts`

- [ ] Install + configure shadcn-vue, add core components (button, input, card, sidebar, dialog, table, form)
- [ ] Write useApi.ts: fetch wrapper with JWT + error handling
- [ ] Write useAuth.ts: login, logout, refresh, user state
- [ ] Write auth.vue layout + login.vue page
- [ ] Write auth middleware (redirect to /login)
- [ ] Write default.vue layout: sidebar + header
- [ ] Write pages/index.vue: dashboard placeholder
- [ ] Verify login flow works end-to-end
- [ ] Commit

---

### Task 10: Frontend — Patients & Questionnaire

**Files:**
- Create: `smiled-web/app/pages/patients/index.vue`, `[id].vue`, `[id]/questionnaire.vue`
- Create: `smiled-web/app/composables/usePatient.ts`
- Create: `smiled-web/app/components/questionnaire/Section*.vue` (6 sections)

- [ ] Write usePatient.ts composable
- [ ] Write patients/index.vue: searchable list with pagination
- [ ] Write patients/[id].vue: patient fiche with tabs
- [ ] Write 6 questionnaire section components with zod validation
- [ ] Write patients/[id]/questionnaire.vue: assembles sections
- [ ] Verify: create patient → fill questionnaire → save → reload persists
- [ ] Commit

---

### Task 11: Frontend — Interactive Dental Chart (Canvas 2D)

**Files:**
- Create: `smiled-web/app/components/dental-chart/DentalChart.vue`, `ToothRenderer.ts`, `ChartInteraction.ts`, `ChartLegend.vue`, `types.ts`, `fdi-geometry.ts`
- Create: `smiled-web/app/components/dental-chart/ToothDetailPanel.vue`
- Create: `smiled-web/app/pages/patients/[id]/schema.vue`
- Create: `smiled-web/app/composables/useDentalChart.ts`

- [ ] Write types.ts: TypeScript types matching Rust types
- [ ] Write fdi-geometry.ts: tooth positions + face hit areas on Canvas
- [ ] Write ToothRenderer.ts: draw tooth with color-coded states + faces
- [ ] Write ChartInteraction.ts: click → identify tooth/face
- [ ] Write DentalChart.vue: Canvas orchestrator, draw all teeth, emit events
- [ ] Write ToothDetailPanel.vue: sidebar with edit forms (statut, endo, prothèse, paro)
- [ ] Write useDentalChart.ts: fetch/save schema via API
- [ ] Write schema.vue: chart + panel + mode switcher + version selector
- [ ] Write ChartLegend.vue
- [ ] Verify: open patient → see teeth → click → edit → save → persists
- [ ] Commit

---

### Task 12: Frontend — Paro, Diagnostic, PDT, Documents, Admin

**Files:**
- Create: remaining pages and components for paro, diagnostic, PDT, documents, historique, admin (actes, materiaux, cabinet, users)

- [ ] Write ParoChart.vue: grid input for 6 sites × N teeth
- [ ] Write paro.vue page + paro global
- [ ] Write diagnostic.vue: create/view findings
- [ ] Write PdtBuilder.vue + PdtFormuleSelect.vue
- [ ] Write pdts.vue: formule tabs + builder + PDF download
- [ ] Write documents.vue: upload + list + link-to-tooth
- [ ] Write historique.vue: timeline per tooth
- [ ] Write actes/index.vue: catalogue CRUD + tarif override
- [ ] Write materiaux/index.vue: hierarchical CRUD
- [ ] Write cabinet/index.vue + users.vue
- [ ] Update docker-compose.yml with full stack (server + web)
- [ ] E2E: docker compose up → login → full patient workflow
- [ ] Commit
