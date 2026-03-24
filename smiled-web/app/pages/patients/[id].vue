<template>
  <div class="-m-6">
    <!-- Loading skeleton -->
    <div v-if="loading" class="p-6">
      <div class="flex items-center gap-4 mb-6">
        <Skeleton class="h-10 w-10 rounded-full" />
        <div class="space-y-2">
          <Skeleton class="h-5 w-48" />
          <Skeleton class="h-4 w-72" />
        </div>
      </div>
      <div class="flex gap-4 mb-6">
        <Skeleton v-for="i in 6" :key="i" class="h-9 w-24 rounded-md" />
      </div>
      <Skeleton class="h-64 w-full rounded-lg" />
    </div>

    <!-- Error -->
    <div v-else-if="error" class="p-6">
      <Alert variant="destructive">
        <AlertDescription>{{ error }}</AlertDescription>
      </Alert>
    </div>

    <template v-else-if="patient">
      <!-- Sticky patient header -->
      <div class="sticky top-0 z-10 bg-background border-b">
        <div class="flex items-center gap-4 px-6 py-3">
          <!-- Back button -->
          <Button
            variant="ghost"
            size="icon"
            class="shrink-0 h-8 w-8 text-muted-foreground hover:text-foreground"
            @click="navigateTo('/patients')"
          >
            <Icon name="lucide:arrow-left" class="h-4 w-4" />
          </Button>

          <!-- Avatar -->
          <div
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full bg-primary/10 text-sm font-bold text-primary"
          >
            {{ initials }}
          </div>

          <!-- Name + badges -->
          <div class="min-w-0 flex-1">
            <div class="flex items-center gap-2">
              <h1 class="truncate text-lg font-semibold text-foreground">
                {{ patient.nom }} {{ patient.prenom }}
              </h1>
              <Badge variant="secondary" class="shrink-0 text-xs">
                {{ age }} ans
              </Badge>
              <Badge variant="secondary" class="shrink-0 text-xs">
                {{ patient.sexe === 'M' ? 'Homme' : 'Femme' }}
              </Badge>
            </div>
          </div>

          <!-- Medical alerts -->
          <div v-if="medicalAlerts.length > 0" class="hidden sm:flex items-center gap-2">
            <Badge
              v-for="alert in medicalAlerts"
              :key="alert"
              variant="outline"
              class="border-amber-300 bg-amber-50 text-amber-800 text-xs"
            >
              <Icon name="lucide:alert-triangle" class="mr-1 h-3 w-3" />
              {{ alert }}
            </Badge>
          </div>
        </div>

        <!-- Mobile alerts -->
        <div
          v-if="medicalAlerts.length > 0"
          class="sm:hidden flex flex-wrap items-center gap-2 px-6 pb-3"
        >
          <Badge
            v-for="alert in medicalAlerts"
            :key="alert"
            variant="outline"
            class="border-amber-300 bg-amber-50 text-amber-800 text-xs"
          >
            <Icon name="lucide:alert-triangle" class="mr-1 h-3 w-3" />
            {{ alert }}
          </Badge>
        </div>

        <!-- Tab navigation -->
        <div class="flex gap-1 px-6 border-b overflow-x-auto">
          <NuxtLink
            v-for="tab in tabs"
            :key="tab.to"
            :to="tab.to"
            class="whitespace-nowrap px-3 py-2 text-sm transition-colors"
            :class="
              isTabActive(tab)
                ? 'border-b-2 border-primary text-primary font-medium'
                : 'text-muted-foreground hover:text-foreground'
            "
          >
            {{ tab.label }}
          </NuxtLink>
        </div>
      </div>

      <!-- Content -->
      <div class="p-6">
        <NuxtPage />
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { Patient, Questionnaire } from '~/types/patient'

const route = useRoute()
const patientId = computed(() => route.params.id as string)

const { getPatient, getQuestionnaire } = usePatient()

const patient = ref<Patient | null>(null)
const questionnaire = ref<Questionnaire | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)

const initials = computed(() => {
  if (!patient.value) return ''
  const n = patient.value.nom?.[0] ?? ''
  const p = patient.value.prenom?.[0] ?? ''
  return `${n}${p}`.toUpperCase()
})

const age = computed(() => {
  if (!patient.value) return 0
  const today = new Date()
  const birth = new Date(patient.value.date_naissance)
  let years = today.getFullYear() - birth.getFullYear()
  const monthDiff = today.getMonth() - birth.getMonth()
  if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birth.getDate())) {
    years--
  }
  return years
})

const medicalAlerts = computed(() => {
  const q = questionnaire.value
  if (!q) return []
  const alerts: string[] = []

  const avk = q.avk as { actif?: boolean } | null
  if (avk?.actif) alerts.push('AVK')

  const endocardite = q.endocardite as { actif?: boolean } | null
  if (endocardite?.actif) alerts.push('Endocardite')

  const bisphos = q.bisphosphonates as { actif?: boolean } | null
  if (bisphos?.actif) alerts.push('Bisphosphonates')

  const allergies = q.allergies as { actif?: boolean; detail?: string } | null
  if (allergies?.actif) {
    alerts.push(allergies.detail ? `Allergies : ${allergies.detail}` : 'Allergies')
  }

  const radio = q.radiotherapie as { actif?: boolean } | null
  if (radio?.actif) alerts.push('Radiotherapie')

  return alerts
})

const tabs = computed(() => [
  { to: `/patients/${patientId.value}`, label: 'Fiche', exact: true },
  { to: `/patients/${patientId.value}/questionnaire`, label: 'Questionnaire', exact: false },
  { to: `/patients/${patientId.value}/schema`, label: 'Schema', exact: false },
  { to: `/patients/${patientId.value}/paro`, label: 'Paro', exact: false },
  { to: `/patients/${patientId.value}/diagnostic`, label: 'Diagnostic', exact: false },
  { to: `/patients/${patientId.value}/pdts`, label: 'PDTs', exact: false },
  { to: `/patients/${patientId.value}/documents`, label: 'Documents', exact: false },
  { to: `/patients/${patientId.value}/historique`, label: 'Historique', exact: false },
])

function isTabActive(tab: { to: string; exact: boolean }): boolean {
  if (tab.exact) {
    return route.path === tab.to || route.path === `${tab.to}/`
  }
  return route.path.startsWith(tab.to)
}

provide('patient', patient)

onMounted(async () => {
  try {
    const response = await getPatient(patientId.value)
    if (response.success && response.data) {
      patient.value = response.data
    } else {
      error.value = response.error ?? 'Patient introuvable'
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Erreur lors du chargement'
  } finally {
    loading.value = false
  }

  // Load questionnaire for medical alerts (non-blocking)
  try {
    const qRes = await getQuestionnaire(patientId.value)
    if (qRes.success && qRes.data) {
      questionnaire.value = qRes.data
    }
  } catch {
    // Questionnaire may not exist yet
  }
})
</script>
