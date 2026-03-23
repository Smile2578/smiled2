# Smiled.IO — Core Clinique Design Spec

**Date**: 2026-03-23
**Auteur**: Simon + Claude
**Statut**: Draft v2 (post-review)
**Scope**: Core Clinique (Phase 1 — fondations + schéma dentaire + diagnostic + PDT)

---

## 1. Contexte & Vision

### 1.1 Objectif
Smiled.IO est un logiciel dentaire moderne visant à concurrencer Veasy (Cegedim), Julie, Logos et les solutions historiques. Le produit mise sur :
- Une **UX de niveau startup tech** (pas un logiciel médical des années 2010)
- Une **IA diagnostic** plus poussée que la concurrence
- Un modèle **open/self-hostable** en plus du SaaS

### 1.2 Positionnement
- **SaaS multi-tenant** par défaut, architecture prête pour **self-hosted**
- **Cible** : du cabinet solo au centre dentaire (modulaire)
- **Conformité** : RGPD + HDS pour le MVP, SESAM-Vitale et certifications planifiées
- **Timeline** : 12+ mois, pas de rush
- **Équipe** : Solo + Claude comme co-développeur

### 1.3 Concurrent principal — Veasy
- ~6 000 dentistes, 500 centres (racheté par Cegedim fév 2024)
- Cloud, IA pour pré-remplissage schéma dentaire
- Forces : base installée, conformité SESAM-Vitale, nomenclature pré-configurée
- Faiblesses : UX legacy, dépendance Cegedim, pas self-hostable, IA limitée

### 1.4 Scope Phase 1 — Core Clinique
Ce spec couvre uniquement le **Core Clinique** :
- Fondations techniques (auth, multi-tenant, API, permissions)
- Patients + questionnaire médical
- Schéma dentaire interactif
- Diagnostic + findings
- Plans de traitement (4 formules)
- Catalogue d'actes et matériaux (CRUD modulaire)
- Gestion documents (radios, photos)

**Hors scope Phase 1** : Agenda/planning, facturation/télétransmission, portail patient, moteur IA diagnostic, orthodontie avancée.

---

## 2. Stack Technique

| Couche | Technologie | Justification |
|--------|-------------|---------------|
| **Frontend** | Nuxt 4 (Vue 3) + TailwindCSS | Pas de lock-in Vercel (Nitro déploie partout), apprentissage Vue |
| **Backend** | Rust (Axum) | Performance brute, mémoire safety compilateur, binaire unique self-hosted |
| **Database** | PostgreSQL (Neon SaaS / vanilla self-hosted) | Données hautement relationnelles, standard médical |
| **ORM/Queries** | sqlx (compile-time checked) | Type-safe, pas de magie ORM, SQL explicite |
| **Real-time** | WebSocket natif Rust (tokio-tungstenite) | Agenda multi-praticien, notifications |
| **Object Storage** | S3 (SaaS) / MinIO (self-hosted) | Radios, photos, PDFs |
| **Auth** | JWT (jsonwebtoken crate) | Stateless, scale horizontal |
| **Cache** | Application-level (in-memory) | Pas de Redis en Phase 1 — cache via `moka` crate pour référentiels |
| **Email** | SMTP via `lettre` crate (SaaS: SES/Resend, self-hosted: any SMTP) | Invitations, password reset |
| **Schéma dentaire** | Canvas 2D via Vue | Performance interactive haute |

### 2.1 Crates Rust clés
- `axum` — HTTP framework
- `sqlx` — Postgres async, compile-time checked queries
- `jsonwebtoken` — JWT
- `aws-sdk-s3` — S3/MinIO
- `tokio` — async runtime
- `tower` — middleware stack
- `serde` / `serde_json` — serialization
- `validator` — input validation
- `argon2` — password hashing
- `moka` — in-memory cache (TTL-based, concurrent)
- `lettre` — email SMTP
- `tokio-tungstenite` — WebSocket

### 2.2 Packages frontend clés
- `nuxt` 4.x — framework Vue
- `tailwindcss` 4.x — styling
- `shadcn-vue` — composants UI
- `pinia` — state management
- `zod` — validation schémas

---

## 3. Architecture

### 3.1 Vue d'ensemble

```
CLIENTS (Navigateur Nuxt 4 / PWA / Tauri futur)
         │
         ▼
API GATEWAY (Rust/Axum)
  ├── Auth (JWT)
  ├── Rate Limiting (tower)
  ├── CORS (configurable, env-based origins)
  └── Tenant Router
         │
         ▼
CORE CLINIQUE SERVICE (monolithe modulaire)
  ├── patients/
  ├── schema_dentaire/
  ├── diagnostic/
  ├── pdt/
  ├── actes/
  ├── materiaux/
  ├── documents/
  └── historique/
         │
    ┌────┴────────────┐
    ▼                 ▼
PostgreSQL         S3/MinIO
(Neon/vanilla)    (fichiers)
```

### 3.2 Principes architecturaux
- **Monolithe modulaire** : un seul binaire Rust avec modules internes bien séparés. Pas de microservices au départ.
- **Multi-tenant : schéma partagé avec Row-Level Security (RLS)** : toutes les tables tenant-scoped ont une colonne `cabinet_id`. Postgres RLS policies filtrent automatiquement par tenant. `SET app.current_tenant = 'uuid'` dans chaque transaction.
- **API REST + WebSocket** : REST pour le CRUD, WebSocket pour le real-time.
- **Stateless backend** : JWT, pas de session server-side.
- **Object Storage abstrait** : S3 en SaaS, MinIO en self-hosted, même interface.
- **CORS** : origins configurables via env var `CORS_ORIGINS` (comma-separated). Self-hosted : wildcard ou URL configurée.

### 3.3 Multi-tenant Strategy (décision architecturale)

**Choix : schéma partagé + RLS** (et non schema-per-tenant)

Raison : schema-per-tenant crée une complexité opérationnelle (migrations × N schemas, connexions pool × N, routing). RLS offre :
- Isolation forte (Postgres-enforced, pas applicatif)
- Un seul schéma SQL, une seule migration
- Compatible self-hosted (un seul cabinet_id fixe, RLS transparent)
- Scale à des centaines de cabinets sans overhead

Implémentation :
```sql
-- Chaque requête set le tenant dans la transaction
SET app.current_tenant = '<cabinet_uuid>';

-- RLS policy exemple
CREATE POLICY tenant_isolation ON patient
  USING (cabinet_id = current_setting('app.current_tenant')::uuid);

ALTER TABLE patient ENABLE ROW LEVEL SECURITY;
```

**Self-hosted** (`TENANT_MODE=single`) : un seul `cabinet_id` configuré via env, RLS actif mais transparent (une seule valeur possible).

### 3.4 Self-hosted

```yaml
# docker-compose.yml
services:
  smiled:
    image: smiled/server:latest
    ports: ["8080:8080"]
    environment:
      DATABASE_URL: postgres://smiled:xxx@db:5432/smiled
      S3_ENDPOINT: http://minio:9000
      S3_BUCKET: smiled-docs
      JWT_SECRET: ${JWT_SECRET}
      TENANT_MODE: single
      DEFAULT_CABINET_ID: ${CABINET_ID}
      CORS_ORIGINS: http://localhost:3000
      SMTP_HOST: smtp.example.com  # optionnel
    depends_on: [db, minio]
  db:
    image: postgres:17
    volumes: ["pgdata:/var/lib/postgresql/data"]
  minio:
    image: minio/minio
    command: server /data
    volumes: ["miniodata:/data"]
volumes:
  pgdata:
  miniodata:
```

---

## 4. Modèle de Données

### 4.0 Conventions globales

- **Enum values** : toujours en snake_case anglais (`presente`, `absente`, pas de mélange FR/EN)
- **Tous les enums** : stockés en `TEXT` avec CHECK constraints, pas de types Postgres custom (simplicité migration)
- **`updated_at`** : présent sur TOUTE table mutable (auto via trigger `updated_at_trigger`)
- **Soft delete** : toute donnée clinique utilise `deleted_at TIMESTAMPTZ` (jamais de hard delete — obligation médico-légale). Les données de référence utilisent `actif BOOLEAN`.
- **Audit log** : table `audit_log` centralisée pour traçabilité (qui a fait quoi, quand)
- **FDI validation** : CHECK constraint sur `numero_fdi` : `numero_fdi IN (11..18, 21..28, 31..38, 41..48, 51..55, 61..65, 71..75, 81..85)`
- **Indexes** : documentés par table pour les queries critiques

```sql
-- Trigger updated_at (appliqué à toutes les tables mutables)
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN NEW.updated_at = now(); RETURN NEW; END;
$$ LANGUAGE plpgsql;

-- Audit log centralisé
CREATE TABLE audit_log (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  cabinet_id UUID NOT NULL,
  user_id UUID NOT NULL,
  action TEXT NOT NULL,        -- create, update, delete, access
  entity_type TEXT NOT NULL,   -- patient, diagnostic, pdt, etc.
  entity_id UUID NOT NULL,
  changes JSONB,               -- {field: {old, new}} pour les updates
  ip_address INET,
  created_at TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX idx_audit_log_entity ON audit_log(entity_type, entity_id);
CREATE INDEX idx_audit_log_user ON audit_log(user_id, created_at);
```

### 4.1 Cabinet & Utilisateurs

```sql
CREATE TABLE cabinet (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  nom TEXT NOT NULL,
  adresse TEXT,
  siret TEXT,
  finess TEXT,
  plan TEXT DEFAULT 'standard',
  created_at TIMESTAMPTZ DEFAULT now(),
  updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE utilisateur (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  cabinet_id UUID NOT NULL REFERENCES cabinet(id),
  role TEXT NOT NULL CHECK (role IN (
    'titulaire', 'associe', 'collaborateur', 'remplacant',
    'specialiste_odf', 'specialiste_co', 'specialiste_mbd',
    'assistant', 'assistant_formation', 'aspbd',
    'secretaire', 'comptable', 'admin', 'prothesiste'
  )),
  nom TEXT NOT NULL,
  prenom TEXT NOT NULL,
  email TEXT UNIQUE NOT NULL,
  password_hash TEXT NOT NULL,
  rpps TEXT,
  num_am TEXT,
  num_ordre TEXT,
  code_specialite_am SMALLINT CHECK (code_specialite_am IN (19, 36, 37, 38)),
  convention TEXT CHECK (convention IN ('secteur_1', 'secteur_2', 'non_conventionne')),
  specialites TEXT[],
  orientations TEXT[],
  type_exercice TEXT CHECK (type_exercice IN ('liberal', 'salarie')),
  date_debut DATE,
  date_fin DATE,
  remplace_id UUID REFERENCES utilisateur(id),
  actif BOOLEAN DEFAULT true,
  created_at TIMESTAMPTZ DEFAULT now(),
  updated_at TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX idx_utilisateur_cabinet ON utilisateur(cabinet_id);
CREATE INDEX idx_utilisateur_email ON utilisateur(email);

-- RBAC permissions
CREATE TABLE permission (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  code TEXT UNIQUE NOT NULL,  -- ex: 'patient.read_clinical', 'diagnostic.create'
  description TEXT
);

CREATE TABLE role_permission (
  role TEXT NOT NULL,
  permission_id UUID NOT NULL REFERENCES permission(id),
  scope TEXT DEFAULT 'all',  -- all, own_patients, read_only
  PRIMARY KEY (role, permission_id)
);

-- Override per-user (optional, for exceptions)
CREATE TABLE user_permission_override (
  user_id UUID NOT NULL REFERENCES utilisateur(id),
  permission_id UUID NOT NULL REFERENCES permission(id),
  granted BOOLEAN NOT NULL,  -- true=grant, false=revoke
  scope TEXT,
  PRIMARY KEY (user_id, permission_id)
);
```

### 4.2 Patient

```sql
CREATE TABLE patient (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  cabinet_id UUID NOT NULL REFERENCES cabinet(id),
  nom TEXT NOT NULL,
  nom_naissance TEXT,
  prenom TEXT NOT NULL,
  sexe TEXT NOT NULL CHECK (sexe IN ('M', 'F')),
  -- Note: sexe = sexe administratif (SS/SESAM-Vitale, binaire).
  -- Un champ identite_genre pourra être ajouté séparément si besoin.
  date_naissance DATE NOT NULL,
  num_ss TEXT,
  email TEXT,
  telephone TEXT,
  adresse TEXT,
  profession TEXT,
  couverture TEXT NOT NULL CHECK (couverture IN ('mutuelle', 'cmu_c2s', 'ame', 'aucune')),
  mutuelle_nom TEXT,
  mutuelle_tableau_garantie BOOLEAN,
  contact_urgence_nom TEXT,
  contact_urgence_tel TEXT,
  contact_urgence_lien TEXT,
  representant_legal_nom TEXT,
  representant_legal_tel TEXT,
  medecin_traitant_nom TEXT,
  medecin_traitant_tel TEXT,
  created_at TIMESTAMPTZ DEFAULT now(),
  updated_at TIMESTAMPTZ DEFAULT now(),
  deleted_at TIMESTAMPTZ  -- soft delete, never hard delete
);
CREATE INDEX idx_patient_cabinet ON patient(cabinet_id);
CREATE INDEX idx_patient_search ON patient(cabinet_id, nom, prenom);
CREATE INDEX idx_patient_ss ON patient(num_ss) WHERE num_ss IS NOT NULL;
```

### 4.3 Questionnaire Médical

```sql
CREATE TABLE questionnaire_medical (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id UUID NOT NULL REFERENCES patient(id),
  cabinet_id UUID NOT NULL REFERENCES cabinet(id),
  version INT NOT NULL DEFAULT 1,
  date_signature DATE,
  signe_par TEXT CHECK (signe_par IN ('patient', 'representant_legal')),
  nom_signataire TEXT,
  prochaine_maj DATE,

  -- Risque hémorragique (P0)
  avk JSONB,              -- {molecule, inr_date, inr_valeur}
  aod_molecule TEXT,
  antiagregants JSONB,    -- {molecules[], therapie: mono|bitherapie}
  hemostase JSONB,        -- {type, severite}

  -- Risque infectieux (P0)
  endocardite JSONB,      -- {risque, detail, antibioprophylaxie_requise}
  immunodepression JSONB, -- {transplantation, chimiotherapie, corticotherapie, biotherapies, vih}
  protheses_articulaires JSONB,

  -- Risque médicamenteux (P0)
  bisphosphonates JSONB,  -- {voie, molecule, duree_mois, indication}
  antiresorptifs JSONB,
  radiotherapie JSONB,    -- {zone, dose_gy, date, risque_orn}

  -- Troubles médicaux
  troubles JSONB,

  -- Médicaments & allergies
  medicaments TEXT[],
  allergies JSONB,

  -- Habitudes
  tabac TEXT,
  alcool TEXT,
  drogues JSONB,
  grossesse_mois INT,
  allaitement BOOLEAN,
  activite_physique TEXT,
  bruxisme TEXT,
  sahos JSONB,
  rgo BOOLEAN,
  tca TEXT,

  -- Dentaire
  dernier_rdv_date DATE,
  brossage_quotidien SMALLINT,
  auxiliaires TEXT[],
  historique_connu TEXT[],
  apprehension TEXT,

  -- RGPD
  notice_information_date DATE,

  created_at TIMESTAMPTZ DEFAULT now(),
  updated_at TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX idx_questionnaire_patient ON questionnaire_medical(patient_id);
```

### 4.4 Schéma Dentaire

```sql
CREATE TABLE schema_dentaire (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id UUID NOT NULL REFERENCES patient(id),
  cabinet_id UUID NOT NULL REFERENCES cabinet(id),
  version INT NOT NULL,
  dentition TEXT NOT NULL CHECK (dentition IN ('permanente', 'lacteale', 'mixte')),
  -- Versioning policy: new version created per clinical session (not per field edit).
  -- A session = one appointment. Edits within the same session update the current version.
  -- Expected volume: ~4-12 versions/patient/year for active treatment.
  created_by UUID NOT NULL REFERENCES utilisateur(id),
  created_at TIMESTAMPTZ DEFAULT now(),
  UNIQUE(patient_id, version)
);
CREATE INDEX idx_schema_patient ON schema_dentaire(patient_id, version DESC);

-- FDI valid numbers (used in CHECK constraints)
-- Permanent: 11-18, 21-28, 31-38, 41-48
-- Deciduous: 51-55, 61-65, 71-75, 81-85

CREATE TABLE dent (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  schema_id UUID NOT NULL REFERENCES schema_dentaire(id) ON DELETE CASCADE,
  numero_fdi SMALLINT NOT NULL,
  successeur_fdi SMALLINT,
  statut TEXT NOT NULL DEFAULT 'presente',
  eruption TEXT,
  prothese_fixe JSONB,
  endo JSONB,
  racine JSONB,
  implant JSONB,
  ortho JSONB,
  traumatisme JSONB,
  usure JSONB,
  esthetique JSONB,
  impaction JSONB,
  paro_mobilite SMALLINT CHECK (paro_mobilite BETWEEN 0 AND 3),
  paro_furcation TEXT,
  paro_recession_class TEXT,
  media_ids UUID[],
  updated_at TIMESTAMPTZ DEFAULT now(),
  UNIQUE(schema_id, numero_fdi)
);
CREATE INDEX idx_dent_schema ON dent(schema_id);

CREATE TABLE face_dent (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  dent_id UUID NOT NULL REFERENCES dent(id) ON DELETE CASCADE,
  face TEXT NOT NULL CHECK (face IN ('M', 'D', 'O', 'V', 'P_L', 'C')),
  etat TEXT NOT NULL DEFAULT 'saine',
  restauration JSONB,
  updated_at TIMESTAMPTZ DEFAULT now(),
  UNIQUE(dent_id, face)
);

CREATE TABLE paro_site (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  dent_id UUID NOT NULL REFERENCES dent(id) ON DELETE CASCADE,
  site TEXT NOT NULL CHECK (site IN ('MB', 'B', 'DB', 'ML', 'L', 'DL')),
  profondeur_poche SMALLINT,
  recession SMALLINT,
  bop BOOLEAN DEFAULT false,
  suppuration BOOLEAN DEFAULT false,
  plaque BOOLEAN DEFAULT false,
  updated_at TIMESTAMPTZ DEFAULT now(),
  UNIQUE(dent_id, site)
);

-- Patient-level clinical data (attached to schema version)
CREATE TABLE occlusion (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  schema_id UUID NOT NULL REFERENCES schema_dentaire(id) ON DELETE CASCADE,
  angle_molaire_g TEXT,
  angle_molaire_d TEXT,
  angle_canine_g TEXT,
  angle_canine_d TEXT,
  overjet_mm DECIMAL,
  overbite_mm DECIMAL,
  beance TEXT,
  articule_croise SMALLINT[],
  guidage_g TEXT,
  guidage_d TEXT,
  courbe_spee TEXT,
  dvo_mm DECIMAL,
  updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE atm (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  schema_id UUID NOT NULL REFERENCES schema_dentaire(id) ON DELETE CASCADE,
  ouverture_max_mm SMALLINT,
  propulsion_mm SMALLINT,
  lateralite_g_mm SMALLINT,
  lateralite_d_mm SMALLINT,
  bruit_g TEXT,
  bruit_d TEXT,
  deviation TEXT,
  douleur_musculaire JSONB,
  blocage TEXT,
  score_helkimo SMALLINT,
  updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE paro_global (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  schema_id UUID NOT NULL REFERENCES schema_dentaire(id) ON DELETE CASCADE,
  staging TEXT,
  grading TEXT,
  indice_plaque_pct SMALLINT,
  bop_global_pct SMALLINT,
  updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE prothese_amovible (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id UUID NOT NULL REFERENCES patient(id),
  cabinet_id UUID NOT NULL REFERENCES cabinet(id),
  type TEXT NOT NULL,
  arcade TEXT NOT NULL CHECK (arcade IN ('maxillaire', 'mandibulaire')),
  kennedy_class TEXT,
  kennedy_modifications SMALLINT,
  dents_remplacees SMALLINT[],
  crochets_sur SMALLINT[],
  materiau_base_id UUID REFERENCES materiau(id),
  date_pose DATE,
  etat TEXT,
  attachements TEXT[],
  created_at TIMESTAMPTZ DEFAULT now(),
  updated_at TIMESTAMPTZ DEFAULT now(),
  deleted_at TIMESTAMPTZ
);

-- Historique par dent (timeline) — links to tooth by FDI (version-agnostic)
-- Documents are linked to teeth by FDI number, not by dent.id,
-- because documents and timeline events persist across schema versions.
CREATE TABLE evenement_dent (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id UUID NOT NULL REFERENCES patient(id),
  cabinet_id UUID NOT NULL REFERENCES cabinet(id),
  dent_fdi SMALLINT NOT NULL,
  date DATE NOT NULL,
  type TEXT NOT NULL CHECK (type IN ('diagnostic', 'traitement', 'observation', 'trauma')),
  acte_id UUID REFERENCES acte(id),
  description TEXT,
  praticien_id UUID NOT NULL REFERENCES utilisateur(id),
  media_ids UUID[],
  schema_version INT,
  created_at TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX idx_evenement_patient_dent ON evenement_dent(patient_id, dent_fdi, date);
```

### 4.5 Diagnostic & Plans de Traitement

```sql
CREATE TABLE diagnostic (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id UUID NOT NULL REFERENCES patient(id),
  cabinet_id UUID NOT NULL REFERENCES cabinet(id),
  praticien_id UUID NOT NULL REFERENCES utilisateur(id),
  schema_version INT NOT NULL,
  date DATE NOT NULL DEFAULT CURRENT_DATE,
  created_at TIMESTAMPTZ DEFAULT now(),
  deleted_at TIMESTAMPTZ
);

CREATE TABLE finding (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  diagnostic_id UUID NOT NULL REFERENCES diagnostic(id) ON DELETE CASCADE,
  type TEXT NOT NULL,
  dent_fdi SMALLINT,
  details JSONB,
  severite SMALLINT NOT NULL CHECK (severite BETWEEN 1 AND 5),
  source TEXT NOT NULL CHECK (source IN ('praticien', 'ia'))
);

CREATE TABLE plan_traitement (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id UUID NOT NULL REFERENCES patient(id),
  cabinet_id UUID NOT NULL REFERENCES cabinet(id),
  praticien_id UUID NOT NULL REFERENCES utilisateur(id),
  diagnostic_id UUID NOT NULL REFERENCES diagnostic(id),
  formule TEXT NOT NULL CHECK (formule IN ('lab', 'compromis_1', 'compromis_2', 'budget')),
  statut TEXT NOT NULL DEFAULT 'brouillon' CHECK (statut IN (
    'brouillon', 'presente', 'accepte', 'en_cours', 'termine'
  )),
  prix_total DECIMAL,
  reste_a_charge_estime DECIMAL,
  pdf_url TEXT,
  created_at TIMESTAMPTZ DEFAULT now(),
  updated_at TIMESTAMPTZ DEFAULT now(),
  deleted_at TIMESTAMPTZ
);

CREATE TABLE ligne_pdt (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  pdt_id UUID NOT NULL REFERENCES plan_traitement(id) ON DELETE CASCADE,
  acte_id UUID NOT NULL REFERENCES acte(id),
  dent_fdi SMALLINT,
  faces TEXT[],
  materiau_id UUID REFERENCES materiau(id),
  teinte_id UUID REFERENCES teinte(id),
  urgence SMALLINT DEFAULT 3 CHECK (urgence BETWEEN 1 AND 5),
  prix_praticien DECIMAL NOT NULL,
  base_secu DECIMAL DEFAULT 0,
  panier_rac TEXT,
  statut TEXT NOT NULL DEFAULT 'a_faire' CHECK (statut IN ('a_faire', 'en_cours', 'fait')),
  ordre INT NOT NULL
);
```

### 4.6 Données de Référence

```sql
CREATE TABLE categorie_acte (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  code TEXT NOT NULL,
  libelle TEXT NOT NULL,
  ordre_affichage INT,
  icone TEXT,
  niveau TEXT NOT NULL DEFAULT 'systeme' CHECK (niveau IN ('systeme', 'cabinet')),
  cabinet_id UUID REFERENCES cabinet(id),
  UNIQUE(code, COALESCE(cabinet_id, '00000000-0000-0000-0000-000000000000'::uuid))
);

CREATE TABLE acte (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  categorie_id UUID NOT NULL REFERENCES categorie_acte(id),
  nomenclature TEXT NOT NULL CHECK (nomenclature IN ('ccam', 'ngap', 'hors_nomenclature')),
  code_ccam TEXT,
  code_ngap TEXT,
  libelle TEXT NOT NULL,
  base_secu DECIMAL DEFAULT 0,
  prix_defaut DECIMAL NOT NULL,
  panier_rac TEXT CHECK (panier_rac IN ('rac_0', 'modere', 'libre')),
  plafond_rac_0 DECIMAL,
  modificateur_jeune BOOLEAN DEFAULT false,
  dent_requise BOOLEAN DEFAULT false,
  faces_requises BOOLEAN DEFAULT false,
  teinte_requise BOOLEAN DEFAULT false,
  nb_seances_typique SMALLINT,
  niveau TEXT NOT NULL DEFAULT 'systeme' CHECK (niveau IN ('systeme', 'cabinet')),
  cabinet_id UUID REFERENCES cabinet(id),
  actif BOOLEAN DEFAULT true,
  date_entree_vigueur DATE,
  date_fin_vigueur DATE,
  notes_convention TEXT,
  updated_at TIMESTAMPTZ DEFAULT now()
);
CREATE INDEX idx_acte_nomenclature ON acte(nomenclature, code_ccam);
CREATE INDEX idx_acte_categorie ON acte(categorie_id);

CREATE TABLE acte_materiau (
  acte_id UUID NOT NULL REFERENCES acte(id),
  materiau_id UUID NOT NULL REFERENCES materiau(id),
  PRIMARY KEY (acte_id, materiau_id)
);

CREATE TABLE tarif_cabinet (
  cabinet_id UUID NOT NULL REFERENCES cabinet(id),
  acte_id UUID NOT NULL REFERENCES acte(id),
  prix DECIMAL NOT NULL,
  actif BOOLEAN DEFAULT true,
  PRIMARY KEY (cabinet_id, acte_id)
);

CREATE TABLE categorie_materiau (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  code TEXT UNIQUE NOT NULL,
  libelle TEXT NOT NULL,
  parent_id UUID REFERENCES categorie_materiau(id)
);

CREATE TABLE materiau (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  categorie_materiau_id UUID NOT NULL REFERENCES categorie_materiau(id),
  code TEXT NOT NULL,
  libelle TEXT NOT NULL,
  marques_courantes TEXT[],
  resistance_flexion_mpa SMALLINT,
  obsolete BOOLEAN DEFAULT false,
  note_reglementaire TEXT,
  niveau TEXT NOT NULL DEFAULT 'systeme' CHECK (niveau IN ('systeme', 'cabinet')),
  cabinet_id UUID REFERENCES cabinet(id),
  actif BOOLEAN DEFAULT true,
  updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE teinte (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  systeme TEXT NOT NULL,
  code TEXT NOT NULL,
  libelle TEXT NOT NULL,
  ordre INT,
  niveau TEXT NOT NULL DEFAULT 'systeme',
  cabinet_id UUID REFERENCES cabinet(id),
  actif BOOLEAN DEFAULT true
);

CREATE TABLE motif_consultation (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  libelle TEXT NOT NULL,
  sous_questions JSONB,
  niveau TEXT NOT NULL DEFAULT 'systeme',
  cabinet_id UUID REFERENCES cabinet(id),
  actif BOOLEAN DEFAULT true
);

CREATE TABLE trouble_medical (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  categorie TEXT NOT NULL,
  libelle TEXT NOT NULL,
  icone TEXT,
  contre_indications TEXT,
  niveau TEXT NOT NULL DEFAULT 'systeme',
  cabinet_id UUID REFERENCES cabinet(id),
  actif BOOLEAN DEFAULT true
);
```

### 4.7 Documents

```sql
CREATE TABLE document (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  patient_id UUID NOT NULL REFERENCES patient(id),
  cabinet_id UUID NOT NULL REFERENCES cabinet(id),
  type TEXT NOT NULL,
  url_storage TEXT NOT NULL,
  filename TEXT,
  mime_type TEXT,
  uploaded_by UUID NOT NULL REFERENCES utilisateur(id),
  uploaded_at TIMESTAMPTZ DEFAULT now()
);

-- Document ↔ dent link (by FDI, version-agnostic — a radio of tooth 36
-- remains relevant across all schema versions)
CREATE TABLE document_dent (
  document_id UUID NOT NULL REFERENCES document(id),
  dent_fdi SMALLINT NOT NULL,
  PRIMARY KEY (document_id, dent_fdi)
);
CREATE INDEX idx_document_patient ON document(patient_id);
```

---

## 5. Rôles & Permissions

### 5.1 Rôles supportés (14)
| Code | Label | RPPS | Facturation |
|------|-------|------|-------------|
| `titulaire` | CD titulaire | Oui | Complète |
| `associe` | CD associé | Oui | Complète |
| `collaborateur` | CD collaborateur | Oui | Ses patients |
| `remplacant` | CD remplaçant | Oui | Temporaire |
| `specialiste_odf` | Spécialiste ODF | Oui | Complète (code 36) |
| `specialiste_co` | Spécialiste CO | Oui | Complète (code 37) |
| `specialiste_mbd` | Spécialiste MBD | Oui | Complète (code 38) |
| `assistant` | Assistant(e) dentaire | Oui | Non |
| `assistant_formation` | Assistant(e) en formation | Oui | Non |
| `aspbd` | Assistant prophylaxie (futur 2028) | Oui | Limité |
| `secretaire` | Secrétaire | Non | Création FSE |
| `comptable` | Comptable (externe) | Non | Lecture seule |
| `admin` | Admin système | N/A | Technique |
| `prothesiste` | Prothésiste (externe) | Oui | N/A |

### 5.2 Permissions (seed data)
Permissions seed dans `permission` table :
- `patient.read_admin`, `patient.write_admin`
- `patient.read_clinical`, `patient.write_clinical`
- `diagnostic.create`, `diagnostic.read`
- `pdt.create`, `pdt.read`, `pdt.update`
- `schema.read`, `schema.write`
- `paro.read`, `paro.write`
- `prescribe`
- `billing.create`, `billing.read`
- `payment.collect`
- `document.upload`, `document.read`
- `radio.take`
- `settings.cabinet`, `settings.users`
- `reports.financial`

Mapping `role_permission` : seed SQL avec la matrice complète (cf. brainstorming section 3).

---

## 6. API Backend (Rust/Axum)

### 6.1 Structure des modules
```
smiled-server/src/
├── main.rs
├── config.rs
├── auth/          (JWT, middleware, RBAC, password reset)
├── tenant/        (RLS setup per-request)
├── core/
│   ├── patient/
│   ├── schema_dentaire/
│   ├── diagnostic/
│   ├── pdt/
│   ├── acte/
│   ├── materiau/
│   ├── document/
│   └── historique/
├── reference/     (teinte, motif, trouble)
├── db/            (pool, migrations, queries)
├── audit/         (audit log middleware)
└── api/           (routes v1, WebSocket)
```

### 6.2 Endpoints

**Auth**
- `POST /api/v1/auth/login` → JWT
- `POST /api/v1/auth/refresh` → nouveau JWT
- `POST /api/v1/auth/forgot-password` → envoie email reset
- `POST /api/v1/auth/reset-password` → reset avec token
- `GET /api/v1/me` → profil utilisateur

**Patients**
- `GET /api/v1/patients` (search, pagination)
- `POST /api/v1/patients`
- `GET /api/v1/patients/:id`
- `PUT /api/v1/patients/:id`
- `GET /api/v1/patients/:id/questionnaire`
- `PUT /api/v1/patients/:id/questionnaire`

**Schéma dentaire**
- `GET /api/v1/patients/:id/schema` (version courante ou ?version=N)
- `POST /api/v1/patients/:id/schema` (nouvelle version)
- `GET /api/v1/patients/:id/schema/versions`
- `GET /api/v1/patients/:id/schema/dent/:fdi`
- `PUT /api/v1/patients/:id/schema/dent/:fdi`
- `GET /api/v1/patients/:id/schema/paro`
- `PUT /api/v1/patients/:id/schema/paro` (write paro 6 sites)
- `GET /api/v1/patients/:id/schema/occlusion`
- `PUT /api/v1/patients/:id/schema/occlusion`
- `GET /api/v1/patients/:id/schema/atm`
- `PUT /api/v1/patients/:id/schema/atm`
- `GET /api/v1/patients/:id/schema/paro-global`
- `PUT /api/v1/patients/:id/schema/paro-global`

**Prothèse amovible**
- `GET /api/v1/patients/:id/protheses-amovibles`
- `POST /api/v1/patients/:id/protheses-amovibles`
- `PUT /api/v1/patients/:id/protheses-amovibles/:pa_id`
- `DELETE /api/v1/patients/:id/protheses-amovibles/:pa_id` (soft)

**Historique dent**
- `GET /api/v1/patients/:id/dent/:fdi/timeline`

**Diagnostic & PDT**
- `POST /api/v1/patients/:id/diagnostics`
- `GET /api/v1/patients/:id/diagnostics`
- `GET /api/v1/patients/:id/diagnostics/:diag_id`
- `POST /api/v1/patients/:id/pdts`
- `GET /api/v1/patients/:id/pdts`
- `PUT /api/v1/patients/:id/pdts/:pdt_id`
- `POST /api/v1/patients/:id/pdts/:pdt_id/pdf`

**Documents**
- `POST /api/v1/patients/:id/documents/upload`
- `GET /api/v1/patients/:id/documents`
- `POST /api/v1/patients/:id/documents/:doc_id/link-dent/:fdi`

**Référentiel (CRUD modulaire)**
- `GET|POST|PUT|DELETE /api/v1/actes`
- `GET|POST|PUT|DELETE /api/v1/materiaux`
- `GET|POST|PUT|DELETE /api/v1/teintes`
- `GET|POST|PUT|DELETE /api/v1/motifs`
- `PUT /api/v1/actes/:id/tarif`
- `PUT /api/v1/actes/:id/toggle`

**Cabinet**
- `GET /api/v1/cabinet`
- `PUT /api/v1/cabinet`
- `GET /api/v1/cabinet/users`
- `POST /api/v1/cabinet/users/invite` (envoie email)
- `PUT /api/v1/cabinet/users/:id`

---

## 7. Frontend (Nuxt 4)

### 7.1 Structure
```
smiled-web/app/
├── layouts/       (default, patient, auth)
├── pages/
│   ├── login.vue
│   ├── forgot-password.vue
│   ├── reset-password.vue
│   ├── index.vue  (dashboard)
│   ├── patients/  (liste, fiche, schema, paro, diagnostic, pdts, docs, questionnaire, historique)
│   ├── actes/     (admin catalogue)
│   ├── materiaux/ (admin matériaux)
│   └── cabinet/   (settings, users)
├── components/
│   ├── dental-chart/    (Canvas 2D interactif)
│   ├── paro-chart/      (charting 6 sites)
│   ├── questionnaire/   (sections modulaires)
│   ├── pdt/             (builder, formules, PDF)
│   └── ui/              (shadcn-vue)
├── composables/
└── utils/
```

### 7.2 Schéma dentaire interactif (composant critique)
- **Canvas 2D** pour le rendu (pas SVG)
- **Vue reactivity** pilote le Canvas
- **Click detection** via hit-testing géométrique
- **Mode mixte** : permanentes / lactéales / mixte
- **Couleurs par état**
- **Sélection dent** → panel latéral avec détail complet

### 7.3 Charting parodontal
- Saisie 6 sites par dent (PD, recession, BOP, suppuration)
- Visualisation graphique (courbe des poches)
- Calcul auto du CAL
- Support saisie vocale (futur)

---

## 8. Seed Data

### 8.1 Actes (~150+)
14 catégories, incluant CCAM, NGAP, et HN. Nouveautés convention 2023-2028.

### 8.2 Matériaux (~80)
Hiérarchie complète. Flag `obsolete=true` pour amalgame (ban UE jan 2025).

### 8.3 Teintes
VITA Classical + VITA 3D-Master.

### 8.4 Permissions
Matrice RBAC complète seed dans `role_permission`.

---

## 9. Décisions Techniques

| # | Décision | Raison |
|---|----------|--------|
| 1 | **RLS** (pas schema-per-tenant) | Un seul schéma SQL, isolation Postgres-enforced, compatible self-hosted |
| 2 | **JSONB** pour structures complexes rarement queryées (endo, implant, racine, trauma) | Flexibilité, pas de JOIN pour lire une dent |
| 3 | **Tables relationnelles** pour données queryées/agrégées (faces, paro sites, findings, lignes PDT) | Indexables, queryables |
| 4 | **Versioning par session clinique** (pas par field edit) | ~4-12 versions/patient/an, volume maîtrisé |
| 5 | **Soft delete** sur toutes données cliniques | Obligation médico-légale, jamais de hard delete |
| 6 | **Audit log centralisé** | Traçabilité HAS |
| 7 | **Document ↔ dent par FDI** (version-agnostic) | Une radio de la dent 36 reste pertinente à travers les versions du schéma |
| 8 | **Pas de MongoDB** | Schéma profondément relationnel |
| 9 | **Pas de Redis** en Phase 1 | `moka` (in-memory cache) suffit pour les référentiels |
| 10 | **Sexe = administratif** (M/F binaire pour SS) | Requis SESAM-Vitale. Champ identité genre séparé si besoin |

---

## 10. Hors Scope (Phases Futures)

| Phase | Contenu |
|-------|---------|
| 2 | Agenda / Planning multi-praticien |
| 3 | Facturation, devis mutuelle, CCAM cotation, tiers payant |
| 4 | Portail patient |
| 5 | Moteur IA diagnostic |
| 6 | Télétransmission SESAM-Vitale, certification HDS |
| 7 | Orthodontie avancée |
| 8 | App mobile |
| 9 | Gestion de stock, commandes labo |

---

## 11. Critères de Succès (Phase 1)

- [ ] Créer un patient, remplir le questionnaire médical complet
- [ ] Schéma dentaire interactif Canvas (32 permanentes + 20 lactéales, 6 faces)
- [ ] Click dent → édition état (statut, faces, endo, paro, prothèse)
- [ ] Charting paro 6 sites fonctionnel
- [ ] Occlusion et ATM structurés
- [ ] Création diagnostic avec findings
- [ ] Création PDT avec 4 formules (lab, compromis 1&2, budget)
- [ ] Génération PDF du PDT
- [ ] Catalogue actes pré-chargé (~150 actes CCAM/NGAP/HN) avec CRUD cabinet
- [ ] Catalogue matériaux hiérarchique avec CRUD
- [ ] Upload/consultation documents liés aux dents
- [ ] Timeline historique par dent
- [ ] Multi-tenant RLS fonctionnel (2+ cabinets isolés)
- [ ] Self-hosted via docker-compose
- [ ] Auth JWT + password reset + 14 rôles RBAC
- [ ] Audit log centralisé
- [ ] Soft delete sur données cliniques
- [ ] Temps de réponse API < 100ms p95
