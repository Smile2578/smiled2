# Code Deletion Log

## 2026-03-24 — Rust Backend Dead Code Cleanup

### Unused Files Deleted
- `src/auth/permissions.rs` — `check_permission()` had 0 call sites in the entire codebase

### Unused Methods Deleted
- `S3Storage::presigned_url` in `src/core/document/storage.rs` — 0 call sites; also removed the now-unused `presigning::PresigningConfig` import, `std::time::Duration` import, and the `StorageError::Presign` variant
- `AppState::new` in `src/state.rs` — 0 call sites; `main.rs` uses only `AppState::with_s3`

### Unused Re-exports Removed
- `pub use crate::core::schema_dentaire::queries::get_tooth_timeline;` from `src/core/historique/mod.rs` — handlers import directly from `schema_dentaire::queries`
- `#[allow(unused_imports)] pub use pool::create_pool;` from `src/db/mod.rs` — `main.rs` already uses `db::pool::create_pool` directly
- `#[allow(unused_imports)] pub use middleware::AuthUser;` from `src/auth/mod.rs` — all handlers use `crate::auth::middleware::AuthUser` directly

### Visibility Tightened
- `insert_tooth`, `insert_tooth_faces`, `insert_paro_sites` in `src/core/schema_dentaire/queries.rs` — changed from `pub` to private; only called from `init_schema_teeth` within the same file

### Impact
- Files deleted: 1 (`permissions.rs`)
- Methods removed: 2 (`presigned_url`, `AppState::new`)
- Enum variants removed: 1 (`StorageError::Presign`)
- Dead re-exports removed: 3
- Suppressed `#[allow(unused_imports)]` directives removed: 2
- Functions made private: 3

### Testing
- `cargo check`: passing
