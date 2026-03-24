<template>
  <div class="p-8">
    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-64">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
    </div>

    <!-- Error -->
    <Alert v-else-if="error" variant="destructive">
      <AlertDescription>{{ error }}</AlertDescription>
    </Alert>

    <template v-else-if="patient">
      <!-- Header -->
      <div class="flex items-start justify-between mb-6">
        <div class="flex items-center gap-4">
          <Button variant="ghost" size="sm" @click="navigateTo('/patients')">
            <Icon name="lucide:arrow-left" class="w-4 h-4 mr-2" />
            Retour
          </Button>
          <Separator orientation="vertical" class="h-6" />
          <div>
            <h1 class="text-2xl font-bold">
              {{ patient.nom }} {{ patient.prenom }}
            </h1>
            <div class="flex items-center gap-3 mt-1">
              <span class="text-muted-foreground text-sm">
                {{ patient.sexe === 'M' ? 'M.' : 'Mme' }}
                · {{ age }} ans
                · né{{ patient.sexe === 'F' ? 'e' : '' }} le {{ formatDate(patient.date_naissance) }}
              </span>
              <Badge :variant="couvertureBadgeVariant(patient.couverture)">
                {{ couvertureLabel(patient.couverture) }}
              </Badge>
            </div>
          </div>
        </div>
      </div>

      <!-- Medical Alert Banner -->
      <div
        v-if="medicalAlerts.length > 0"
        class="mb-6 flex items-center gap-3 rounded-md border border-amber-200 bg-amber-50 px-4 py-3 text-amber-800"
      >
        <Icon name="lucide:alert-triangle" class="w-5 h-5 shrink-0" />
        <div class="flex flex-wrap items-center gap-2 text-sm font-medium">
          <span
            v-for="alert in medicalAlerts"
            :key="alert"
            class="rounded-full bg-amber-200/60 px-2.5 py-0.5 text-xs font-semibold"
          >
            {{ alert }}
          </span>
        </div>
      </div>

      <!-- Tab navigation -->
      <nav class="flex gap-1 border-b mb-6">
        <NuxtLink
          v-for="tab in tabs"
          :key="tab.to"
          :to="tab.to"
          :exact="tab.exact"
          class="px-4 py-2 text-sm font-medium text-muted-foreground hover:text-foreground border-b-2 border-transparent transition-colors -mb-px"
          :class="isTabActive(tab) ? 'text-foreground border-primary' : ''"
        >
          {{ tab.label }}
        </NuxtLink>
      </nav>

      <!-- Child route -->
      <NuxtPage />
    </template>
  </div>
</template>

<script setup lang="ts">
import type { Patient, Questionnaire } from '~/types/patient'
import { formatDate } from '~/utils/format'
import { couvertureLabel, couvertureBadgeVariant } from '~/utils/display'

const route = useRoute()
const patientId = computed(() => route.params.id as string)

const { getPatient, getQuestionnaire } = usePatient()

const patient = ref<Patient | null>(null)
const questionnaire = ref<Questionnaire | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)

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
  if (radio?.actif) alerts.push('Radiothérapie')

  return alerts
})

const tabs = computed(() => [
  { to: `/patients/${patientId.value}`, label: 'Fiche', exact: true },
  { to: `/patients/${patientId.value}/questionnaire`, label: 'Questionnaire', exact: false },
  { to: `/patients/${patientId.value}/schema`, label: 'Schéma dentaire', exact: false },
  { to: `/patients/${patientId.value}/paro`, label: 'Paro', exact: false },
  { to: `/patients/${patientId.value}/diagnostic`, label: 'Diagnostic', exact: false },
  { to: `/patients/${patientId.value}/pdts`, label: 'Plans de traitement', exact: false },
  { to: `/patients/${patientId.value}/documents`, label: 'Documents', exact: false },
  { to: `/patients/${patientId.value}/historique`, label: 'Historique', exact: false },
])

const age = computed(() => {
  if (!patient.value) return 0
  const today = new Date()
  const birthDate = new Date(patient.value.date_naissance)
  let years = today.getFullYear() - birthDate.getFullYear()
  const monthDiff = today.getMonth() - birthDate.getMonth()
  if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birthDate.getDate())) {
    years--
  }
  return years
})

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
    // Questionnaire may not exist yet — no alert to show
  }
})
</script>
