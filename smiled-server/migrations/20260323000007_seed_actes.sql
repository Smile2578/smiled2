-- Migration 007: Seed ~150 dental acts across 14 categories
-- Convention dentaire 2023-2028, codes CCAM/NGAP valid 2024-2026
-- Sources: CCAM v56+, NGAP juillet 2023, convention nationale dentaire

-- ─── Categories ──────────────────────────────────────────────────────────────
INSERT INTO categorie_acte (id, code, libelle, ordre_affichage, icone) VALUES
  ('c1000000-0000-0000-0000-000000000001', 'CONS',    'Consultations et prévention',           1,  'stethoscope'),
  ('c1000000-0000-0000-0000-000000000002', 'RADIO',   'Radiologie',                            2,  'x-ray'),
  ('c1000000-0000-0000-0000-000000000003', 'ENDO',    'Endodontie',                            3,  'activity'),
  ('c1000000-0000-0000-0000-000000000004', 'PERIO',   'Parodontologie',                        4,  'layers'),
  ('c1000000-0000-0000-0000-000000000005', 'CHIR',    'Chirurgie',                             5,  'scissors'),
  ('c1000000-0000-0000-0000-000000000006', 'PROTH_F', 'Prothèse fixe',                         6,  'crown'),
  ('c1000000-0000-0000-0000-000000000007', 'PROTH_A', 'Prothèse amovible',                     7,  'smile'),
  ('c1000000-0000-0000-0000-000000000008', 'IMPL',    'Implantologie',                         8,  'anchor'),
  ('c1000000-0000-0000-0000-000000000009', 'ODF',     'Orthodontie',                           9,  'align-center'),
  ('c1000000-0000-0000-0000-000000000010', 'ESTH',    'Esthétique',                           10,  'sparkles'),
  ('c1000000-0000-0000-0000-000000000011', 'REST',    'Odontologie restauratrice',            11,  'tool'),
  ('c1000000-0000-0000-0000-000000000012', 'PED',     'Pédodontie',                           12,  'baby'),
  ('c1000000-0000-0000-0000-000000000013', 'OCCL',    'Occlusodontie',                        13,  'settings'),
  ('c1000000-0000-0000-0000-000000000014', 'DIVERS',  'Divers / Hors nomenclature',           14,  'more-horizontal');

-- ─── CONS — Consultations et prévention ──────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ngap, code_ccam, libelle, base_secu, prix_defaut, panier_rac, modificateur_jeune, date_entree_vigueur, notes_convention) VALUES
  ('c1000000-0000-0000-0000-000000000001', 'ngap',  'C',    NULL,             'Consultation de médecine bucco-dentaire',                       26.50,  26.50,  'rac_0',  false, '2023-10-01', 'Secteur 1 tarif opposable'),
  ('c1000000-0000-0000-0000-000000000001', 'ngap',  'CS',   NULL,             'Consultation spécialisée (spécialiste qualifié)',                35.00,  35.00,  'rac_0',  false, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000001', 'ngap',  'CC',   NULL,             'Consultation coordonnée (dans cadre MT)',                        26.50,  26.50,  'rac_0',  false, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000001', 'ngap',  'D',    NULL,             'Détartrage supra et sous-gingival, polissage (deux séances/an)', 28.92,  28.92,  'rac_0',  false, '2023-10-01', 'Deux séances annuelles remboursées'),
  ('c1000000-0000-0000-0000-000000000001', 'ngap',  'SCC',  NULL,             'Scellement sillons (dents 6 et 7, 6-12 ans)',                    16.38,  16.38,  'rac_0',  true,  '2023-10-01', 'Enfants 6-12 ans, molaires permanentes'),
  ('c1000000-0000-0000-0000-000000000001', 'ccam',  NULL,   'HBMD001',        'Application de vernis fluoré (3 ans et moins)',                   0.00,  12.00,  NULL,     true,  '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000001', 'ngap',  'BP',   NULL,             'Bilan de santé bucco-dentaire (M''T Dents 3-24 ans)',             40.00,  40.00,  'rac_0',  true,  '2023-10-01', 'Examen préventif M''T Dents'),
  ('c1000000-0000-0000-0000-000000000001', 'ccam',  NULL,   'HBQD001',        'Bilan parodontal approfondi',                                     0.00,  60.00,  NULL,     false, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000001', 'hors_nomenclature', NULL, NULL,   'Consultation initiale longue (anamnèse complète)',                 0.00,  85.00,  'libre',  false, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000001', 'hors_nomenclature', NULL, NULL,   'Urgence dentaire (hors heures ouvrables)',                         0.00,  65.00,  'libre',  false, NULL, NULL);

-- ─── RADIO — Radiologie ───────────────────────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ngap, code_ccam, libelle, base_secu, prix_defaut, panier_rac, dent_requise, date_entree_vigueur, notes_convention) VALUES
  ('c1000000-0000-0000-0000-000000000002', 'ngap',  NULL,   NULL,             'Rétro-alvéolaire (1 film)',                                       5.64,   5.64,  'rac_0',  true,  '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000002', 'ngap',  'RS',   NULL,             'Bilan long cône (12 rétros minimaux)',                           125.04, 125.04, 'rac_0',  false, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000002', 'ngap',  'P',    NULL,             'Panoramique dentaire (orthopantomogramme)',                       26.88,  26.88,  'rac_0',  false, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000002', 'ccam',  NULL,   'LAQK002',        'Téléradiographie de profil',                                      0.00,  55.00,  NULL,     false, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000002', 'ccam',  NULL,   'LAQK003',        'Téléradiographie de face',                                        0.00,  45.00,  NULL,     false, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000002', 'ccam',  NULL,   'LAQK012',        'Tomographie volumique à faisceau conique (CBCT) dento-maxillaire', 0.00, 200.00, 'libre',  false, '2023-10-01', 'Cône beam — non remboursé SS'),
  ('c1000000-0000-0000-0000-000000000002', 'hors_nomenclature', NULL, NULL,   'CBCT implantaire (volumétrie 3D)',                                 0.00, 220.00,  'libre',  false, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000002', 'hors_nomenclature', NULL, NULL,   'Céphalométrie numérique + analyse',                               0.00,  80.00,  'libre',  false, NULL, NULL);

-- ─── ENDO — Endodontie ────────────────────────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ngap, code_ccam, libelle, base_secu, prix_defaut, panier_rac, dent_requise, nb_seances_typique, date_entree_vigueur, notes_convention) VALUES
  ('c1000000-0000-0000-0000-000000000003', 'ngap',  NULL,   NULL,             'Coiffage pulpaire direct (MTA/biocéramique)',                     18.06,  18.06,  'rac_0',  true,  2, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000003', 'ngap',  NULL,   NULL,             'Pulpotomie cervicale',                                           24.07,  24.07,  'rac_0',  true,  1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000003', 'ngap',  NULL,   'HBFD001',        'Traitement endodontique 1 canal (monoradiculée)',                 107.30, 107.30, 'rac_0',  true,  2, '2023-10-01', 'Remboursement SS depuis convention 2024'),
  ('c1000000-0000-0000-0000-000000000003', 'ngap',  NULL,   'HBFD002',        'Traitement endodontique 2 canaux',                               134.13, 134.13, 'rac_0',  true,  2, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000003', 'ngap',  NULL,   'HBFD003',        'Traitement endodontique 3 canaux',                               160.95, 160.95, 'rac_0',  true,  3, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000003', 'ngap',  NULL,   'HBFD004',        'Traitement endodontique 4 canaux (molaire)',                     187.78, 187.78, 'rac_0',  true,  3, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000003', 'ccam',  NULL,   'HBFD010',        'Retraitement endodontique 1 canal',                               0.00, 180.00,  'libre',  true,  3, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000003', 'ccam',  NULL,   'HBFD011',        'Retraitement endodontique 2 canaux',                              0.00, 220.00,  'libre',  true,  3, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000003', 'ccam',  NULL,   'HBFD012',        'Retraitement endodontique 3+ canaux',                             0.00, 260.00,  'libre',  true,  4, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000003', 'hors_nomenclature', NULL, NULL,   'Inlay core coulé (pivotage préprothétique)',                       0.00, 220.00,  'libre',  true,  1, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000003', 'hors_nomenclature', NULL, NULL,   'Inlay core fibré (tenon préfabriqué + composite)',                  0.00, 150.00, 'libre',  true,  1, NULL, NULL);

-- ─── PERIO — Parodontologie ───────────────────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ngap, code_ccam, libelle, base_secu, prix_defaut, panier_rac, nb_seances_typique, date_entree_vigueur, notes_convention) VALUES
  ('c1000000-0000-0000-0000-000000000004', 'ccam',  NULL,   'HBQD100',        'Surfaçage radiculaire (par sextant)',                              28.30,  28.30,  'rac_0',  2, '2023-10-01', 'Maximum 4 sextants remboursés/an depuis 2024'),
  ('c1000000-0000-0000-0000-000000000004', 'ccam',  NULL,   'HBQD200',        'Lambeau chirurgical d''assainissement parodontal (par sextant)',    0.00, 350.00,  'libre',  2, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000004', 'ccam',  NULL,   'HBQD300',        'Greffe gingivale conjonctive enfouie (CTG)',                        0.00, 650.00,  'libre',  1, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000004', 'ccam',  NULL,   'HBQD301',        'Greffe gingivale épithélio-conjonctive (FGG)',                      0.00, 550.00,  'libre',  1, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000004', 'ccam',  NULL,   'HBQD400',        'Régénération tissulaire guidée (RTG) + membrane résorbable',        0.00, 750.00,  'libre',  1, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000004', 'ccam',  NULL,   'HBQD401',        'Greffe osseuse parodontale (comblement de poche)',                  0.00, 600.00,  'libre',  1, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000004', 'hors_nomenclature', NULL, NULL,   'Assainissement parodontal complet (1 à 3 jours)',                   0.00,1200.00,  'libre',  3, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000004', 'hors_nomenclature', NULL, NULL,   'Réévaluation parodontale post-traitement',                          0.00,  60.00,  NULL,     1, NULL, NULL);

-- ─── CHIR — Chirurgie ─────────────────────────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ngap, code_ccam, libelle, base_secu, prix_defaut, panier_rac, dent_requise, nb_seances_typique, date_entree_vigueur, notes_convention) VALUES
  ('c1000000-0000-0000-0000-000000000005', 'ngap',  'D1',   NULL,             'Extraction d''une dent permanente simple (mono-radiculée)',        33.66,  33.66,  'rac_0',  true,  1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000005', 'ngap',  'D2',   NULL,             'Extraction d''une dent permanente (pluri-radiculée, complexe)',    50.50,  50.50,  'rac_0',  true,  1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000005', 'ccam',  NULL,   'HBJA001',        'Avulsion dent de sagesse incluse (ostéotomie)',                     61.60,  61.60,  'rac_0',  true,  1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000005', 'ccam',  NULL,   'HBJA002',        'Avulsion dent de sagesse semi-incluse',                            45.00,  45.00,  'rac_0',  true,  1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000005', 'ccam',  NULL,   'HBJA003',        'Avulsion dent incluse non sapale (canine, prémolaire)',             57.50,  57.50,  'rac_0',  true,  1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000005', 'ccam',  NULL,   'HBJA010',        'Alvéolectomie / régularisation osseuse (par secteur)',              0.00, 180.00,  'libre',  false, 1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000005', 'ccam',  NULL,   'HBJA020',        'Résection apicale avec obturation rétrograde',                     0.00, 350.00,  'libre',  true,  1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000005', 'ccam',  NULL,   'HBJA030',        'Biopsie de muqueuse buccale',                                      0.00, 120.00,  NULL,     false, 1, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000005', 'ccam',  NULL,   'HBJA040',        'Frenectomie (frein labial ou lingual)',                             0.00, 180.00,  NULL,     false, 1, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000005', 'ccam',  NULL,   'HBJA050',        'Kyste odontogénique — énucléation (accès simple)',                  0.00, 280.00,  NULL,     false, 1, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000005', 'hors_nomenclature', NULL, NULL,   'Traction orthodontique de dent incluse',                            0.00, 420.00,  'libre',  true,  2, NULL, NULL);

-- ─── PROTH_F — Prothèse fixe ──────────────────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ngap, code_ccam, libelle, base_secu, prix_defaut, panier_rac, plafond_rac_0, teinte_requise, dent_requise, nb_seances_typique, date_entree_vigueur, notes_convention) VALUES
  -- RAC0 crowns (convention 2024)
  ('c1000000-0000-0000-0000-000000000006', 'ccam', NULL, 'HBLD001', 'Couronne céramo-métallique (CCM) — RAC0',                        107.50, 107.50, 'rac_0', 1070.00, true, true, 3, '2024-01-01', 'Plafond RAC0 secteur 2 variable par zone'),
  ('c1000000-0000-0000-0000-000000000006', 'ccam', NULL, 'HBLD002', 'Couronne céramique monolithique (zircone/e.max) — RAC0',         107.50, 107.50, 'rac_0', 1070.00, true, true, 3, '2024-01-01', NULL),
  ('c1000000-0000-0000-0000-000000000006', 'ccam', NULL, 'HBLD003', 'Couronne tout-métal (NP, CoCr)',                                 107.50, 107.50, 'rac_0', 1070.00, false, true, 2, '2024-01-01', NULL),
  -- Modéré
  ('c1000000-0000-0000-0000-000000000006', 'ccam', NULL, 'HBLD010', 'Couronne implanto-portée vissée (PFM)',                            0.00,1200.00, 'modere', NULL, true, true, 2, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000006', 'ccam', NULL, 'HBLD011', 'Couronne implanto-portée full céramique',                          0.00,1400.00, 'modere', NULL, true, true, 2, '2023-10-01', NULL),
  -- Bridge
  ('c1000000-0000-0000-0000-000000000006', 'ccam', NULL, 'HBLD020', 'Élément de bridge intermédiaire (pontique) — RAC0',              107.50, 107.50, 'rac_0', 1070.00, true, false, 3, '2024-01-01', 'Prix par élément'),
  ('c1000000-0000-0000-0000-000000000006', 'ccam', NULL, 'HBLD021', 'Bridge 3 éléments sur dents naturelles',                          0.00,2200.00, 'modere', NULL, true, true, 3, '2023-10-01', NULL),
  -- Libre
  ('c1000000-0000-0000-0000-000000000006', 'ccam', NULL, 'HBLD030', 'Facette céramique (e.max pressé)',                                 0.00, 800.00, 'libre', NULL, true, true, 2, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000006', 'ccam', NULL, 'HBLD031', 'Couronne provisoire directe (PMMA)',                               0.00, 120.00, 'libre', NULL, false, true, 1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000006', 'hors_nomenclature', NULL, NULL, 'Inlay-onlay céramique (e.max / zircone) — 1 face',         0.00, 600.00, 'libre', NULL, true, true, 2, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000006', 'hors_nomenclature', NULL, NULL, 'Inlay-onlay céramique — 2+ faces',                         0.00, 750.00, 'libre', NULL, true, true, 2, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000006', 'hors_nomenclature', NULL, NULL, 'Overlay céramique complète (recouvrement cuspidien total)', 0.00, 950.00, 'libre', NULL, true, true, 2, NULL, NULL);

-- ─── PROTH_A — Prothèse amovible ─────────────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ngap, code_ccam, libelle, base_secu, prix_defaut, panier_rac, plafond_rac_0, nb_seances_typique, date_entree_vigueur, notes_convention) VALUES
  ('c1000000-0000-0000-0000-000000000007', 'ccam', NULL, 'HBPD001', 'PAP résine (1-4 dents)',                                           208.74, 208.74, 'rac_0', 630.00, 3, '2024-01-01', 'RAC0 si conditions respectées convention 2024'),
  ('c1000000-0000-0000-0000-000000000007', 'ccam', NULL, 'HBPD002', 'PAP résine (5 dents et plus)',                                     246.29, 246.29, 'rac_0', 745.00, 3, '2024-01-01', NULL),
  ('c1000000-0000-0000-0000-000000000007', 'ccam', NULL, 'HBPD003', 'PAP métal (châssis coulé, 1-4 dents)',                               0.00, 900.00, 'modere', NULL, 3, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000007', 'ccam', NULL, 'HBPD004', 'PAP métal (châssis coulé, 5+ dents)',                                0.00,1100.00, 'modere', NULL, 4, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000007', 'ccam', NULL, 'HBPD010', 'PAC complète mono-maxillaire (résine PMMA)',                        246.29, 246.29, 'rac_0', 745.00, 3, '2024-01-01', NULL),
  ('c1000000-0000-0000-0000-000000000007', 'ccam', NULL, 'HBPD011', 'PAC complète bi-maxillaire (résine PMMA)',                          492.58, 492.58, 'rac_0', 1490.00, 4, '2024-01-01', NULL),
  ('c1000000-0000-0000-0000-000000000007', 'hors_nomenclature', NULL, NULL, 'Prothèse sur implants (overdenture, 2 attachements)',         0.00,2200.00, 'libre', NULL, 4, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000007', 'ngap', 'R', NULL,       'Réparation PAP/PAC simple (adjonction dent)',                       12.50,  12.50, 'rac_0', NULL, 1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000007', 'hors_nomenclature', NULL, NULL, 'Rebasage direct (résine auto-polymérisante)',                  0.00, 120.00, 'libre', NULL, 1, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000007', 'hors_nomenclature', NULL, NULL, 'Rebasage indirect (laboratoire)',                              0.00, 280.00, 'libre', NULL, 2, NULL, NULL);

-- ─── IMPL — Implantologie ─────────────────────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ccam, libelle, base_secu, prix_defaut, panier_rac, dent_requise, nb_seances_typique, notes_convention) VALUES
  ('c1000000-0000-0000-0000-000000000008', 'ccam', 'HBRD001', 'Pose d''implant endo-osseux (titane)',                                   0.00, 900.00, 'libre', true,  1, NULL),
  ('c1000000-0000-0000-0000-000000000008', 'ccam', 'HBRD002', 'Mise en charge / deuxième temps chirurgical (cicatriciel)',              0.00, 280.00, 'libre', true,  1, NULL),
  ('c1000000-0000-0000-0000-000000000008', 'ccam', 'HBRD003', 'Pilier implantaire (vissé, standard)',                                   0.00, 350.00, 'libre', true,  1, NULL),
  ('c1000000-0000-0000-0000-000000000008', 'ccam', 'HBRD004', 'Pilier angulé ou sur-mesure (CFAO)',                                     0.00, 500.00, 'libre', true,  1, NULL),
  ('c1000000-0000-0000-0000-000000000008', 'ccam', 'HBRD010', 'Comblement sinusien (Sinus lift) par voie crêtale (méthode d''Summers)',  0.00,1200.00, 'libre', false, 1, NULL),
  ('c1000000-0000-0000-0000-000000000008', 'ccam', 'HBRD011', 'Comblement sinusien par voie latérale (open sinus)',                     0.00,2500.00, 'libre', false, 1, NULL),
  ('c1000000-0000-0000-0000-000000000008', 'ccam', 'HBRD020', 'ROG (régénération osseuse guidée) — membrane + comblement',              0.00,1500.00, 'libre', false, 1, NULL),
  ('c1000000-0000-0000-0000-000000000008', 'hors_nomenclature', NULL, 'Dépose implant fracturé ou en échec',                             0.00, 400.00, 'libre', true,  1, NULL),
  ('c1000000-0000-0000-0000-000000000008', 'hors_nomenclature', NULL, 'Implant zircone monopièce',                                       0.00,1100.00, 'libre', true,  1, NULL),
  ('c1000000-0000-0000-0000-000000000008', 'hors_nomenclature', NULL, 'Gouttière radiologique / guide chirurgical (par arcade)',          0.00, 250.00, NULL,    false, 1, NULL);

-- ─── ODF — Orthodontie ────────────────────────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ccam, libelle, base_secu, prix_defaut, panier_rac, nb_seances_typique, notes_convention) VALUES
  ('c1000000-0000-0000-0000-000000000009', 'ngap', NULL, 'Consultation orthodontique initiale + analyse',                               0.00,  80.00, NULL,    1, NULL),
  ('c1000000-0000-0000-0000-000000000009', 'ngap', NULL, 'Moulages d''étude + empreintes numériques',                                   0.00, 120.00, NULL,    1, NULL),
  ('c1000000-0000-0000-0000-000000000009', 'ccam', 'HBQD050', 'Traitement ODF par appareil amovible (par semestre, <16 ans)',           193.50, 193.50, 'rac_0', 2, 'Remboursé SS jusqu''à 16 ans'),
  ('c1000000-0000-0000-0000-000000000009', 'ccam', 'HBQD051', 'Traitement ODF par multi-attaches fixes (par semestre, <16 ans)',        193.50, 193.50, 'rac_0', 12, 'Remboursé SS jusqu''à 16 ans'),
  ('c1000000-0000-0000-0000-000000000009', 'ccam', 'HBQD052', 'Traitement ODF adulte multi-attaches (par semestre, HN)',                 0.00, 600.00, 'libre', 12, NULL),
  ('c1000000-0000-0000-0000-000000000009', 'hors_nomenclature', NULL, 'Gouttières thermoformées invisibles (contention active)',          0.00,1500.00, 'libre',  8, NULL),
  ('c1000000-0000-0000-0000-000000000009', 'hors_nomenclature', NULL, 'Contention fixe collée (fil multibrin)',                           0.00, 180.00, 'libre',  1, NULL),
  ('c1000000-0000-0000-0000-000000000009', 'hors_nomenclature', NULL, 'Contention amovible (Hawley, Essix)',                              0.00, 150.00, 'libre',  1, NULL),
  ('c1000000-0000-0000-0000-000000000009', 'hors_nomenclature', NULL, 'Pose de bagues orthodontiques (par arcade)',                       0.00, 320.00, 'libre',  1, NULL),
  ('c1000000-0000-0000-0000-000000000009', 'hors_nomenclature', NULL, 'Activation mensuelle multi-attaches',                              0.00,  80.00, 'libre',  1, NULL);

-- ─── ESTH — Esthétique ────────────────────────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ccam, libelle, base_secu, prix_defaut, panier_rac, teinte_requise, dent_requise, nb_seances_typique) VALUES
  ('c1000000-0000-0000-0000-000000000010', 'hors_nomenclature', NULL, 'Blanchiment ambulatoire (gouttières + gel 2 semaines)',            0.00, 350.00, 'libre', false, false, 2),
  ('c1000000-0000-0000-0000-000000000010', 'hors_nomenclature', NULL, 'Blanchiment en cabinet (lampe / laser)',                           0.00, 500.00, 'libre', false, false, 1),
  ('c1000000-0000-0000-0000-000000000010', 'hors_nomenclature', NULL, 'Blanchiment dent dévitalisée (walking bleach)',                    0.00, 180.00, 'libre', false, true,  3),
  ('c1000000-0000-0000-0000-000000000010', 'hors_nomenclature', NULL, 'Facette composite directe (cosmétique, 1 dent)',                   0.00, 280.00, 'libre', true,  true,  1),
  ('c1000000-0000-0000-0000-000000000010', 'hors_nomenclature', NULL, 'Diastème dentaire — fermeture composite (2 dents)',                0.00, 420.00, 'libre', true,  false, 1),
  ('c1000000-0000-0000-0000-000000000010', 'hors_nomenclature', NULL, 'Reconstitution coronaire esthétique globale (6 dents)',            0.00,2800.00, 'libre', true,  false, 3);

-- ─── REST — Odontologie restauratrice ─────────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ngap, code_ccam, libelle, base_secu, prix_defaut, panier_rac, faces_requises, dent_requise, nb_seances_typique, date_entree_vigueur, notes_convention) VALUES
  ('c1000000-0000-0000-0000-000000000011', 'ngap', 'S1', NULL, 'Soin conservateur dent permanente 1 face (amalgame interdit 2025)',    15.60,  15.60, 'rac_0', true, true, 1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000011', 'ngap', 'S2', NULL, 'Soin conservateur dent permanente 2 faces',                           19.38,  19.38, 'rac_0', true, true, 1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000011', 'ngap', 'S3', NULL, 'Soin conservateur dent permanente 3 faces et plus',                   26.25,  26.25, 'rac_0', true, true, 1, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000011', 'ccam', NULL, 'HBMD100', 'Restauration au composite — 1 face (résine nano-hybride)',         0.00,  70.00, 'libre', true, true, 1, '2024-01-01', 'Composite hors nomenclature NGAP mais remboursé CCAM depuis 2024 pour dents antérieures'),
  ('c1000000-0000-0000-0000-000000000011', 'ccam', NULL, 'HBMD101', 'Restauration au composite — 2 faces',                             0.00,  90.00, 'libre', true, true, 1, '2024-01-01', NULL),
  ('c1000000-0000-0000-0000-000000000011', 'ccam', NULL, 'HBMD102', 'Restauration au composite — 3 faces et plus',                     0.00, 120.00, 'libre', true, true, 1, '2024-01-01', NULL),
  ('c1000000-0000-0000-0000-000000000011', 'ccam', NULL, 'HBMD110', 'Restauration au CVI (verre ionomère) — 1 face',                   0.00,  45.00, NULL,   true, true, 1, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000011', 'ccam', NULL, 'HBMD111', 'Restauration au CVIMAR — 1 face',                                 0.00,  55.00, NULL,   true, true, 1, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000011', 'hors_nomenclature', NULL, NULL, 'Inlay composite — 2 faces',                                0.00, 420.00, 'libre', true, true, 2, NULL, NULL),
  ('c1000000-0000-0000-0000-000000000011', 'hors_nomenclature', NULL, NULL, 'Onlay composite — 3+ faces (cuspides couvertes)',           0.00, 580.00, 'libre', true, true, 2, NULL, NULL);

-- ─── PED — Pédodontie ────────────────────────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ngap, code_ccam, libelle, base_secu, prix_defaut, panier_rac, modificateur_jeune, dent_requise, faces_requises, date_entree_vigueur, notes_convention) VALUES
  ('c1000000-0000-0000-0000-000000000012', 'ngap', NULL, 'HBMD010', 'Coiffage pulpaire indirect (CVI / hydroxyde de calcium)',          11.00,  11.00, 'rac_0', true, true, false,  '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000012', 'ngap', NULL, 'HBMD011', 'Pulpotomie partielle (MTA, dent temporaire)',                      21.00,  21.00, 'rac_0', true, true, false,  '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000012', 'ngap', 'SCC', NULL,     'Scellement prophylactique sillons (6 et 7, 6-12 ans)',             16.38,  16.38, 'rac_0', true, true, false,  '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000012', 'ngap', NULL, 'HBMD020', 'Application topique de fluorure professionnel (6 mois et moins)', 0.00,   12.00,  NULL,   true, false, false, '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000012', 'ngap', NULL, NULL,      'Maintien d''espace (perte prématurée dent temporaire)',            18.06,  18.06, 'rac_0', true, true, false,  '2023-10-01', NULL),
  ('c1000000-0000-0000-0000-000000000012', 'hors_nomenclature', NULL, NULL, 'Couronne pédiatrique acier inoxydable',                    0.00,  120.00, 'libre', true, true, false,  NULL, NULL),
  ('c1000000-0000-0000-0000-000000000012', 'hors_nomenclature', NULL, NULL, 'Couronne pédiatrique zircone (esthétique)',                 0.00,  280.00, 'libre', true, true, false,  NULL, NULL),
  ('c1000000-0000-0000-0000-000000000012', 'hors_nomenclature', NULL, NULL, 'Bilan bucco-dentaire enfant (photo intra-orale + OPT)',     0.00,   65.00,  NULL,   true, false, false, NULL, NULL);

-- ─── OCCL — Occlusodontie ─────────────────────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ccam, libelle, base_secu, prix_defaut, panier_rac, nb_seances_typique) VALUES
  ('c1000000-0000-0000-0000-000000000013', 'hors_nomenclature', NULL, 'Analyse occlusale instrumentale + montage articulateur',           0.00, 220.00, 'libre', 2),
  ('c1000000-0000-0000-0000-000000000013', 'hors_nomenclature', NULL, 'Gouttière de protection nyctiparafonction (bruxisme)',             0.00, 350.00, 'libre', 2),
  ('c1000000-0000-0000-0000-000000000013', 'hors_nomenclature', NULL, 'Orthèse d''avancée mandibulaire (OAM) pour SAHOS',                0.00, 800.00, 'libre', 3),
  ('c1000000-0000-0000-0000-000000000013', 'hors_nomenclature', NULL, 'Ajustement occlusale (meulage sélectif)',                         0.00,  90.00, 'libre', 1),
  ('c1000000-0000-0000-0000-000000000013', 'hors_nomenclature', NULL, 'Gouttière de repositionnement condylien (ATM)',                   0.00, 450.00, 'libre', 3),
  ('c1000000-0000-0000-0000-000000000013', 'hors_nomenclature', NULL, 'Enregistrement arc facial + transfert articulateur semi-adaptatif', 0.00, 120.00, NULL,   1);

-- ─── DIVERS — Divers / Hors nomenclature ─────────────────────────────────────
INSERT INTO acte (categorie_id, nomenclature, code_ngap, libelle, base_secu, prix_defaut, panier_rac, nb_seances_typique) VALUES
  ('c1000000-0000-0000-0000-000000000014', 'ngap',  'C', 'Certificat médical dentaire (déclaration sinistre, MDPH)',            0.00,  26.50, NULL, 1),
  ('c1000000-0000-0000-0000-000000000014', 'hors_nomenclature', NULL, 'Devis / plan de traitement détaillé',                    0.00,   0.00, NULL, 1),
  ('c1000000-0000-0000-0000-000000000014', 'hors_nomenclature', NULL, 'Urgence douloureuse — prescription + pansement sédatif', 0.00,  55.00, NULL, 1),
  ('c1000000-0000-0000-0000-000000000014', 'hors_nomenclature', NULL, 'Gestion d''une alvéolite sèche (curetage + méchage)',    0.00,  65.00, NULL, 2),
  ('c1000000-0000-0000-0000-000000000014', 'hors_nomenclature', NULL, 'Prescription antibiotique + arrêt de travail',           0.00,  15.00, NULL, 1),
  ('c1000000-0000-0000-0000-000000000014', 'hors_nomenclature', NULL, 'Désensibilisation (hypersensibilité dentinaire)',         0.00,  40.00, NULL, 2),
  ('c1000000-0000-0000-0000-000000000014', 'hors_nomenclature', NULL, 'Traumatisme dentaire — bilan initial + contention',      0.00, 150.00, NULL, 1),
  ('c1000000-0000-0000-0000-000000000014', 'hors_nomenclature', NULL, 'Recollage fragment coronaire',                           0.00,  80.00, NULL, 1),
  ('c1000000-0000-0000-0000-000000000014', 'hors_nomenclature', NULL, 'Consultation conseil (deuxième avis)',                   0.00,  85.00, NULL, 1),
  ('c1000000-0000-0000-0000-000000000014', 'hors_nomenclature', NULL, 'Anesthésie locale supplémentaire (volume majeur)',       0.00,  20.00, NULL, 1);
