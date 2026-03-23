-- Migration 003: Dental schema, teeth, perio, occlusion, ATM, prosthetics, events

-- ─── Schema dentaire (versioned snapshot) ────────────────────────────────────
CREATE TABLE schema_dentaire (
  id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id  UUID        NOT NULL REFERENCES patient(id),
  cabinet_id  UUID        NOT NULL REFERENCES cabinet(id),
  version     INT         NOT NULL,
  dentition   TEXT        NOT NULL CHECK (dentition IN ('permanente', 'lacteale', 'mixte')),
  created_by  UUID        NOT NULL REFERENCES utilisateur(id),
  created_at  TIMESTAMPTZ DEFAULT now(),
  UNIQUE(patient_id, version)
);

CREATE INDEX idx_schema_patient ON schema_dentaire(patient_id, version DESC);

-- ─── Dent ────────────────────────────────────────────────────────────────────
-- FDI numbers: permanent 11-48, deciduous 51-85
-- statut: present | absent | extracted | implant | bridge_pontic
CREATE TABLE dent (
  id               UUID      PRIMARY KEY DEFAULT gen_random_uuid(),
  schema_id        UUID      NOT NULL REFERENCES schema_dentaire(id) ON DELETE CASCADE,
  numero_fdi       SMALLINT  NOT NULL CHECK (
    (numero_fdi BETWEEN 11 AND 18) OR
    (numero_fdi BETWEEN 21 AND 28) OR
    (numero_fdi BETWEEN 31 AND 38) OR
    (numero_fdi BETWEEN 41 AND 48) OR
    (numero_fdi BETWEEN 51 AND 55) OR
    (numero_fdi BETWEEN 61 AND 65) OR
    (numero_fdi BETWEEN 71 AND 75) OR
    (numero_fdi BETWEEN 81 AND 85)
  ),
  successeur_fdi   SMALLINT,
  statut           TEXT      NOT NULL DEFAULT 'presente',
  eruption         TEXT,
  -- Clinical JSONB blobs — structure enforced at application layer
  prothese_fixe    JSONB,   -- {type, materiau_id, teinte_id, date_pose, labo}
  endo             JSONB,   -- {traitement, date, praticien_id, longueur_travail[]}
  racine           JSONB,   -- {resorption, fracture, dilaceration}
  implant          JSONB,   -- {marque, ref, longueur, diametre, date_pose, prothesiste_id}
  ortho            JSONB,   -- {appareil, date_debut, objectif}
  traumatisme      JSONB,   -- {type, date, classification_andreasen}
  usure            JSONB,   -- {type, score_bewe}
  esthetique       JSONB,   -- {teinte_vita, restauration_esthetique}
  impaction        JSONB,   -- {type, angulation, winter_class, pell_gregory}
  paro_mobilite    SMALLINT CHECK (paro_mobilite BETWEEN 0 AND 3),
  paro_furcation   TEXT,
  paro_recession_class TEXT,
  media_ids        UUID[],
  updated_at       TIMESTAMPTZ DEFAULT now(),
  UNIQUE(schema_id, numero_fdi)
);

CREATE INDEX idx_dent_schema ON dent(schema_id);

CREATE TRIGGER trg_dent_updated_at
  BEFORE UPDATE ON dent
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── Face dent ───────────────────────────────────────────────────────────────
-- P_L = palatal/lingual, C = cervical
CREATE TABLE face_dent (
  id           UUID    PRIMARY KEY DEFAULT gen_random_uuid(),
  dent_id      UUID    NOT NULL REFERENCES dent(id) ON DELETE CASCADE,
  face         TEXT    NOT NULL CHECK (face IN ('M', 'D', 'O', 'V', 'P_L', 'C')),
  etat         TEXT    NOT NULL DEFAULT 'saine',
  restauration JSONB,  -- {type, materiau_id, date, praticien_id}
  updated_at   TIMESTAMPTZ DEFAULT now(),
  UNIQUE(dent_id, face)
);

CREATE TRIGGER trg_face_dent_updated_at
  BEFORE UPDATE ON face_dent
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── Paro site ───────────────────────────────────────────────────────────────
-- 6 sites per tooth: MB/B/DB (buccal row) + ML/L/DL (lingual row)
CREATE TABLE paro_site (
  id               UUID     PRIMARY KEY DEFAULT gen_random_uuid(),
  dent_id          UUID     NOT NULL REFERENCES dent(id) ON DELETE CASCADE,
  site             TEXT     NOT NULL CHECK (site IN ('MB', 'B', 'DB', 'ML', 'L', 'DL')),
  profondeur_poche SMALLINT,            -- mm
  recession        SMALLINT,            -- mm
  bop              BOOLEAN  DEFAULT false,
  suppuration      BOOLEAN  DEFAULT false,
  plaque           BOOLEAN  DEFAULT false,
  updated_at       TIMESTAMPTZ DEFAULT now(),
  UNIQUE(dent_id, site)
);

CREATE TRIGGER trg_paro_site_updated_at
  BEFORE UPDATE ON paro_site
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── Occlusion ───────────────────────────────────────────────────────────────
CREATE TABLE occlusion (
  id               UUID    PRIMARY KEY DEFAULT gen_random_uuid(),
  schema_id        UUID    NOT NULL REFERENCES schema_dentaire(id) ON DELETE CASCADE,
  angle_molaire_g  TEXT,
  angle_molaire_d  TEXT,
  angle_canine_g   TEXT,
  angle_canine_d   TEXT,
  overjet_mm       DECIMAL,
  overbite_mm      DECIMAL,
  beance           TEXT,
  articule_croise  SMALLINT[],
  guidage_g        TEXT,
  guidage_d        TEXT,
  courbe_spee      TEXT,
  dvo_mm           DECIMAL,
  updated_at       TIMESTAMPTZ DEFAULT now()
);

CREATE TRIGGER trg_occlusion_updated_at
  BEFORE UPDATE ON occlusion
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── ATM (Articulation Temporo-Mandibulaire) ─────────────────────────────────
CREATE TABLE atm (
  id                UUID    PRIMARY KEY DEFAULT gen_random_uuid(),
  schema_id         UUID    NOT NULL REFERENCES schema_dentaire(id) ON DELETE CASCADE,
  ouverture_max_mm  SMALLINT,
  propulsion_mm     SMALLINT,
  lateralite_g_mm   SMALLINT,
  lateralite_d_mm   SMALLINT,
  bruit_g           TEXT,
  bruit_d           TEXT,
  deviation         TEXT,
  douleur_musculaire JSONB,  -- {muscles[], intensite_eva}
  blocage           TEXT,
  score_helkimo     SMALLINT,
  updated_at        TIMESTAMPTZ DEFAULT now()
);

CREATE TRIGGER trg_atm_updated_at
  BEFORE UPDATE ON atm
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── Paro global ─────────────────────────────────────────────────────────────
-- Staging/grading per AAP/EFP 2018 classification
CREATE TABLE paro_global (
  id                UUID    PRIMARY KEY DEFAULT gen_random_uuid(),
  schema_id         UUID    NOT NULL REFERENCES schema_dentaire(id) ON DELETE CASCADE,
  staging           TEXT,   -- I | II | III | IV
  grading           TEXT,   -- A | B | C
  indice_plaque_pct SMALLINT,
  bop_global_pct    SMALLINT,
  updated_at        TIMESTAMPTZ DEFAULT now()
);

CREATE TRIGGER trg_paro_global_updated_at
  BEFORE UPDATE ON paro_global
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── Prothèse amovible ───────────────────────────────────────────────────────
-- materiau_base_id FK deferred — added after materiau table in migration 005
CREATE TABLE prothese_amovible (
  id                    UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id            UUID        NOT NULL REFERENCES patient(id),
  cabinet_id            UUID        NOT NULL REFERENCES cabinet(id),
  type                  TEXT        NOT NULL,
  arcade                TEXT        NOT NULL CHECK (arcade IN ('maxillaire', 'mandibulaire')),
  kennedy_class         TEXT,
  kennedy_modifications SMALLINT,
  dents_remplacees      SMALLINT[],
  crochets_sur          SMALLINT[],
  materiau_base_id      UUID,       -- FK added in 005_reference_data
  date_pose             DATE,
  etat                  TEXT,
  attachements          TEXT[],
  created_at            TIMESTAMPTZ DEFAULT now(),
  updated_at            TIMESTAMPTZ DEFAULT now(),
  deleted_at            TIMESTAMPTZ
);

CREATE TRIGGER trg_prothese_amovible_updated_at
  BEFORE UPDATE ON prothese_amovible
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── Événement dent ──────────────────────────────────────────────────────────
-- Chronological log of clinical events per tooth
-- acte_id FK deferred — added after acte table in migration 005
CREATE TABLE evenement_dent (
  id             UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id     UUID        NOT NULL REFERENCES patient(id),
  cabinet_id     UUID        NOT NULL REFERENCES cabinet(id),
  dent_fdi       SMALLINT    NOT NULL,
  date           DATE        NOT NULL,
  type           TEXT        NOT NULL CHECK (type IN ('diagnostic', 'traitement', 'observation', 'trauma')),
  acte_id        UUID,       -- FK added in 005_reference_data
  description    TEXT,
  praticien_id   UUID        NOT NULL REFERENCES utilisateur(id),
  media_ids      UUID[],
  schema_version INT,
  created_at     TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_evenement_patient_dent ON evenement_dent(patient_id, dent_fdi, date);
