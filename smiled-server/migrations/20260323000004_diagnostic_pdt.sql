-- Migration 004: Diagnostic, findings and treatment plan tables

-- ─── Diagnostic ──────────────────────────────────────────────────────────────
CREATE TABLE diagnostic (
  id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id      UUID        NOT NULL REFERENCES patient(id),
  cabinet_id      UUID        NOT NULL REFERENCES cabinet(id),
  praticien_id    UUID        NOT NULL REFERENCES utilisateur(id),
  schema_version  INT         NOT NULL,
  date            DATE        NOT NULL DEFAULT CURRENT_DATE,
  created_at      TIMESTAMPTZ DEFAULT now(),
  deleted_at      TIMESTAMPTZ
);

CREATE INDEX idx_diagnostic_patient ON diagnostic(patient_id, date DESC);

-- ─── Finding ─────────────────────────────────────────────────────────────────
-- A clinical observation attached to a diagnostic
-- severite: 1=minimal ... 5=critical
CREATE TABLE finding (
  id             UUID     PRIMARY KEY DEFAULT gen_random_uuid(),
  diagnostic_id  UUID     NOT NULL REFERENCES diagnostic(id) ON DELETE CASCADE,
  type           TEXT     NOT NULL,
  dent_fdi       SMALLINT,
  details        JSONB,
  severite       SMALLINT NOT NULL CHECK (severite BETWEEN 1 AND 5),
  source         TEXT     NOT NULL CHECK (source IN ('praticien', 'ia'))
);

CREATE INDEX idx_finding_diagnostic ON finding(diagnostic_id);

-- ─── Plan de traitement ──────────────────────────────────────────────────────
-- formule mirrors the "lab / compromis / budget" options from the spec
-- FK deferred: acte_id, materiau_id, teinte_id added in 005
CREATE TABLE plan_traitement (
  id                       UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id               UUID        NOT NULL REFERENCES patient(id),
  cabinet_id               UUID        NOT NULL REFERENCES cabinet(id),
  praticien_id             UUID        NOT NULL REFERENCES utilisateur(id),
  diagnostic_id            UUID        NOT NULL REFERENCES diagnostic(id),
  formule                  TEXT        NOT NULL CHECK (formule IN ('lab', 'compromis_1', 'compromis_2', 'budget')),
  statut                   TEXT        NOT NULL DEFAULT 'brouillon' CHECK (statut IN (
    'brouillon', 'presente', 'accepte', 'en_cours', 'termine'
  )),
  prix_total               DECIMAL,
  reste_a_charge_estime    DECIMAL,
  pdf_url                  TEXT,
  created_at               TIMESTAMPTZ DEFAULT now(),
  updated_at               TIMESTAMPTZ DEFAULT now(),
  deleted_at               TIMESTAMPTZ
);

CREATE INDEX idx_pdt_patient ON plan_traitement(patient_id, created_at DESC);

CREATE TRIGGER trg_plan_traitement_updated_at
  BEFORE UPDATE ON plan_traitement
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── Ligne PDT ───────────────────────────────────────────────────────────────
-- Individual line item in a treatment plan
-- urgence: 1=emergency ... 5=elective
-- ordre: display order within the plan
-- FK deferred: acte_id, materiau_id, teinte_id added in 005
CREATE TABLE ligne_pdt (
  id                UUID     PRIMARY KEY DEFAULT gen_random_uuid(),
  pdt_id            UUID     NOT NULL REFERENCES plan_traitement(id) ON DELETE CASCADE,
  acte_id           UUID,    -- FK added in 005_reference_data
  dent_fdi          SMALLINT,
  faces             TEXT[],
  materiau_id       UUID,    -- FK added in 005_reference_data
  teinte_id         UUID,    -- FK added in 005_reference_data
  urgence           SMALLINT DEFAULT 3 CHECK (urgence BETWEEN 1 AND 5),
  prix_praticien    DECIMAL  NOT NULL,
  base_secu         DECIMAL  DEFAULT 0,
  panier_rac        TEXT,
  statut            TEXT     NOT NULL DEFAULT 'a_faire' CHECK (statut IN ('a_faire', 'en_cours', 'fait')),
  ordre             INT      NOT NULL
);

CREATE INDEX idx_ligne_pdt ON ligne_pdt(pdt_id, ordre);
