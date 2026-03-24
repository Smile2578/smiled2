<template>
  <div>
    <!-- Toolbar -->
    <div class="flex items-center justify-between mb-4">
      <p class="text-xs text-gray-500">Cliquez sur une dent pour la modifier</p>

      <div class="flex items-center gap-3">
        <!-- Version selector -->
        <div v-if="versions.length > 0" class="flex items-center gap-2">
          <label class="text-xs text-gray-500">Version :</label>
          <select
            v-model.number="selectedVersion"
            class="rounded-md border border-gray-300 bg-white px-2 py-1.5 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
            @change="handleVersionChange"
          >
            <option v-for="v in versions" :key="v.id" :value="v.version">
              v{{ v.version }} — {{ formatDate(v.created_at) }}
            </option>
          </select>
        </div>

        <!-- Nouvelle version button -->
        <button
          class="flex items-center gap-2 rounded-md bg-blue-600 text-white px-3 py-1.5 text-sm font-medium hover:bg-blue-700 transition-colors disabled:opacity-50"
          :disabled="loading"
          @click="showNewVersionModal = true"
        >
          <Icon name="lucide:plus" class="w-4 h-4" />
          Nouvelle version
        </button>
      </div>
    </div>

    <!-- Error banner -->
    <div v-if="error" class="mt-4 p-3 bg-red-50 border border-red-200 rounded-md text-sm text-red-600">
      {{ error }}
    </div>

    <!-- Loading state -->
    <div v-if="loading" class="flex items-center justify-center h-64">
      <div class="flex items-center gap-3 text-gray-400">
        <Icon name="lucide:loader-2" class="w-5 h-5 animate-spin" />
        <span>Chargement du schéma...</span>
      </div>
    </div>

    <!-- No schema state -->
    <div v-else-if="!schema && !loading" class="flex flex-col items-center justify-center gap-4 text-gray-400 py-16">
      <Icon name="lucide:scan-face" class="w-16 h-16 opacity-30" />
      <p class="text-base">Aucun schéma dentaire pour ce patient</p>
      <button
        class="flex items-center gap-2 rounded-md bg-blue-600 text-white px-4 py-2 text-sm font-medium hover:bg-blue-700 transition-colors"
        @click="showNewVersionModal = true"
      >
        <Icon name="lucide:plus" class="w-4 h-4" />
        Créer le schéma initial
      </button>
    </div>

    <!-- Chart + panel layout -->
    <div v-else-if="schema" class="flex overflow-hidden">
      <!-- Main chart area -->
      <div class="flex-1 flex flex-col overflow-auto gap-4">
        <!-- Dentition badge -->
        <div class="flex items-center gap-2">
          <span class="inline-flex items-center rounded-full px-2.5 py-1 text-xs font-medium bg-blue-50 text-blue-700 border border-blue-200">
            {{ dentitionLabel }}
          </span>
          <span class="text-xs text-gray-400">Version {{ schema.schema.version }}</span>
        </div>

        <!-- Canvas chart -->
        <div class="overflow-x-auto">
          <DentalChart
            :teeth="schema.dents"
            :selected-fdi="selection.fdi"
            :selected-face="selection.face"
            @tooth-selected="handleToothSelected"
          />
        </div>

        <!-- Legend -->
        <ChartLegend />

        <!-- Occlusion / ATM summary (compact) -->
        <div v-if="schema.atm || schema.occlusion" class="flex gap-4 flex-wrap">
          <div v-if="schema.atm" class="flex items-center gap-2 text-xs text-gray-500 bg-gray-50 rounded px-3 py-1.5 border">
            <Icon name="lucide:activity" class="w-3.5 h-3.5" />
            ATM :
            <span v-if="schema.atm.bruxisme" class="text-amber-600 font-medium">Bruxisme</span>
            <span v-if="schema.atm.douleur_droite || schema.atm.douleur_gauche" class="text-red-600 font-medium">Douleurs</span>
            <span v-if="!schema.atm.bruxisme && !schema.atm.douleur_droite && !schema.atm.douleur_gauche">RAS</span>
          </div>
          <div v-if="schema.occlusion" class="flex items-center gap-2 text-xs text-gray-500 bg-gray-50 rounded px-3 py-1.5 border">
            <Icon name="lucide:align-center" class="w-3.5 h-3.5" />
            Classe : {{ schema.occlusion.classe_angle ?? 'Non renseignée' }}
          </div>
        </div>
      </div>

      <!-- Right sidebar -->
      <ToothDetailPanel
        :selected-entry="selectedEntry"
        :selected-face="selection.face"
        :saving="saving"
        :save-error="saveError"
        :save-success="saveSuccess"
        @close="clearSelection"
        @save="handleSave"
      />
    </div>
  </div>

  <!-- New version modal -->
  <Teleport to="body">
    <div
      v-if="showNewVersionModal"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/40"
      @click.self="showNewVersionModal = false"
    >
      <div class="bg-white rounded-xl shadow-xl p-6 w-full max-w-sm">
        <h2 class="text-base font-semibold mb-4">Nouvelle version du schéma</h2>

        <div class="space-y-3">
          <label class="text-sm text-gray-600">Type de dentition</label>
          <select
            v-model="newDentition"
            class="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="permanente">Permanente (adulte)</option>
            <option value="temporaire">Temporaire (enfant)</option>
            <option value="mixte">Mixte</option>
          </select>
        </div>

        <div class="flex gap-3 mt-6">
          <button
            class="flex-1 rounded-md border border-gray-300 px-4 py-2 text-sm hover:bg-gray-50 transition-colors"
            @click="showNewVersionModal = false"
          >
            Annuler
          </button>
          <button
            class="flex-1 rounded-md bg-blue-600 text-white px-4 py-2 text-sm font-medium hover:bg-blue-700 transition-colors disabled:opacity-50"
            :disabled="loading"
            @click="handleCreateVersion"
          >
            Créer
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import type { FaceName, ToothEntry, ToothStatus, FaceState } from '~/components/dental-chart/types'
import { TOOTH_STATUS_LABELS } from '~/components/dental-chart/types'
import DentalChart from '~/components/dental-chart/DentalChart.vue'
import ToothDetailPanel from '~/components/dental-chart/ToothDetailPanel.vue'
import ChartLegend from '~/components/dental-chart/ChartLegend.vue'

const route = useRoute()
const patientId = computed(() => route.params.id as string)

const { schema, versions, loading, error, loadSchema, loadVersions, createNewVersion, updateTooth } =
  useDentalChart(patientId)

const selectedVersion = ref<number | null>(null)
const showNewVersionModal = ref(false)
const newDentition = ref<'permanente' | 'temporaire' | 'mixte'>('permanente')

const selection = ref<{ fdi: number | null; face: FaceName | null }>({ fdi: null, face: null })
const saving = ref(false)
const saveError = ref<string | null>(null)
const saveSuccess = ref(false)

const dentitionLabel = computed(() => {
  const map: Record<string, string> = {
    permanente: 'Dentition permanente',
    temporaire: 'Dentition temporaire',
    mixte: 'Dentition mixte',
  }
  return map[schema.value?.schema.dentition ?? ''] ?? ''
})

const selectedEntry = computed<ToothEntry | null>(() => {
  if (!selection.value.fdi || !schema.value) return null
  return schema.value.dents.find((e) => e.tooth.numero_fdi === selection.value.fdi) ?? null
})

function handleToothSelected(fdi: number, face: FaceName | null): void {
  // Toggle selection off if clicking the same tooth+face
  if (selection.value.fdi === fdi && selection.value.face === face) {
    selection.value = { fdi: null, face: null }
    return
  }
  selection.value = { fdi, face }
  saveError.value = null
  saveSuccess.value = false
}

function clearSelection(): void {
  selection.value = { fdi: null, face: null }
}

async function handleSave(
  fdi: number,
  statut: ToothStatus,
  faceEtat: FaceState | null,
  mobilite: number,
  endoTraite: boolean,
  protheseType: string,
  notes: string,
): Promise<void> {
  saving.value = true
  saveError.value = null
  saveSuccess.value = false

  try {
    const data: Record<string, unknown> = {
      statut,
      paro_mobilite: mobilite,
      notes: notes || null,
      endo: endoTraite ? { traite: true } : null,
      prothese_fixe: protheseType ? { type: protheseType } : null,
    }

    // If a face state change was made, include face updates
    if (faceEtat && selection.value.face && schema.value) {
      const entry = schema.value.dents.find((e) => e.tooth.numero_fdi === fdi)
      if (entry) {
        const updatedFaces = entry.faces.map((f) =>
          f.face === selection.value.face ? { ...f, etat: faceEtat } : f,
        )
        data.faces = updatedFaces
      }
    }

    await updateTooth(fdi, data)
    saveSuccess.value = true
    setTimeout(() => {
      saveSuccess.value = false
    }, 2500)
  } catch {
    saveError.value = 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

async function handleVersionChange(): Promise<void> {
  if (selectedVersion.value !== null) {
    clearSelection()
    await loadSchema(selectedVersion.value)
  }
}

async function handleCreateVersion(): Promise<void> {
  showNewVersionModal.value = false
  await createNewVersion(newDentition.value)
  if (schema.value) {
    selectedVersion.value = schema.value.schema.version
  }
}

function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString('fr-FR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
  })
}

// Initial load
onMounted(async () => {
  await loadVersions()
  if (versions.value.length > 0) {
    selectedVersion.value = versions.value[versions.value.length - 1]?.version ?? null
    await loadSchema()
  }
})
</script>
