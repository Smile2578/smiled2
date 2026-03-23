<template>
  <div class="p-8">
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-2xl font-bold">Tableau de bord</h1>
      <p class="text-muted-foreground mt-1">
        Bienvenue, {{ userName }}
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
          <p class="text-2xl font-bold">—</p>
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
          <p class="text-xs text-muted-foreground mt-1">Ce mois-ci</p>
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
          <p class="text-xs text-muted-foreground mt-1">En cours</p>
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
          <p class="text-xs text-muted-foreground mt-1">Générés cette semaine</p>
        </CardContent>
      </Card>
    </div>

    <!-- Quick actions -->
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <Card>
        <CardHeader>
          <CardTitle>Accès rapide</CardTitle>
          <CardDescription>Actions fréquentes</CardDescription>
        </CardHeader>
        <CardContent class="flex flex-col gap-3">
          <NuxtLink to="/patients/nouveau">
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
          <CardTitle>À propos</CardTitle>
          <CardDescription>Smiled.IO — Core Clinique</CardDescription>
        </CardHeader>
        <CardContent>
          <p class="text-sm text-muted-foreground">
            Logiciel dentaire moderne. Gérez vos patients, schémas dentaires,
            questionnaires médicaux, diagnostics et plans de traitement.
          </p>
          <p class="text-xs text-muted-foreground mt-4">Version 0.1.0</p>
        </CardContent>
      </Card>
    </div>
  </div>
</template>

<script setup lang="ts">
const authStore = useAuthStore()

const userName = computed(() => {
  const user = authStore.user
  if (!user) return ''
  return `${user.prenom} ${user.nom}`
})
</script>
