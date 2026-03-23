-- Migration 011: RLS hardening + schema fixes
-- Fixes: WITH CHECK clauses, FORCE RLS, current_setting crash-safety,
--        soft-delete filtering, monetary precision, missing indexes/constraints

-- ============================================================================
-- 1. Drop existing RLS policies and recreate with WITH CHECK + crash-safe
-- ============================================================================

-- audit_log
DROP POLICY IF EXISTS tenant_isolation ON audit_log;
CREATE POLICY tenant_isolation ON audit_log
  USING (
    current_setting('app.current_tenant', true) IS NOT NULL
    AND cabinet_id = current_setting('app.current_tenant', true)::uuid
  )
  WITH CHECK (
    cabinet_id = current_setting('app.current_tenant', true)::uuid
  );
ALTER TABLE audit_log FORCE ROW LEVEL SECURITY;

-- patient (with soft-delete filter on read path)
DROP POLICY IF EXISTS tenant_isolation ON patient;
CREATE POLICY tenant_isolation ON patient
  USING (
    current_setting('app.current_tenant', true) IS NOT NULL
    AND cabinet_id = current_setting('app.current_tenant', true)::uuid
    AND deleted_at IS NULL
  )
  WITH CHECK (
    cabinet_id = current_setting('app.current_tenant', true)::uuid
  );
ALTER TABLE patient FORCE ROW LEVEL SECURITY;

-- Allow reading soft-deleted patients for admin/audit purposes
CREATE POLICY tenant_isolation_deleted ON patient
  FOR SELECT
  USING (
    current_setting('app.current_tenant', true) IS NOT NULL
    AND cabinet_id = current_setting('app.current_tenant', true)::uuid
    AND deleted_at IS NOT NULL
  );

-- questionnaire_medical
DROP POLICY IF EXISTS tenant_isolation ON questionnaire_medical;
CREATE POLICY tenant_isolation ON questionnaire_medical
  USING (
    current_setting('app.current_tenant', true) IS NOT NULL
    AND cabinet_id = current_setting('app.current_tenant', true)::uuid
  )
  WITH CHECK (
    cabinet_id = current_setting('app.current_tenant', true)::uuid
  );
ALTER TABLE questionnaire_medical FORCE ROW LEVEL SECURITY;

-- schema_dentaire
DROP POLICY IF EXISTS tenant_isolation ON schema_dentaire;
CREATE POLICY tenant_isolation ON schema_dentaire
  USING (
    current_setting('app.current_tenant', true) IS NOT NULL
    AND cabinet_id = current_setting('app.current_tenant', true)::uuid
  )
  WITH CHECK (
    cabinet_id = current_setting('app.current_tenant', true)::uuid
  );
ALTER TABLE schema_dentaire FORCE ROW LEVEL SECURITY;

-- diagnostic (with soft-delete filter)
DROP POLICY IF EXISTS tenant_isolation ON diagnostic;
CREATE POLICY tenant_isolation ON diagnostic
  USING (
    current_setting('app.current_tenant', true) IS NOT NULL
    AND cabinet_id = current_setting('app.current_tenant', true)::uuid
    AND deleted_at IS NULL
  )
  WITH CHECK (
    cabinet_id = current_setting('app.current_tenant', true)::uuid
  );
ALTER TABLE diagnostic FORCE ROW LEVEL SECURITY;

-- plan_traitement (with soft-delete filter)
DROP POLICY IF EXISTS tenant_isolation ON plan_traitement;
CREATE POLICY tenant_isolation ON plan_traitement
  USING (
    current_setting('app.current_tenant', true) IS NOT NULL
    AND cabinet_id = current_setting('app.current_tenant', true)::uuid
    AND deleted_at IS NULL
  )
  WITH CHECK (
    cabinet_id = current_setting('app.current_tenant', true)::uuid
  );
ALTER TABLE plan_traitement FORCE ROW LEVEL SECURITY;

-- evenement_dent
DROP POLICY IF EXISTS tenant_isolation ON evenement_dent;
CREATE POLICY tenant_isolation ON evenement_dent
  USING (
    current_setting('app.current_tenant', true) IS NOT NULL
    AND cabinet_id = current_setting('app.current_tenant', true)::uuid
  )
  WITH CHECK (
    cabinet_id = current_setting('app.current_tenant', true)::uuid
  );
ALTER TABLE evenement_dent FORCE ROW LEVEL SECURITY;

-- prothese_amovible (with soft-delete filter)
DROP POLICY IF EXISTS tenant_isolation ON prothese_amovible;
CREATE POLICY tenant_isolation ON prothese_amovible
  USING (
    current_setting('app.current_tenant', true) IS NOT NULL
    AND cabinet_id = current_setting('app.current_tenant', true)::uuid
    AND deleted_at IS NULL
  )
  WITH CHECK (
    cabinet_id = current_setting('app.current_tenant', true)::uuid
  );
ALTER TABLE prothese_amovible FORCE ROW LEVEL SECURITY;

-- document
DROP POLICY IF EXISTS tenant_isolation ON document;
CREATE POLICY tenant_isolation ON document
  USING (
    current_setting('app.current_tenant', true) IS NOT NULL
    AND cabinet_id = current_setting('app.current_tenant', true)::uuid
  )
  WITH CHECK (
    cabinet_id = current_setting('app.current_tenant', true)::uuid
  );
ALTER TABLE document FORCE ROW LEVEL SECURITY;

-- ============================================================================
-- 2. Monetary columns: add NUMERIC(10,2) precision
-- ============================================================================

-- plan_traitement
ALTER TABLE plan_traitement
  ALTER COLUMN prix_total TYPE NUMERIC(10, 2),
  ALTER COLUMN reste_a_charge_estime TYPE NUMERIC(10, 2);

-- ligne_pdt
ALTER TABLE ligne_pdt
  ALTER COLUMN prix_praticien TYPE NUMERIC(10, 2),
  ALTER COLUMN base_secu TYPE NUMERIC(10, 2);

-- acte
ALTER TABLE acte
  ALTER COLUMN base_secu TYPE NUMERIC(10, 2),
  ALTER COLUMN prix_defaut TYPE NUMERIC(10, 2),
  ALTER COLUMN plafond_rac_0 TYPE NUMERIC(10, 2);

-- tarif_cabinet
ALTER TABLE tarif_cabinet
  ALTER COLUMN prix TYPE NUMERIC(10, 2);

-- occlusion (clinical measurements: mm precision)
ALTER TABLE occlusion
  ALTER COLUMN overjet_mm TYPE NUMERIC(5, 1),
  ALTER COLUMN overbite_mm TYPE NUMERIC(5, 1),
  ALTER COLUMN dvo_mm TYPE NUMERIC(5, 1);

-- ============================================================================
-- 3. Missing indexes and constraints
-- ============================================================================

-- Soft-delete partial indexes for common queries
CREATE INDEX idx_patient_active ON patient(cabinet_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_diagnostic_active ON diagnostic(cabinet_id, patient_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_pdt_active ON plan_traitement(cabinet_id, patient_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_prothese_active ON prothese_amovible(cabinet_id, patient_id) WHERE deleted_at IS NULL;

-- audit_log cabinet index
CREATE INDEX idx_audit_log_cabinet ON audit_log(cabinet_id, created_at DESC);

-- teinte uniqueness constraint
CREATE UNIQUE INDEX idx_teinte_systeme_code
  ON teinte (systeme, code, COALESCE(cabinet_id, '00000000-0000-0000-0000-000000000000'::uuid));

-- ============================================================================
-- 4. diagnostic: add updated_at column + trigger
-- ============================================================================

ALTER TABLE diagnostic ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ DEFAULT now();
CREATE TRIGGER trg_diagnostic_updated_at
  BEFORE UPDATE ON diagnostic
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
