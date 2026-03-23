-- Migration 002: Patient and medical questionnaire tables

-- ─── Patient ─────────────────────────────────────────────────────────────────
CREATE TABLE patient (
  id                       UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  cabinet_id               UUID        NOT NULL REFERENCES cabinet(id),
  nom                      TEXT        NOT NULL,
  nom_naissance            TEXT,
  prenom                   TEXT        NOT NULL,
  sexe                     TEXT        NOT NULL CHECK (sexe IN ('M', 'F')),
  date_naissance           DATE        NOT NULL,
  num_ss                   TEXT,
  email                    TEXT,
  telephone                TEXT,
  adresse                  TEXT,
  profession               TEXT,
  couverture               TEXT        NOT NULL CHECK (couverture IN ('mutuelle', 'cmu_c2s', 'ame', 'aucune')),
  mutuelle_nom             TEXT,
  mutuelle_tableau_garantie BOOLEAN,
  contact_urgence_nom      TEXT,
  contact_urgence_tel      TEXT,
  contact_urgence_lien     TEXT,
  representant_legal_nom   TEXT,
  representant_legal_tel   TEXT,
  medecin_traitant_nom     TEXT,
  medecin_traitant_tel     TEXT,
  created_at               TIMESTAMPTZ DEFAULT now(),
  updated_at               TIMESTAMPTZ DEFAULT now(),
  deleted_at               TIMESTAMPTZ
);

CREATE INDEX idx_patient_cabinet ON patient(cabinet_id);
CREATE INDEX idx_patient_search  ON patient(cabinet_id, nom, prenom);
CREATE INDEX idx_patient_ss      ON patient(num_ss) WHERE num_ss IS NOT NULL;

CREATE TRIGGER trg_patient_updated_at
  BEFORE UPDATE ON patient
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── Questionnaire médical ───────────────────────────────────────────────────
-- JSONB fields hold structured sub-answers for complex clinical questions
-- (e.g., AVK molecules, INR, allergies detail, etc.)
CREATE TABLE questionnaire_medical (
  id                       UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id               UUID        NOT NULL REFERENCES patient(id),
  cabinet_id               UUID        NOT NULL REFERENCES cabinet(id),
  version                  INT         NOT NULL DEFAULT 1,
  date_signature           DATE,
  signe_par                TEXT        CHECK (signe_par IN ('patient', 'representant_legal')),
  nom_signataire           TEXT,
  prochaine_maj            DATE,
  -- Cardiovascular / haemostasis
  avk                      JSONB,       -- {molecule, inr, controle_date}
  aod_molecule             TEXT,
  antiagregants            JSONB,       -- {molecules[], dose}
  hemostase                JSONB,       -- {troubles[], details}
  endocardite              JSONB,       -- {haut_risque, molecules[]}
  -- Bone / orthopaedic
  immunodepression         JSONB,
  protheses_articulaires   JSONB,       -- {localisation[], date_pose}
  bisphosphonates          JSONB,       -- {molecule, voie, duree_ans}
  antiresorptifs           JSONB,
  radiotherapie            JSONB,       -- {localisation, date, grays}
  -- General medical history
  troubles                 JSONB,       -- array of {categorie, libelle, details}
  medicaments              TEXT[],
  allergies                JSONB,       -- [{substance, type, reaction}]
  -- Lifestyle
  tabac                    TEXT,        -- 'non' | 'sevrage' | 'actif_<n>_paquets'
  alcool                   TEXT,
  drogues                  JSONB,
  -- Reproductive
  grossesse_mois           INT,
  allaitement              BOOLEAN,
  -- Habits
  activite_physique        TEXT,
  bruxisme                 TEXT,
  sahos                    JSONB,       -- {traitement, type}
  rgo                      BOOLEAN,
  tca                      TEXT,
  -- Dental history
  dernier_rdv_date         DATE,
  brossage_quotidien       SMALLINT,
  auxiliaires              TEXT[],
  historique_connu         TEXT[],
  apprehension             TEXT,
  notice_information_date  DATE,
  created_at               TIMESTAMPTZ DEFAULT now(),
  updated_at               TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_questionnaire_patient ON questionnaire_medical(patient_id);

CREATE TRIGGER trg_questionnaire_updated_at
  BEFORE UPDATE ON questionnaire_medical
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();
