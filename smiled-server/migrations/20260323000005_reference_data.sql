-- Migration 005: Reference data tables + deferred FK constraints

-- ─── Catégorie acte ──────────────────────────────────────────────────────────
CREATE TABLE categorie_acte (
  id              UUID    PRIMARY KEY DEFAULT gen_random_uuid(),
  code            TEXT    NOT NULL,
  libelle         TEXT    NOT NULL,
  ordre_affichage INT,
  icone           TEXT,
  niveau          TEXT    NOT NULL DEFAULT 'systeme' CHECK (niveau IN ('systeme', 'cabinet')),
  cabinet_id      UUID    REFERENCES cabinet(id)
);

-- Unique index: allows one system-level entry per code (cabinet_id NULL)
-- and one entry per (code, cabinet) for cabinet-level overrides
CREATE UNIQUE INDEX idx_categorie_acte_code_cabinet
  ON categorie_acte (code, COALESCE(cabinet_id, '00000000-0000-0000-0000-000000000000'::uuid));

-- ─── Acte ────────────────────────────────────────────────────────────────────
CREATE TABLE acte (
  id                    UUID    PRIMARY KEY DEFAULT gen_random_uuid(),
  categorie_id          UUID    NOT NULL REFERENCES categorie_acte(id),
  nomenclature          TEXT    NOT NULL CHECK (nomenclature IN ('ccam', 'ngap', 'hors_nomenclature')),
  code_ccam             TEXT,
  code_ngap             TEXT,
  libelle               TEXT    NOT NULL,
  base_secu             DECIMAL DEFAULT 0,
  prix_defaut           DECIMAL NOT NULL,
  panier_rac            TEXT    CHECK (panier_rac IN ('rac_0', 'modere', 'libre')),
  plafond_rac_0         DECIMAL,
  modificateur_jeune    BOOLEAN DEFAULT false,
  dent_requise          BOOLEAN DEFAULT false,
  faces_requises        BOOLEAN DEFAULT false,
  teinte_requise        BOOLEAN DEFAULT false,
  nb_seances_typique    SMALLINT,
  niveau                TEXT    NOT NULL DEFAULT 'systeme' CHECK (niveau IN ('systeme', 'cabinet')),
  cabinet_id            UUID    REFERENCES cabinet(id),
  actif                 BOOLEAN DEFAULT true,
  date_entree_vigueur   DATE,
  date_fin_vigueur      DATE,
  notes_convention      TEXT,
  updated_at            TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_acte_nomenclature ON acte(nomenclature, code_ccam);
CREATE INDEX idx_acte_categorie    ON acte(categorie_id);

CREATE TRIGGER trg_acte_updated_at
  BEFORE UPDATE ON acte
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── Catégorie matériau ──────────────────────────────────────────────────────
CREATE TABLE categorie_materiau (
  id        UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  code      TEXT UNIQUE NOT NULL,
  libelle   TEXT NOT NULL,
  parent_id UUID REFERENCES categorie_materiau(id)
);

-- ─── Matériau ────────────────────────────────────────────────────────────────
CREATE TABLE materiau (
  id                       UUID    PRIMARY KEY DEFAULT gen_random_uuid(),
  categorie_materiau_id    UUID    NOT NULL REFERENCES categorie_materiau(id),
  code                     TEXT    NOT NULL,
  libelle                  TEXT    NOT NULL,
  marques_courantes        TEXT[],
  resistance_flexion_mpa   SMALLINT,
  obsolete                 BOOLEAN DEFAULT false,
  note_reglementaire       TEXT,
  niveau                   TEXT    NOT NULL DEFAULT 'systeme' CHECK (niveau IN ('systeme', 'cabinet')),
  cabinet_id               UUID    REFERENCES cabinet(id),
  actif                    BOOLEAN DEFAULT true,
  updated_at               TIMESTAMPTZ DEFAULT now()
);

CREATE TRIGGER trg_materiau_updated_at
  BEFORE UPDATE ON materiau
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── Acte-Matériau (junction) ────────────────────────────────────────────────
CREATE TABLE acte_materiau (
  acte_id     UUID NOT NULL REFERENCES acte(id),
  materiau_id UUID NOT NULL REFERENCES materiau(id),
  PRIMARY KEY (acte_id, materiau_id)
);

-- ─── Tarif cabinet ───────────────────────────────────────────────────────────
CREATE TABLE tarif_cabinet (
  cabinet_id UUID    NOT NULL REFERENCES cabinet(id),
  acte_id    UUID    NOT NULL REFERENCES acte(id),
  prix       DECIMAL NOT NULL,
  actif      BOOLEAN DEFAULT true,
  PRIMARY KEY (cabinet_id, acte_id)
);

-- ─── Teinte ──────────────────────────────────────────────────────────────────
CREATE TABLE teinte (
  id         UUID    PRIMARY KEY DEFAULT gen_random_uuid(),
  systeme    TEXT    NOT NULL,
  code       TEXT    NOT NULL,
  libelle    TEXT    NOT NULL,
  ordre      INT,
  niveau     TEXT    NOT NULL DEFAULT 'systeme',
  cabinet_id UUID    REFERENCES cabinet(id),
  actif      BOOLEAN DEFAULT true
);

-- ─── Motif consultation ──────────────────────────────────────────────────────
CREATE TABLE motif_consultation (
  id              UUID    PRIMARY KEY DEFAULT gen_random_uuid(),
  libelle         TEXT    NOT NULL,
  sous_questions  JSONB,
  niveau          TEXT    NOT NULL DEFAULT 'systeme',
  cabinet_id      UUID    REFERENCES cabinet(id),
  actif           BOOLEAN DEFAULT true
);

-- ─── Trouble médical ─────────────────────────────────────────────────────────
CREATE TABLE trouble_medical (
  id                  UUID    PRIMARY KEY DEFAULT gen_random_uuid(),
  categorie           TEXT    NOT NULL,
  libelle             TEXT    NOT NULL,
  icone               TEXT,
  contre_indications  TEXT,
  niveau              TEXT    NOT NULL DEFAULT 'systeme',
  cabinet_id          UUID    REFERENCES cabinet(id),
  actif               BOOLEAN DEFAULT true
);

-- ─── Document ────────────────────────────────────────────────────────────────
CREATE TABLE document (
  id           UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id   UUID        NOT NULL REFERENCES patient(id),
  cabinet_id   UUID        NOT NULL REFERENCES cabinet(id),
  type         TEXT        NOT NULL,
  url_storage  TEXT        NOT NULL,
  filename     TEXT,
  mime_type    TEXT,
  uploaded_by  UUID        NOT NULL REFERENCES utilisateur(id),
  uploaded_at  TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_document_patient ON document(patient_id);

-- ─── Document-Dent (junction) ────────────────────────────────────────────────
CREATE TABLE document_dent (
  document_id UUID     NOT NULL REFERENCES document(id),
  dent_fdi    SMALLINT NOT NULL,
  PRIMARY KEY (document_id, dent_fdi)
);

-- ─── Deferred FK constraints ─────────────────────────────────────────────────
-- These reference materiau/acte/teinte which are created in this very migration,
-- but the columns were added in earlier migrations without FK enforcement.

ALTER TABLE prothese_amovible
  ADD CONSTRAINT fk_prothese_materiau
  FOREIGN KEY (materiau_base_id) REFERENCES materiau(id);

ALTER TABLE evenement_dent
  ADD CONSTRAINT fk_evenement_acte
  FOREIGN KEY (acte_id) REFERENCES acte(id);

ALTER TABLE ligne_pdt
  ADD CONSTRAINT fk_ligne_acte
  FOREIGN KEY (acte_id) REFERENCES acte(id);

ALTER TABLE ligne_pdt
  ADD CONSTRAINT fk_ligne_materiau
  FOREIGN KEY (materiau_id) REFERENCES materiau(id);

ALTER TABLE ligne_pdt
  ADD CONSTRAINT fk_ligne_teinte
  FOREIGN KEY (teinte_id) REFERENCES teinte(id);
