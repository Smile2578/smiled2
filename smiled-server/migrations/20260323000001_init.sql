-- Migration 001: Foundation tables
-- Cabinet, utilisateur, RBAC tables + audit log + updated_at trigger

-- ─── Trigger: auto-update updated_at ─────────────────────────────────────────
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- ─── Audit log ───────────────────────────────────────────────────────────────
CREATE TABLE audit_log (
  id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  cabinet_id  UUID        NOT NULL,
  user_id     UUID        NOT NULL,
  action      TEXT        NOT NULL,
  entity_type TEXT        NOT NULL,
  entity_id   UUID        NOT NULL,
  changes     JSONB,
  ip_address  INET,
  created_at  TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_audit_log_entity ON audit_log(entity_type, entity_id);
CREATE INDEX idx_audit_log_user   ON audit_log(user_id, created_at);

-- ─── Cabinet ─────────────────────────────────────────────────────────────────
CREATE TABLE cabinet (
  id         UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  nom        TEXT        NOT NULL,
  adresse    TEXT,
  siret      TEXT,
  finess     TEXT,
  plan       TEXT        DEFAULT 'standard',
  created_at TIMESTAMPTZ DEFAULT now(),
  updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TRIGGER trg_cabinet_updated_at
  BEFORE UPDATE ON cabinet
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── Utilisateur ─────────────────────────────────────────────────────────────
CREATE TABLE utilisateur (
  id                   UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  cabinet_id           UUID        NOT NULL REFERENCES cabinet(id),
  role                 TEXT        NOT NULL CHECK (role IN (
    'titulaire', 'associe', 'collaborateur', 'remplacant',
    'specialiste_odf', 'specialiste_co', 'specialiste_mbd',
    'assistant', 'assistant_formation', 'aspbd',
    'secretaire', 'comptable', 'admin', 'prothesiste'
  )),
  nom                  TEXT        NOT NULL,
  prenom               TEXT        NOT NULL,
  email                TEXT        UNIQUE NOT NULL,
  password_hash        TEXT        NOT NULL,
  rpps                 TEXT,
  num_am               TEXT,
  num_ordre            TEXT,
  code_specialite_am   SMALLINT    CHECK (code_specialite_am IN (19, 36, 37, 38)),
  convention           TEXT        CHECK (convention IN ('secteur_1', 'secteur_2', 'non_conventionne')),
  specialites          TEXT[],
  orientations         TEXT[],
  type_exercice        TEXT        CHECK (type_exercice IN ('liberal', 'salarie')),
  date_debut           DATE,
  date_fin             DATE,
  remplace_id          UUID        REFERENCES utilisateur(id),
  actif                BOOLEAN     DEFAULT true,
  created_at           TIMESTAMPTZ DEFAULT now(),
  updated_at           TIMESTAMPTZ DEFAULT now()
);

CREATE INDEX idx_utilisateur_cabinet ON utilisateur(cabinet_id);
CREATE INDEX idx_utilisateur_email   ON utilisateur(email);

CREATE TRIGGER trg_utilisateur_updated_at
  BEFORE UPDATE ON utilisateur
  FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ─── RBAC ────────────────────────────────────────────────────────────────────
CREATE TABLE permission (
  id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  code        TEXT UNIQUE NOT NULL,
  description TEXT
);

CREATE TABLE role_permission (
  role          TEXT NOT NULL,
  permission_id UUID NOT NULL REFERENCES permission(id),
  scope         TEXT DEFAULT 'all',
  PRIMARY KEY (role, permission_id)
);

CREATE TABLE user_permission_override (
  user_id       UUID    NOT NULL REFERENCES utilisateur(id),
  permission_id UUID    NOT NULL REFERENCES permission(id),
  granted       BOOLEAN NOT NULL,
  scope         TEXT,
  PRIMARY KEY (user_id, permission_id)
);
