<template>
  <div class="p-8">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold">Cabinet</h1>
        <p class="text-muted-foreground mt-1">Informations et paramètres du cabinet</p>
      </div>
    </div>

    <!-- Tabs -->
    <Tabs v-model="activeTab">
      <TabsList>
        <TabsTrigger value="infos">Informations</TabsTrigger>
        <TabsTrigger value="users" @click="navigateTo('/cabinet/users')">
          Utilisateurs
        </TabsTrigger>
      </TabsList>

      <TabsContent value="infos" class="mt-6">
        <div v-if="loading" class="flex items-center justify-center h-32">
          <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
        </div>

        <form v-else class="space-y-6 max-w-2xl" @submit.prevent="handleSave">
          <Card>
            <CardHeader>
              <CardTitle class="text-base">Identité du cabinet</CardTitle>
            </CardHeader>
            <CardContent class="space-y-4">
              <div class="space-y-2">
                <Label for="nom_cabinet">Nom du cabinet *</Label>
                <Input
                  id="nom_cabinet"
                  v-model="form.nom"
                  required
                  :disabled="!editing"
                  placeholder="Ex : Cabinet Dentaire Dr Martin"
                />
              </div>

              <div class="space-y-2">
                <Label for="adresse_cabinet">Adresse</Label>
                <Textarea
                  id="adresse_cabinet"
                  v-model="form.adresse"
                  :disabled="!editing"
                  :rows="2"
                  placeholder="12 rue de la Paix, 75001 Paris"
                />
              </div>

              <div class="grid grid-cols-2 gap-4">
                <div class="space-y-2">
                  <Label for="telephone_cabinet">Téléphone</Label>
                  <Input
                    id="telephone_cabinet"
                    v-model="form.telephone"
                    :disabled="!editing"
                    placeholder="01 23 45 67 89"
                  />
                </div>
                <div class="space-y-2">
                  <Label for="email_cabinet">Email</Label>
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

          <Card>
            <CardHeader>
              <CardTitle class="text-base">Identifiants administratifs</CardTitle>
            </CardHeader>
            <CardContent class="grid grid-cols-2 gap-4">
              <div class="space-y-2">
                <Label for="siret">SIRET</Label>
                <Input
                  id="siret"
                  v-model="form.siret"
                  :disabled="!editing"
                  placeholder="123 456 789 00012"
                />
              </div>
              <div class="space-y-2">
                <Label for="finess">FINESS</Label>
                <Input
                  id="finess"
                  v-model="form.finess"
                  :disabled="!editing"
                  placeholder="75 000 0000"
                />
              </div>
            </CardContent>
          </Card>

          <Alert v-if="saveError" variant="destructive">
            <AlertDescription>{{ saveError }}</AlertDescription>
          </Alert>
          <Alert v-if="saveSuccess">
            <Icon name="lucide:check-circle" class="w-4 h-4" />
            <AlertDescription>Informations enregistrées.</AlertDescription>
          </Alert>

          <div class="flex gap-3">
            <Button v-if="!editing" type="button" variant="outline" @click="editing = true">
              <Icon name="lucide:pencil" class="w-4 h-4 mr-2" />
              Modifier
            </Button>
            <template v-else>
              <Button type="button" variant="outline" @click="cancelEditing">Annuler</Button>
              <Button type="submit" :disabled="saving">
                <Icon v-if="saving" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
                Enregistrer
              </Button>
            </template>
          </div>
        </form>
      </TabsContent>
    </Tabs>
  </div>
</template>

<script setup lang="ts">
import type { Cabinet } from '~/composables/useCabinet'

definePageMeta({ layout: 'default' })

const { getCabinet, updateCabinet } = useCabinet()

const cabinet = ref<Cabinet | null>(null)
const loading = ref(true)
const editing = ref(false)
const saving = ref(false)
const saveError = ref<string | null>(null)
const saveSuccess = ref(false)
const activeTab = ref('infos')

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
