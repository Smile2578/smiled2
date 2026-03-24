-- pg_trgm extension + trigram index for patient name search
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE INDEX IF NOT EXISTS idx_patient_search_trgm ON patient
  USING gin ((lower(nom) || ' ' || lower(prenom)) gin_trgm_ops)
  WHERE deleted_at IS NULL;
