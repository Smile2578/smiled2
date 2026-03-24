<script setup lang="ts">
import {
  ArrowRight,
  Play,
  Activity,
  Shield,
  Users,
  FileText,
  Server,
  ClipboardList,
  Star,
  Check,
  X,
  Stethoscope,
  Zap,
  ChevronRight,
} from 'lucide-vue-next'

definePageMeta({ layout: false })

useHead({
  title: 'Smiled.IO — Le logiciel dentaire du futur',
  meta: [
    { name: 'description', content: 'Smiled.IO combine precision clinique et experience moderne pour transformer votre cabinet dentaire.' },
  ],
  htmlAttrs: { class: 'scroll-smooth' },
})

const features = [
  {
    icon: Activity,
    title: 'Schema dentaire interactif',
    description: 'Visualisez et annotez chaque dent en temps reel avec un canvas haute performance. Navigation clavier, accessibilite complete.',
  },
  {
    icon: ClipboardList,
    title: 'Questionnaire medical',
    description: 'Formulaires intelligents adaptes a chaque patient. Alertes medicales automatiques, historique complet.',
  },
  {
    icon: FileText,
    title: 'Plans de traitement',
    description: 'Creez des plans de traitement structures avec estimation des couts, etapes et suivi de progression.',
  },
  {
    icon: Stethoscope,
    title: 'Gestion des actes CCAM',
    description: 'Plus de 150 actes references : CCAM, NGAP et hors nomenclature. Codes a jour 2024-2026.',
  },
  {
    icon: Users,
    title: 'Multi-cabinet & multi-roles',
    description: '14 roles distincts avec permissions granulaires. Du titulaire au remplacant, chaque profil est couvert.',
  },
  {
    icon: Server,
    title: 'Self-hosted & securise',
    description: 'Vos donnees restent chez vous. Deploiement Docker, chiffrement bout en bout, conformite RGPD.',
  },
]

const comparisonFeatures = [
  { name: 'Schema dentaire interactif', smiled: true, veasy: false, julie: false, logos: false },
  { name: 'Self-hosted (vos donnees)', smiled: true, veasy: false, julie: false, logos: true },
  { name: 'RBAC multi-roles (14 roles)', smiled: true, veasy: false, julie: false, logos: false },
  { name: 'API ouverte', smiled: true, veasy: false, julie: false, logos: false },
  { name: 'Actes CCAM integres (+150)', smiled: true, veasy: true, julie: true, logos: true },
  { name: 'Interface moderne', smiled: true, veasy: true, julie: false, logos: false },
  { name: 'Multi-cabinet', smiled: true, veasy: true, julie: false, logos: true },
  { name: 'Open source', smiled: true, veasy: false, julie: false, logos: false },
]

const testimonials = [
  {
    quote: 'Smiled.IO a transforme la gestion de mon cabinet. Le schema dentaire interactif est un game-changer pour les consultations.',
    name: 'Dr. Sophie Martin',
    role: 'Chirurgien-dentiste, Paris',
    initials: 'SM',
  },
  {
    quote: 'Enfin un logiciel dentaire qui ne ressemble pas a un logiciel des annees 2000. Mes assistantes l\'ont adopte en une journee.',
    name: 'Dr. Lucas Renard',
    role: 'Chirurgien-dentiste, Lyon',
    initials: 'LR',
  },
  {
    quote: 'Le self-hosting etait un critere non negociable pour nous. Smiled.IO est le seul a proposer ca avec une UX aussi soignee.',
    name: 'Dr. Amira Khelifi',
    role: 'Chirurgien-dentiste, Marseille',
    initials: 'AK',
  },
]

const stats = [
  { value: '+150', label: 'Actes CCAM', sublabel: 'references' },
  { value: '14', label: 'Roles RBAC', sublabel: 'granulaires' },
  { value: '32', label: 'Dents', sublabel: 'schema interactif' },
  { value: '100%', label: 'Self-hosted', sublabel: 'vos donnees' },
]

// Intersection Observer for scroll animations
function useScrollReveal() {
  onMounted(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            entry.target.classList.add('revealed')
            observer.unobserve(entry.target)
          }
        }
      },
      { threshold: 0.1, rootMargin: '0px 0px -40px 0px' },
    )

    const elements = document.querySelectorAll('.reveal-on-scroll')
    for (const el of elements) {
      observer.observe(el)
    }
  })
}

useScrollReveal()

// Dental chart hover state
const hoveredTooth = ref<number | null>(null)
const upperTeeth = [18, 17, 16, 15, 14, 13, 12, 11, 21, 22, 23, 24, 25, 26, 27, 28]
const lowerTeeth = [48, 47, 46, 45, 44, 43, 42, 41, 31, 32, 33, 34, 35, 36, 37, 38]
</script>

<template>
  <div class="min-h-screen bg-slate-950 text-white antialiased">
    <!-- Navigation -->
    <nav class="fixed top-0 z-50 w-full border-b border-white/5 bg-slate-950/80 backdrop-blur-xl">
      <div class="mx-auto flex h-16 max-w-7xl items-center justify-between px-4 sm:px-6 lg:px-8">
        <div class="flex items-center gap-2">
          <img src="~/assets/img/logomini.png" alt="Smiled.IO" class="h-8 w-8 rounded-lg" />
          <span class="text-lg font-bold tracking-tight">Smiled.IO</span>
        </div>
        <div class="hidden items-center gap-8 md:flex">
          <a href="#features" class="text-sm text-slate-400 transition-colors hover:text-white">Fonctionnalites</a>
          <a href="#demo" class="text-sm text-slate-400 transition-colors hover:text-white">Demo</a>
          <a href="#comparison" class="text-sm text-slate-400 transition-colors hover:text-white">Comparatif</a>
          <a href="#testimonials" class="text-sm text-slate-400 transition-colors hover:text-white">Temoignages</a>
        </div>
        <div class="flex items-center gap-3">
          <NuxtLink
            to="/login"
            class="hidden text-sm text-slate-400 transition-colors hover:text-white sm:block"
          >
            Se connecter
          </NuxtLink>
          <NuxtLink
            to="/login"
            class="inline-flex h-9 items-center gap-2 rounded-lg bg-teal-500 px-4 text-sm font-medium text-white transition-all hover:bg-teal-400 hover:shadow-lg hover:shadow-teal-500/20"
          >
            Demarrer
            <ArrowRight class="h-3.5 w-3.5" />
          </NuxtLink>
        </div>
      </div>
    </nav>

    <!-- Hero -->
    <section class="relative flex min-h-screen items-center overflow-hidden pt-16">
      <!-- Animated gradient orbs -->
      <div class="pointer-events-none absolute inset-0 overflow-hidden">
        <div class="absolute -left-40 -top-40 h-[600px] w-[600px] rounded-full bg-teal-500/20 blur-[128px] animate-pulse-slow" />
        <div class="absolute -right-20 top-1/4 h-[500px] w-[500px] rounded-full bg-cyan-500/10 blur-[128px] animate-pulse-slower" />
        <div class="absolute bottom-0 left-1/3 h-[400px] w-[400px] rounded-full bg-emerald-500/10 blur-[128px] animate-pulse-slow" />
        <!-- Grid pattern -->
        <div class="absolute inset-0 bg-[linear-gradient(rgba(255,255,255,0.02)_1px,transparent_1px),linear-gradient(90deg,rgba(255,255,255,0.02)_1px,transparent_1px)] bg-[size:64px_64px]" />
      </div>

      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div class="grid items-center gap-12 lg:grid-cols-2 lg:gap-20">
          <!-- Left column -->
          <div class="max-w-2xl">
            <div class="mb-6 inline-flex items-center gap-2 rounded-full border border-teal-500/20 bg-teal-500/10 px-4 py-1.5 text-sm text-teal-300">
              <span class="relative flex h-2 w-2">
                <span class="absolute inline-flex h-full w-full animate-ping rounded-full bg-teal-400 opacity-75" />
                <span class="relative inline-flex h-2 w-2 rounded-full bg-teal-400" />
              </span>
              Nouvelle generation de logiciel dentaire
            </div>

            <h1 class="text-4xl font-bold tracking-tight sm:text-5xl md:text-6xl lg:text-7xl">
              <span class="block text-white">Le logiciel</span>
              <span class="block text-white">dentaire</span>
              <span class="block bg-gradient-to-r from-teal-400 via-cyan-400 to-emerald-400 bg-clip-text text-transparent">
                du futur
              </span>
            </h1>

            <p class="mt-6 max-w-lg text-lg leading-relaxed text-slate-400 sm:text-xl">
              Smiled.IO combine precision clinique et experience moderne pour transformer votre cabinet.
            </p>

            <div class="mt-10 flex flex-col gap-4 sm:flex-row">
              <NuxtLink
                to="/login"
                class="group inline-flex h-12 items-center justify-center gap-2 rounded-xl bg-teal-500 px-8 text-base font-semibold text-white transition-all hover:bg-teal-400 hover:shadow-xl hover:shadow-teal-500/25"
              >
                Demarrer gratuitement
                <ArrowRight class="h-4 w-4 transition-transform group-hover:translate-x-0.5" />
              </NuxtLink>
              <button
                class="group inline-flex h-12 items-center justify-center gap-2 rounded-xl border border-white/10 bg-white/5 px-8 text-base font-semibold text-white transition-all hover:border-white/20 hover:bg-white/10"
              >
                <Play class="h-4 w-4 text-teal-400" />
                Voir la demo
              </button>
            </div>

            <p class="mt-4 text-sm text-slate-500">
              Pas de carte bancaire requise. Installation en 5 minutes.
            </p>
          </div>

          <!-- Right column: Dashboard mockup -->
          <div class="relative hidden lg:block">
            <div class="relative">
              <!-- Glow behind card -->
              <div class="absolute -inset-4 rounded-3xl bg-gradient-to-r from-teal-500/20 via-cyan-500/10 to-emerald-500/20 blur-2xl" />

              <!-- Dashboard card -->
              <div class="relative rotate-1 rounded-2xl border border-white/10 bg-slate-900/80 p-1 shadow-2xl backdrop-blur-sm transition-transform duration-500 hover:rotate-0">
                <div class="rounded-xl bg-slate-900 p-6">
                  <!-- Top bar -->
                  <div class="flex items-center justify-between border-b border-white/5 pb-4">
                    <div class="flex items-center gap-3">
                      <div class="h-3 w-3 rounded-full bg-red-500/80" />
                      <div class="h-3 w-3 rounded-full bg-yellow-500/80" />
                      <div class="h-3 w-3 rounded-full bg-green-500/80" />
                    </div>
                    <div class="text-xs text-slate-500">smiled.io/dashboard</div>
                  </div>

                  <!-- Mock content -->
                  <div class="mt-4 grid grid-cols-3 gap-3">
                    <div class="rounded-lg bg-slate-800/50 p-3">
                      <div class="text-xs text-slate-500">Patients</div>
                      <div class="mt-1 text-lg font-bold text-white">1,247</div>
                      <div class="mt-1 text-xs text-teal-400">+12%</div>
                    </div>
                    <div class="rounded-lg bg-slate-800/50 p-3">
                      <div class="text-xs text-slate-500">RDV aujourd'hui</div>
                      <div class="mt-1 text-lg font-bold text-white">18</div>
                      <div class="mt-1 text-xs text-teal-400">3 en cours</div>
                    </div>
                    <div class="rounded-lg bg-slate-800/50 p-3">
                      <div class="text-xs text-slate-500">CA mensuel</div>
                      <div class="mt-1 text-lg font-bold text-white">42.8k</div>
                      <div class="mt-1 text-xs text-teal-400">+8%</div>
                    </div>
                  </div>

                  <!-- Mini dental chart -->
                  <div class="mt-4 rounded-lg border border-white/5 bg-slate-800/30 p-4">
                    <div class="mb-2 text-xs font-medium text-slate-400">Schema dentaire — Patient #1247</div>
                    <div class="flex justify-center gap-1">
                      <div
                        v-for="i in 16" :key="'u' + i"
                        class="h-4 w-3 rounded-sm transition-colors duration-200"
                        :class="i === 5 || i === 12 ? 'bg-amber-500/60' : i === 8 ? 'bg-red-500/60' : 'bg-teal-500/30'"
                      />
                    </div>
                    <div class="mt-1 flex justify-center gap-1">
                      <div
                        v-for="i in 16" :key="'l' + i"
                        class="h-4 w-3 rounded-sm transition-colors duration-200"
                        :class="i === 3 ? 'bg-amber-500/60' : i === 10 ? 'bg-red-500/60' : 'bg-teal-500/30'"
                      />
                    </div>
                  </div>

                  <!-- Patient list preview -->
                  <div class="mt-4 space-y-2">
                    <div v-for="n in 3" :key="n" class="flex items-center gap-3 rounded-lg bg-slate-800/30 p-2">
                      <div class="flex h-8 w-8 items-center justify-center rounded-full bg-teal-500/20 text-xs font-medium text-teal-400">
                        {{ ['SM', 'LR', 'AK'][n - 1] }}
                      </div>
                      <div class="flex-1">
                        <div class="h-2.5 w-24 rounded bg-slate-700" />
                        <div class="mt-1 h-2 w-16 rounded bg-slate-800" />
                      </div>
                      <div class="h-6 w-16 rounded bg-teal-500/10 text-center text-[10px] leading-6 text-teal-400">
                        {{ ['14:00', '14:30', '15:00'][n - 1] }}
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Scroll indicator -->
      <div class="absolute bottom-8 left-1/2 -translate-x-1/2">
        <div class="flex h-8 w-5 items-start justify-center rounded-full border border-white/20 p-1">
          <div class="h-1.5 w-1 animate-bounce rounded-full bg-white/60" />
        </div>
      </div>
    </section>

    <!-- Stats bar -->
    <section class="relative border-y border-white/5 bg-slate-900/50 backdrop-blur-sm">
      <div class="mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
        <div class="grid grid-cols-2 gap-8 md:grid-cols-4">
          <div
            v-for="(stat, i) in stats" :key="i"
            class="reveal-on-scroll opacity-0 translate-y-4 transition-all duration-700 text-center"
            :style="{ transitionDelay: `${i * 100}ms` }"
          >
            <div class="text-3xl font-bold text-teal-400 sm:text-4xl">{{ stat.value }}</div>
            <div class="mt-1 text-sm font-medium text-white">{{ stat.label }}</div>
            <div class="text-xs text-slate-500">{{ stat.sublabel }}</div>
          </div>
        </div>
      </div>
    </section>

    <!-- Features -->
    <section id="features" class="relative py-24 sm:py-32">
      <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div class="reveal-on-scroll opacity-0 translate-y-4 transition-all duration-700 mx-auto max-w-2xl text-center">
          <div class="mb-4 inline-flex items-center gap-2 rounded-full border border-teal-500/20 bg-teal-500/10 px-3 py-1 text-xs font-medium text-teal-300">
            Fonctionnalites
          </div>
          <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl">
            Tout ce dont votre cabinet a besoin
          </h2>
          <p class="mt-4 text-lg text-slate-400">
            Une suite complete concue pour le quotidien des chirurgiens-dentistes.
          </p>
        </div>

        <div class="mt-16 grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
          <div
            v-for="(feature, i) in features" :key="i"
            class="reveal-on-scroll opacity-0 translate-y-4 transition-all duration-700 group relative rounded-2xl border border-white/5 bg-slate-900/50 p-6 backdrop-blur-sm hover:-translate-y-1 hover:border-teal-500/20 hover:shadow-lg hover:shadow-teal-500/5"
            :style="{ transitionDelay: `${i * 100}ms` }"
          >
            <div class="mb-4 inline-flex h-11 w-11 items-center justify-center rounded-xl bg-teal-500/10 text-teal-400 transition-colors group-hover:bg-teal-500/20">
              <component :is="feature.icon" class="h-5 w-5" />
            </div>
            <h3 class="text-lg font-semibold text-white">{{ feature.title }}</h3>
            <p class="mt-2 text-sm leading-relaxed text-slate-400">{{ feature.description }}</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Interactive dental chart preview -->
    <section id="demo" class="relative overflow-hidden py-24 sm:py-32">
      <!-- Background gradient -->
      <div class="pointer-events-none absolute inset-0">
        <div class="absolute inset-0 bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950" />
        <div class="absolute left-1/2 top-1/2 h-[800px] w-[800px] -translate-x-1/2 -translate-y-1/2 rounded-full bg-teal-500/5 blur-[128px]" />
      </div>

      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div class="reveal-on-scroll opacity-0 translate-y-4 transition-all duration-700 mx-auto max-w-2xl text-center">
          <div class="mb-4 inline-flex items-center gap-2 rounded-full border border-teal-500/20 bg-teal-500/10 px-3 py-1 text-xs font-medium text-teal-300">
            Demo interactive
          </div>
          <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl">
            Schema dentaire nouvelle generation
          </h2>
          <p class="mt-4 text-lg text-slate-400">
            Survolez les dents pour decouvrir l'interface. Chaque dent est cliquable, annotable et tracable.
          </p>
        </div>

        <div class="reveal-on-scroll opacity-0 translate-y-4 transition-all duration-1000 mt-16">
          <div class="mx-auto max-w-4xl rounded-2xl border border-white/10 bg-slate-900/80 p-1 shadow-2xl backdrop-blur-sm">
            <div class="rounded-xl bg-slate-900 p-6 sm:p-8">
              <div class="grid gap-8 lg:grid-cols-[1fr,280px]">
                <!-- Chart area -->
                <div>
                  <div class="mb-6 flex items-center justify-between">
                    <h3 class="text-sm font-medium text-slate-300">Vue adulte — 32 dents</h3>
                    <div class="flex gap-2">
                      <span class="inline-flex items-center gap-1.5 text-xs text-slate-500">
                        <span class="h-2.5 w-2.5 rounded-sm bg-teal-500/40" /> Sain
                      </span>
                      <span class="inline-flex items-center gap-1.5 text-xs text-slate-500">
                        <span class="h-2.5 w-2.5 rounded-sm bg-amber-500/60" /> Soins
                      </span>
                      <span class="inline-flex items-center gap-1.5 text-xs text-slate-500">
                        <span class="h-2.5 w-2.5 rounded-sm bg-red-500/60" /> Urgent
                      </span>
                    </div>
                  </div>

                  <!-- Upper arch -->
                  <div class="mb-2 text-center text-[10px] font-medium uppercase tracking-widest text-slate-600">Maxillaire</div>
                  <div class="flex justify-center gap-1 sm:gap-1.5">
                    <div
                      v-for="tooth in upperTeeth" :key="tooth"
                      class="group/tooth relative flex h-10 w-6 cursor-pointer items-center justify-center rounded-md border text-[10px] font-medium transition-all duration-200 sm:h-12 sm:w-8 sm:text-xs"
                      :class="
                        hoveredTooth === tooth
                          ? 'border-teal-400 bg-teal-500/30 text-teal-300 scale-110 shadow-lg shadow-teal-500/20'
                          : tooth === 15 || tooth === 26
                            ? 'border-amber-500/30 bg-amber-500/10 text-amber-400 hover:border-amber-400 hover:bg-amber-500/20'
                            : tooth === 11
                              ? 'border-red-500/30 bg-red-500/10 text-red-400 hover:border-red-400 hover:bg-red-500/20'
                              : 'border-white/10 bg-white/5 text-slate-400 hover:border-teal-500/30 hover:bg-teal-500/10 hover:text-teal-300'
                      "
                      @mouseenter="hoveredTooth = tooth"
                      @mouseleave="hoveredTooth = null"
                    >
                      {{ tooth }}
                    </div>
                  </div>

                  <!-- Separator -->
                  <div class="my-3 flex items-center gap-2">
                    <div class="h-px flex-1 bg-gradient-to-r from-transparent via-white/10 to-transparent" />
                    <div class="text-[10px] text-slate-600">D / G</div>
                    <div class="h-px flex-1 bg-gradient-to-r from-transparent via-white/10 to-transparent" />
                  </div>

                  <!-- Lower arch -->
                  <div class="flex justify-center gap-1 sm:gap-1.5">
                    <div
                      v-for="tooth in lowerTeeth" :key="tooth"
                      class="group/tooth relative flex h-10 w-6 cursor-pointer items-center justify-center rounded-md border text-[10px] font-medium transition-all duration-200 sm:h-12 sm:w-8 sm:text-xs"
                      :class="
                        hoveredTooth === tooth
                          ? 'border-teal-400 bg-teal-500/30 text-teal-300 scale-110 shadow-lg shadow-teal-500/20'
                          : tooth === 46
                            ? 'border-amber-500/30 bg-amber-500/10 text-amber-400 hover:border-amber-400 hover:bg-amber-500/20'
                            : tooth === 31
                              ? 'border-red-500/30 bg-red-500/10 text-red-400 hover:border-red-400 hover:bg-red-500/20'
                              : 'border-white/10 bg-white/5 text-slate-400 hover:border-teal-500/30 hover:bg-teal-500/10 hover:text-teal-300'
                      "
                      @mouseenter="hoveredTooth = tooth"
                      @mouseleave="hoveredTooth = null"
                    >
                      {{ tooth }}
                    </div>
                  </div>
                  <div class="mt-2 text-center text-[10px] font-medium uppercase tracking-widest text-slate-600">Mandibulaire</div>
                </div>

                <!-- Detail panel -->
                <div class="rounded-xl border border-white/5 bg-slate-800/30 p-4">
                  <Transition name="fade" mode="out-in">
                    <div v-if="hoveredTooth" :key="hoveredTooth">
                      <div class="mb-3 flex items-center gap-2">
                        <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-teal-500/20 text-sm font-bold text-teal-400">
                          {{ hoveredTooth }}
                        </div>
                        <div>
                          <div class="text-sm font-medium text-white">
                            Dent {{ hoveredTooth }}
                          </div>
                          <div class="text-xs text-slate-500">
                            {{ hoveredTooth <= 28 ? 'Maxillaire' : 'Mandibulaire' }}
                            {{ (hoveredTooth >= 11 && hoveredTooth <= 28) || (hoveredTooth >= 31 && hoveredTooth <= 38) ? '' : '' }}
                          </div>
                        </div>
                      </div>

                      <div class="space-y-2">
                        <div class="rounded-lg bg-slate-800/50 p-2">
                          <div class="text-[10px] uppercase tracking-wider text-slate-500">Etat</div>
                          <div class="mt-0.5 text-xs font-medium" :class="hoveredTooth === 11 || hoveredTooth === 31 ? 'text-red-400' : hoveredTooth === 15 || hoveredTooth === 26 || hoveredTooth === 46 ? 'text-amber-400' : 'text-teal-400'">
                            {{ hoveredTooth === 11 || hoveredTooth === 31 ? 'Carie profonde — Urgent' : hoveredTooth === 15 || hoveredTooth === 26 || hoveredTooth === 46 ? 'Composite a surveiller' : 'Saine — RAS' }}
                          </div>
                        </div>
                        <div class="rounded-lg bg-slate-800/50 p-2">
                          <div class="text-[10px] uppercase tracking-wider text-slate-500">Derniere intervention</div>
                          <div class="mt-0.5 text-xs text-slate-300">
                            {{ hoveredTooth === 15 || hoveredTooth === 26 ? 'Composite — 12/2024' : hoveredTooth === 46 ? 'Detartrage — 06/2025' : '—' }}
                          </div>
                        </div>
                        <div class="rounded-lg bg-slate-800/50 p-2">
                          <div class="text-[10px] uppercase tracking-wider text-slate-500">Acte prevu</div>
                          <div class="mt-0.5 text-xs text-slate-300">
                            {{ hoveredTooth === 11 || hoveredTooth === 31 ? 'HBMD038 — Restauration' : 'Aucun' }}
                          </div>
                        </div>
                      </div>
                    </div>
                    <div v-else class="flex h-full items-center justify-center py-8">
                      <div class="text-center">
                        <div class="mx-auto mb-3 flex h-12 w-12 items-center justify-center rounded-full bg-teal-500/10">
                          <Activity class="h-5 w-5 text-teal-400" />
                        </div>
                        <p class="text-sm text-slate-400">Survolez une dent</p>
                        <p class="text-xs text-slate-600">pour voir les details</p>
                      </div>
                    </div>
                  </Transition>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Comparison -->
    <section id="comparison" class="relative py-24 sm:py-32">
      <div class="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div class="reveal-on-scroll opacity-0 translate-y-4 transition-all duration-700 mx-auto max-w-2xl text-center">
          <div class="mb-4 inline-flex items-center gap-2 rounded-full border border-teal-500/20 bg-teal-500/10 px-3 py-1 text-xs font-medium text-teal-300">
            Comparatif
          </div>
          <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl">
            Pourquoi Smiled.IO ?
          </h2>
          <p class="mt-4 text-lg text-slate-400">
            Comparez objectivement les fonctionnalites essentielles.
          </p>
        </div>

        <div class="reveal-on-scroll opacity-0 translate-y-4 transition-all duration-700 mt-12 overflow-x-auto">
          <table class="w-full min-w-[640px]">
            <thead>
              <tr class="border-b border-white/10">
                <th class="pb-4 pr-4 text-left text-sm font-medium text-slate-400">Fonctionnalite</th>
                <th class="pb-4 px-4 text-center">
                  <div class="inline-flex flex-col items-center gap-1 rounded-xl border border-teal-500/30 bg-teal-500/10 px-4 py-2">
                    <span class="text-sm font-bold text-teal-400">Smiled.IO</span>
                  </div>
                </th>
                <th class="pb-4 px-4 text-center text-sm font-medium text-slate-500">Veasy</th>
                <th class="pb-4 px-4 text-center text-sm font-medium text-slate-500">Julie</th>
                <th class="pb-4 px-4 text-center text-sm font-medium text-slate-500">LOGOSw</th>
              </tr>
            </thead>
            <tbody>
              <tr
                v-for="(row, i) in comparisonFeatures" :key="i"
                class="border-b border-white/5 transition-colors hover:bg-white/[0.02]"
              >
                <td class="py-3 pr-4 text-sm text-slate-300">{{ row.name }}</td>
                <td class="py-3 px-4 text-center">
                  <span class="inline-flex h-6 w-6 items-center justify-center rounded-full" :class="row.smiled ? 'bg-teal-500/20' : 'bg-red-500/20'">
                    <Check v-if="row.smiled" class="h-3.5 w-3.5 text-teal-400" />
                    <X v-else class="h-3.5 w-3.5 text-red-400" />
                  </span>
                </td>
                <td class="py-3 px-4 text-center">
                  <span class="inline-flex h-6 w-6 items-center justify-center rounded-full" :class="row.veasy ? 'bg-teal-500/20' : 'bg-white/5'">
                    <Check v-if="row.veasy" class="h-3.5 w-3.5 text-teal-400" />
                    <X v-else class="h-3.5 w-3.5 text-slate-600" />
                  </span>
                </td>
                <td class="py-3 px-4 text-center">
                  <span class="inline-flex h-6 w-6 items-center justify-center rounded-full" :class="row.julie ? 'bg-teal-500/20' : 'bg-white/5'">
                    <Check v-if="row.julie" class="h-3.5 w-3.5 text-teal-400" />
                    <X v-else class="h-3.5 w-3.5 text-slate-600" />
                  </span>
                </td>
                <td class="py-3 px-4 text-center">
                  <span class="inline-flex h-6 w-6 items-center justify-center rounded-full" :class="row.logos ? 'bg-teal-500/20' : 'bg-white/5'">
                    <Check v-if="row.logos" class="h-3.5 w-3.5 text-teal-400" />
                    <X v-else class="h-3.5 w-3.5 text-slate-600" />
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </section>

    <!-- Testimonials -->
    <section id="testimonials" class="relative py-24 sm:py-32">
      <div class="pointer-events-none absolute inset-0">
        <div class="absolute inset-0 bg-gradient-to-b from-slate-950 via-slate-900/50 to-slate-950" />
      </div>

      <div class="relative mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
        <div class="reveal-on-scroll opacity-0 translate-y-4 transition-all duration-700 mx-auto max-w-2xl text-center">
          <div class="mb-4 inline-flex items-center gap-2 rounded-full border border-teal-500/20 bg-teal-500/10 px-3 py-1 text-xs font-medium text-teal-300">
            Temoignages
          </div>
          <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl">
            Concu par des chirurgiens-dentistes,<br class="hidden sm:block" />
            pour des chirurgiens-dentistes
          </h2>
        </div>

        <div class="mt-16 grid gap-6 md:grid-cols-3">
          <div
            v-for="(t, i) in testimonials" :key="i"
            class="reveal-on-scroll opacity-0 translate-y-4 transition-all duration-700 group relative rounded-2xl border border-white/5 bg-slate-900/50 p-6 backdrop-blur-sm hover:border-white/10"
            :style="{ transitionDelay: `${i * 150}ms` }"
          >
            <!-- Stars -->
            <div class="mb-4 flex gap-1">
              <Star v-for="s in 5" :key="s" class="h-4 w-4 fill-amber-400 text-amber-400" />
            </div>

            <p class="text-sm leading-relaxed text-slate-300">
              &laquo; {{ t.quote }} &raquo;
            </p>

            <div class="mt-6 flex items-center gap-3">
              <div class="flex h-10 w-10 items-center justify-center rounded-full bg-gradient-to-br from-teal-500 to-cyan-500 text-sm font-bold text-white">
                {{ t.initials }}
              </div>
              <div>
                <div class="text-sm font-medium text-white">{{ t.name }}</div>
                <div class="text-xs text-slate-500">{{ t.role }}</div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- CTA -->
    <section class="relative py-24 sm:py-32">
      <div class="pointer-events-none absolute inset-0">
        <div class="absolute left-1/2 top-1/2 h-[600px] w-[600px] -translate-x-1/2 -translate-y-1/2 rounded-full bg-teal-500/10 blur-[128px]" />
      </div>

      <div class="relative mx-auto max-w-3xl px-4 text-center sm:px-6 lg:px-8">
        <div class="reveal-on-scroll opacity-0 translate-y-4 transition-all duration-700">
          <h2 class="text-3xl font-bold tracking-tight text-white sm:text-4xl md:text-5xl">
            Pret a moderniser<br />votre cabinet ?
          </h2>
          <p class="mt-6 text-lg text-slate-400">
            Rejoignez les praticiens qui ont choisi l'excellence technologique.
          </p>

          <div class="mt-10 flex flex-col items-center gap-4 sm:flex-row sm:justify-center">
            <NuxtLink
              to="/login"
              class="group inline-flex h-14 items-center gap-2 rounded-xl bg-teal-500 px-10 text-lg font-semibold text-white transition-all hover:bg-teal-400 hover:shadow-xl hover:shadow-teal-500/25"
            >
              Demarrer gratuitement
              <ChevronRight class="h-5 w-5 transition-transform group-hover:translate-x-0.5" />
            </NuxtLink>
          </div>
          <p class="mt-4 text-sm text-slate-500">
            Pas de carte bancaire requise. Deploiement Docker en 5 minutes.
          </p>
        </div>
      </div>
    </section>

    <!-- Footer -->
    <footer class="border-t border-white/5 bg-slate-950">
      <div class="mx-auto max-w-7xl px-4 py-12 sm:px-6 lg:px-8">
        <div class="grid gap-8 sm:grid-cols-2 lg:grid-cols-4">
          <!-- Brand -->
          <div class="sm:col-span-2 lg:col-span-1">
            <div class="flex items-center gap-2">
              <img src="~/assets/img/logomini.png" alt="Smiled.IO" class="h-8 w-8 rounded-lg" />
              <span class="text-lg font-bold tracking-tight text-white">Smiled.IO</span>
            </div>
            <p class="mt-3 max-w-xs text-sm text-slate-500">
              Le logiciel dentaire moderne, self-hosted et open source.
            </p>
          </div>

          <!-- Produit -->
          <div>
            <h4 class="text-sm font-semibold text-white">Produit</h4>
            <ul class="mt-3 space-y-2">
              <li><a href="#features" class="text-sm text-slate-500 transition-colors hover:text-slate-300">Fonctionnalites</a></li>
              <li><a href="#demo" class="text-sm text-slate-500 transition-colors hover:text-slate-300">Demo</a></li>
              <li><a href="#comparison" class="text-sm text-slate-500 transition-colors hover:text-slate-300">Comparatif</a></li>
              <li><a href="#" class="text-sm text-slate-500 transition-colors hover:text-slate-300">Documentation</a></li>
            </ul>
          </div>

          <!-- Entreprise -->
          <div>
            <h4 class="text-sm font-semibold text-white">Entreprise</h4>
            <ul class="mt-3 space-y-2">
              <li><a href="#" class="text-sm text-slate-500 transition-colors hover:text-slate-300">A propos</a></li>
              <li><a href="#" class="text-sm text-slate-500 transition-colors hover:text-slate-300">Contact</a></li>
              <li><a href="#" class="text-sm text-slate-500 transition-colors hover:text-slate-300">Blog</a></li>
            </ul>
          </div>

          <!-- Legal -->
          <div>
            <h4 class="text-sm font-semibold text-white">Legal</h4>
            <ul class="mt-3 space-y-2">
              <li><a href="#" class="text-sm text-slate-500 transition-colors hover:text-slate-300">Mentions legales</a></li>
              <li><a href="#" class="text-sm text-slate-500 transition-colors hover:text-slate-300">Politique de confidentialite</a></li>
              <li><a href="#" class="text-sm text-slate-500 transition-colors hover:text-slate-300">CGU</a></li>
            </ul>
          </div>
        </div>

        <div class="mt-12 flex flex-col items-center justify-between gap-4 border-t border-white/5 pt-8 sm:flex-row">
          <p class="text-xs text-slate-600">&copy; {{ new Date().getFullYear() }} Smiled.IO. Tous droits reserves.</p>
          <div class="flex items-center gap-4">
            <a href="#" class="text-slate-600 transition-colors hover:text-slate-400">
              <Shield class="h-4 w-4" />
            </a>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<style scoped>
/* Scroll reveal */
.reveal-on-scroll {
  transition-property: opacity, transform;
}

.reveal-on-scroll.revealed {
  opacity: 1 !important;
  transform: translateY(0) !important;
}

/* Pulse animations for background orbs */
.animate-pulse-slow {
  animation: pulse-glow 6s ease-in-out infinite;
}

.animate-pulse-slower {
  animation: pulse-glow 8s ease-in-out infinite;
  animation-delay: 2s;
}

@keyframes pulse-glow {
  0%, 100% {
    opacity: 0.4;
    transform: scale(1);
  }
  50% {
    opacity: 0.7;
    transform: scale(1.1);
  }
}

/* Fade transition for dental chart detail panel */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease, transform 0.15s ease;
}

.fade-enter-from {
  opacity: 0;
  transform: translateY(4px);
}

.fade-leave-to {
  opacity: 0;
  transform: translateY(-4px);
}

/* Smooth scroll for the whole page */
html {
  scroll-behavior: smooth;
}
</style>
