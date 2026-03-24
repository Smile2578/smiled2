# Analyse Concurrentielle -- Marche du Logiciel Dentaire en France

**Date**: 2026-03-24
**Auteur**: Simon + Claude (Opus 4.6)
**Statut**: v1
**Objectif**: Cartographier le paysage concurrentiel pour positionner Smiled.IO

---

## 1. Synthese du marche

Le marche mondial du logiciel de gestion de cabinet dentaire est estime a ~2,5 milliards USD en 2025 (TCAC ~11% vers 3,6 milliards en 2029). En France, le marche est domine par une poignee d'acteurs historiques avec une consolidation en cours (rachat de Visiodent/Veasy par Cegedim en fevrier 2024).

**Chiffres cles France** :
- ~45 000 chirurgiens-dentistes en exercice
- Veasy : ~6 000 dentistes + 500 centres (leader cloud)
- Julie Solutions : ~20 000 utilisateurs (leader historique en volume)
- LOGOSw + Imagex : ~10% du marche
- Desmos (Orisha) : 600+ centres, 8 000 utilisateurs (leader centres de sante)
- Nouveaux entrants : Matisse, Giantix, Dental Pilote, Weclever Dental

**Tendance structurelle** : Migration du on-premise vers le cloud, acceleration IA en 2025-2026, generalisation ordonnance numerique en 2026, Segur du numerique en cours pour les chirurgiens-dentistes (pas encore d'arrete publie).

---

## 2. Matrice concurrentielle

### 2.1 Concurrents directs (logiciels de gestion)

| Critere | Veasy (Cegedim) | Julie / JULiA | LOGOSw | Desmos (Orisha) | Matisse | Giantix | Weclever |
|---------|----------------|---------------|--------|-----------------|---------|---------|----------|
| **Architecture** | Cloud 100% | Desktop + Cloud (JULiA) | Desktop (client lourd) | Cloud | Cloud hybride | Cloud | Cloud 100% |
| **IA integree** | Oui (pre-remplissage schema, devis) | Oui (JULiA: images, synthese, tableaux de bord) | Non | Oui (MAJ schema depuis pano) | Oui (comptes-rendus, agenda intelligent) | Oui (pano, dictee, analytics) | Non |
| **Schema dentaire interactif** | Oui | Oui | Oui | Oui (avec IA) | Oui | Oui | Oui |
| **Teletransmission SESAM-Vitale** | Oui | Oui | Oui | Oui | Oui | En cours | Oui |
| **RDV en ligne** | Oui (integre) | Oui | Oui | Oui | Doctolib sync | Oui (SMS illimites) | Oui |
| **Imagerie DICOM** | Oui | Oui (rattachement radios) | Oui (DICOM/TWAIN) | Oui | Oui | Oui | Oui |
| **Multi-praticien** | Oui | Oui | Oui | Oui (centres) | Oui | Oui | Oui |
| **App mobile** | Non specifie | Non specifie | Non | Non specifie | Non specifie | Non specifie | Responsive web |
| **Self-hosted** | Non | Non | Oui (client lourd) | Non | Non (mais mode offline) | Non | Non |
| **Open source** | Non | Non | Non | Non | Non | Non | Non |
| **API ouverte** | Non | Non | Non | Non | 50+ integrations | Non specifie | Non specifie |
| **Certification HDS** | Oui (Cegedim) | Non specifie | Non | Non specifie | Oui | Non specifie | Oui |
| **Label LAP (HAS)** | Non specifie | Non specifie | Oui (1er certifie V2) | Non | Non specifie | Non specifie | Non |
| **Paro charting** | Basique | Non specifie | Oui (module ODF, paro) | Non specifie | Non specifie | Non specifie | Non specifie |
| **Anciennete** | 2018 (Visiodent 1983) | 1983 (~42 ans) | ~30 ans | 10+ ans | 2024 (nouveau) | 2025 (pre-lancement) | Recente |
| **Prix indicatif** | 100-180 EUR/mois | Licence ~1200 EUR + 800 EUR/an MAJ | Sur devis | Sur devis | Sur devis | Gratuit (beta) | Sur devis |
| **Cible principale** | Cabinets + centres | Cabinets (jeunes praticiens) | Cabinets (fideles) | Centres de sante | Cabinets modernes | Cabinets innovants | Petits cabinets |

### 2.2 Solutions d'imagerie (complementaires)

| Critere | Planmeca Romexis 7 | Sidexis 4 (Dentsply Sirona) |
|---------|--------------------|-----------------------------|
| **Type** | Imagerie + planification | Imagerie + diagnostic |
| **IA** | Oui (segmentation auto, numerotation dents, plan implantaire en 9s) | Basique (filtres, mesures) |
| **CBCT 3D** | Oui (nerf mandibulaire, os) | Oui |
| **CAD/CAM** | Oui (Planmeca ecosystem) | Oui (CEREC ecosystem) |
| **Traitement local** | Oui (pas de cloud) | Oui |
| **Integration logiciel gestion** | Via DICOM/bridges | Via DICOM/DS Core |

### 2.3 IA specialisees (add-ons)

| Solution | Fonction | Modele |
|----------|----------|--------|
| **Allisone** | Diagnostic radio, aide a la communication patient | SaaS, HDS France |
| **WeDiagnostix** | Analyse panoramiques, rapports radio | Desktop + serveur France (pas Cloud Act) |
| **Askara** | Dictee vocale, comptes-rendus auto, lettres collegues | SaaS, 1800+ praticiens, ~8h/semaine economisees |
| **Colorisse** | Colorisation automatique des couronnes | Specialise esthetique |

---

## 3. Pain points praticiens (non resolus par le marche)

Sources : forums Eugenol, avis Capterra, temoignages utilisateurs, analyses comparatives.

### 3.1 Pain points critiques

| # | Pain point | Gravite | Qui est concerne | Solutions actuelles |
|---|-----------|---------|------------------|---------------------|
| 1 | **Vendor lock-in et portabilite des donnees** | CRITIQUE | Tous | EcoDentist a supprime l'export DSIO; Visiodent crashe sans recours; migration = perte de donnees |
| 2 | **Pannes et indisponibilite (cloud pur)** | HAUTE | Utilisateurs cloud | Visiodent/Veasy : pannes radio + carte vitale signalees sur Eugenol; 0 mode offline |
| 3 | **Support technique lent ou injoignable** | HAUTE | Petits cabinets | Difficile de joindre les editeurs pour info commerciale; Logos = exception (30min resolution) |
| 4 | **UX datee et workflows lents** | HAUTE | Tous | Logiciels concus dans les annees 2000 avec des couches ajoutees; pas de refonte UX |
| 5 | **Facturation/teletransmission complexe** | HAUTE | Tous | Erreurs de saisie CCAM, rejets tiers payant, nomenclatures pas a jour |
| 6 | **Absence d'interoperabilite reelle** | MOYENNE | Centres multi-logiciels | Chaque editeur a son silo; pas de standard d'echange au-dela de DSIO basique |
| 7 | **Prix opaques** | MOYENNE | Jeunes praticiens | "Sur devis" partout, pas de grille publique, engagement long terme |
| 8 | **Questionnaire medical papier** | MOYENNE | Tous | La plupart des cabinets utilisent encore du papier; solutions numeriques fragmentees |
| 9 | **Communication patient insuffisante** | MOYENNE | Tous | Pas d'explication visuelle du diagnostic, pas de portail patient |
| 10 | **Manque de donnees de pilotage** | BASSE | Cabinets multi-praticiens | Analytics basiques, pas de tableaux de bord temps reel |

### 3.2 Frustrations recurrentes (verbatim forums)

- *"Visiodent: j'ai perdu des donnees pendant 3 jours suite a un crash"*
- *"ODS (Desmos/Juxta) est une usine a gaz"*
- *"Je suis passe de Julie a Logos apres 2 ans de galere"*
- *"EcoDentist a supprime l'acces a mes donnees locales sans prevenir"*
- *"Impossible d'avoir un commercial au telephone chez [editeur]"*

---

## 4. Tendances 2025-2026

### 4.1 IA en cabinet dentaire

L'IA est passee de "gadget" a "outil attendu" en 2025 (source : ADF 2025). Generalisations prevues en 2026.

| Application IA | Maturite | Acteurs |
|---------------|----------|---------|
| Diagnostic radio (caries, lesions) | Production | Allisone, WeDiagnostix, Pearl |
| Pre-remplissage schema dentaire | Production | Veasy, Desmos, Giantix |
| Comptes-rendus automatiques (voice-to-text) | Production | Askara (1800 praticiens) |
| Planification implantaire automatique | Production | Romexis 7 (9 secondes) |
| Agenda intelligent (optimisation creneaux) | Emergent | Matisse |
| Analytics predictifs (activite cabinet) | Emergent | Giantix |
| Proposition de traitements | Emergent | Veasy |

### 4.2 Reglementation et numerique

| Sujet | Echeance | Impact |
|-------|----------|--------|
| **Ordonnance numerique** | Generalisation 2026 | Obligation pour dentistes, sages-femmes, medecins |
| **Segur numerique (LGC dentaire)** | En cours (pas d'arrete publie) | Logiciels devront etre "references Segur" ; financement Etat pour mise a niveau |
| **Mon Espace Sante / DMP** | Actif | 820 000+ examens bucco-dentaires alimentes (nov 2025) |
| **SESAM-Vitale v1.40 add.8** | Obligatoire fin 2025 | MAJ cahier des charges teletransmission |
| **FAMI 2025** | Declaration avant mars 2026 | 490 EUR + jusqu'a 750 EUR bonus par praticien |

### 4.3 Teleconsultation dentaire

La teledentisterie reste embryonnaire en France : cadre reglementaire strict, pas de remboursement systematique. En juin 2025, remuneration prevue pour la regulation telephonique d'urgences dentaires (SAMU/Centre 15). C'est un complement, pas un remplacement -- opportunite limitee a court terme.

### 4.4 Digital Workflow

La tendance "tout numerique" s'accelere : empreintes optiques, CFAO, DSD (Digital Smile Design), impression 3D chairside. L'integration logiciel de gestion <-> workflow numerique reste un point faible du marche.

---

## 5. Pricing models du marche

| Modele | Exemples | Prix indicatif | Tendance |
|--------|----------|----------------|----------|
| **SaaS mensuel** | Veasy, Matisse, Weclever | 70-180 EUR/mois | Dominant, en croissance |
| **Licence + maintenance** | Julie, LOGOSw | 1200 EUR licence + 800 EUR/an | En declin |
| **Freemium** | Julie (tier Free), Giantix (beta) | 0 EUR (fonctions limitees) | Emergent |
| **Sur devis (centres)** | Desmos, Veasy Enterprise | Variable | Standard pour centres |

**Observations** :
- Aucun acteur ne propose de modele transparent avec grille publique detaillee
- Les jeunes praticiens sont sensibles au prix (installation = investissement lourd)
- Le FAMI (490 EUR) couvre a peine 3-4 mois d'abonnement SaaS

---

## 6. Integrations cles du marche

| Integration | Importance | Couverture marche |
|------------|-----------|-------------------|
| **SESAM-Vitale (teletransmission)** | Obligatoire | Tous les logiciels agrees |
| **DMP / Mon Espace Sante** | Obligatoire (a terme) | En cours de deploiement |
| **Doctolib** (prise de RDV) | Haute | Matisse, Dental Pilote, certains via API |
| **Imagerie DICOM** | Haute | Tous (mais qualite d'integration variable) |
| **Vidal / BCB** (base medicamenteuse) | Moyenne (LAP) | LOGOSw (certifie), Julie, Matisse |
| **EDI Mutuelles (tiers payant)** | Haute | Desmos (1% rejet), Veasy, Julie |
| **Ordoclic** (ordonnance numerique) | Emergente | Matisse |
| **Comptabilite** (export) | Moyenne | Plupart des logiciels |
| **Labo prothese** (commandes) | Basse | LOGOSw (module labo) |
| **Signature electronique** | Emergente | Matisse, Giantix, Weclever |

---

## 7. Workflow type d'un cabinet dentaire moderne

```
ACCUEIL
  1. Patient arrive / nouveau patient
  2. Verification identite (carte vitale, INS)
  3. Questionnaire medical (idealement pre-rempli en ligne)
  4. Mise a jour dossier administratif
     |
     v
CONSULTATION
  5. Anamnese + examen clinique
  6. Radiographies (panoramique, retroalveolaire, CBCT si besoin)
  7. Remplissage schema dentaire (etat dent par dent)
  8. Diagnostic (findings, pathologies detectees)
  9. Discussion avec le patient (explication visuelle)
     |
     v
PLANIFICATION
  10. Etablissement plan de traitement (1 a 4 formules comparatives)
  11. Generation devis (CCAM, hors nomenclature, 100% Sante)
  12. Signature electronique du devis par le patient
  13. Envoi DPEC a la mutuelle (tiers payant)
  14. Planification des RDV de soins
      |
      v
SOINS
  15. Execution des actes selon le plan
  16. Saisie des actes realises (CCAM/NGAP)
  17. Tracabilite (materiaux, lots implantaires, sterilisation)
  18. Notes cliniques + photos/radios per-operatoires
      |
      v
FACTURATION
  19. Facturation des actes (FSE electronique)
  20. Teletransmission SESAM-Vitale
  21. Gestion tiers payant (envoi EDI mutuelle)
  22. Encaissement (CB, especes, virement, SEPA)
      |
      v
SUIVI
  23. Compte-rendu post-operatoire (idealement auto-genere IA)
  24. Alimentation DMP / Mon Espace Sante
  25. Relance devis non acceptes
  26. Rappels RDV de suivi (SMS/email)
  27. Collecte avis patient
  28. Tableaux de bord activite (CA, taux acceptation devis, etc.)
```

---

## 8. Opportunites de disruption pour Smiled.IO

### 8.1 Top 10 innovations classees par impact x faisabilite

| # | Innovation | Impact | Faisabilite | Score | Justification |
|---|-----------|--------|-------------|-------|---------------|
| **1** | **Self-hosted + open source (coeur)** | TRES HAUT | HAUTE | 10/10 | AUCUN concurrent ne propose ca. Portabilite totale, souverainete des donnees, compliance facilitee. Differenciateur unique. |
| **2** | **Schema dentaire Canvas 2D de nouvelle generation** | HAUT | HAUTE | 9/10 | C'est le coeur du metier. UX fluide, intuitive, tactile. Les schemas actuels sont vieillissants. Deja en cours dans la spec. |
| **3** | **Tarification transparente et publique** | HAUT | TRES HAUTE | 9/10 | Frustration universelle sur les prix opaques. Publier une grille claire = confiance immediate. Zero cout technique. |
| **4** | **Mode offline-first avec sync** | HAUT | MOYENNE | 8/10 | Les pannes cloud sont un cauchemar signale sur Eugenol. Offline-first = resilience. Matisse a un mode offline basique. |
| **5** | **API ouverte documentee (REST + WebSocket)** | HAUT | HAUTE | 8/10 | Permet l'ecosysteme : integrations IA tierces (Allisone, WeDiagnostix, Askara), labos, comptabilite. Aucun concurrent n'a une vraie API ouverte. |
| **6** | **Portail patient avec questionnaire medical pre-consultation** | MOYEN | HAUTE | 7/10 | Le questionnaire papier est encore la norme. Pre-remplissage en ligne avant la visite = gain de temps. Odontodoc existe mais n'est pas integre aux logiciels. |
| **7** | **Export DSIO natif + migration assistee** | HAUT | MOYENNE | 7/10 | Pain point majeur (vendor lock-in). Proposer un import/export DSIO impeccable + outil de migration = argument de vente tueur. |
| **8** | **IA diagnostic integree (pano -> schema dentaire)** | TRES HAUT | BASSE (Phase 2+) | 7/10 | Differenciateur fort mais complexite technique elevee. Peut etre fait via integration API avec Allisone/WeDiagnostix en attendant un moteur propre. |
| **9** | **Communication patient visuelle** | MOYEN | MOYENNE | 6/10 | Montrer au patient son schema annote, ses radios commentees, ses options de traitement visuellement. Peu de logiciels font ca bien. |
| **10** | **Dashboard temps reel multi-praticien** | MOYEN | HAUTE | 6/10 | WebSocket deja dans la stack. Metrics en temps reel (patients en salle, CA du jour, taux occupation). Les solutions actuelles ont des analytics basiques. |

### 8.2 Innovations a explorer (Phase 2+)

| Innovation | Horizon | Notes |
|-----------|---------|-------|
| Reconnaissance vocale integree (type Askara) | Phase 3 | Whisper/local first, pas besoin de SaaS tiers |
| Ordonnance numerique native | Phase 2 | Obligation 2026 |
| Integration Doctolib bidirectionnelle | Phase 2 | API Doctolib Partners |
| Module parodontologie avance | Phase 2 | Charting PSR + sondages, peu de logiciels le font bien |
| Signature electronique integree | Phase 2 | eIDAS, devis + consentement |
| Integration labo prothese (commandes numeriques) | Phase 3 | Marche niche mais valeur ajoutee |
| Portail patient complet (historique, RDV, documents) | Phase 3 | Differenciateur patient-centric |
| Module ODF/orthodontie | Phase 3 | Marche specialise, peu d'acteurs |

---

## 9. Positionnement strategique recommande pour Smiled.IO

### 9.1 Differenciateurs uniques (moat)

1. **Open source + self-hostable** : Position unique sur le marche francais. Attire les praticiens soucieux de souverainete, les centres de sante avec DSI, les enseignants/chercheurs. Aucun concurrent ne le fait.

2. **Stack moderne (Rust + Nuxt 4)** : Performance superieure, securite memoire, binaire unique pour le self-hosted. Pas de dette technique legacy.

3. **API-first** : Contrairement aux monolithes fermes du marche, Smiled.IO peut devenir une plateforme sur laquelle des tiers branchent leurs IA (Allisone, Askara, WeDiagnostix) sans avoir a re-developper.

4. **UX startup** : Les logiciels concurrents ont 20-40 ans. Meme les "nouveaux" (Matisse, Giantix) utilisent des frameworks classiques. L'UX peut etre un differenciateur visible en 30 secondes de demo.

### 9.2 Risques a surveiller

| Risque | Probabilite | Mitigation |
|--------|------------|------------|
| Segur du numerique impose des contraintes non prevues | Moyenne | Suivre les travaux LGC, participer aux ateliers editeurs |
| Cegedim (Veasy) accelere sur l'IA | Haute | Miser sur l'ouverture (API) plutot que sur un moteur IA propre |
| Certification SESAM-Vitale longue et couteuse | Haute | Planifier des le Phase 2 ; utiliser le FAMI comme levier |
| Adoption lente (resistance au changement) | Haute | Migration DSIO parfaite + essai gratuit genereux + formation |
| Concurrence des nouveaux entrants (Giantix, Matisse) | Moyenne | Le self-hosted est un moat; l'open source attire les contributeurs |

### 9.3 Go-to-market suggere

1. **Phase 1** (maintenant) : Core clinique open source, schema dentaire superieur, API documentee
2. **Phase 2** (6-12 mois) : Facturation + teletransmission, ordonnance numerique, integration Doctolib
3. **Phase 3** (12-18 mois) : IA diagnostic (via partenaires puis propre), portail patient, paro avance
4. **Lancement beta** : Cibler 5-10 cabinets early adopters (tech-savvy, jeunes praticiens) via Eugenol + reseaux sociaux dentaires
5. **Pricing** : Freemium pour solo (1 praticien) + SaaS abordable (< 100 EUR/mois) + self-hosted gratuit (support payant)

---

## Sources

- [Veasy Solution](https://www.veasy-solution.com/)
- [Veasy -- Prodentaire 2026](https://prodentaire.fr/veasy-le-logiciel-dentaire-pour-digitaliser-votre-cabinet/)
- [Veasy -- LonaSante](https://www.lonasante.com/veasy/)
- [Veasy -- Capterra](https://www.capterra.com/p/207643/Veasy/)
- [LOGOSw](https://www.logosw.net/)
- [LOGOSw -- LonaSante](https://www.lonasante.com/logosw/)
- [Julie Solutions](https://www.julie.fr/)
- [JULiA -- Julie Solutions](https://www.julie.fr/article-julia-reinvente-le-pilotage-des-activites-de-chirurgie-dentaire-avec-une-approche-nouvelle-generation)
- [JULiA -- Dentaire365](https://www.dentaire365.fr/chirurgien-dentiste/equipement/julia-nouveau-logiciel-du-cabinet-dentaire-appuye-sur-lintelligence-artificielle/)
- [Desmos -- Orisha Healthcare](https://healthcare.orisha.com/logiciel-centre-sante/desmos-centres-sante-dentaire/)
- [Desmos -- Eugenol](https://forum.eugenol.com/sujets/422540-desmos-ex-ods)
- [Visiodent / Cegedim](https://www.visiodent.com/)
- [Rachat Cegedim -- L'Information Dentaire](https://www.information-dentaire.fr/actualites/logiciel-metier-cegedim-rachete-visiodent/)
- [Planmeca Romexis 7](https://www.planmeca.com/fr/info-presse/salle-de-presse/2025/le-logiciel-planmeca-romexis-7-optimise-les-flux-de-travail-dentaires-grace-a-lia-integree/)
- [Sidexis 4 -- Dentsply Sirona](https://www.dentsplysirona.com/fr-fr/decouvrir/decouvrir-par-marque/sidexis-4.html)
- [ADF 2025 : IA en cabinet dentaire](https://www.dentaire365.fr/a-decouvrir/adf-2025-lintelligence-artificielle-sinstalle-vraiment-dans-les-cabinets-dentaires/)
- [IA dentisterie -- Dentalespace](https://www.dentalespace.com/praticien/actualites/ia-dentisterie-avancees-challenges/)
- [Tendances dentisterie 2025 -- Gsmile](https://www.centre-dentaire-gsmile.fr/divers/les-grandes-tendances-de-la-dentisterie-en-2025/)
- [Matisse Dentaire](https://www.matisse-dentaire.com/)
- [Giantix](https://giantix.com/)
- [Dental Pilote](https://www.dentalpilote.com/)
- [Weclever Dental](https://weclever-dental.com/)
- [Allisone](https://en.allisone.ai/)
- [WeDiagnostix](https://wediagnostix.ai/)
- [Askara](https://www.askara.ai/en)
- [Segur numerique dentaire](https://esante.gouv.fr/segur/chirurgiens-dentistes)
- [Ordonnance numerique 2025](https://esante.gouv.fr/doctrine/ordonnance-numerique)
- [FAMI dentaire 2025](https://www.ameli.fr/chirurgien-dentiste/exercice-liberal/vie-cabinet/forfait-d-aide-la-modernisation-et-l-informatisation-du-cabinet)
- [Logiciel dentaire comparatif 2026 -- LonaSante](https://www.lonasante.com/logiciel-dentaire/)
- [Top 9 logiciels dentiste 2026 -- Indy](https://www.indy.fr/guide/profession-liberale/ouvrir-un-cabinet-medical/comparatif-logiciel-dentiste/)
- [Logiciel dentaire classement 2026 -- JasperDental](https://www.jasperdental.fr/blog/logiciel-dentaire)
- [Forum Eugenol -- logiciel dentaire](https://forum.eugenol.com/sujets/405627-quel-logiciel-dentaire)
- [Forum Eugenol -- quel logiciel choisir](https://forum.eugenol.com/sujets/431304-quel-logiciel-dentaire-choisir)
- [Odontodoc -- questionnaire medical](https://www.odontodoc.fr/)
- [Open Dental](https://www.getdentra.com/resources/guides/open-dental-software-review)
- [Clear.Dental -- open source](https://clear.dental/)
- [DMP SESAM-Vitale](https://www.sesam-vitale.fr/en/dmp)
- [Teledentisterie -- Dentalespace](https://www.dentalespace.com/praticien/actualites/teledentisterie-nouvelle-ere/)
