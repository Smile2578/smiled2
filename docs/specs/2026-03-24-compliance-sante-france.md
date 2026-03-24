# Smiled.IO -- Rapport de Conformite Reglementaire Sante France

**Date** : 2026-03-24
**Auteur** : Claude (recherche approfondie)
**Statut** : Reference v1
**Scope** : HDS, RGPD, ANSSI/PGSSI-S, SESAM-Vitale, DMP, Segur, CCAM/NGAP

---

## Table des matieres

1. [HDS -- Hebergement de Donnees de Sante](#1-hds)
2. [ANSSI / PGSSI-S](#2-anssi-pgssi-s)
3. [RGPD applique a la sante](#3-rgpd)
4. [SESAM-Vitale / FSE / ADRi / SCOR](#4-sesam-vitale)
5. [DMP / Pro Sante Connect](#5-dmp)
6. [Segur du numerique en sante](#6-segur)
7. [CCAM/NGAP specifique dentaire](#7-ccam-ngap)
8. [Audit de l'implementation actuelle](#8-audit)
9. [Checklist compliance](#9-checklist)
10. [Roadmap priorisee](#10-roadmap)
11. [References officielles](#11-references)

---

## 1. HDS -- Hebergement de Donnees de Sante {#1-hds}

### 1.1 Cadre legal

**Textes** :
- Article L.1111-8 du Code de la sante publique (CSP)
- Decret n 2018-137 du 26 fevrier 2018 (referentiel HDS)
- Referentiel de certification HDS rev. 2 (ASIP Sante / ANS, mis a jour 2024)

**Principe** : Toute personne physique ou morale qui heberge des donnees de sante a caractere personnel recueillies a l'occasion d'activites de prevention, de diagnostic, de soins ou de suivi social et medico-social, pour le compte de personnes physiques ou morales a l'origine de la production ou du recueil desdites donnees, DOIT etre certifiee HDS.

### 1.2 Editeur vs Hebergeur : la distinction critique

| Situation | Obligation HDS |
|-----------|---------------|
| Smiled.IO heberge les donnees en SaaS (cloud) pour des cabinets clients | **OUI** -- Smiled.IO est hebergeur de fait |
| Smiled.IO self-hosted (le cabinet heberge ses propres donnees) | **NON** -- Le cabinet est responsable de son propre hebergement |
| Smiled.IO utilise un cloud provider (AWS, OVH, Scaleway) | L'infra provider DOIT etre certifie HDS. Smiled.IO reste responsable de la couche applicative |

**Conclusion pour Smiled.IO** :
- **Mode SaaS** : le cloud provider doit etre certifie HDS (ex: OVH Healthcare, Scaleway HDS, Clever Cloud HDS, Outscale). Smiled.IO doit soit etre lui-meme certifie HDS, soit sous-traiter a un hebergeur certifie et etablir un contrat de sous-traitance conforme.
- **Mode self-hosted** : pas de certification HDS necessaire pour Smiled.IO, mais il est recommande de fournir un guide de deploiement securise au cabinet.

### 1.3 Les 6 activites HDS certifiables

La certification HDS couvre 6 activites :
1. Mise a disposition et maintien en condition operationnelle de l'infrastructure materielle
2. Mise a disposition et maintien en condition operationnelle de l'infrastructure virtuelle
3. Mise a disposition et maintien en condition operationnelle de la plateforme logicielle (PaaS)
4. Administration et exploitation du systeme d'information
5. Sauvegarde externalisee
6. **Hebergeur infogere** (activites 1 a 5 incluses)

Pour un SaaS dentaire, les activites 3 a 6 sont les plus pertinentes.

### 1.4 Le referentiel HDS 2024-2026

Base sur **ISO 27001:2022** + surcharges specifiques sante :
- Politique de securite documentee
- Gestion des incidents de securite
- Plan de continuite d'activite (PCA/PRA)
- Chiffrement des donnees au repos ET en transit
- Localisation des donnees en UE (exigence renforcee post-Schrems II)
- Audit de securite annuel
- Notification des violations de donnees (< 72h CNIL + notification patients si risque eleve)
- Clause de reversibilite (le client peut recuperer ses donnees)

### 1.5 Actions pour Smiled.IO

| Action | Priorite | Effort |
|--------|----------|--------|
| Choisir un hebergeur certifie HDS pour le mode SaaS | CRITIQUE | Faible (choix commercial) |
| Implementer le chiffrement at rest (PostgreSQL + S3) | HAUTE | Moyen |
| Implementer TLS partout (API, DB, S3) | HAUTE | Faible |
| Documenter le PCA/PRA | MOYENNE | Moyen |
| Implementer la clause de reversibilite (export complet des donnees) | HAUTE | Moyen |
| Etablir un contrat de sous-traitance HDS avec l'hebergeur | CRITIQUE | Faible (juridique) |

---

## 2. ANSSI / PGSSI-S {#2-anssi-pgssi-s}

### 2.1 Referentiel PGSSI-S

La **Politique Generale de Securite des Systemes d'Information de Sante** (PGSSI-S), publiee par l'ANS (ex-ASIP Sante), definit les exigences de securite pour tous les systemes traitant des donnees de sante en France.

**Documents cles** :
- Guide pratique pour les editeurs de logiciels de sante (ANS, 2024)
- Referentiel d'identification electronique des acteurs de sante
- Referentiel d'imputabilite (tracabilite des actions)

### 2.2 Exigences d'authentification

| Exigence | Requis | Smiled.IO actuel |
|----------|--------|-----------------|
| Mot de passe >= 12 caracteres (CNIL 2024 + ANSSI) | OUI | **NON** (8 caracteres) |
| Complexite : 3 types de caracteres (maj, min, chiffres, speciaux) | OUI | **NON** (pas de verification) |
| Verrouillage apres N tentatives echouees | OUI (5-10 max) | **NON** |
| Hashage : Argon2id, bcrypt ou scrypt | OUI | **OUI** (Argon2id) |
| Expiration des sessions | OUI (12h max pour acces clinique) | PARTIEL (24h JWT) |
| Re-authentification pour actions sensibles | OUI | **NON** |
| MFA pour acces distant | RECOMMANDE (obligatoire Segur Vague 2) | **NON** |
| Pro Sante Connect (PSC) pour praticiens | OBLIGATOIRE (Segur) | **NON** |

### 2.3 Exigences de tracabilite (imputabilite)

La PGSSI-S exige un **journal d'imputabilite** qui enregistre :
- QUI : identifiant de l'utilisateur
- QUOI : type d'action (creation, lecture, modification, suppression, impression, export)
- SUR QUOI : entite concernee (patient, dossier, document)
- QUAND : horodatage precis (serveur NTP)
- D'OU : adresse IP source
- RESULTAT : succes ou echec de l'action

**Conservation** : 6 mois minimum en ligne, 1 an minimum en archive (recommandation ANSSI : 3 ans).

**Smiled.IO actuel** : La table `audit_log` existe dans le schema mais **aucun code ne l'alimente**. C'est un gap critique.

### 2.4 Exigences de chiffrement

| Donnee | Exigence | Standard |
|--------|----------|---------|
| En transit (API) | TLS 1.2 minimum, TLS 1.3 recommande | ANSSI RGS |
| En transit (DB) | TLS obligatoire | PostgreSQL `sslmode=require` |
| En transit (S3) | HTTPS obligatoire | MinIO TLS |
| Au repos (DB) | Chiffrement disque ou TDE | `pgcrypto` ou chiffrement volume |
| Au repos (S3) | SSE (Server-Side Encryption) | MinIO SSE-S3 ou SSE-KMS |
| Au repos (sauvegardes) | Chiffrement obligatoire | GPG ou AES-256 |
| Donnees sensibles en base | Chiffrement applicatif recommande | `num_ss`, `password_hash` (deja hashe) |

### 2.5 Exigences reseau

- CORS restrictif (pas de wildcard en production)
- Headers de securite : `X-Frame-Options`, `X-Content-Type-Options`, `Referrer-Policy`, `Content-Security-Policy`, `Strict-Transport-Security`
- Rate limiting sur tous les endpoints
- Protection CSRF
- Validation stricte de tous les inputs

**Smiled.IO actuel** : 3 headers de securite presents (X-Frame-Options, X-Content-Type-Options, Referrer-Policy). **Manquent** : CSP, HSTS, rate limiting.

---

## 3. RGPD applique a la sante {#3-rgpd}

### 3.1 Base legale du traitement

Pour un logiciel dentaire, la base legale du traitement des donnees de sante est :
- **Article 9.2(h) RGPD** : Necessaire aux fins de la medecine preventive/du travail, de diagnostic medical, de prise en charge sanitaire
- **Article L.1110-4 CSP** : Secret medical
- Le **consentement n'est PAS la base legale** pour le dossier medical (c'est l'obligation legale de tenue du dossier). Le consentement s'applique pour les traitements annexes (newsletter, marketing, etc.)

### 3.2 Durees de conservation

| Type de donnee | Duree de conservation | Texte de reference |
|----------------|----------------------|-------------------|
| Dossier medical patient | **20 ans** a compter de la derniere consultation | Art. R.1112-7 CSP |
| Dossier medical mineur | 20 ans a compter de la date de naissance + 28 ans (soit jusqu'a 28 ans min) | Art. R.1112-7 CSP |
| Radiographies dentaires | 20 ans (partie integrante du dossier) | idem |
| Consentement eclaire signe | 20 ans | Obligation probatoire |
| Donnees de facturation | 10 ans (obligation comptable) | Code de commerce |
| Logs d'audit / tracabilite | **6 mois en ligne + archive 1 a 3 ans** | PGSSI-S + ANSSI |
| Donnees du personnel (praticiens) | 5 ans apres depart | Code du travail |

**Implication pour Smiled.IO** :
- Le soft-delete actuel (`deleted_at`) est correct pour les donnees cliniques : il ne faut JAMAIS hard-delete un dossier patient.
- Il faut un mecanisme d'archivage apres 20 ans (pas de suppression physique possible avant).
- Le `num_ss` doit etre conserve pendant toute la duree du dossier.

### 3.3 Droit a l'effacement vs obligation de conservation

Le **droit a l'effacement (art. 17 RGPD)** est **limite** pour les donnees de sante :
- Le patient ne peut PAS demander la suppression de son dossier medical avant 20 ans
- Il peut demander la suppression de donnees NON medicales (email marketing, preferences UI)
- Il peut demander la **portabilite** (export de son dossier)
- Le praticien peut opposer l'obligation de conservation au patient

**Implementation recommandee** :
```
Patient demande effacement →
  Si donnees cliniques → REFUS avec motif legal (Art. R.1112-7 CSP)
  Si donnees marketing/preferences → Suppression
  Si donnees factuelles non medicales → Anonymisation possible apres 10 ans
```

### 3.4 Consentement eclaire

En dentaire, le consentement eclaire est requis pour :
- Chaque plan de traitement accepte par le patient
- Les actes chirurgicaux
- Les actes comportant un risque specifique
- L'utilisation de materiaux specifiques (ex: amalgames mercure → banni 2025, mais historique)

**Format** : Document ecrit, signe (electroniquement ou manuscrit), archive dans le dossier patient. Doit contenir :
- Nature de l'acte
- Risques previsibles
- Alternatives therapeutiques
- Consequences en cas de refus

### 3.5 Registre de traitement (Art. 30 RGPD)

Un editeur de logiciel dentaire doit maintenir un registre de traitement. Template adapte :

| Champ | Valeur pour Smiled.IO |
|-------|----------------------|
| Responsable de traitement | Le cabinet dentaire (chaque tenant) |
| Sous-traitant | Smiled.IO SAS (editeur + hebergeur SaaS) |
| Finalite | Gestion du dossier medical patient, diagnostic, plans de traitement |
| Categories de donnees | Donnees d'identification, donnees de sante (art. 9), N SS, radiographies |
| Destinataires | Praticiens du cabinet, organismes d'assurance maladie (FSE), DMP |
| Transferts hors UE | NON (hebergement UE obligatoire) |
| Duree de conservation | 20 ans (dossier medical), 10 ans (facturation) |
| Mesures de securite | Chiffrement, RLS, RBAC, audit log, Argon2id, JWT |

### 3.6 DPO (Delegue a la Protection des Donnees)

**Obligatoire** pour Smiled.IO si :
- Traitement a grande echelle de donnees de sante (Art. 37.1(c) RGPD) → OUI des que plusieurs cabinets en SaaS
- Meme un cabinet solo n'est pas considere "grande echelle", mais l'editeur SaaS l'est

**Conclusion** : Un DPO est **obligatoire** pour Smiled.IO SAS des la mise en production SaaS. Peut etre externe (cabinet specialise RGPD sante).

### 3.7 AIPD (Analyse d'Impact relative a la Protection des Donnees)

**Obligatoire** pour Smiled.IO car :
- Traitement de donnees de sante a grande echelle (critere 1 de la liste CNIL)
- Utilisation innovative (IA diagnostic prevu)
- Donnees de personnes vulnerables (patients)

L'AIPD doit etre realisee **AVANT** la mise en production et mise a jour regulierement.

---

## 4. SESAM-Vitale / FSE / ADRi / SCOR {#4-sesam-vitale}

### 4.1 Vue d'ensemble de l'ecosysteme

L'ecosysteme SESAM-Vitale est le systeme de teletransmission des feuilles de soins en France. Il implique :

| Composant | Role |
|-----------|------|
| Carte Vitale (CV) | Carte a puce du patient (droits SS) |
| CPS (Carte de Professionnel de Sante) | Authentification du praticien |
| FSE (Feuille de Soins Electronique) | Equivalent numerique de la feuille de soins papier |
| ADRi (Acquisition des Droits integree) | Consultation en ligne des droits du patient |
| SCOR (SCannage des ORdonnances) | Numerisation et teletransmission des ordonnances |
| GIE SESAM-Vitale | Organisme qui gere tout l'ecosysteme |

### 4.2 Integration technique SESAM-Vitale

**C'est le point le plus complexe et le plus restrictif.**

Pour integrer SESAM-Vitale, un logiciel doit :

1. **Obtenir l'agrement** du GIE SESAM-Vitale (Centre National de Depot et d'Agrement -- CNDA)
2. **Utiliser les composants officiels** :
   - **Galss** (Gestionnaire d'Acces au Lecteur Sante Social) -- middleware d'acces aux lecteurs de cartes
   - **API SESAM-Vitale** -- librairies C/C++ fournies par le GIE
   - Elles sont distribuees sous licence restrictive, **pas open-source**
3. **Passer la certification** (tests de conformite CNDA)
4. **Maintenir un numero d'agrement** (renouvellement periodique)

**Techniquement** :
- Les API SESAM-Vitale sont des librairies **natives C/C++** (DLL Windows, .so Linux)
- Elles communiquent avec les lecteurs de cartes via le middleware Galss
- L'integration dans un logiciel web/SaaS necessite un **composant local** (agent/service) sur le poste du praticien
- Alternative : utiliser un **concentrateur de teletransmission** (ex: Resip, Cegedim, Pharmagest)

### 4.3 ADRi (Acquisition des DRoits integree)

ADRi permet de consulter les droits d'un patient en temps reel sans carte Vitale physique.

- **API** : service web SOAP/XML fourni par l'Assurance Maladie (CNAM)
- **Prerequis** : authentification via CPS + agrement SESAM-Vitale
- **Donnees retournees** : situation du patient (regime, caisse, droits AMO, CMU-C/C2S, ALD)

### 4.4 FSE (Feuille de Soins Electronique)

La FSE est le document numerique qui remplace la feuille de soins papier :

1. Le praticien cree la FSE dans le logiciel
2. Le patient insere sa carte Vitale
3. Le logiciel lit les droits et cree la FSE
4. Le praticien signe avec sa CPS
5. La FSE est transmise a l'organisme d'assurance maladie via concentrateur ou flux direct

**Format** : Flux SESAM-Vitale normalise (binaire proprietaire, pas du XML standard)

### 4.5 SCOR (SCannage des ORdonnances)

- Numerisation des ordonnances et pieces justificatives
- Transmission en piece jointe de la FSE
- Format : TIFF ou PDF, compresse

### 4.6 Strategy pour Smiled.IO

| Approche | Avantage | Inconvenient |
|----------|----------|--------------|
| **Integration directe SESAM-Vitale** | Controle total, pas de dependance | 12-18 mois de dev + certification CNDA, composant natif obligatoire |
| **Concentrateur externe** (Resip, Juxta) | Rapide a integrer (API), pas de certification propre | Cout par FSE, dependance fournisseur |
| **Hybride** : concentrateur au debut, natif ensuite | Time-to-market rapide + independance future | Double travail |

**Recommandation** : Approche hybride. Commencer par un concentrateur (Resip est le plus repandu chez les editeurs), puis envisager l'integration native si le volume le justifie.

---

## 5. DMP / Pro Sante Connect {#5-dmp}

### 5.1 DMP (Dossier Medical Partage)

Le DMP est le dossier medical numerique national de chaque patient, gere par l'Assurance Maladie via Mon Espace Sante.

**Obligations pour un logiciel dentaire** :
- Depuis le **Segur du numerique**, l'alimentation du DMP est **obligatoire** pour tous les professionnels de sante
- Les documents dentaires a envoyer au DMP :
  - Comptes rendus operatoires (extractions, chirurgie)
  - Bilans radiologiques (panoramiques, retroalveolaires)
  - Plans de traitement valides
  - Lettres de correspondance entre praticiens
  - Ordonnances

### 5.2 API DMP

- **API** : Services web SOAP + profils IHE (Integrating the Healthcare Enterprise)
- **Profils** : XDS.b (Cross-Enterprise Document Sharing)
- **Format des documents** : CDA R2 (Clinical Document Architecture) niveaux 1 et 3
- **Transport** : HTTPS + certificat d'etablissement ou via Pro Sante Connect

### 5.3 Pro Sante Connect (PSC)

PSC est le **fournisseur d'identite national** pour les professionnels de sante. Il remplace progressivement l'authentification par CPS pour les services en ligne.

**Obligation** : PSC est **obligatoire** pour le Segur du numerique. Tout logiciel de sante doit proposer l'authentification via PSC.

**Techniquement** :
- Protocole : **OpenID Connect (OIDC)** standard
- Provider : `auth.esante.gouv.fr`
- Scopes : `openid`, `scope_all` (pour les praticiens)
- Claims retournes : RPPS, nom, prenom, specialite, structure d'exercice
- Flow : Authorization Code avec PKCE

**Integration dans Smiled.IO** :
```
PSC Login Flow:
1. Le praticien clique "Se connecter avec Pro Sante Connect"
2. Redirection vers auth.esante.gouv.fr
3. Authentification via CPS physique ou e-CPS (app mobile)
4. Retour avec authorization_code
5. Smiled.IO echange le code contre un access_token + id_token
6. Extraction des claims (RPPS, nom, role)
7. Correspondance avec l'utilisateur Smiled.IO (via RPPS)
8. Creation de la session JWT Smiled.IO
```

### 5.4 Actions pour Smiled.IO

| Action | Priorite | Dependance |
|--------|----------|------------|
| Implementer PSC (OIDC) | OBLIGATOIRE (Segur) | Inscription aupres de l'ANS |
| Alimenter le DMP | OBLIGATOIRE (Segur) | PSC + generation CDA R2 |
| Gerer les profils IHE XDS.b | HAUTE | Librairie CDA |

---

## 6. Segur du numerique en sante {#6-segur}

### 6.1 Contexte

Le **Segur du numerique en sante** (2021-2027) est le programme national de modernisation des systemes d'information de sante. Il impose des exigences d'interoperabilite a tous les editeurs de logiciels de sante.

### 6.2 Vague 2 : Logiciels de cabinet

La **Vague 2** concerne les logiciels de gestion de cabinet (dont les logiciels dentaires) :

**Exigences fonctionnelles obligatoires** :
1. **Alimentation du DMP** : envoi automatique des documents cliniques
2. **MSS (Messagerie Securisee de Sante)** : envoi/reception via MSSante
3. **INS (Identite Nationale de Sante)** : qualification de l'identite patient via INSi
4. **PSC (Pro Sante Connect)** : authentification des praticiens
5. **Interoperabilite CI-SIS** : respect du Cadre d'Interoperabilite des SIS

### 6.3 Referentiel d'interoperabilite CI-SIS

Le **CI-SIS** (Cadre d'Interoperabilite des Systemes d'Information de Sante) definit les formats d'echange :

| Standard | Usage |
|----------|-------|
| **HL7 FHIR R4** | API REST pour echange de donnees structurees (ressources Patient, Observation, etc.) |
| **HL7 CDA R2** | Documents cliniques (comptes rendus, prescriptions) |
| **IHE XDS.b** | Partage de documents (alimentation DMP) |
| **INS** | Identifiant National de Sante (matricule INS + OID) |
| **LOINC** | Codes pour observations et analyses |
| **SNOMED CT** | Terminologie clinique (en cours d'adoption en France) |
| **CIM-10** | Classification des maladies (diagnostics) |
| **CCAM** | Classification des actes medicaux |
| **NGAP** | Nomenclature generale des actes professionnels |

### 6.4 Financement Segur

Le Segur prevoit un **financement des editeurs** pour la mise en conformite :
- Les editeurs qui referencent leur logiciel au Segur recoivent une prime par utilisateur
- Condition : passer la certification Segur (tests de conformite ANS)
- Deadline : les deadlines ont ete prolongees, la Vague 2 pour les cabinets dentaires est en cours (2025-2027)

### 6.5 Actions pour Smiled.IO

| Exigence Segur | Priorite | Complexite |
|----------------|----------|------------|
| Pro Sante Connect (OIDC) | OBLIGATOIRE | Moyenne (OIDC standard) |
| INSi (qualification identite) | OBLIGATOIRE | Haute (API CNAM) |
| DMP (alimentation) | OBLIGATOIRE | Haute (CDA R2 + IHE XDS.b) |
| MSSante (messagerie) | OBLIGATOIRE | Moyenne (SMTP securise + annuaire) |
| FHIR R4 (export/import) | RECOMMANDE | Haute (mapping modele dentaire → FHIR) |
| Certification Segur | OBLIGATOIRE pour financement | Haute (tests de conformite) |

---

## 7. CCAM / NGAP specifique dentaire {#7-ccam-ngap}

### 7.1 CCAM dentaire 2024-2026

La CCAM (Classification Commune des Actes Medicaux) est mise a jour regulierement. Les codes pertinents pour le dentaire :

**Familles CCAM dentaires** :
- **HBBD** : Actes sur la dent (soins conservateurs)
- **HBGD** : Actes sur le parodonte
- **HBLD** : Protheses dentaires
- **HBMD** : Orthodontie
- **HBJD** : Chirurgie buccale
- **HBED** : Endodontie
- **HBFD** : Implantologie (certains codes)

**Mises a jour recentes (2024-2026)** :
- Revalorisation des soins conservateurs (SPR 50 → 60 en 2025)
- Nouveaux codes pour les inlays/onlays en ceramique (100% Sante)
- Reforme du panier 100% Sante : elargissement progressif des couronnes et bridges
- Codes CCAM pour bridges colles (HBLD038, etc.)
- Codes pour les protheses sur implants (non rembourses mais codifies)

### 7.2 NGAP codes encore utilises en dentaire

Malgre le passage a la CCAM, certains actes dentaires restent codes en NGAP :

| Code NGAP | Libelle | Usage |
|-----------|---------|-------|
| C / CS | Consultation / Consultation specialiste | Consultation de base |
| SC (SPR) | Soin conservateur | Certains soins de base |
| TO | Traitement orthodontique | Orthodontie (semestre) |
| DC | Dent creuse (detartrage) | Detartrage complet |
| SC 12 | Sealant | Scellement de sillons |

**Note** : La CCAM remplace progressivement la NGAP. En 2026, la majorite des actes dentaires sont en CCAM, mais quelques actes NGAP persistent pour l'orthodontie et les consultations.

### 7.3 Hors nomenclature (HN)

Les actes hors nomenclature ne sont pas rembourses par l'Assurance Maladie :
- Implants dentaires (pose)
- Blanchiment
- Facettes
- Certaines protheses esthetiques
- Chirurgie pre-implantaire (greffe osseuse, sinus lift)
- Orthodontie adulte (hors certains cas)

**Codification** : Pas de code officiel. Chaque editeur peut definir ses propres codes internes. Il est recommande de suivre les conventions du GIE SESAM-Vitale pour la lisibilite.

### 7.4 Table de correspondance acte/remboursement

Pour chaque acte, Smiled.IO doit gerer :
- Code CCAM ou NGAP
- Base de remboursement Secu (tarif opposable)
- Panier RAC 0 / maitrise / libre (reforme 100% Sante)
- Plafond RAC 0 si applicable
- Prix praticien (honoraires libres si secteur 2 ou HN)
- Taux de remboursement (70% actes techniques, 60% protheses, 100% RAC 0)

**Deja implemente** : La table `acte` dans les migrations contient `base_secu`, `prix_defaut`, `plafond_rac_0`, `panier`, et environ 150 actes seeds. C'est un bon debut.

---

## 8. Audit de l'implementation actuelle {#8-audit}

### 8.1 Matrice de conformite

| Domaine | Exigence | Statut | Gap |
|---------|----------|--------|-----|
| **Authentification** | Mot de passe >= 12 chars | **NON** | Seuil a 8 chars dans `reset_password` |
| | Complexite MDP (3 types chars) | **NON** | Aucune validation |
| | Verrouillage apres echecs | **NON** | Pas de compteur d'echecs |
| | MFA | **NON** | Non implemente |
| | Pro Sante Connect | **NON** | Non implemente |
| **Sessions** | Duree max 12h (clinique) | **PARTIEL** | JWT 24h par defaut |
| | Refresh token en HttpOnly cookie | **NON** | localStorage (XSS vulnerable) |
| | Invalidation de session (logout) | **NON** | JWT stateless, pas de blacklist |
| **Tracabilite** | Audit log alimente | **NON** | Table existe, code absent |
| | Actions lecture tracees | **NON** | |
| | IP source enregistree | **NON** | |
| | Conservation 6 mois+ | **NON** | |
| **Chiffrement** | TLS en transit (API) | **NON CONFIG** | Pas de TLS natif (reverse proxy attendu) |
| | TLS DB | **NON** | `DATABASE_URL` sans `sslmode` |
| | TLS S3 | **NON** | MinIO en HTTP |
| | Chiffrement at rest | **NON** | Pas configure |
| **Rate limiting** | Tous endpoints | **NON** | Aucun rate limiter |
| | Login specifiquement | **NON** | Brute force possible |
| **Headers securite** | X-Frame-Options | **OUI** | DENY |
| | X-Content-Type-Options | **OUI** | nosniff |
| | Referrer-Policy | **OUI** | strict-origin-when-cross-origin |
| | Content-Security-Policy | **NON** | Absent |
| | Strict-Transport-Security | **NON** | Absent |
| **CORS** | Pas de wildcard en prod | **OUI** | Warning si wildcard |
| **Input validation** | Toutes les entrees | **PARTIEL** | Validator derive present, pas systematique |
| **Soft delete** | Donnees cliniques | **OUI** | `deleted_at` sur patient, diagnostic, PDT |
| **RLS** | Isolation multi-tenant | **OUI** | Hardened avec FORCE RLS + WITH CHECK |
| **RBAC** | 14 roles, 23 permissions | **OUI** | Matrice complete |
| **Password hashing** | Argon2id | **OUI** | Params solides (64MiB, 3 iter, 4 lanes) |
| **Anti-enumeration** | Login et forgot-password | **OUI** | Reponses generiques |
| **SESAM-Vitale** | Integration | **NON** | Hors scope Phase 1 |
| **DMP** | Alimentation | **NON** | Hors scope Phase 1 |
| **INSi** | Qualification identite | **NON** | Hors scope Phase 1 |
| **FHIR** | Export/Import | **NON** | Non implemente |

### 8.2 Vulnerabilites critiques identifiees

1. **Tokens en localStorage** (`smiled-web/app/stores/auth.ts` lignes 22-35)
   - Vulnerable aux attaques XSS
   - Un script malveillant peut exfiltrer les tokens
   - **Correction** : migrer vers des cookies HttpOnly + Secure + SameSite=Strict

2. **Pas de rate limiting** sur `/auth/login`
   - Brute force possible
   - **Correction** : ajouter un rate limiter (tower-governor ou middleware custom)

3. **Audit log non alimente**
   - Table `audit_log` presente en base mais jamais ecrite
   - Aucune tracabilite des actions
   - **Correction** : middleware d'audit sur toutes les routes mutantes + lecture patient

4. **JWT non revocable**
   - Le logout ne fait que supprimer le token cote client
   - Un token vole reste valide jusqu'a expiration
   - **Correction** : implementer une blacklist de tokens (Redis ou table PG) ou reduire la duree JWT a 15-30min

5. **Pas de validation de complexite du mot de passe**
   - Minimum 8 caracteres (devrait etre 12+)
   - Aucune exigence de complexite
   - **Correction** : validation Zod/validator avec regles ANSSI

6. **Pas de TLS pour la connexion PostgreSQL**
   - `DATABASE_URL` sans `sslmode=require`
   - Donnees en clair entre l'app et la DB
   - **Correction** : activer `sslmode=require` minimum (verify-full en production)

---

## 9. Checklist compliance {#9-checklist}

### Phase 1 : Securite de base (avant toute mise en production, meme beta)

- [ ] Migrer tokens vers cookies HttpOnly/Secure/SameSite=Strict
- [ ] Implementer rate limiting (5 req/min sur login, 100 req/min global)
- [ ] Renforcer politique MDP (12+ chars, 3 types, via zxcvbn ou regles ANSSI)
- [ ] Implementer l'audit log (middleware d'insertion sur toutes les mutations)
- [ ] Ajouter header HSTS (`Strict-Transport-Security: max-age=31536000; includeSubDomains`)
- [ ] Ajouter header CSP (`Content-Security-Policy: default-src 'self'; ...`)
- [ ] Reduire duree JWT access a 30 min (garder refresh 30 jours)
- [ ] Implementer verrouillage de compte (5 echecs → lock 15 min)
- [ ] Activer TLS pour PostgreSQL (`sslmode=require`)
- [ ] Activer HTTPS pour MinIO/S3
- [ ] Documenter la politique de confidentialite
- [ ] Documenter les CGU/CGV

### Phase 2 : Conformite RGPD/HDS (avant mise en production SaaS)

- [ ] Choisir et contracter avec un hebergeur certifie HDS
- [ ] Rediger et publier le registre de traitement
- [ ] Realiser l'AIPD (Analyse d'Impact)
- [ ] Nommer un DPO (interne ou externe)
- [ ] Implementer l'export de donnees patient (portabilite)
- [ ] Implementer le chiffrement at rest (DB + S3)
- [ ] Implementer les sauvegardes chiffrees
- [ ] Rediger le PCA/PRA
- [ ] Rediger le contrat de sous-traitance (Art. 28 RGPD)
- [ ] Implementer la notification de violation de donnees (processus)

### Phase 3 : Segur du numerique (avant certification)

- [ ] Implementer Pro Sante Connect (OIDC)
- [ ] Implementer INSi (qualification identite patient)
- [ ] Implementer l'alimentation DMP (CDA R2 + IHE XDS.b)
- [ ] Implementer MSSante (messagerie securisee)
- [ ] Generer des documents CDA R2 conformes CI-SIS
- [ ] Tester et certifier aupres de l'ANS

### Phase 4 : Teletransmission (pour utilisation en cabinet)

- [ ] Choisir l'approche SESAM-Vitale (concentrateur vs natif)
- [ ] Integrer ADRi (consultation droits patient)
- [ ] Integrer la creation de FSE
- [ ] Integrer SCOR (numerisation ordonnances)
- [ ] Obtenir l'agrement CNDA
- [ ] Tester le circuit de teletransmission complet

---

## 10. Roadmap priorisee {#10-roadmap}

### Horizon 1 : Securite fondamentale (4-6 semaines)
**Objectif** : Rendre le logiciel deployable en mode self-hosted securise.

| Tache | Effort | Impact |
|-------|--------|--------|
| Cookies HttpOnly pour tokens | 2-3 jours | CRITIQUE |
| Rate limiting (tower-governor) | 1-2 jours | CRITIQUE |
| Audit log middleware | 3-5 jours | CRITIQUE |
| Politique MDP ANSSI | 1 jour | HAUTE |
| Headers securite manquants (HSTS, CSP) | 1 jour | HAUTE |
| Verrouillage de compte | 2 jours | HAUTE |
| Reduire JWT a 30 min | 0.5 jour | HAUTE |
| TLS PostgreSQL | 1 jour | HAUTE |
| TLS MinIO | 1 jour | HAUTE |

### Horizon 2 : Conformite RGPD/HDS (2-3 mois)
**Objectif** : Preparer le deploiement SaaS multi-tenant.

| Tache | Effort | Impact |
|-------|--------|--------|
| Choix hebergeur HDS | 1-2 semaines | CRITIQUE |
| AIPD | 2-3 semaines | OBLIGATOIRE |
| DPO designation | 1 semaine | OBLIGATOIRE |
| Export donnees patient | 1-2 semaines | HAUTE |
| Chiffrement at rest | 1-2 semaines | HAUTE |
| Sauvegardes chiffrees + PCA/PRA | 2-3 semaines | HAUTE |
| Documentation juridique (CGU, politique confidentialite) | 2-3 semaines | OBLIGATOIRE |

### Horizon 3 : Segur / Interoperabilite (6-12 mois)
**Objectif** : Certification Segur + interoperabilite nationale.

| Tache | Effort | Impact |
|-------|--------|--------|
| Pro Sante Connect (OIDC) | 2-3 semaines | OBLIGATOIRE |
| INSi | 3-4 semaines | OBLIGATOIRE |
| DMP alimentation | 4-6 semaines | OBLIGATOIRE |
| MSSante | 2-3 semaines | OBLIGATOIRE |
| Generation CDA R2 | 3-4 semaines | OBLIGATOIRE |
| Tests + certification ANS | 4-8 semaines | OBLIGATOIRE |

### Horizon 4 : Teletransmission SESAM-Vitale (12-18 mois)
**Objectif** : Teletransmission native ou via concentrateur.

| Tache | Effort | Impact |
|-------|--------|--------|
| Integration concentrateur (Resip/Juxta) | 4-6 semaines | HAUTE |
| ADRi | 2-3 semaines | HAUTE |
| FSE | 4-6 semaines | HAUTE |
| SCOR | 2-3 semaines | MOYENNE |
| Certification CNDA | 3-6 mois | OBLIGATOIRE |

---

## 11. References officielles {#11-references}

### Textes legislatifs et reglementaires
- **Code de la sante publique** : Art. L.1111-8 (HDS), Art. L.1110-4 (secret medical), Art. R.1112-7 (conservation)
- **Decret n 2018-137** du 26 fevrier 2018 relatif a l'hebergement de donnees de sante
- **RGPD** : Reglement (UE) 2016/679, en particulier Art. 9, 17, 28, 30, 35, 37
- **Loi Informatique et Libertes** : Loi n 78-17 du 6 janvier 1978 modifiee

### Referentiels ANS (Agence du Numerique en Sante)
- **PGSSI-S** : https://esante.gouv.fr/produits-services/pgssi-s
- **CI-SIS** : https://esante.gouv.fr/produits-services/ci-sis
- **Pro Sante Connect** : https://industriels.esante.gouv.fr/produits-et-services/pro-sante-connect
- **INSi** : https://industriels.esante.gouv.fr/produits-et-services/ins
- **DMP** : https://industriels.esante.gouv.fr/produits-et-services/dmp
- **MSSante** : https://industriels.esante.gouv.fr/produits-et-services/mssante

### Segur du numerique
- **Portail Segur** : https://industriels.esante.gouv.fr/segur-du-numerique-en-sante
- **Vague 2** : https://industriels.esante.gouv.fr/segur-du-numerique-en-sante/vague-2
- **DSR (Dossier de Specifications de Referencement)** : disponible sur le portail industriels

### SESAM-Vitale
- **GIE SESAM-Vitale** : https://www.sesam-vitale.fr
- **CNDA** : https://www.sesam-vitale.fr/cnda
- **ADRi** : https://www.ameli.fr/medecin/exercice-liberal/presciptions-realisations/adri

### CNIL
- **Recommandation mots de passe** (2024) : https://www.cnil.fr/fr/mots-de-passe-recommandations
- **Guide AIPD** : https://www.cnil.fr/fr/RGPD-analyse-impact-protection-des-donnees-aipd
- **Registre de traitement** : https://www.cnil.fr/fr/RGPD-le-registre-des-activites-de-traitement

### CCAM / NGAP
- **CCAM en ligne** : https://www.ameli.fr/accueil-de-la-ccam/index.php
- **Convention dentaire** : https://www.ameli.fr/chirurgien-dentiste/textes-reference/convention
- **100% Sante dentaire** : https://www.ameli.fr/chirurgien-dentiste/exercice-liberal/facturation-remuneration/100-pour-cent-sante

---

## Annexe A : Architecture cible securisee

```
INTERNET
    |
    v
[Reverse Proxy / WAF]     <- TLS termination, rate limiting, WAF rules
    |                         (Nginx, Caddy, ou Cloudflare)
    v
[Smiled API Server]        <- Cookies HttpOnly, audit middleware, RBAC
    |           |
    v           v
[PostgreSQL]  [MinIO/S3]   <- TLS, chiffrement at rest, sauvegardes chiffrees
    |
    v
[Audit Log DB]             <- Retention 3 ans, read-only apres ecriture
```

## Annexe B : Comparaison hebergeurs HDS France

| Hebergeur | Certifie HDS | PostgreSQL manage | S3 compatible | Prix indicatif |
|-----------|-------------|-------------------|---------------|----------------|
| OVH Healthcare | Oui | Oui (Cloud DB) | Oui (Object Storage) | ~~ |
| Scaleway | Oui (HDS) | Oui (Managed DB) | Oui (Object Storage) | ~~ |
| Clever Cloud | Oui (HDS) | Oui (PostgreSQL) | Oui (Cellar/S3) | ~~~ |
| Outscale (Dassault) | Oui | Non (EC2-like) | Oui (OSU) | ~~~ |
| Docaposte | Oui | Sur mesure | Sur mesure | ~~~~ |
| Azure France (MS) | Oui (HDS) | Oui (Azure DB) | Oui (Blob) | ~~~ |

**Recommandation** : OVH Healthcare ou Scaleway HDS pour le meilleur rapport qualite/prix avec PostgreSQL manage + S3 compatible.

---

*Ce document est un guide de recherche base sur l'etat de la reglementation francaise en mars 2026. Il ne constitue pas un avis juridique. Un avocat specialise en droit de la sante et un DPO doivent valider la strategie de conformite avant toute mise en production.*
