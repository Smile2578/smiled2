<template>
  <div>
    <!-- Header -->
    <div class="mb-6 flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold text-foreground">Parodontologie</h2>
        <p class="text-sm text-muted-foreground">
          Classification AAP/EFP 2017 et releve parodontal
        </p>
      </div>
      <div class="flex items-center gap-2 text-sm">
        <template v-if="autoSaving">
          <Icon name="lucide:loader-2" class="h-3.5 w-3.5 animate-spin text-muted-foreground" />
          <span class="text-muted-foreground">Sauvegarde...</span>
        </template>
        <template v-else-if="autoSaveError">
          <Icon name="lucide:alert-circle" class="h-3.5 w-3.5 text-destructive" />
          <span class="text-destructive">{{ autoSaveError }}</span>
        </template>
        <template v-else-if="autoSaveLastSaved">
          <Icon name="lucide:check-circle" class="h-3.5 w-3.5 text-green-600" />
          <span class="text-muted-foreground">
            Sauvegarde a {{ formatTime(autoSaveLastSaved) }}
          </span>
        </template>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="space-y-6">
      <Card>
        <CardHeader>
          <Skeleton class="h-5 w-48" />
          <Skeleton class="h-4 w-64" />
        </CardHeader>
        <CardContent class="grid grid-cols-4 gap-4">
          <div v-for="i in 4" :key="i" class="space-y-2">
            <Skeleton class="h-4 w-20" />
            <Skeleton class="h-10 w-full" />
          </div>
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <Skeleton class="h-5 w-40" />
        </CardHeader>
        <CardContent>
          <Skeleton class="h-64 w-full" />
        </CardContent>
      </Card>
    </div>

    <template v-else>
      <!-- Paro Global Card -->
      <Card class="mb-6 shadow-sm">
        <CardHeader class="pb-4">
          <div class="flex items-center gap-2">
            <Icon name="lucide:activity" class="h-4 w-4 text-muted-foreground" />
            <CardTitle class="text-base">Classification parodontale</CardTitle>
          </div>
          <CardDescription>Staging & Grading</CardDescription>
        </CardHeader>
        <CardContent>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-4">
            <div class="space-y-2">
              <Label for="staging">Staging</Label>
              <Select v-model="globalForm.staging">
                <SelectTrigger id="staging">
                  <SelectValue placeholder="Non renseigne" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="I">Stade I</SelectItem>
                  <SelectItem value="II">Stade II</SelectItem>
                  <SelectItem value="III">Stade III</SelectItem>
                  <SelectItem value="IV">Stade IV</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div class="space-y-2">
              <Label for="grading">Grading</Label>
              <Select v-model="globalForm.grading">
                <SelectTrigger id="grading">
                  <SelectValue placeholder="Non renseigne" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="A">Grade A (lent)</SelectItem>
                  <SelectItem value="B">Grade B (modere)</SelectItem>
                  <SelectItem value="C">Grade C (rapide)</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div class="space-y-2">
              <Label for="indice_plaque">Indice de plaque (%)</Label>
              <Input
                id="indice_plaque"
                v-model.number="globalForm.indice_plaque_pct"
                type="number"
                min="0"
                max="100"
                step="1"
                placeholder="Ex : 35"
              />
            </div>

            <div class="space-y-2">
              <Label for="bop_pct">BOP global (%)</Label>
              <Input
                id="bop_pct"
                v-model.number="globalForm.bop_pct"
                type="number"
                min="0"
                max="100"
                step="1"
                placeholder="Ex : 20"
              />
            </div>
          </div>

          <div class="mt-4 space-y-2">
            <Label for="global_notes">Notes cliniques</Label>
            <Textarea
              id="global_notes"
              v-model="globalForm.notes"
              placeholder="Observations generales..."
              :rows="2"
            />
          </div>

          <div class="flex justify-end mt-4">
            <Button
              :disabled="savingGlobal"
              class="bg-primary hover:bg-primary/90 text-primary-foreground"
              @click="handleSaveGlobal"
            >
              <Icon v-if="savingGlobal" name="lucide:loader-2" class="mr-2 h-4 w-4 animate-spin" />
              <Icon v-else name="lucide:save" class="mr-2 h-4 w-4" />
              Enregistrer classification
            </Button>
          </div>
        </CardContent>
      </Card>

      <!-- Paro Sites Grid -->
      <Card class="shadow-sm">
        <CardHeader class="flex flex-row items-center justify-between">
          <div>
            <div class="flex items-center gap-2">
              <Icon name="lucide:grid-3x3" class="h-4 w-4 text-muted-foreground" />
              <CardTitle class="text-base">Releve parodontal</CardTitle>
            </div>
            <CardDescription>
              6 sites par dent (MB, B, DB cote vestibulaire / ML, L, DL cote lingual)
            </CardDescription>
          </div>
          <Button
            :disabled="savingSites"
            class="bg-primary hover:bg-primary/90 text-primary-foreground"
            @click="handleSaveSites"
          >
            <Icon v-if="savingSites" name="lucide:loader-2" class="mr-2 h-4 w-4 animate-spin" />
            <Icon v-else name="lucide:save" class="mr-2 h-4 w-4" />
            Enregistrer le releve
          </Button>
        </CardHeader>
        <CardContent>
          <Alert v-if="error" variant="destructive" class="mb-4">
            <AlertDescription>{{ error }}</AlertDescription>
          </Alert>
          <Alert v-if="saveSuccess" class="mb-4">
            <Icon name="lucide:check-circle" class="h-4 w-4" />
            <AlertDescription>Releve enregistre avec succes.</AlertDescription>
          </Alert>

          <ParoChart :sites="paroSites" @update="handleSitesUpdate" />
        </CardContent>
      </Card>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { ParoSite, ParoSiteUpdate } from '~/composables/useParo'
import ParoChart from '~/components/paro/ParoChart.vue'

function formatTime(date: Date): string {
  return date.toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' })
}

const route = useRoute()
const patientId = route.params.id as string

const { getParoSites, updateParoSites, getParoGlobal, updateParoGlobal } = useParo()

const paroSites = ref<ParoSite[]>([])
const loading = ref(true)
const error = ref<string | null>(null)
const savingSites = ref(false)
const savingGlobal = ref(false)
const saveSuccess = ref(false)

const pendingSiteUpdates = ref<ParoSiteUpdate[]>([])

const globalForm = reactive({
  staging: '' as string,
  grading: '' as string,
  indice_plaque_pct: null as number | null,
  bop_pct: null as number | null,
  notes: '',
})

function handleSitesUpdate(sites: ParoSiteUpdate[]): void {
  pendingSiteUpdates.value = sites
}

async function handleSaveSites(): Promise<void> {
  if (pendingSiteUpdates.value.length === 0) return
  savingSites.value = true
  error.value = null
  saveSuccess.value = false

  try {
    const response = await updateParoSites(patientId, pendingSiteUpdates.value)
    if (response.success && response.data) {
      paroSites.value = response.data
      saveSuccess.value = true
      setTimeout(() => { saveSuccess.value = false }, 3000)
    } else {
      error.value = response.error ?? 'Erreur lors de la sauvegarde'
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Erreur lors de la sauvegarde'
  } finally {
    savingSites.value = false
  }
}

async function handleSaveGlobal(): Promise<void> {
  savingGlobal.value = true

  try {
    const payload = {
      staging: globalForm.staging || null,
      grading: globalForm.grading || null,
      indice_plaque_pct: globalForm.indice_plaque_pct,
      bop_pct: globalForm.bop_pct,
      notes: globalForm.notes || null,
    }
    await updateParoGlobal(patientId, payload)
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Erreur lors de la sauvegarde'
  } finally {
    savingGlobal.value = false
  }
}

onMounted(async () => {
  try {
    const [sitesRes, globalRes] = await Promise.all([
      getParoSites(patientId),
      getParoGlobal(patientId),
    ])

    if (sitesRes.success && sitesRes.data) {
      paroSites.value = sitesRes.data
    }

    if (globalRes.success && globalRes.data) {
      const g = globalRes.data
      globalForm.staging = g.staging ?? ''
      globalForm.grading = g.grading ?? ''
      globalForm.indice_plaque_pct = g.indice_plaque_pct
      globalForm.bop_pct = g.bop_pct
      globalForm.notes = g.notes ?? ''
    }
  } catch {
    // First time — no data yet, use defaults
  } finally {
    loading.value = false
  }
})

// Auto-save for global classification form
const globalFormData = computed(() => ({ ...globalForm }))

const { saving: autoSaving, lastSavedAt: autoSaveLastSaved, error: autoSaveError } = useAutoSave(
  globalFormData,
  async (data) => {
    if (loading.value) return
    const payload = {
      staging: data.staging || null,
      grading: data.grading || null,
      indice_plaque_pct: data.indice_plaque_pct,
      bop_pct: data.bop_pct,
      notes: data.notes || null,
    }
    await updateParoGlobal(patientId, payload)
  },
  2000,
)
</script>
