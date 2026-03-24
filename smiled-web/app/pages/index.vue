<template>
  <div class="p-0 space-y-6">
    <!-- Header -->
    <header>
      <h1 class="text-2xl font-bold text-foreground">
        Bonjour, Dr. {{ userLastName }}
      </h1>
      <p class="text-sm text-muted-foreground mt-1">
        Voici un apercu de votre journee &mdash; {{ todayFormatted }}
      </p>
    </header>

    <!-- Stats Cards -->
    <section class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
      <Card v-for="stat in statCards" :key="stat.label">
        <CardContent class="p-5">
          <div class="flex items-center gap-4">
            <div class="flex-shrink-0 w-10 h-10 rounded-full bg-primary/10 flex items-center justify-center">
              <Icon :name="stat.icon" class="w-5 h-5 text-primary" />
            </div>
            <div class="min-w-0">
              <p class="text-[11px] font-semibold uppercase tracking-wider text-muted-foreground">{{ stat.label }}</p>
              <template v-if="loading">
                <Skeleton class="h-7 w-16 mt-1" />
              </template>
              <template v-else>
                <p class="text-2xl font-bold text-foreground">{{ stat.value }}</p>
              </template>
              <p class="text-xs mt-0.5" :class="stat.trendColor">{{ stat.trend }}</p>
            </div>
          </div>
        </CardContent>
      </Card>
    </section>

    <!-- Main content: 2-column layout -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Left column (2/3) -->
      <div class="lg:col-span-2 space-y-6">
        <!-- Prochains rendez-vous -->
        <Card>
          <CardHeader class="pb-3">
            <div class="flex items-center justify-between">
              <CardTitle class="text-base font-semibold">Prochains rendez-vous</CardTitle>
              <NuxtLink to="/agenda" class="text-xs text-primary hover:underline">Voir l'agenda</NuxtLink>
            </div>
          </CardHeader>
          <CardContent>
            <div v-if="appointments.length === 0" class="flex flex-col items-center justify-center py-10 text-muted-foreground">
              <Icon name="lucide:calendar-x" class="w-10 h-10 mb-3 opacity-40" />
              <p class="text-sm">Aucun rendez-vous aujourd'hui</p>
            </div>
            <div v-else class="divide-y divide-border">
              <div v-for="appt in appointments" :key="appt.id" class="flex items-center gap-4 py-3 first:pt-0 last:pb-0" :class="{ 'border-l-2 border-l-primary pl-3': appt.status === 'confirme', 'border-l-2 border-l-amber-400 pl-3': appt.status === 'en_attente' }">
                <div class="flex-shrink-0 w-14 text-center">
                  <p class="text-sm font-bold text-foreground">{{ appt.time }}</p>
                </div>
                <div class="flex-1 min-w-0">
                  <NuxtLink :to="`/patients/${appt.patientId}`" class="text-sm font-medium text-foreground hover:text-primary transition-colors">{{ appt.patientName }}</NuxtLink>
                  <div class="mt-0.5">
                    <Badge variant="secondary" class="text-[10px]">{{ appt.motif }}</Badge>
                  </div>
                </div>
                <span class="inline-block w-2 h-2 rounded-full flex-shrink-0" :class="{ 'bg-primary': appt.status === 'confirme', 'bg-amber-400': appt.status === 'en_attente' }" />
              </div>
            </div>
          </CardContent>
        </Card>

        <!-- Patients recents -->
        <Card>
          <CardHeader class="pb-3">
            <div class="flex items-center justify-between">
              <CardTitle class="text-base font-semibold">Patients recents</CardTitle>
              <NuxtLink to="/patients" class="text-xs text-primary hover:underline">Voir tous les patients</NuxtLink>
            </div>
          </CardHeader>
          <CardContent>
            <template v-if="loading">
              <div class="space-y-3">
                <div v-for="i in 5" :key="i" class="flex items-center gap-3">
                  <Skeleton class="w-9 h-9 rounded-full flex-shrink-0" />
                  <div class="flex-1">
                    <Skeleton class="h-4 w-32 mb-1" />
                    <Skeleton class="h-3 w-24" />
                  </div>
                  <Skeleton class="h-4 w-20" />
                </div>
              </div>
            </template>
            <template v-else-if="recentPatients.length === 0">
              <div class="flex flex-col items-center justify-center py-10 text-muted-foreground">
                <Icon name="lucide:user-x" class="w-10 h-10 mb-3 opacity-40" />
                <p class="text-sm">Aucun patient enregistre</p>
              </div>
            </template>
            <template v-else>
              <div class="divide-y divide-border">
                <div v-for="p in recentPatients" :key="p.id" class="flex items-center gap-3 py-3 first:pt-0 last:pb-0">
                  <div class="flex-shrink-0 w-9 h-9 rounded-full bg-gradient-to-br from-primary to-primary/70 flex items-center justify-center">
                    <span class="text-xs font-semibold text-white">{{ p.prenom.charAt(0) }}{{ p.nom.charAt(0) }}</span>
                  </div>
                  <div class="flex-1 min-w-0">
                    <p class="text-sm font-medium text-foreground truncate">{{ p.nom }} {{ p.prenom }}</p>
                    <p class="text-xs text-muted-foreground">Derniere visite : {{ formatDate(p.updated_at) }}</p>
                  </div>
                  <NuxtLink :to="`/patients/${p.id}`" class="flex-shrink-0 text-xs text-primary hover:underline">Voir dossier</NuxtLink>
                </div>
              </div>
            </template>
          </CardContent>
        </Card>
      </div>

      <!-- Right column (1/3) -->
      <div class="space-y-6">
        <!-- Alertes medicales -->
        <Card>
          <CardHeader class="pb-3">
            <div class="flex items-center gap-2">
              <Icon name="lucide:alert-triangle" class="w-4 h-4 text-amber-500" />
              <CardTitle class="text-base font-semibold">Alertes medicales</CardTitle>
            </div>
            <CardDescription>{{ medicalAlerts.length }} patient{{ medicalAlerts.length > 1 ? 's' : '' }} avec alertes actives</CardDescription>
          </CardHeader>
          <CardContent>
            <div v-if="medicalAlerts.length === 0" class="text-center py-6 text-muted-foreground">
              <Icon name="lucide:shield-check" class="w-8 h-8 mx-auto mb-2 opacity-40" />
              <p class="text-sm">Aucune alerte active</p>
            </div>
            <div v-else class="space-y-3">
              <div v-for="alert in medicalAlerts" :key="alert.id" class="flex items-start gap-3">
                <div class="flex-shrink-0 mt-1.5 w-2 h-2 rounded-full" :class="{ 'bg-red-500': alert.severity === 'high', 'bg-amber-500': alert.severity === 'medium', 'bg-yellow-400': alert.severity === 'low' }" />
                <div class="min-w-0">
                  <p class="text-sm font-medium text-foreground">{{ alert.patientName }}</p>
                  <Badge :variant="alert.severity === 'high' ? 'destructive' : 'secondary'" class="text-[10px] mt-0.5">{{ alert.type }}</Badge>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        <!-- Activite recente -->
        <Card>
          <CardHeader class="pb-3">
            <CardTitle class="text-base font-semibold">Activite recente</CardTitle>
          </CardHeader>
          <CardContent>
            <div v-if="recentActivity.length === 0" class="text-center py-6 text-muted-foreground">
              <Icon name="lucide:activity" class="w-8 h-8 mx-auto mb-2 opacity-40" />
              <p class="text-sm">Aucune activite recente</p>
            </div>
            <div v-else class="space-y-0">
              <div v-for="(activity, idx) in recentActivity" :key="idx" class="relative flex gap-3">
                <div class="flex flex-col items-center">
                  <div class="w-2 h-2 rounded-full bg-primary mt-1.5 flex-shrink-0" />
                  <div v-if="idx < recentActivity.length - 1" class="w-px flex-1 bg-border mt-1" />
                </div>
                <div class="pb-4 min-w-0">
                  <p class="text-sm text-foreground">{{ activity.description }}</p>
                  <p class="text-xs text-muted-foreground mt-0.5">{{ activity.time }}</p>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Patient } from '~/types/patient'
import { formatDate, formatPrice } from '~/utils/format'

const authStore = useAuthStore()
const { listPatients } = usePatient()

const loading = ref(true)
const recentPatients = ref<Patient[]>([])

const stats = reactive({
  patients: { total: 0, trend: 0 },
  rdvToday: 0,
  actesMonth: 0,
  actesTrend: 0,
  revenueFormatted: formatPrice(0),
  revenueTrend: 0,
})

// Placeholder: appointments (no appointment API yet)
const appointments = ref([
  { id: '1', time: '09:00', patientName: 'Dupont Marie', patientId: '', motif: 'Detartrage', status: 'confirme' as const },
  { id: '2', time: '10:30', patientName: 'Martin Jean', patientId: '', motif: 'Controle', status: 'en_attente' as const },
  { id: '3', time: '11:15', patientName: 'Leroy Sophie', patientId: '', motif: 'Couronne', status: 'confirme' as const },
  { id: '4', time: '14:00', patientName: 'Bernard Paul', patientId: '', motif: 'Extraction', status: 'confirme' as const },
  { id: '5', time: '15:30', patientName: 'Petit Claire', patientId: '', motif: 'Composite', status: 'en_attente' as const },
])

// Placeholder: medical alerts (no dedicated endpoint yet)
const medicalAlerts = ref([
  { id: '1', patientName: 'Dupont Marie', type: 'AVK', severity: 'high' as const },
  { id: '2', patientName: 'Bernard Paul', type: 'Allergie latex', severity: 'high' as const },
  { id: '3', patientName: 'Leroy Sophie', type: 'Bisphosphonates', severity: 'medium' as const },
])

// Placeholder: recent activity (no audit log endpoint yet)
const recentActivity = ref([
  { description: 'Schema dentaire mis a jour — Dupont M.', time: 'il y a 2h' },
  { description: 'Nouveau diagnostic — Martin J.', time: 'il y a 3h' },
  { description: 'PDT valide — Leroy S.', time: 'il y a 5h' },
  { description: 'Patient cree — Bernard P.', time: 'hier' },
  { description: 'Questionnaire signe — Petit C.', time: 'hier' },
])

const userLastName = computed(() => authStore.user?.nom ?? '')

const todayFormatted = computed(() =>
  new Date().toLocaleDateString('fr-FR', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  }),
)

const statCards = computed(() => [
  {
    icon: 'lucide:users',
    label: 'Patients',
    value: stats.patients.total,
    trend: `+${stats.patients.trend} ce mois`,
    trendColor: 'text-primary',
  },
  {
    icon: 'lucide:calendar-check',
    label: "RDV aujourd'hui",
    value: stats.rdvToday,
    trend: 'rendez-vous prevus',
    trendColor: 'text-muted-foreground',
  },
  {
    icon: 'lucide:stethoscope',
    label: 'Actes ce mois',
    value: stats.actesMonth,
    trend: `+${stats.actesTrend} vs mois dernier`,
    trendColor: 'text-primary',
  },
  {
    icon: 'lucide:euro',
    label: 'CA ce mois',
    value: stats.revenueFormatted,
    trend: `+${stats.revenueTrend} % vs mois dernier`,
    trendColor: 'text-primary',
  },
])

onMounted(async () => {
  try {
    const res = await listPatients({ limit: 5 })
    if (res.success && res.data) {
      recentPatients.value = res.data
      const total = res.meta?.total ?? res.data.length
      stats.patients.total = total
      stats.patients.trend = Math.min(total, 3)
    }
    // Placeholder stats — replace when real endpoints exist
    stats.rdvToday = appointments.value.length
    stats.actesMonth = 47
    stats.actesTrend = 12
    stats.revenueFormatted = formatPrice(18450)
    stats.revenueTrend = 8
  } catch {
    // Silently degrade — stats show 0
  } finally {
    loading.value = false
  }
})
</script>
