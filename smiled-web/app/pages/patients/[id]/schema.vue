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
        <span>Chargement du schema...</span>
      </div>
    </div>

    <!-- No schema state -->
    <div v-else-if="!schema && !loading" class="flex flex-col items-center justify-center gap-4 text-gray-400 py-16">
      <Icon name="lucide:scan-face" class="w-16 h-16 opacity-30" />
      <p class="text-base">Aucun schema dentaire pour ce patient</p>
      <button
        class="flex items-center gap-2 rounded-md bg-blue-600 text-white px-4 py-2 text-sm font-medium hover:bg-blue-700 transition-colors"
        @click="showNewVersionModal = true"
      >
        <Icon name="lucide:plus" class="w-4 h-4" />
        Creer le schema initial
      </button>
    </div>

    <!-- Chart + panel layout -->
    <div v-else-if="schema" class="relative">
      <div class="flex overflow-hidden">
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
          <div ref="chartContainerRef" class="overflow-x-auto relative">
            <DentalChart
              :teeth="schema.dents"
              :selected-fdi="selection.fdi"
              :selected-face="selection.face"
              @tooth-selected="handleToothSelected"
              @tooth-click="handleToothClick"
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
              Classe : {{ schema.occlusion.classe_angle ?? 'Non renseignee' }}
            </div>
          </div>
        </div>

        <!-- Right sidebar (full details mode) -->
        <ToothDetailPanel
          v-if="showFullDetail"
          :selected-entry="selectedEntry"
          :selected-face="selection.face"
          :saving="saving"
          :save-error="saveError"
          :save-success="saveSuccess"
          @close="closeFullDetail"
          @save="handleSave"
        />
      </div>

      <!-- Tooth quick popover -->
      <Teleport to="body">
        <div
          v-if="popover.visible && selectedEntry && !showFullDetail"
          ref="popoverRef"
          class="fixed z-50 bg-white rounded-xl shadow-xl border border-gray-200 p-4 w-72"
          :style="popoverStyle"
        >
          <!-- Header -->
          <div class="flex items-center justify-between mb-3">
            <h3 class="font-semibold text-gray-900 text-sm">Dent {{ selectedEntry.tooth.numero_fdi }}</h3>
            <button
              class="text-gray-400 hover:text-gray-600 transition-colors p-0.5"
              title="Fermer"
              @click="closePopover"
            >
              <Icon name="lucide:x" class="w-3.5 h-3.5" />
            </button>
          </div>

          <!-- Quick status buttons -->
          <div class="mb-3">
            <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1.5">Statut</p>
            <div class="flex flex-wrap gap-1.5">
              <button
                v-for="status in quickStatuses"
                :key="status.value"
                class="px-2.5 py-1 text-xs rounded-md border transition-colors"
                :class="selectedEntry.tooth.statut === status.value
                  ? 'bg-blue-600 text-white border-blue-600'
                  : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-50'"
                @click="handleQuickStatus(status.value)"
              >
                {{ status.label }}
              </button>
            </div>
          </div>

          <!-- Face toggle buttons -->
          <div class="mb-3">
            <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1.5">Faces</p>
            <div class="flex gap-1.5">
              <button
                v-for="face in faceButtons"
                :key="face.name"
                class="w-9 h-9 text-xs font-medium rounded-md border transition-colors"
                :class="face.state !== 'saine'
                  ? 'bg-red-50 text-red-700 border-red-300'
                  : 'bg-white text-gray-700 border-gray-300 hover:bg-gray-50'"
                :title="face.label"
                @click="handleFaceToggle(face.name)"
              >
                {{ face.short }}
              </button>
            </div>
          </div>

          <!-- Notes textarea -->
          <div class="mb-3">
            <p class="text-xs font-semibold text-gray-500 uppercase tracking-wide mb-1.5">Notes</p>
            <textarea
              v-model="popoverNotes"
              rows="2"
              placeholder="Observations..."
              class="w-full rounded-md border border-gray-300 px-2.5 py-1.5 text-xs resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
              @blur="handleNotesBlur"
            />
          </div>

          <!-- Details link -->
          <button
            class="w-full text-center text-xs text-blue-600 hover:text-blue-800 font-medium py-1"
            @click="openFullDetail"
          >
            Details complets
          </button>
        </div>
      </Teleport>
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
        <h2 class="text-base font-semibold mb-4">Nouvelle version du schema</h2>

        <div class="space-y-3">
          <label class="text-sm text-gray-600">Type de dentition</label>
          <select
            v-model="newDentition"
            class="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="permanente">Permanente (adulte)</option>
            <option value="lacteale">Lacteale (enfant)</option>
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
            Creer
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import type { FaceName, ToothEntry, ToothStatus, FaceState } from '~/components/dental-chart/types'
import { TOOTH_STATUS_LABELS, FACE_LABELS } from '~/components/dental-chart/types'
import DentalChart from '~/components/dental-chart/DentalChart.vue'
import ToothDetailPanel from '~/components/dental-chart/ToothDetailPanel.vue'
import ChartLegend from '~/components/dental-chart/ChartLegend.vue'

const route = useRoute()
const patientId = computed(() => route.params.id as string)

const { schema, versions, loading, error, loadSchema, loadVersions, createNewVersion, updateTooth, updateFaceState } =
  useDentalChart(patientId)

const selectedVersion = ref<number | null>(null)
const showNewVersionModal = ref(false)
const newDentition = ref<'permanente' | 'lacteale' | 'mixte'>('permanente')

const selection = ref<{ fdi: number | null; face: FaceName | null }>({ fdi: null, face: null })
const saving = ref(false)
const saveError = ref<string | null>(null)
const saveSuccess = ref(false)

// Popover state
const popover = ref<{ visible: boolean; clientX: number; clientY: number }>({
  visible: false,
  clientX: 0,
  clientY: 0,
})
const popoverRef = ref<HTMLDivElement | null>(null)
const chartContainerRef = ref<HTMLDivElement | null>(null)
const showFullDetail = ref(false)
const popoverNotes = ref('')

const quickStatuses: { value: ToothStatus; label: string }[] = [
  { value: 'presente', label: 'Saine' },
  { value: 'absente', label: 'Absente' },
  { value: 'implant', label: 'Implant' },
  { value: 'couronne', label: 'Couronne' },
]

const faceButtons = computed(() => {
  const faces: { name: FaceName; short: string; label: string; state: FaceState }[] = [
    { name: 'V', short: 'V', label: 'Vestibulaire', state: 'saine' },
    { name: 'M', short: 'M', label: 'Mesiale', state: 'saine' },
    { name: 'O', short: 'O', label: 'Occlusale', state: 'saine' },
    { name: 'D', short: 'D', label: 'Distale', state: 'saine' },
    { name: 'P_L', short: 'P/L', label: 'Palatine/Linguale', state: 'saine' },
  ]

  if (!selectedEntry.value) return faces

  return faces.map((f) => {
    const faceData = selectedEntry.value!.faces.find((fd) => fd.face === f.name)
    return { ...f, state: (faceData?.etat as FaceState) ?? 'saine' }
  })
})

const popoverStyle = computed(() => {
  const POPOVER_WIDTH = 288 // w-72 = 18rem = 288px
  const POPOVER_MARGIN = 8

  let left = popover.value.clientX + 12
  let top = popover.value.clientY - 20

  // Keep popover within viewport
  if (typeof window !== 'undefined') {
    if (left + POPOVER_WIDTH + POPOVER_MARGIN > window.innerWidth) {
      left = popover.value.clientX - POPOVER_WIDTH - 12
    }
    if (top < POPOVER_MARGIN) {
      top = POPOVER_MARGIN
    }
    if (top + 350 > window.innerHeight) {
      top = window.innerHeight - 360
    }
  }

  return { left: `${left}px`, top: `${top}px` }
})

const dentitionLabel = computed(() => {
  const map: Record<string, string> = {
    permanente: 'Dentition permanente',
    lacteale: 'Dentition lacteale',
    mixte: 'Dentition mixte',
  }
  return map[schema.value?.schema.dentition ?? ''] ?? ''
})

const selectedEntry = computed<ToothEntry | null>(() => {
  if (!selection.value.fdi || !schema.value) return null
  return schema.value.dents.find((e) => e.tooth.numero_fdi === selection.value.fdi) ?? null
})

function handleToothSelected(fdi: number, face: FaceName | null): void {
  // fdi === 0 means deselect (from keyboard Escape)
  if (fdi === 0) {
    clearSelection()
    return
  }

  // Toggle selection off if clicking the same tooth+face
  if (selection.value.fdi === fdi && selection.value.face === face) {
    clearSelection()
    return
  }
  selection.value = { fdi, face }
  saveError.value = null
  saveSuccess.value = false

  // Sync popover notes
  const entry = schema.value?.dents.find((e) => e.tooth.numero_fdi === fdi)
  popoverNotes.value = entry?.tooth.notes ?? ''
}

function handleToothClick(fdi: number, face: FaceName | null, clientX: number, clientY: number): void {
  // Also select the tooth
  handleToothSelected(fdi, face)

  // Show popover at click position
  showFullDetail.value = false
  popover.value = { visible: true, clientX, clientY }
}

function closePopover(): void {
  popover.value = { visible: false, clientX: 0, clientY: 0 }
  clearSelection()
}

function openFullDetail(): void {
  popover.value = { visible: false, clientX: 0, clientY: 0 }
  showFullDetail.value = true
}

function closeFullDetail(): void {
  showFullDetail.value = false
  clearSelection()
}

function clearSelection(): void {
  selection.value = { fdi: null, face: null }
  popover.value = { visible: false, clientX: 0, clientY: 0 }
  showFullDetail.value = false
}

async function handleQuickStatus(statut: ToothStatus): Promise<void> {
  if (!selectedEntry.value) return

  saving.value = true
  try {
    await updateTooth(selectedEntry.value.tooth.numero_fdi, { statut })
    saveSuccess.value = true
    setTimeout(() => { saveSuccess.value = false }, 2500)
  } catch {
    saveError.value = 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

async function handleFaceToggle(face: FaceName): Promise<void> {
  if (!selectedEntry.value) return

  const faceData = selectedEntry.value.faces.find((f) => f.face === face)
  const currentState = (faceData?.etat as FaceState) ?? 'saine'
  // Toggle between saine and carie for quick editing
  const newState: FaceState = currentState === 'saine' ? 'carie' : 'saine'

  saving.value = true
  try {
    await updateFaceState(selectedEntry.value.tooth.numero_fdi, face, newState)
  } catch {
    saveError.value = 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

async function handleNotesBlur(): Promise<void> {
  if (!selectedEntry.value) return
  if (popoverNotes.value === (selectedEntry.value.tooth.notes ?? '')) return

  try {
    await updateTooth(selectedEntry.value.tooth.numero_fdi, { notes: popoverNotes.value || null })
  } catch {
    saveError.value = 'Erreur lors de la sauvegarde des notes'
  }
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

// Close popover on outside click
function handleDocumentClick(event: MouseEvent): void {
  if (!popover.value.visible) return

  const target = event.target as HTMLElement
  // Don't close if clicking inside the popover
  if (popoverRef.value?.contains(target)) return
  // Don't close if clicking on the canvas (handleToothClick will handle it)
  if (chartContainerRef.value?.contains(target)) return

  closePopover()
}

// Initial load
onMounted(async () => {
  document.addEventListener('click', handleDocumentClick, true)
  await loadVersions()
  if (versions.value.length > 0) {
    selectedVersion.value = versions.value[versions.value.length - 1]?.version ?? null
    await loadSchema()
  }
})

onUnmounted(() => {
  document.removeEventListener('click', handleDocumentClick, true)
})
</script>
