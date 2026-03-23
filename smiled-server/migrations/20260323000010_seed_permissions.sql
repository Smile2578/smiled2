-- Migration 010: Permissions + RBAC matrix for 14 roles
-- Based on spec section "Matrice RBAC" — researched by 5 deep-research agents

-- ─── Permissions ─────────────────────────────────────────────────────────────
INSERT INTO permission (id, code, description) VALUES
  ('ee000001-0000-0000-0000-000000000001', 'patient.read_admin',    'Lire les données administratives patient (coordonnées, SS, mutuelle)'),
  ('ee000001-0000-0000-0000-000000000002', 'patient.write_admin',   'Créer et modifier les données administratives patient'),
  ('ee000001-0000-0000-0000-000000000003', 'patient.read_clinical', 'Lire le dossier clinique complet (schéma, questionnaire, diagnostic)'),
  ('ee000001-0000-0000-0000-000000000004', 'patient.write_clinical','Modifier le dossier clinique (schéma dentaire, questionnaire)'),
  ('ee000001-0000-0000-0000-000000000005', 'diagnostic.create',     'Créer un diagnostic clinique'),
  ('ee000001-0000-0000-0000-000000000006', 'diagnostic.read',       'Consulter les diagnostics'),
  ('ee000001-0000-0000-0000-000000000007', 'pdt.create',            'Créer un plan de traitement'),
  ('ee000001-0000-0000-0000-000000000008', 'pdt.read',              'Consulter les plans de traitement'),
  ('ee000001-0000-0000-0000-000000000009', 'pdt.update',            'Modifier un plan de traitement (statut, prix)'),
  ('ee000001-0000-0000-0000-000000000010', 'schema.read',           'Consulter le schéma dentaire'),
  ('ee000001-0000-0000-0000-000000000011', 'schema.write',          'Modifier le schéma dentaire (dents, faces, sites paro)'),
  ('ee000001-0000-0000-0000-000000000012', 'paro.read',             'Consulter les données parodontales'),
  ('ee000001-0000-0000-0000-000000000013', 'paro.write',            'Saisir et modifier le sondage parodontal'),
  ('ee000001-0000-0000-0000-000000000014', 'prescribe',             'Émettre des prescriptions médicamenteuses'),
  ('ee000001-0000-0000-0000-000000000015', 'billing.create',        'Créer des feuilles de soins et devis'),
  ('ee000001-0000-0000-0000-000000000016', 'billing.read',          'Consulter la facturation'),
  ('ee000001-0000-0000-0000-000000000017', 'payment.collect',       'Encaisser des paiements patients'),
  ('ee000001-0000-0000-0000-000000000018', 'document.upload',       'Téléverser des documents (radios, photos, PDF)'),
  ('ee000001-0000-0000-0000-000000000019', 'document.read',         'Consulter les documents patients'),
  ('ee000001-0000-0000-0000-000000000020', 'radio.take',            'Réaliser des actes radiologiques'),
  ('ee000001-0000-0000-0000-000000000021', 'settings.cabinet',      'Gérer les paramètres du cabinet'),
  ('ee000001-0000-0000-0000-000000000022', 'settings.users',        'Gérer les utilisateurs et leurs rôles'),
  ('ee000001-0000-0000-0000-000000000023', 'reports.financial',     'Accéder aux rapports financiers');

-- ─── RBAC Matrix ─────────────────────────────────────────────────────────────
-- Roles:
--   titulaire          — Chirurgien-dentiste propriétaire (full access)
--   associe            — Chirurgien-dentiste associé (full clinical, limited admin)
--   collaborateur      — Chirurgien-dentiste collaborateur libéral
--   remplacant         — Praticien remplaçant (accès temporaire limité)
--   specialiste_odf    — Spécialiste orthodontie dentofaciale
--   specialiste_co     — Spécialiste chirurgie orale
--   specialiste_mbd    — Spécialiste médecine bucco-dentaire
--   assistant          — Assistant(e) dentaire qualifié(e)
--   assistant_formation— Assistant(e) dentaire en formation
--   aspbd              — Assistant(e) de soins en prévention bucco-dentaire
--   secretaire         — Secrétaire médical(e)
--   comptable          — Comptable cabinet
--   admin              — Administrateur système (IT)
--   prothesiste        — Prothésiste (accès plans de traitement pour fabrication)

-- ── titulaire ──────────────────────────────────────────────────────────────
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('titulaire', 'ee000001-0000-0000-0000-000000000001', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000002', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000003', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000004', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000005', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000006', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000007', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000008', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000009', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000010', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000011', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000012', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000013', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000014', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000015', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000016', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000017', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000018', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000019', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000020', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000021', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000022', 'all'),
  ('titulaire', 'ee000001-0000-0000-0000-000000000023', 'all');

-- ── associe ────────────────────────────────────────────────────────────────
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('associe', 'ee000001-0000-0000-0000-000000000001', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000002', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000003', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000004', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000005', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000006', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000007', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000008', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000009', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000010', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000011', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000012', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000013', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000014', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000015', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000016', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000017', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000018', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000019', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000020', 'all'),
  ('associe', 'ee000001-0000-0000-0000-000000000023', 'own');  -- own patients only for financial

-- ── collaborateur ──────────────────────────────────────────────────────────
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('collaborateur', 'ee000001-0000-0000-0000-000000000001', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000002', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000003', 'all'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000004', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000005', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000006', 'all'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000007', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000008', 'all'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000009', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000010', 'all'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000011', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000012', 'all'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000013', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000014', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000015', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000016', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000017', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000018', 'own'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000019', 'all'),
  ('collaborateur', 'ee000001-0000-0000-0000-000000000020', 'own');

-- ── remplacant ─────────────────────────────────────────────────────────────
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('remplacant', 'ee000001-0000-0000-0000-000000000001', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000002', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000003', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000004', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000005', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000006', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000007', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000008', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000009', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000010', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000011', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000012', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000013', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000014', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000015', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000016', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000017', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000018', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000019', 'own'),
  ('remplacant', 'ee000001-0000-0000-0000-000000000020', 'own');

-- ── specialiste_odf ────────────────────────────────────────────────────────
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000001', 'all'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000002', 'own'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000003', 'all'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000004', 'own'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000005', 'own'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000006', 'all'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000007', 'own'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000008', 'all'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000009', 'own'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000010', 'all'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000011', 'own'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000014', 'own'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000015', 'own'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000016', 'own'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000018', 'own'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000019', 'all'),
  ('specialiste_odf', 'ee000001-0000-0000-0000-000000000020', 'own');

-- ── specialiste_co ─────────────────────────────────────────────────────────
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000001', 'all'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000002', 'own'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000003', 'all'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000004', 'own'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000005', 'own'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000006', 'all'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000007', 'own'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000008', 'all'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000009', 'own'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000010', 'all'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000011', 'own'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000014', 'own'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000015', 'own'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000016', 'own'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000018', 'own'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000019', 'all'),
  ('specialiste_co', 'ee000001-0000-0000-0000-000000000020', 'own');

-- ── specialiste_mbd ────────────────────────────────────────────────────────
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000001', 'all'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000002', 'own'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000003', 'all'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000004', 'own'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000005', 'own'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000006', 'all'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000007', 'own'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000008', 'all'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000009', 'own'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000010', 'all'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000011', 'own'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000012', 'all'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000013', 'own'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000014', 'own'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000015', 'own'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000016', 'own'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000018', 'own'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000019', 'all'),
  ('specialiste_mbd', 'ee000001-0000-0000-0000-000000000020', 'own');

-- ── assistant ──────────────────────────────────────────────────────────────
-- Assistants may read clinical data, help take x-rays under supervision, upload docs
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('assistant', 'ee000001-0000-0000-0000-000000000001', 'all'),
  ('assistant', 'ee000001-0000-0000-0000-000000000002', 'all'),
  ('assistant', 'ee000001-0000-0000-0000-000000000003', 'all'),
  ('assistant', 'ee000001-0000-0000-0000-000000000006', 'all'),
  ('assistant', 'ee000001-0000-0000-0000-000000000008', 'all'),
  ('assistant', 'ee000001-0000-0000-0000-000000000010', 'all'),
  ('assistant', 'ee000001-0000-0000-0000-000000000012', 'all'),
  ('assistant', 'ee000001-0000-0000-0000-000000000016', 'all'),
  ('assistant', 'ee000001-0000-0000-0000-000000000017', 'all'),
  ('assistant', 'ee000001-0000-0000-0000-000000000018', 'all'),
  ('assistant', 'ee000001-0000-0000-0000-000000000019', 'all'),
  ('assistant', 'ee000001-0000-0000-0000-000000000020', 'all');

-- ── assistant_formation ────────────────────────────────────────────────────
-- Training assistants: read-only on clinical, no billing, no prescriptions
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('assistant_formation', 'ee000001-0000-0000-0000-000000000001', 'all'),
  ('assistant_formation', 'ee000001-0000-0000-0000-000000000003', 'all'),
  ('assistant_formation', 'ee000001-0000-0000-0000-000000000006', 'all'),
  ('assistant_formation', 'ee000001-0000-0000-0000-000000000008', 'all'),
  ('assistant_formation', 'ee000001-0000-0000-0000-000000000010', 'all'),
  ('assistant_formation', 'ee000001-0000-0000-0000-000000000012', 'all'),
  ('assistant_formation', 'ee000001-0000-0000-0000-000000000019', 'all');

-- ── aspbd ──────────────────────────────────────────────────────────────────
-- ASPBD: prevention-focused, limited scope
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('aspbd', 'ee000001-0000-0000-0000-000000000001', 'all'),
  ('aspbd', 'ee000001-0000-0000-0000-000000000002', 'all'),
  ('aspbd', 'ee000001-0000-0000-0000-000000000003', 'all'),
  ('aspbd', 'ee000001-0000-0000-0000-000000000006', 'all'),
  ('aspbd', 'ee000001-0000-0000-0000-000000000010', 'all'),
  ('aspbd', 'ee000001-0000-0000-0000-000000000018', 'all'),
  ('aspbd', 'ee000001-0000-0000-0000-000000000019', 'all');

-- ── secretaire ─────────────────────────────────────────────────────────────
-- Admin/scheduling focus — no clinical write, no prescriptions
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('secretaire', 'ee000001-0000-0000-0000-000000000001', 'all'),
  ('secretaire', 'ee000001-0000-0000-0000-000000000002', 'all'),
  ('secretaire', 'ee000001-0000-0000-0000-000000000006', 'all'),
  ('secretaire', 'ee000001-0000-0000-0000-000000000008', 'all'),
  ('secretaire', 'ee000001-0000-0000-0000-000000000015', 'all'),
  ('secretaire', 'ee000001-0000-0000-0000-000000000016', 'all'),
  ('secretaire', 'ee000001-0000-0000-0000-000000000017', 'all'),
  ('secretaire', 'ee000001-0000-0000-0000-000000000019', 'all');

-- ── comptable ──────────────────────────────────────────────────────────────
-- Financial access only
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('comptable', 'ee000001-0000-0000-0000-000000000016', 'all'),
  ('comptable', 'ee000001-0000-0000-0000-000000000023', 'all');

-- ── admin ──────────────────────────────────────────────────────────────────
-- IT admin — settings only, no clinical data
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('admin', 'ee000001-0000-0000-0000-000000000021', 'all'),
  ('admin', 'ee000001-0000-0000-0000-000000000022', 'all');

-- ── prothesiste ────────────────────────────────────────────────────────────
-- External lab technician — read PDT and clinical photos only
INSERT INTO role_permission (role, permission_id, scope) VALUES
  ('prothesiste', 'ee000001-0000-0000-0000-000000000008', 'all'),
  ('prothesiste', 'ee000001-0000-0000-0000-000000000010', 'all'),
  ('prothesiste', 'ee000001-0000-0000-0000-000000000019', 'all');
