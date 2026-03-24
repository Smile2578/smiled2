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
import type { Patient } from '~/types/patient'
import { formatDate } from '~/utils/format'

const route = useRoute()
const patientId = computed(() => route.params.id as string)

const { getPatient } = usePatient()

const patient = ref<Patient | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)

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

function couvertureLabel(couverture: string): string {
  const labels: Record<string, string> = {
    mutuelle: 'Mutuelle',
    cmu_c2s: 'CMU / C2S',
    ame: 'AME',
    aucune: 'Aucune',
  }
  return labels[couverture] ?? couverture
}

function couvertureBadgeVariant(couverture: string): 'default' | 'secondary' | 'destructive' | 'outline' {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    mutuelle: 'default',
    cmu_c2s: 'secondary',
    ame: 'secondary',
    aucune: 'outline',
  }
  return variants[couverture] ?? 'outline'
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
})
</script>
