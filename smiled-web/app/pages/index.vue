<template>
  <div class="min-h-screen bg-background text-foreground">
    <!-- ─── Navbar ─── -->
    <nav class="fixed top-0 inset-x-0 z-50 border-b border-border/40 bg-background/80 backdrop-blur-md">
      <div class="max-w-7xl mx-auto px-6 h-16 flex items-center justify-between">
        <NuxtLink to="/" class="flex items-center gap-2.5">
          <img src="~/assets/img/logomini.png" alt="Smiled.IO" class="size-9 rounded-xl" />
          <span class="text-lg font-semibold tracking-tight">Smiled.IO</span>
        </NuxtLink>
        <div class="hidden md:flex items-center gap-8 text-sm">
          <a href="#fonctionnalites" class="text-muted-foreground hover:text-foreground transition-colors">Fonctionnalites</a>
          <a href="#avantages" class="text-muted-foreground hover:text-foreground transition-colors">Pourquoi Smiled</a>
          <a href="#technique" class="text-muted-foreground hover:text-foreground transition-colors">Technique</a>
        </div>
        <div class="flex items-center gap-3">
          <NuxtLink to="/login" class="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors">
            Connexion
          </NuxtLink>
          <NuxtLink to="/login">
            <Button size="sm">Demarrer</Button>
          </NuxtLink>
        </div>
      </div>
    </nav>

    <!-- ─── Hero ─── -->
    <section class="relative pt-32 pb-24 overflow-hidden">
      <!-- Gradient background -->
      <div class="absolute inset-0 -z-10">
        <div class="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[600px] bg-gradient-to-b from-primary/[0.08] via-primary/[0.03] to-transparent rounded-full blur-3xl" />
      </div>

      <div class="max-w-7xl mx-auto px-6">
        <div class="max-w-3xl mx-auto text-center">
          <!-- Badge -->
          <div class="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border border-primary/20 bg-primary/[0.06] text-primary text-xs font-semibold mb-8">
            <span class="w-1.5 h-1.5 rounded-full bg-primary animate-pulse" />
            Alternative open-source a Veasy, Julie & Logos
          </div>

          <h1 class="text-5xl sm:text-6xl font-bold leading-[1.08] tracking-tight">
            Le logiciel dentaire
            <br />
            <span class="bg-gradient-to-r from-primary to-teal-400 bg-clip-text text-transparent">
              que vous attendiez
            </span>
          </h1>

          <p class="mt-6 text-lg text-muted-foreground leading-relaxed max-w-xl mx-auto">
            Schema dentaire Canvas 2D, diagnostics structures, plans de traitement multi-formules.
            Self-hostable. Conforme RGPD. Construit avec et pour les praticiens.
          </p>

          <div class="flex items-center justify-center gap-4 mt-10">
            <NuxtLink to="/login">
              <Button size="lg" class="gap-2 text-base px-8 h-12">
                Essayer Smiled.IO
                <Icon name="lucide:arrow-right" class="w-4 h-4" />
              </Button>
            </NuxtLink>
            <a href="#fonctionnalites">
              <Button variant="outline" size="lg" class="text-base px-6 h-12">
                Voir les fonctionnalites
              </Button>
            </a>
          </div>
        </div>

        <!-- Hero illustration: dental chart preview -->
        <div class="mt-16 max-w-4xl mx-auto">
          <div class="relative rounded-2xl border border-border/60 bg-card shadow-xl shadow-black/[0.03] overflow-hidden">
            <!-- Window chrome -->
            <div class="flex items-center gap-2 px-4 py-3 border-b border-border/40 bg-muted/30">
              <div class="flex gap-1.5">
                <span class="w-3 h-3 rounded-full bg-red-400/60" />
                <span class="w-3 h-3 rounded-full bg-amber-400/60" />
                <span class="w-3 h-3 rounded-full bg-green-400/60" />
              </div>
              <div class="flex-1 mx-8">
                <div class="h-6 rounded-md bg-muted/50 max-w-xs mx-auto flex items-center justify-center">
                  <span class="text-[10px] text-muted-foreground">localhost:3000/patients/schema</span>
                </div>
              </div>
            </div>
            <!-- App preview content -->
            <div class="p-8 bg-gradient-to-b from-card to-muted/20">
              <div class="grid grid-cols-3 gap-6">
                <!-- Sidebar mini -->
                <div class="space-y-3">
                  <div class="flex items-center gap-2 mb-4">
                    <img src="~/assets/img/logomini.png" alt="" class="size-6 rounded-md" />
                    <span class="text-xs font-semibold text-foreground">Smiled.IO</span>
                  </div>
                  <div v-for="item in previewNav" :key="item.label" class="flex items-center gap-2.5 px-2.5 py-1.5 rounded-md text-xs" :class="item.active ? 'bg-primary/10 text-primary font-medium' : 'text-muted-foreground'">
                    <Icon :name="item.icon" class="w-3.5 h-3.5" />
                    {{ item.label }}
                  </div>
                </div>
                <!-- Main area -->
                <div class="col-span-2 space-y-4">
                  <div class="flex items-center justify-between">
                    <div>
                      <p class="text-sm font-semibold text-foreground">Schema dentaire — Dupont Marie</p>
                      <p class="text-[11px] text-muted-foreground mt-0.5">Version 3 — 24 mars 2026</p>
                    </div>
                    <div class="flex gap-1.5">
                      <div class="px-2 py-1 rounded text-[10px] font-medium bg-primary/10 text-primary">Enregistrer</div>
                      <div class="px-2 py-1 rounded text-[10px] font-medium bg-muted text-muted-foreground">Nouvelle version</div>
                    </div>
                  </div>
                  <!-- Tooth grid mock -->
                  <div class="bg-background rounded-lg border border-border/40 p-4">
                    <div class="flex justify-center gap-1">
                      <div v-for="i in 16" :key="'upper-'+i" class="w-5 h-8 rounded-sm" :class="[3,7,11].includes(i) ? 'bg-primary/30 border border-primary/40' : [5,14].includes(i) ? 'bg-amber-400/30 border border-amber-400/40' : 'bg-muted border border-border/60'" />
                    </div>
                    <div class="w-full h-px bg-border/40 my-2" />
                    <div class="flex justify-center gap-1">
                      <div v-for="i in 16" :key="'lower-'+i" class="w-5 h-8 rounded-sm" :class="[4,9].includes(i) ? 'bg-destructive/20 border border-destructive/30' : [2,12].includes(i) ? 'bg-primary/30 border border-primary/40' : 'bg-muted border border-border/60'" />
                    </div>
                  </div>
                  <!-- Legend -->
                  <div class="flex gap-4 text-[10px] text-muted-foreground">
                    <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-sm bg-primary/40" /> Soin realise</span>
                    <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-sm bg-amber-400/40" /> A surveiller</span>
                    <span class="flex items-center gap-1"><span class="w-2 h-2 rounded-sm bg-destructive/30" /> Extraction</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── Features ─── -->
    <section id="fonctionnalites" class="py-24 border-t border-border/40">
      <div class="max-w-7xl mx-auto px-6">
        <div class="text-center mb-16">
          <p class="text-sm font-semibold text-primary mb-3">Fonctionnalites</p>
          <h2 class="text-3xl sm:text-4xl font-bold tracking-tight">
            Chaque outil clinique, repense
          </h2>
          <p class="mt-4 text-muted-foreground max-w-lg mx-auto">
            Pas un portage de logiciel des annees 2000. Une experience concue pour la facon dont vous travaillez reellement.
          </p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
          <div
            v-for="feature in features"
            :key="feature.title"
            class="group relative p-7 rounded-2xl border border-border/50 bg-card hover:border-primary/30 transition-all duration-300 hover:shadow-lg hover:shadow-primary/[0.04]"
          >
            <div class="flex items-start gap-5">
              <div class="flex-shrink-0 w-11 h-11 rounded-xl bg-gradient-to-br from-primary/15 to-primary/5 flex items-center justify-center group-hover:from-primary/25 group-hover:to-primary/10 transition-all duration-300">
                <Icon :name="feature.icon" class="w-5 h-5 text-primary" />
              </div>
              <div class="min-w-0">
                <h3 class="text-base font-semibold text-foreground mb-1.5">{{ feature.title }}</h3>
                <p class="text-sm text-muted-foreground leading-relaxed">{{ feature.description }}</p>
                <div v-if="feature.tags" class="flex flex-wrap gap-1.5 mt-3">
                  <span v-for="tag in feature.tags" :key="tag" class="inline-flex px-2 py-0.5 rounded text-[10px] font-medium bg-muted text-muted-foreground">
                    {{ tag }}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── Avantages vs concurrence ─── -->
    <section id="avantages" class="py-24 bg-muted/30 border-y border-border/40">
      <div class="max-w-7xl mx-auto px-6">
        <div class="text-center mb-16">
          <p class="text-sm font-semibold text-primary mb-3">Pourquoi Smiled.IO</p>
          <h2 class="text-3xl sm:text-4xl font-bold tracking-tight">
            Ce que les autres ne font pas
          </h2>
        </div>

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
          <div v-for="advantage in advantages" :key="advantage.title" class="text-center p-6">
            <div class="w-12 h-12 rounded-2xl bg-primary/10 flex items-center justify-center mx-auto mb-4">
              <Icon :name="advantage.icon" class="w-6 h-6 text-primary" />
            </div>
            <h3 class="text-base font-semibold text-foreground mb-2">{{ advantage.title }}</h3>
            <p class="text-sm text-muted-foreground leading-relaxed">{{ advantage.description }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── Tech ─── -->
    <section id="technique" class="py-24 border-b border-border/40">
      <div class="max-w-7xl mx-auto px-6">
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
          <div>
            <p class="text-sm font-semibold text-primary mb-3">Architecture</p>
            <h2 class="text-3xl font-bold tracking-tight mb-6">
              Construit pour durer
            </h2>
            <p class="text-muted-foreground leading-relaxed mb-8">
              Backend Rust pour la performance et la securite memoire.
              Frontend Nuxt 4 pour une UX reactive. PostgreSQL avec Row-Level Security
              pour l'isolation multi-tenant. Le tout deployable en un <code class="px-1.5 py-0.5 rounded bg-muted text-foreground text-sm font-mono">docker compose up</code>.
            </p>
            <div class="space-y-3">
              <div v-for="tech in techStack" :key="tech.name" class="flex items-center gap-3">
                <div class="w-2 h-2 rounded-full bg-primary" />
                <span class="text-sm"><strong class="text-foreground">{{ tech.name }}</strong> <span class="text-muted-foreground">— {{ tech.desc }}</span></span>
              </div>
            </div>
          </div>
          <!-- Code block -->
          <div class="rounded-2xl border border-border/60 bg-[#0f1117] overflow-hidden shadow-xl">
            <div class="flex items-center gap-2 px-4 py-3 border-b border-white/[0.06]">
              <div class="flex gap-1.5">
                <span class="w-3 h-3 rounded-full bg-white/10" />
                <span class="w-3 h-3 rounded-full bg-white/10" />
                <span class="w-3 h-3 rounded-full bg-white/10" />
              </div>
              <span class="text-[11px] text-white/40 ml-2 font-mono">docker-compose.yml</span>
            </div>
            <div class="p-5 font-mono text-[13px] leading-6 text-white/70">
              <p><span class="text-emerald-400">services</span>:</p>
              <p class="ml-4"><span class="text-sky-400">smiled-server</span>:</p>
              <p class="ml-8">image: <span class="text-amber-300">smiled/server:latest</span></p>
              <p class="ml-8">ports: [<span class="text-amber-300">"8080:8080"</span>]</p>
              <p class="ml-4"><span class="text-sky-400">smiled-web</span>:</p>
              <p class="ml-8">image: <span class="text-amber-300">smiled/web:latest</span></p>
              <p class="ml-8">ports: [<span class="text-amber-300">"3000:3000"</span>]</p>
              <p class="ml-4"><span class="text-sky-400">db</span>:</p>
              <p class="ml-8">image: <span class="text-amber-300">postgres:17</span></p>
              <p class="ml-4"><span class="text-sky-400">minio</span>:</p>
              <p class="ml-8">image: <span class="text-amber-300">minio/minio</span></p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── Stats ─── -->
    <section class="py-16 border-b border-border/40">
      <div class="max-w-7xl mx-auto px-6">
        <div class="grid grid-cols-2 lg:grid-cols-4 gap-8">
          <div v-for="stat in stats" :key="stat.label" class="text-center">
            <p class="text-4xl font-bold text-foreground tabular-nums">{{ stat.value }}</p>
            <p class="text-sm text-muted-foreground mt-1.5">{{ stat.label }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- ─── CTA ─── -->
    <section class="py-24">
      <div class="max-w-7xl mx-auto px-6 text-center">
        <img src="~/assets/img/logomini.png" alt="" class="size-14 rounded-2xl mx-auto mb-6" />
        <h2 class="text-3xl sm:text-4xl font-bold tracking-tight">
          Pret a moderniser votre cabinet ?
        </h2>
        <p class="mt-4 text-muted-foreground max-w-md mx-auto">
          Open-source, self-hostable, concu pour les praticiens.
          Aucun engagement, aucun lock-in.
        </p>
        <div class="mt-10 flex items-center justify-center gap-4">
          <NuxtLink to="/login">
            <Button size="lg" class="gap-2 text-base px-8 h-12">
              Commencer gratuitement
              <Icon name="lucide:arrow-right" class="w-4 h-4" />
            </Button>
          </NuxtLink>
        </div>
      </div>
    </section>

    <!-- ─── Footer ─── -->
    <footer class="border-t border-border/40 py-10 bg-muted/20">
      <div class="max-w-7xl mx-auto px-6 flex flex-col sm:flex-row items-center justify-between gap-4">
        <div class="flex items-center gap-2.5">
          <img src="~/assets/img/logomini.png" alt="" class="size-7 rounded-lg" />
          <span class="text-sm font-medium text-foreground">Smiled.IO</span>
        </div>
        <p class="text-xs text-muted-foreground">
          &copy; {{ new Date().getFullYear() }} Smiled.IO — Logiciel dentaire open-source
        </p>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: false,
  colorMode: 'light',
})

const previewNav = [
  { icon: 'lucide:layout-dashboard', label: 'Tableau de bord', active: false },
  { icon: 'lucide:users', label: 'Patients', active: false },
  { icon: 'lucide:grid-3x3', label: 'Schema dentaire', active: true },
  { icon: 'lucide:stethoscope', label: 'Diagnostics', active: false },
  { icon: 'lucide:file-text', label: 'Plans de traitement', active: false },
  { icon: 'lucide:bar-chart-3', label: 'Parodontologie', active: false },
]

const features = [
  {
    icon: 'lucide:grid-3x3',
    title: 'Schema dentaire Canvas 2D',
    description: 'Visualisez l\'etat dentaire complet sur un schema interactif haute fidelite. Click-to-edit sur chaque dent et chaque face, avec codage couleur par statut et versioning automatique.',
    tags: ['32 dents FDI', '6 faces/dent', 'Versioning', 'HiDPI'],
  },
  {
    icon: 'lucide:clipboard-check',
    title: 'Questionnaire medical structure',
    description: '6 sections couvrant les risques hemorragiques, infectieux, medicamenteux, troubles generaux, habitudes et antecedents dentaires. Signature patient integree.',
    tags: ['6 sections', 'RGPD', 'Signature'],
  },
  {
    icon: 'lucide:search',
    title: 'Diagnostic avec constats',
    description: 'Creez des diagnostics detailles lies a des dents specifiques. Chaque constat reference une dent, une face, et un type de pathologie pour un suivi precis.',
    tags: ['Lie aux dents', 'Historise', 'Multi-diagnostic'],
  },
  {
    icon: 'lucide:file-text',
    title: 'Plans de traitement 4 formules',
    description: 'Generez jusqu\'a 4 propositions de traitement par diagnostic avec calcul automatique des couts, selection d\'actes CCAM/NGAP et choix de materiaux par ligne.',
    tags: ['4 formules', '~150 actes CCAM', 'Calcul auto', 'PDF'],
  },
  {
    icon: 'lucide:bar-chart-3',
    title: 'Bilan parodontal complet',
    description: 'Saisie des 6 sites par dent : profondeur de poche, recession, saignement, plaque. Indices globaux (FMPS, FMBS) et suivi ATM/occlusion.',
    tags: ['6 sites/dent', 'FMPS/FMBS', 'ATM', 'Occlusion'],
  },
  {
    icon: 'lucide:image',
    title: 'Documents et radiographies',
    description: 'Upload drag & drop de radios, photos et documents. Chaque fichier peut etre lie a une ou plusieurs dents pour un classement anatomique.',
    tags: ['Drag & drop', 'S3/MinIO', 'Lien dent'],
  },
]

const advantages = [
  {
    icon: 'lucide:server',
    title: 'Self-hostable',
    description: 'Deployez sur votre propre serveur avec Docker Compose. Vos donnees restent chez vous, pas chez un editeur.',
  },
  {
    icon: 'lucide:shield-check',
    title: 'Securite de niveau bancaire',
    description: 'Rust pour la securite memoire, RLS PostgreSQL pour l\'isolation, Argon2id pour les mots de passe, rate limiting integre.',
  },
  {
    icon: 'lucide:code-2',
    title: 'Open-source',
    description: 'Code auditable, pas de boite noire. Contribuez, adaptez, integrez avec vos outils existants.',
  },
  {
    icon: 'lucide:zap',
    title: 'Performance native',
    description: 'Backend Rust compile en binaire natif. Temps de reponse en millisecondes, meme sur un serveur modeste.',
  },
  {
    icon: 'lucide:users',
    title: 'Multi-tenant natif',
    description: 'Du cabinet solo au centre multi-praticiens. 14 roles professionnels avec permissions granulaires.',
  },
  {
    icon: 'lucide:database',
    title: 'Catalogue riche',
    description: '~150 actes CCAM/NGAP/HN, ~80 materiaux, teintes VITA. Base references convention 2023-2028.',
  },
]

const techStack = [
  { name: 'Rust + Axum', desc: 'Backend compile, zero-cost abstractions' },
  { name: 'PostgreSQL 17 + RLS', desc: 'Isolation multi-tenant au niveau base' },
  { name: 'Nuxt 4 + Vue 3', desc: 'SSR, auto-imports, shadcn-vue' },
  { name: 'S3 / MinIO', desc: 'Stockage documents, compatible cloud et self-hosted' },
  { name: 'Docker Compose', desc: 'Deploiement en une commande' },
]

const stats = [
  { value: '~150', label: 'Actes CCAM/NGAP references' },
  { value: '~80', label: 'Materiaux catalogues' },
  { value: '14', label: 'Roles professionnels' },
  { value: '1 cmd', label: 'Pour deployer' },
]
</script>
