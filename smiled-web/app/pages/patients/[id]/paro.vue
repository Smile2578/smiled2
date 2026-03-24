<template>
  <div>
    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-64">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
    </div>

    <template v-else>
      <!-- Paro Global Card -->
      <Card class="mb-6">
        <CardHeader>
          <CardTitle class="text-base">Classification parodontale</CardTitle>
          <CardDescription>AAP/EFP 2017 — Staging & Grading</CardDescription>
        </CardHeader>
        <CardContent>
          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div class="space-y-2">
              <Label for="staging">Staging</Label>
              <Select v-model="globalForm.staging">
                <SelectTrigger id="staging">
                  <SelectValue placeholder="Non renseigné" />
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
                  <SelectValue placeholder="Non renseigné" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="A">Grade A (lent)</SelectItem>
                  <SelectItem value="B">Grade B (modéré)</SelectItem>
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
              placeholder="Observations générales..."
              :rows="2"
            />
          </div>

          <div class="flex justify-end mt-4">
            <Button :disabled="savingGlobal" @click="handleSaveGlobal">
              <Icon v-if="savingGlobal" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
              <Icon v-else name="lucide:save" class="w-4 h-4 mr-2" />
              Enregistrer classification
            </Button>
          </div>
        </CardContent>
      </Card>

      <!-- Paro Sites Grid -->
      <Card>
        <CardHeader class="flex flex-row items-center justify-between">
          <div>
            <CardTitle class="text-base">Relevé parodontal</CardTitle>
            <CardDescription>6 sites par dent (MB, B, DB côté vestibulaire / ML, L, DL côté lingual)</CardDescription>
          </div>
          <Button :disabled="savingSites" @click="handleSaveSites">
            <Icon v-if="savingSites" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
            <Icon v-else name="lucide:save" class="w-4 h-4 mr-2" />
            Enregistrer le relevé
          </Button>
        </CardHeader>
        <CardContent>
          <Alert v-if="error" variant="destructive" class="mb-4">
            <AlertDescription>{{ error }}</AlertDescription>
          </Alert>
          <Alert v-if="saveSuccess" class="mb-4">
            <Icon name="lucide:check-circle" class="w-4 h-4" />
            <AlertDescription>Relevé enregistré avec succès.</AlertDescription>
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
</script>
