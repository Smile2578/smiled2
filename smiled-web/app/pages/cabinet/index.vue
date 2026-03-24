<template>
  <div class="max-w-4xl">
    <!-- Page Header -->
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-semibold tracking-tight">Parametres du cabinet</h1>
        <p class="text-sm text-muted-foreground mt-1">Gerez les informations et la configuration de votre cabinet</p>
      </div>
      <div v-if="!loading">
        <Button v-if="!editing" variant="outline" @click="editing = true">
          <PencilIcon class="w-4 h-4 mr-2" />
          Modifier
        </Button>
        <div v-else class="flex items-center gap-2">
          <Button variant="outline" @click="cancelEditing">Annuler</Button>
          <Button class="bg-teal-600 hover:bg-teal-700 text-white" :disabled="saving" @click="handleSave">
            <Loader2 v-if="saving" class="w-4 h-4 mr-2 animate-spin" />
            Enregistrer
          </Button>
        </div>
      </div>
    </div>

    <!-- Navigation Tabs -->
    <Tabs default-value="info" class="mb-8">
      <TabsList>
        <TabsTrigger value="info">Informations</TabsTrigger>
        <TabsTrigger value="team" @click="navigateTo('/cabinet/users')">Equipe</TabsTrigger>
      </TabsList>
    </Tabs>

    <!-- Loading Skeleton -->
    <div v-if="loading" class="space-y-6">
      <Card>
        <CardHeader class="pb-4">
          <div class="flex items-center gap-3">
            <Skeleton class="w-9 h-9 rounded-lg" />
            <div class="space-y-1.5">
              <Skeleton class="h-4 w-40" />
              <Skeleton class="h-3 w-56" />
            </div>
          </div>
        </CardHeader>
        <CardContent class="space-y-4">
          <Skeleton class="h-9 w-full" />
          <Skeleton class="h-16 w-full" />
          <div class="grid grid-cols-2 gap-4">
            <Skeleton class="h-9" />
            <Skeleton class="h-9" />
          </div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader class="pb-4">
          <div class="flex items-center gap-3">
            <Skeleton class="w-9 h-9 rounded-lg" />
            <div class="space-y-1.5">
              <Skeleton class="h-4 w-48" />
              <Skeleton class="h-3 w-64" />
            </div>
          </div>
        </CardHeader>
        <CardContent class="grid grid-cols-2 gap-4">
          <Skeleton class="h-9" />
          <Skeleton class="h-9" />
        </CardContent>
      </Card>
    </div>

    <form v-else class="space-y-6" @submit.prevent="handleSave">
      <!-- Informations generales -->
      <Card>
        <CardHeader class="pb-4">
          <div class="flex items-center gap-3">
            <div class="w-9 h-9 rounded-lg bg-teal-50 flex items-center justify-center">
              <Building2 class="w-5 h-5 text-teal-600" />
            </div>
            <div>
              <CardTitle class="text-base font-semibold">Informations generales</CardTitle>
              <p class="text-xs text-muted-foreground">Identite et coordonnees du cabinet</p>
            </div>
          </div>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="space-y-2">
            <Label for="nom_cabinet" class="text-sm font-medium">Nom du cabinet *</Label>
            <Input
              id="nom_cabinet"
              v-model="form.nom"
              required
              :disabled="!editing"
              placeholder="Ex : Cabinet Dentaire Dr Martin"
            />
          </div>

          <div class="space-y-2">
            <Label for="adresse_cabinet" class="text-sm font-medium">Adresse</Label>
            <Textarea
              id="adresse_cabinet"
              v-model="form.adresse"
              :disabled="!editing"
              :rows="2"
              placeholder="12 rue de la Paix, 75001 Paris"
            />
          </div>

          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="telephone_cabinet" class="text-sm font-medium">Telephone</Label>
              <Input
                id="telephone_cabinet"
                v-model="form.telephone"
                :disabled="!editing"
                placeholder="01 23 45 67 89"
              />
            </div>
            <div class="space-y-2">
              <Label for="email_cabinet" class="text-sm font-medium">Email</Label>
              <Input
                id="email_cabinet"
                v-model="form.email"
                type="email"
                :disabled="!editing"
                placeholder="contact@cabinet.fr"
              />
            </div>
          </div>
        </CardContent>
      </Card>

      <!-- Identifiants administratifs -->
      <Card>
        <CardHeader class="pb-4">
          <div class="flex items-center gap-3">
            <div class="w-9 h-9 rounded-lg bg-blue-50 flex items-center justify-center">
              <ShieldCheck class="w-5 h-5 text-blue-600" />
            </div>
            <div>
              <CardTitle class="text-base font-semibold">Identifiants administratifs</CardTitle>
              <p class="text-xs text-muted-foreground">SIRET, FINESS et numeros reglementaires</p>
            </div>
          </div>
        </CardHeader>
        <CardContent class="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div class="space-y-2">
            <Label for="siret" class="text-sm font-medium">SIRET</Label>
            <Input
              id="siret"
              v-model="form.siret"
              :disabled="!editing"
              placeholder="123 456 789 00012"
            />
          </div>
          <div class="space-y-2">
            <Label for="finess" class="text-sm font-medium">FINESS</Label>
            <Input
              id="finess"
              v-model="form.finess"
              :disabled="!editing"
              placeholder="75 000 0000"
            />
          </div>
        </CardContent>
      </Card>

      <!-- Alerts -->
      <Alert v-if="saveError" variant="destructive">
        <AlertDescription>{{ saveError }}</AlertDescription>
      </Alert>
      <div
        v-if="saveSuccess"
        class="flex items-center gap-2 p-3 rounded-lg bg-emerald-50 border border-emerald-200 text-emerald-700 text-sm"
      >
        <CheckCircle class="w-4 h-4" />
        <span>Informations enregistrees avec succes.</span>
      </div>
    </form>
  </div>
</template>

<script setup lang="ts">
import { Building2, CheckCircle, Loader2, Pencil as PencilIcon, ShieldCheck } from 'lucide-vue-next'
import type { Cabinet } from '~/composables/useCabinet'

definePageMeta({ layout: 'default' })

const { getCabinet, updateCabinet } = useCabinet()

const cabinet = ref<Cabinet | null>(null)
const loading = ref(true)
const editing = ref(false)
const saving = ref(false)
const saveError = ref<string | null>(null)
const saveSuccess = ref(false)

const form = reactive({
  nom: '',
  adresse: '',
  telephone: '',
  email: '',
  siret: '',
  finess: '',
})

function syncForm(c: Cabinet): void {
  form.nom = c.nom
  form.adresse = c.adresse ?? ''
  form.telephone = c.telephone ?? ''
  form.email = c.email ?? ''
  form.siret = c.siret ?? ''
  form.finess = c.finess ?? ''
}

function cancelEditing(): void {
  if (cabinet.value) syncForm(cabinet.value)
  editing.value = false
  saveError.value = null
}

async function handleSave(): Promise<void> {
  saving.value = true
  saveError.value = null
  saveSuccess.value = false

  try {
    const response = await updateCabinet({
      nom: form.nom,
      adresse: form.adresse || null,
      telephone: form.telephone || null,
      email: form.email || null,
      siret: form.siret || null,
      finess: form.finess || null,
    })

    if (response.success && response.data) {
      cabinet.value = response.data
      editing.value = false
      saveSuccess.value = true
      setTimeout(() => { saveSuccess.value = false }, 3000)
    } else {
      saveError.value = response.error ?? 'Erreur lors de la sauvegarde'
    }
  } catch (err) {
    saveError.value = err instanceof Error ? err.message : 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

onMounted(async () => {
  try {
    const response = await getCabinet()
    if (response.success && response.data) {
      cabinet.value = response.data
      syncForm(response.data)
    }
  } catch {
    // ignore
  } finally {
    loading.value = false
  }
})
</script>
