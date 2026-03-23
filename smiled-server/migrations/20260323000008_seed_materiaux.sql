-- Migration 008: Seed ~80 dental materials with full hierarchy
-- Amalgame flagged obsolete=true (EU ban January 2025 — Regulation 2023/1481)
-- UUID prefix bb00 = categorie_materiau, bb10 = materiau

-- ─── Categories (hierarchy) ──────────────────────────────────────────────────
INSERT INTO categorie_materiau (id, code, libelle, parent_id) VALUES
  -- Level 1 roots
  ('bb000000-0000-0000-0000-000000000001', 'CERAM',    'Céramiques',                     NULL),
  ('bb000000-0000-0000-0000-000000000002', 'COMPO',    'Composites',                     NULL),
  ('bb000000-0000-0000-0000-000000000003', 'CVI_ALL',  'Verres ionomères (CVI)',          NULL),
  ('bb000000-0000-0000-0000-000000000004', 'METAL',    'Métaux et alliages',              NULL),
  ('bb000000-0000-0000-0000-000000000005', 'RESINE',   'Résines prothétiques',            NULL),
  ('bb000000-0000-0000-0000-000000000006', 'IMPL_MAT', 'Implants',                        NULL),
  ('bb000000-0000-0000-0000-000000000007', 'COMBOSS',  'Comblement et régénération',      NULL),
  ('bb000000-0000-0000-0000-000000000008', 'EMPRE',    'Matériaux d''empreinte',           NULL),
  ('bb000000-0000-0000-0000-000000000009', 'CIMENT',   'Ciments et agents de liaison',    NULL),
  -- Level 2: ceramics sub-categories
  ('bb000000-0000-0000-0000-000000000011', 'ZIRC',     'Zircone',                         'bb000000-0000-0000-0000-000000000001'),
  ('bb000000-0000-0000-0000-000000000012', 'VITRO',    'Vitrocéramique (e.max / LS2)',    'bb000000-0000-0000-0000-000000000001'),
  ('bb000000-0000-0000-0000-000000000013', 'FELD',     'Céramique feldspathique',         'bb000000-0000-0000-0000-000000000001'),
  -- Level 2: composites sub-categories
  ('bb000000-0000-0000-0000-000000000021', 'COMPO_UNI', 'Composites universels / nano-hybrides', 'bb000000-0000-0000-0000-000000000002'),
  ('bb000000-0000-0000-0000-000000000022', 'COMPO_BF',  'Composites bulk-fill',                  'bb000000-0000-0000-0000-000000000002'),
  ('bb000000-0000-0000-0000-000000000023', 'COMPO_FL',  'Composites flowable',                   'bb000000-0000-0000-0000-000000000002'),
  ('bb000000-0000-0000-0000-000000000024', 'COMPO_CB',  'Core build-up (reconstitution)',         'bb000000-0000-0000-0000-000000000002'),
  -- Level 2: CVI sub-categories
  ('bb000000-0000-0000-0000-000000000031', 'CVI_CONV',  'CVI conventionnel',                     'bb000000-0000-0000-0000-000000000003'),
  ('bb000000-0000-0000-0000-000000000032', 'CVI_MAR',   'CVIMAR (modifié résine)',                'bb000000-0000-0000-0000-000000000003'),
  -- Level 2: metal sub-categories
  ('bb000000-0000-0000-0000-000000000041', 'NP_METAL',  'Alliage non précieux (CoCr, NiCr)',      'bb000000-0000-0000-0000-000000000004'),
  ('bb000000-0000-0000-0000-000000000042', 'TITA_ALLIA','Titane pur / alliage titane',            'bb000000-0000-0000-0000-000000000004'),
  ('bb000000-0000-0000-0000-000000000043', 'OR_ALLIA',  'Alliage or (semi-précieux)',              'bb000000-0000-0000-0000-000000000004'),
  ('bb000000-0000-0000-0000-000000000044', 'AMALG',     'Amalgame (interdit EU 2025)',             'bb000000-0000-0000-0000-000000000004'),
  -- Level 2: resin sub-categories
  ('bb000000-0000-0000-0000-000000000051', 'PMMA',      'PMMA (acrylique prothétique)',            'bb000000-0000-0000-0000-000000000005'),
  ('bb000000-0000-0000-0000-000000000052', 'POLY_AM',   'Polyamide (prothèse flexible)',          'bb000000-0000-0000-0000-000000000005'),
  -- Level 2: implant sub-categories
  ('bb000000-0000-0000-0000-000000000061', 'IMPL_TI4',  'Implant titane grade 4 (cp-Ti)',         'bb000000-0000-0000-0000-000000000006'),
  ('bb000000-0000-0000-0000-000000000062', 'IMPL_TI5',  'Implant titane grade 5 (Ti-6Al-4V)',     'bb000000-0000-0000-0000-000000000006'),
  ('bb000000-0000-0000-0000-000000000063', 'IMPL_ZIR',  'Implant zircone',                        'bb000000-0000-0000-0000-000000000006'),
  -- Level 2: bone graft
  ('bb000000-0000-0000-0000-000000000071', 'COMBAUT',   'Greffon autogène',                       'bb000000-0000-0000-0000-000000000007'),
  ('bb000000-0000-0000-0000-000000000072', 'COMBXEN',   'Xénogreffe (os bovin/porcin)',            'bb000000-0000-0000-0000-000000000007'),
  ('bb000000-0000-0000-0000-000000000073', 'COMBSYN',   'Substitut osseux synthétique',           'bb000000-0000-0000-0000-000000000007'),
  ('bb000000-0000-0000-0000-000000000074', 'MEMB',      'Membranes (résorbables / non résorbables)', 'bb000000-0000-0000-0000-000000000007');

-- ─── Matériaux ────────────────────────────────────────────────────────────────

-- == ZIRCONE ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes, resistance_flexion_mpa, note_reglementaire) VALUES
  ('bb000000-0000-0000-0000-000000000011', 'ZIRC_3Y',    'Zircone 3Y-TZP (haute résistance)',                      ARRAY['IPS e.max ZirCAD Prime','Lava Plus','Cercon ht'],  1200, NULL),
  ('bb000000-0000-0000-0000-000000000011', 'ZIRC_4Y',    'Zircone 4Y-TZP (translucente)',                          ARRAY['Katana UTML','Cerec Zirconia','DD cube X2'],        800, NULL),
  ('bb000000-0000-0000-0000-000000000011', 'ZIRC_5Y',    'Zircone 5Y-TZP (ultra-translucente)',                    ARRAY['Katana HT','IPS e.max ZirCAD MT Multi','LuxaCam'], 650, NULL),
  ('bb000000-0000-0000-0000-000000000011', 'ZIRC_COL',   'Zircone colorée gradient (multi-couches)',                ARRAY['Katana UTML Multi','DD cube color'],                700, NULL);

-- == VITROCÉRAMIQUE ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes, resistance_flexion_mpa) VALUES
  ('bb000000-0000-0000-0000-000000000012', 'EMAX_CAD',   'IPS e.max CAD (disilicate lithium, CFAO)',               ARRAY['IPS e.max CAD'],                                    360),
  ('bb000000-0000-0000-0000-000000000012', 'EMAX_PRESS', 'IPS e.max Press (disilicate lithium, pressé)',           ARRAY['IPS e.max Press'],                                  400),
  ('bb000000-0000-0000-0000-000000000012', 'VITA_VM9',   'Céramique de stratification (VITA VM9)',                  ARRAY['VITA VM9','Noritake EX-3'],                          70);

-- == CÉRAMIQUE FELDSPATHIQUE ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes, resistance_flexion_mpa) VALUES
  ('bb000000-0000-0000-0000-000000000013', 'FELD_CAO',   'Céramique feldspathique CFAO (bloc)',                    ARRAY['VITA Mark II','CEREC Blocs C'],                      130),
  ('bb000000-0000-0000-0000-000000000013', 'FELD_STRAT', 'Céramique feldspathique conventionnelle (stratifiée)',   ARRAY['Noritake EX-3','VITA VMK Master'],                    80);

-- == COMPOSITES UNIVERSELS / NANO-HYBRIDES ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes) VALUES
  ('bb000000-0000-0000-0000-000000000021', 'COMP_UNI_A', 'Composite nano-hybride universel (1 teinte)',            ARRAY['Filtek One Bulk Fill','Harmonize','OMNICHROMA']),
  ('bb000000-0000-0000-0000-000000000021', 'COMP_NANO',  'Composite nanorempli universel',                         ARRAY['Filtek Supreme Ultra','Estelite Sigma Quick','IPS Empress Direct']),
  ('bb000000-0000-0000-0000-000000000021', 'COMP_MH',    'Composite micro-hybride (antérieur/postérieur)',          ARRAY['Tetric EvoCeram','Ceram X Duo','Venus Diamond']),
  ('bb000000-0000-0000-0000-000000000021', 'COMP_POST',  'Composite postérieur haute résistance',                  ARRAY['Filtek P60','Quixfil','X-tra fil']);

-- == COMPOSITES BULK-FILL ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes) VALUES
  ('bb000000-0000-0000-0000-000000000022', 'BF_PACK',    'Composite bulk-fill condensable (4 mm)',                 ARRAY['Tetric PowerFill','SonicFill 2','Filtek Bulk Fill Posterior']),
  ('bb000000-0000-0000-0000-000000000022', 'BF_FLOW_4',  'Composite bulk-fill flowable (4 mm)',                    ARRAY['SDR Plus','Filtek Bulk Fill Flow','Ionoseal']),
  ('bb000000-0000-0000-0000-000000000022', 'BF_TRANS',   'Composite bulk-fill translucide monolithique',           ARRAY['Tetric PowerCem','Filtek One','OMNICHROMA Blocker']);

-- == COMPOSITES FLOWABLE ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes) VALUES
  ('bb000000-0000-0000-0000-000000000023', 'FLOW_LOW',   'Composite flowable basse viscosité',                     ARRAY['Tetric EvoFlow','Filtek Z350 XT Flow','Ice Flow']),
  ('bb000000-0000-0000-0000-000000000023', 'FLOW_HV',    'Composite flowable haute viscosité',                     ARRAY['Constic','Estelite Flow Quick','G-ænial Universal Flo']),
  ('bb000000-0000-0000-0000-000000000023', 'FLOW_LINER', 'Liner composite (sous cavité)',                          ARRAY['Vertise Flow','Scotchbond Universal Plus Adhesive']);

-- == CORE BUILD-UP ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes) VALUES
  ('bb000000-0000-0000-0000-000000000024', 'CORE_DC',    'Composite dual-cure (reconstitution coronaire)',          ARRAY['LuxaCore Z-Dual','Rebilda DC','Multcore Flow']),
  ('bb000000-0000-0000-0000-000000000024', 'CORE_LC',    'Composite photo-polymérisable (noyau)',                  ARRAY['Filtek Bulk Fill','ParaCore']);

-- == CVI CONVENTIONNEL ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes) VALUES
  ('bb000000-0000-0000-0000-000000000031', 'CVI_CONV_R', 'CVI conventionnel restaurateur (type II)',               ARRAY['Ketac Molar','Fuji IX GP','Riva Self Cure']),
  ('bb000000-0000-0000-0000-000000000031', 'CVI_BASE',   'CVI de base / liner (type III)',                         ARRAY['Fuji II LC','Photac Fil','GC Fuji II']);

-- == CVIMAR ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes) VALUES
  ('bb000000-0000-0000-0000-000000000032', 'CVIMAR_R',   'CVIMAR restaurateur photo-activable',                    ARRAY['Vitremer','Fuji II LC','Photac Fil']),
  ('bb000000-0000-0000-0000-000000000032', 'CVIMAR_HC',  'CVIMAR haute concentration fluorure',                    ARRAY['Riva LC','Ketac Nano','GC Fuji II LC']);

-- == ALLIAGES NON PRÉCIEUX ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes, resistance_flexion_mpa, note_reglementaire) VALUES
  ('bb000000-0000-0000-0000-000000000041', 'COCR_PFM',   'Alliage CoCr pour PFM (céramo-métallique)',              ARRAY['Wirobond C+','Remanium GM 800+','Bellabond Plus'], 600, 'Absence de nickel recommandée (allergie)'),
  ('bb000000-0000-0000-0000-000000000041', 'COCR_FRAM',  'Alliage CoCr pour châssis prothétique',                  ARRAY['Wironit extrahart','Remanium 2000+','Vitallium'],  900, NULL),
  ('bb000000-0000-0000-0000-000000000041', 'NICR',       'Alliage NiCr (usage restreint allergie)',                 ARRAY['Degulor M','Remanium CA'],                         500, 'Allergie Ni — consentement requis');

-- == TITANE ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes, resistance_flexion_mpa) VALUES
  ('bb000000-0000-0000-0000-000000000042', 'TITA_G4',    'Titane grade 4 (cpTi, pur commercial)',                  ARRAY['Nobel Biocare Ti','Straumann Ti'],                 550),
  ('bb000000-0000-0000-0000-000000000042', 'TITA_G5',    'Titane grade 5 (Ti-6Al-4V ELI)',                         ARRAY['Straumann BLX Ti','Zimmer Ti','Biomet 3i'],        950);

-- == OR ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes, resistance_flexion_mpa) VALUES
  ('bb000000-0000-0000-0000-000000000043', 'OR_AU',      'Alliage or type 3 (75 % Au, couronne)',                  ARRAY['Degulor 3','Orplid P','Stabilor G'],               450),
  ('bb000000-0000-0000-0000-000000000043', 'OR_AU_CAST', 'Alliage or coulé type 4 (inlay/onlay)',                  ARRAY['Stabilor N','Degulor INC'],                        700);

-- == AMALGAME (OBSOLÈTE) ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes, obsolete, note_reglementaire) VALUES
  ('bb000000-0000-0000-0000-000000000044', 'AMALG_AG',   'Amalgame dentaire (Ag-Hg) — INTERDIT EU 2025',           ARRAY['Permite C','GS-80','Sybraloy'],                   true, 'Interdit depuis le 1er janvier 2025 (Règlement UE 2023/1481). Usage uniquement dans cas dérogatoires stricts.');

-- == RÉSINES PROTHÉTIQUES ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes) VALUES
  ('bb000000-0000-0000-0000-000000000051', 'PMMA_BASE',  'PMMA base de prothèse (poudre + liquide)',               ARRAY['SR Ivocap Plus','ProBase Cold','Vertex RS']),
  ('bb000000-0000-0000-0000-000000000051', 'PMMA_DENT',  'PMMA dents prothétiques (antérieures/postérieures)',     ARRAY['SR Vivodent DCL','Phonares II','Vita Physiodens']),
  ('bb000000-0000-0000-0000-000000000051', 'PMMA_MILL',  'PMMA usinable CFAO (bloc CAD/CAM)',                      ARRAY['PMMA Yamahachi','Hitachi QD Cubing','Zahn PMMA']),
  ('bb000000-0000-0000-0000-000000000052', 'POLYAM',     'Polyamide flexible (prothèse sans crochets)',            ARRAY['Valplast','TCS','Flex Nylon']);

-- == IMPLANTS TITANE GRADE 4 ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes, note_reglementaire) VALUES
  ('bb000000-0000-0000-0000-000000000061', 'IMPL_STR_SP', 'Implant Straumann Standard Plus (cpTi)',                ARRAY['Straumann SLActive SLA'],                          NULL),
  ('bb000000-0000-0000-0000-000000000061', 'IMPL_NBA_CC', 'Implant Nobel Biocare Conical Connection',              ARRAY['Nobel Active','NobelParallel CC'],                 NULL),
  ('bb000000-0000-0000-0000-000000000061', 'IMPL_ZIM',   'Implant Zimmer TSV (Tapered Screw-Vent)',               ARRAY['Zimmer Tapered','Zimmer NanoTite'],                NULL);

-- == IMPLANTS TITANE GRADE 5 ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes, note_reglementaire) VALUES
  ('bb000000-0000-0000-0000-000000000062', 'IMPL_BLX',   'Implant Straumann BLX (Ti-Grade 5)',                    ARRAY['Straumann BLX'],                                   NULL),
  ('bb000000-0000-0000-0000-000000000062', 'IMPL_EVO',   'Implant Neoss Pro-Active (Ti-Grade 5)',                 ARRAY['Neoss ProActive'],                                  NULL),
  ('bb000000-0000-0000-0000-000000000062', 'IMPL_ANKY',  'Implant Ankylos C/X (Ti-Grade 4/5)',                   ARRAY['Dentsply Sirona Ankylos'],                          NULL);

-- == IMPLANTS ZIRCONE ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes, resistance_flexion_mpa, note_reglementaire) VALUES
  ('bb000000-0000-0000-0000-000000000063', 'IMPL_ZIR_1', 'Implant zircone monopièce (patients allergiques titane)', ARRAY['Straumann PURE Ceramic','Zeramex T6'],           800, 'Indication restreinte — allergie titane confirmée'),
  ('bb000000-0000-0000-0000-000000000063', 'IMPL_ZIR_2', 'Implant zircone 2 pièces',                              ARRAY['Bredent Z-Systems z3'],                            900, NULL);

-- == COMBLEMENT OSSEUX AUTOGÈNE ==
INSERT INTO materiau (categorie_materiau_id, code, libelle) VALUES
  ('bb000000-0000-0000-0000-000000000071', 'COMB_AUT',   'Greffon osseux autogène (site donneur intra-oral)'),
  ('bb000000-0000-0000-0000-000000000071', 'COMB_AUT_X', 'Greffon osseux autogène (site donneur extra-oral — crête iliaque)');

-- == XÉNOGREFFE ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes) VALUES
  ('bb000000-0000-0000-0000-000000000072', 'XEN_BOV',    'Minéral d''os bovin déprotéinisé (DBBM)',               ARRAY['Bio-Oss','Cerasorb M','OssixPlus']),
  ('bb000000-0000-0000-0000-000000000072', 'XEN_POR',    'Substitut osseux porcin',                               ARRAY['OsteoBiol Gen-Os','mp3']),
  ('bb000000-0000-0000-0000-000000000072', 'XEN_EQ',     'Substitut osseux équin (cortico-spongieux)',             ARRAY['Biocare Horse Bone']);

-- == SUBSTITUT SYNTHÉTIQUE ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes) VALUES
  ('bb000000-0000-0000-0000-000000000073', 'SYN_HA',     'Hydroxyapatite synthétique',                            ARRAY['Cerasorb Classic','OsteoGraft HA','NovaBone']),
  ('bb000000-0000-0000-0000-000000000073', 'SYN_BTCP',   'Bêta-tricalcium phosphate (ß-TCP)',                      ARRAY['Cerasorb M','Vitoss BA','ChronOS']),
  ('bb000000-0000-0000-0000-000000000073', 'SYN_BIPHAS', 'Substitut biphasique HA/ß-TCP',                         ARRAY['Bioresorb','SynthoGraft']);

-- == MEMBRANES ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes) VALUES
  ('bb000000-0000-0000-0000-000000000074', 'MEMB_RES',   'Membrane résorbable (collagène bovin)',                 ARRAY['Bio-Gide','Jason membrane','Ossix Plus']),
  ('bb000000-0000-0000-0000-000000000074', 'MEMB_NR',    'Membrane non résorbable (PTFE renforcé)',               ARRAY['Gore-Tex','TefGen-FD','Cytoplast PTFE']);

-- == MATÉRIAUX D'EMPREINTE ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes) VALUES
  ('bb000000-0000-0000-0000-000000000008', 'EMP_ALG',    'Alginate (empreinte préliminaire)',                     ARRAY['Alginoplast','Zhermack CA37','Kromalon']),
  ('bb000000-0000-0000-0000-000000000008', 'EMP_SIL_A',  'Silicone par addition (A-silicone) — précision)',       ARRAY['Impregum Penta','Express XT','Aquasil Ultra']),
  ('bb000000-0000-0000-0000-000000000008', 'EMP_SIL_C',  'Silicone par condensation (C-silicone)',                ARRAY['Speedex','Zetaplus','Optosil Comfort']),
  ('bb000000-0000-0000-0000-000000000008', 'EMP_POL',    'Polyéther (haute précision, implantaire)',              ARRAY['Impregum Penta','Permadyne Penta H','Multimodel 3M']);

-- == CIMENTS ET AGENTS DE LIAISON ==
INSERT INTO materiau (categorie_materiau_id, code, libelle, marques_courantes) VALUES
  ('bb000000-0000-0000-0000-000000000009', 'CIM_ZOE',    'Ciment ZOE (oxyde zinc eugénol — provisoire)',          ARRAY['Temp Bond','Cavit','IRM']),
  ('bb000000-0000-0000-0000-000000000009', 'CIM_VI',     'Ciment verre ionomère (scellement final)',              ARRAY['Ketac Cem','Fuji Plus','RelyX Luting 2']),
  ('bb000000-0000-0000-0000-000000000009', 'CIM_RESIN',  'Ciment résine dual-cure (scellement prothétique)',      ARRAY['RelyX Ultimate','Panavia F2.0','Variolink Esthetic']),
  ('bb000000-0000-0000-0000-000000000009', 'CIM_MTA',    'MTA (minéral trioxyde agrégat — coiffage/réparation)',  ARRAY['ProRoot MTA','Biodentine','MM-MTA']),
  ('bb000000-0000-0000-0000-000000000009', 'CIM_BIO',    'Ciment biocéramique (endodontie — obturation canalaire)', ARRAY['EndoSequence BC Sealer','AH Plus Bioceramic','Well-Root ST']),
  ('bb000000-0000-0000-0000-000000000009', 'CIM_ENDO',   'Ciment d''obturation canalaire (résine-époxy)',         ARRAY['AH Plus','Tubli-Seal','RealSeal']);
