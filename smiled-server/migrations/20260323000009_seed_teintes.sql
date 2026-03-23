-- Migration 009: Seed tooth shades — VITA Classical + VITA 3D-Master
-- VITA Classical: 16 shades (A1-D4, historically 4 hue groups)
-- VITA 3D-Master: 29 shades (5 lightness groups, 0M to 5M)

-- ─── VITA Classical (1956 — référence universelle) ────────────────────────────
-- Groupes: A (rougeâtre-brun), B (jaunâtre), C (grisâtre), D (rougeâtre-gris)
INSERT INTO teinte (systeme, code, libelle, ordre) VALUES
  ('VITA_Classical', 'A1',   'A1 — Blanc clair légèrement rougeâtre',         1),
  ('VITA_Classical', 'A2',   'A2 — Blanc légèrement rougeâtre',               2),
  ('VITA_Classical', 'A3',   'A3 — Jaune-rougeâtre moyen',                    3),
  ('VITA_Classical', 'A3.5', 'A3.5 — Jaune-rougeâtre plus intense',           4),
  ('VITA_Classical', 'A4',   'A4 — Jaune-rougeâtre foncé',                    5),
  ('VITA_Classical', 'B1',   'B1 — Blanc très légèrement jaunâtre',           6),
  ('VITA_Classical', 'B2',   'B2 — Blanc légèrement jaunâtre',                7),
  ('VITA_Classical', 'B3',   'B3 — Jaune moyen',                              8),
  ('VITA_Classical', 'B4',   'B4 — Jaune foncé',                              9),
  ('VITA_Classical', 'C1',   'C1 — Blanc légèrement grisâtre',               10),
  ('VITA_Classical', 'C2',   'C2 — Gris clair',                              11),
  ('VITA_Classical', 'C3',   'C3 — Gris moyen',                              12),
  ('VITA_Classical', 'C4',   'C4 — Gris foncé',                              13),
  ('VITA_Classical', 'D2',   'D2 — Rougeâtre-gris clair',                    14),
  ('VITA_Classical', 'D3',   'D3 — Rougeâtre-gris moyen',                    15),
  ('VITA_Classical', 'D4',   'D4 — Rougeâtre-gris foncé',                    16);

-- ─── VITA 3D-Master (1998 — système tridimensionnel) ─────────────────────────
-- Structure: [valeur L (0-5)][saturation M/L/R][teinte 1-3]
-- L = lightness (luminosité), M/L/R = medium/left/right (axe teinte)
-- Groupe 0: dents très claires (blanchies)
INSERT INTO teinte (systeme, code, libelle, ordre) VALUES
  ('VITA_3D_Master', '0M1',  '0M1 — Très clair neutre 1',                    1),
  ('VITA_3D_Master', '0M2',  '0M2 — Très clair neutre 2',                    2),
  ('VITA_3D_Master', '0M3',  '0M3 — Très clair neutre 3',                    3),
  -- Groupe 1: dents claires
  ('VITA_3D_Master', '1M1',  '1M1 — Clair neutre 1',                         4),
  ('VITA_3D_Master', '1M2',  '1M2 — Clair neutre 2',                         5),
  -- Groupe 2: dents de clarté moyenne
  ('VITA_3D_Master', '2L1.5','2L1.5 — Clarté moyenne, teinte gauche 1.5',    6),
  ('VITA_3D_Master', '2L2.5','2L2.5 — Clarté moyenne, teinte gauche 2.5',    7),
  ('VITA_3D_Master', '2M1',  '2M1 — Clarté moyenne, neutre 1',               8),
  ('VITA_3D_Master', '2M2',  '2M2 — Clarté moyenne, neutre 2',               9),
  ('VITA_3D_Master', '2M3',  '2M3 — Clarté moyenne, neutre 3',              10),
  ('VITA_3D_Master', '2R1.5','2R1.5 — Clarté moyenne, teinte droite 1.5',   11),
  ('VITA_3D_Master', '2R2.5','2R2.5 — Clarté moyenne, teinte droite 2.5',   12),
  -- Groupe 3: dents de clarté standard
  ('VITA_3D_Master', '3L1.5','3L1.5 — Standard, teinte gauche 1.5',         13),
  ('VITA_3D_Master', '3L2.5','3L2.5 — Standard, teinte gauche 2.5',         14),
  ('VITA_3D_Master', '3M1',  '3M1 — Standard, neutre 1',                    15),
  ('VITA_3D_Master', '3M2',  '3M2 — Standard, neutre 2',                    16),
  ('VITA_3D_Master', '3M3',  '3M3 — Standard, neutre 3',                    17),
  ('VITA_3D_Master', '3R1.5','3R1.5 — Standard, teinte droite 1.5',         18),
  ('VITA_3D_Master', '3R2.5','3R2.5 — Standard, teinte droite 2.5',         19),
  -- Groupe 4: dents foncées
  ('VITA_3D_Master', '4L1.5','4L1.5 — Foncé, teinte gauche 1.5',            20),
  ('VITA_3D_Master', '4L2.5','4L2.5 — Foncé, teinte gauche 2.5',            21),
  ('VITA_3D_Master', '4M1',  '4M1 — Foncé, neutre 1',                       22),
  ('VITA_3D_Master', '4M2',  '4M2 — Foncé, neutre 2',                       23),
  ('VITA_3D_Master', '4M3',  '4M3 — Foncé, neutre 3',                       24),
  ('VITA_3D_Master', '4R1.5','4R1.5 — Foncé, teinte droite 1.5',            25),
  ('VITA_3D_Master', '4R2.5','4R2.5 — Foncé, teinte droite 2.5',            26),
  -- Groupe 5: dents très foncées
  ('VITA_3D_Master', '5M1',  '5M1 — Très foncé, neutre 1',                  27),
  ('VITA_3D_Master', '5M2',  '5M2 — Très foncé, neutre 2',                  28),
  ('VITA_3D_Master', '5M3',  '5M3 — Très foncé, neutre 3',                  29);
