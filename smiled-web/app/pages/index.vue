<template>
  <div class="p-8">
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-2xl font-bold">Tableau de bord</h1>
      <p class="text-muted-foreground mt-1">
        Bienvenue, {{ userName }} — {{ todayFormatted }}
      </p>
    </div>

    <!-- Stats cards -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
      <Card>
        <CardHeader class="flex flex-row items-center justify-between pb-2">
          <CardTitle class="text-sm font-medium text-muted-foreground">
            Patients
          </CardTitle>
          <Icon name="lucide:users" class="w-4 h-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <p class="text-2xl font-bold">{{ patientCount ?? '...' }}</p>
          <p class="text-xs text-muted-foreground mt-1">Total patients enregistrés</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between pb-2">
          <CardTitle class="text-sm font-medium text-muted-foreground">
            Consultations
          </CardTitle>
          <Icon name="lucide:calendar" class="w-4 h-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <p class="text-2xl font-bold">—</p>
          <p class="text-xs text-muted-foreground mt-1">Bientôt disponible</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between pb-2">
          <CardTitle class="text-sm font-medium text-muted-foreground">
            Plans de traitement
          </CardTitle>
          <Icon name="lucide:clipboard-check" class="w-4 h-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <p class="text-2xl font-bold">—</p>
          <p class="text-xs text-muted-foreground mt-1">Bientôt disponible</p>
        </CardContent>
      </Card>

      <Card>
        <CardHeader class="flex flex-row items-center justify-between pb-2">
          <CardTitle class="text-sm font-medium text-muted-foreground">
            Documents
          </CardTitle>
          <Icon name="lucide:file-text" class="w-4 h-4 text-muted-foreground" />
        </CardHeader>
        <CardContent>
          <p class="text-2xl font-bold">—</p>
          <p class="text-xs text-muted-foreground mt-1">Bientôt disponible</p>
        </CardContent>
      </Card>
    </div>

    <!-- Quick actions + Recent patients -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <Card>
        <CardHeader>
          <CardTitle>Accès rapide</CardTitle>
          <CardDescription>Actions fréquentes</CardDescription>
        </CardHeader>
        <CardContent class="flex flex-col gap-3">
          <NuxtLink to="/patients">
            <Button class="w-full justify-start" variant="outline">
              <Icon name="lucide:user-plus" class="w-4 h-4 mr-2" />
              Nouveau patient
            </Button>
          </NuxtLink>
          <NuxtLink to="/patients">
            <Button class="w-full justify-start" variant="outline">
              <Icon name="lucide:search" class="w-4 h-4 mr-2" />
              Rechercher un patient
            </Button>
          </NuxtLink>
        </CardContent>
      </Card>

      <Card>
        <CardHeader>
          <CardTitle>Patients récents</CardTitle>
          <CardDescription>Derniers patients consultés</CardDescription>
        </CardHeader>
        <CardContent>
          <div v-if="recentPatients.length === 0" class="text-center py-4 text-muted-foreground text-sm">
            Aucun patient enregistré
          </div>
          <div v-else class="space-y-2">
            <NuxtLink
              v-for="p in recentPatients"
              :key="p.id"
              :to="`/patients/${p.id}`"
              class="flex items-center justify-between p-2 rounded-md hover:bg-muted transition-colors"
            >
              <div class="flex items-center gap-3">
                <div class="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center text-xs font-medium text-primary">
                  {{ p.prenom.charAt(0) }}{{ p.nom.charAt(0) }}
                </div>
                <div>
                  <p class="text-sm font-medium">{{ p.nom }} {{ p.prenom }}</p>
                  <p class="text-xs text-muted-foreground">{{ formatDate(p.date_naissance) }}</p>
                </div>
              </div>
              <Icon name="lucide:chevron-right" class="w-4 h-4 text-muted-foreground" />
            </NuxtLink>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Patient } from '~/types/patient'
import { formatDate } from '~/utils/format'

const authStore = useAuthStore()
const { listPatients } = usePatient()

const patientCount = ref<number | null>(null)
const recentPatients = ref<Patient[]>([])

const userName = computed(() => {
  const user = authStore.user
  if (!user) return ''
  return `${user.prenom} ${user.nom}`
})

const todayFormatted = computed(() => {
  return new Date().toLocaleDateString('fr-FR', {
    weekday: 'long',
    year: 'numeric',
    month: 'long',
    day: 'numeric',
  })
})

onMounted(async () => {
  try {
    const res = await listPatients({ limit: 5 })
    if (res.success && res.data) {
      recentPatients.value = res.data
      patientCount.value = res.meta?.total ?? res.data.length
    }
  } catch {
    patientCount.value = null
  }
})
</script>
