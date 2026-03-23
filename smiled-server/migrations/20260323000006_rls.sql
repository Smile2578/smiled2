-- Migration 006: Row Level Security (RLS) for all tenant-scoped tables
-- Policy pattern: cabinet_id must match current_setting('app.current_tenant')::uuid
-- Tables without direct cabinet_id (dent, face_dent, etc.) are accessed only
-- through schema_dentaire which has RLS, so no duplicate RLS needed on children.

-- ─── Helper: current tenant UUID ─────────────────────────────────────────────
-- The application middleware sets app.current_tenant before each query.
-- Superuser connections (migrations, admin scripts) bypass RLS automatically.

-- ─── audit_log ───────────────────────────────────────────────────────────────
ALTER TABLE audit_log ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON audit_log
  USING (cabinet_id = current_setting('app.current_tenant')::uuid);

-- ─── patient ──────────────────────────────────────────────────────────────────
ALTER TABLE patient ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON patient
  USING (cabinet_id = current_setting('app.current_tenant')::uuid);

-- ─── questionnaire_medical ───────────────────────────────────────────────────
ALTER TABLE questionnaire_medical ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON questionnaire_medical
  USING (cabinet_id = current_setting('app.current_tenant')::uuid);

-- ─── schema_dentaire ─────────────────────────────────────────────────────────
ALTER TABLE schema_dentaire ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON schema_dentaire
  USING (cabinet_id = current_setting('app.current_tenant')::uuid);

-- ─── diagnostic ──────────────────────────────────────────────────────────────
ALTER TABLE diagnostic ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON diagnostic
  USING (cabinet_id = current_setting('app.current_tenant')::uuid);

-- ─── plan_traitement ─────────────────────────────────────────────────────────
ALTER TABLE plan_traitement ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON plan_traitement
  USING (cabinet_id = current_setting('app.current_tenant')::uuid);

-- ─── evenement_dent ──────────────────────────────────────────────────────────
ALTER TABLE evenement_dent ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON evenement_dent
  USING (cabinet_id = current_setting('app.current_tenant')::uuid);

-- ─── prothese_amovible ───────────────────────────────────────────────────────
ALTER TABLE prothese_amovible ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON prothese_amovible
  USING (cabinet_id = current_setting('app.current_tenant')::uuid);

-- ─── document ────────────────────────────────────────────────────────────────
ALTER TABLE document ENABLE ROW LEVEL SECURITY;

CREATE POLICY tenant_isolation ON document
  USING (cabinet_id = current_setting('app.current_tenant')::uuid);

-- ─── Note on child tables (dent, face_dent, paro_site, etc.) ─────────────────
-- These are always accessed via JOIN through schema_dentaire (or dent), which
-- already has RLS enforced. Direct SELECT on child tables without a parent join
-- would return rows but they cannot be matched to a patient without traversal.
-- If stricter isolation is required in a future migration, add:
--   CREATE POLICY tenant_isolation ON dent USING (
--     schema_id IN (SELECT id FROM schema_dentaire WHERE cabinet_id = current_setting('app.current_tenant')::uuid)
--   );
