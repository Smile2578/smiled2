# CLAUDE.md — Smiled.IO

## Projet

Smiled.IO est un logiciel dentaire moderne (concurrent Veasy/Logos/Julie).
Phase 1 = Core Clinique : schéma dentaire interactif, questionnaire médical, diagnostic, plans de traitement.

## Stack

- **Backend** : Rust (Axum) + PostgreSQL (RLS multi-tenant) + S3/MinIO
- **Frontend** : Nuxt 4 (Vue 3) + TailwindCSS + shadcn-vue + Canvas 2D
- **Self-hosted** : Docker Compose (TENANT_MODE=single/multi)

## Documents Clés

- **Spec** : `docs/specs/2026-03-23-smiled-core-clinique-design.md` — Architecture, modèle de données complet, API, rôles
- **Plan** : `docs/plans/2026-03-23-smiled-core-clinique.md` — 12 tâches d'implémentation ordonnées
- **Vault** : `/Users/simonbelissa/Documents/Smile/projets/Smiled/Smiled.IO/` — Doc originale (deck, cahier des charges, mockups)

## Mode Autonome (/ralph ou /start)

Quand tu démarres une session dans ce projet :

1. **Lis le plan** (`docs/plans/2026-03-23-smiled-core-clinique.md`)
2. **Identifie la prochaine tâche non complétée** (cherche les `- [ ]` non cochés)
3. **Exécute la tâche** en utilisant le skill `superpowers:subagent-driven-development` :
   - Dispatch un subagent par tâche
   - Review le travail du subagent après chaque tâche
   - Coche les steps complétés (`- [x]`) dans le plan
   - Commit après chaque tâche
4. **Passe à la tâche suivante** automatiquement
5. **Loop** jusqu'à ce que toutes les tâches soient complétées ou qu'un blocker soit rencontré

### Si blocker

- Si un build casse → `/build-fix` avant de continuer
- Si un test échoue → fix le test ou l'implémentation, ne pas skip
- Si une décision d'architecture est nécessaire → STOP et demander à Simon
- Si le contexte window est trop chargé → `/compact` et reprendre

### Règles d'implémentation

- **TDD** : écrire les tests d'abord quand c'est pertinent (API endpoints)
- **Immutabilité** : pas de mutation, créer de nouveaux objets
- **Fichiers courts** : 200-400 lignes max, 800 absolu
- **Fonctions courtes** : <50 lignes
- **Pas de console.log** dans le code final
- **Commit après chaque tâche** : `feat: <description>` en anglais
- **Langue** : code + comments EN, UI FR

### Commandes utiles

```bash
# Backend
cd smiled-server
cargo run                    # lancer le serveur
cargo test                   # lancer les tests
cargo clippy                 # linter
sqlx migrate run             # appliquer les migrations

# Frontend
cd smiled-web
npm run dev                  # lancer le dev server
npm run build                # vérifier le build
npm run lint                 # linter

# Infra
docker compose up -d db minio   # démarrer Postgres + MinIO
docker compose up               # stack complète
```

## Structure du Projet

```
smiled/
├── smiled-server/           # Backend Rust (Axum)
│   ├── src/
│   │   ├── main.rs
│   │   ├── config.rs
│   │   ├── auth/           # JWT, RBAC, password
│   │   ├── tenant/         # RLS middleware
│   │   ├── core/           # Modules métier
│   │   │   ├── patient/
│   │   │   ├── schema_dentaire/
│   │   │   ├── diagnostic/
│   │   │   ├── pdt/
│   │   │   ├── acte/
│   │   │   ├── materiau/
│   │   │   ├── document/
│   │   │   └── historique/
│   │   ├── reference/      # Teinte, motif, trouble
│   │   ├── db/             # Pool, migrations
│   │   ├── audit/          # Audit log
│   │   └── api/            # Routes v1, WebSocket
│   ├── migrations/         # SQL migrations + seed
│   ├── tests/              # Integration tests
│   ├── Cargo.toml
│   └── Dockerfile
├── smiled-web/              # Frontend Nuxt 4
│   ├── app/
│   │   ├── layouts/
│   │   ├── pages/
│   │   ├── components/
│   │   │   ├── dental-chart/   # Canvas 2D interactif
│   │   │   ├── paro-chart/
│   │   │   ├── questionnaire/
│   │   │   ├── pdt/
│   │   │   └── ui/             # shadcn-vue
│   │   ├── composables/
│   │   └── utils/
│   ├── nuxt.config.ts
│   └── package.json
├── docker-compose.yml
├── docs/
│   ├── specs/              # Design specs
│   └── plans/              # Implementation plans
└── CLAUDE.md               # Ce fichier
```

## Seed Data

Le projet inclut des données de seed riches basées sur une recherche approfondie :
- **~150 actes dentaires** : CCAM, NGAP, hors nomenclature (convention 2023-2028, codes 2024-2026)
- **~80 matériaux** : hiérarchie complète (zircone 3Y/4Y/5Y, e.max, composites, CVI, implants, comblement...)
- **Teintes** : VITA Classical + 3D-Master
- **14 rôles** : titulaire, associé, collaborateur, remplaçant, 3 spécialistes, assistant, ASPBD...
- **Matrice RBAC** : permissions granulaires par rôle

Ne pas simplifier ces seed data — elles sont le résultat de 5 agents de deep research.

## Auth

- **Better Auth** gère l'authentification (pas le backend Rust)
- Config serveur : `smiled-web/server/auth.ts` (pg pool, schema `auth`)
- Client : `app/composables/useAuthClient.ts` (createAuthClient)
- Store : `app/stores/auth.ts` (getSession, signIn, signOut — PAS useSession en SSR)
- Handler Nuxt : `server/api/auth/[...all].ts`
- Tables dans le schema `auth` de PostgreSQL (user, session, account, verification)
- Credentials test : `simon@smiled.io` / `Smiled2026!`

## Design System

- **Polices** : Inter via `@nuxt/fonts` (400, 500, 600, 700)
- **Couleurs** : Teal médical (`--primary: 173 80% 36%`), neutrals slate
- **CSS** : `app/assets/css/tailwind.css` (tokens) + `tailwind.config.ts` (mapping)
- **Module** : `shadcn-nuxt` gère les composants UI (pas de scan manuel)
- **Dark mode** : `@nuxtjs/color-mode` forcé en light (pas de dark implémenté)
- **Anti-flash** : `ignore: ['app/components/ui/**']` + index.ts barrels pour shadcn-nuxt

## Leçons Apprises

- `useSession()` de Better Auth crash en SSR Nuxt (hasOwnProperty). Utiliser `getSession()` async côté client uniquement.
- `@nuxtjs/color-mode` stocke la préférence dans localStorage/cookie. Changer `storageKey` pour invalider les anciennes valeurs.
- shadcn-nuxt module nécessite des `index.ts` barrel exports dans chaque composant UI + `ignore: ['app/components/ui/**']` dans nuxt.config pour éviter les doublons.
- Les fichiers `.ts` verrouillés par le dev server Nuxt sur Windows — tuer node.exe avant d'écrire.
- `pnpm remove` casse les node_modules si le dev server tourne (EPERM). Toujours arrêter avant.
