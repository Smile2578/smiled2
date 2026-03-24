<template>
  <div>
    <!-- Toolbar -->
    <div class="mb-4 flex items-center justify-between">
      <p class="text-sm text-muted-foreground">Cliquez sur une dent pour la modifier</p>

      <div class="flex items-center gap-3">
        <!-- Version selector -->
        <div v-if="versions.length > 0" class="flex items-center gap-2">
          <Label class="text-xs text-muted-foreground">Version :</Label>
          <Select
            :model-value="selectedVersion != null ? String(selectedVersion) : undefined"
            @update:model-value="(v) => { selectedVersion = Number(v); handleVersionChange() }"
          >
            <SelectTrigger class="h-8 w-48 text-xs">
              <SelectValue placeholder="Selectionner" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem
                v-for="v in versions"
                :key="v.id"
                :value="String(v.version)"
              >
                v{{ v.version }} — {{ formatDate(v.created_at) }}
              </SelectItem>
            </SelectContent>
          </Select>
        </div>

        <!-- New version button -->
        <Button
          size="sm"
          class="bg-primary hover:bg-primary/90 text-primary-foreground"
          :disabled="loading"
          @click="showNewVersionModal = true"
        >
          <Icon name="lucide:plus" class="mr-1.5 h-4 w-4" />
          Nouvelle version
        </Button>
      </div>
    </div>

    <!-- Error banner -->
    <Alert v-if="error" variant="destructive" class="mb-4">
      <AlertDescription>{{ error }}</AlertDescription>
    </Alert>

    <!-- Loading state -->
    <div v-if="loading" class="flex items-center justify-center h-64">
      <div class="flex items-center gap-3 text-muted-foreground">
        <Icon name="lucide:loader-2" class="h-5 w-5 animate-spin" />
        <span>Chargement du schema...</span>
      </div>
    </div>

    <!-- No schema state -->
    <div
      v-else-if="!schema && !loading"
      class="flex flex-col items-center justify-center gap-4 py-16 text-center"
    >
      <div class="mx-auto flex h-14 w-14 items-center justify-center rounded-full bg-muted">
        <Icon name="lucide:scan-face" class="h-7 w-7 text-muted-foreground" />
      </div>
      <p class="text-base font-medium text-foreground">Aucun schema dentaire</p>
      <p class="text-sm text-muted-foreground">Creez le schema initial pour ce patient</p>
      <Button
        class="bg-primary hover:bg-primary/90 text-primary-foreground"
        @click="showNewVersionModal = true"
      >
        <Icon name="lucide:plus" class="mr-2 h-4 w-4" />
        Creer le schema initial
      </Button>
    </div>

    <!-- Chart + panel layout -->
    <div v-else-if="schema" class="relative">
      <div class="flex overflow-hidden">
        <!-- Main chart area -->
        <div class="flex-1 flex flex-col overflow-auto gap-4">
          <!-- Dentition badge -->
          <div class="flex items-center gap-2">
            <Badge variant="secondary" class="text-xs">
              {{ dentitionLabel }}
            </Badge>
            <span class="text-xs text-muted-foreground">
              Version {{ schema.schema.version }}
            </span>
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

          <!-- Occlusion / ATM summary -->
          <div v-if="schema.atm || schema.occlusion" class="flex gap-4 flex-wrap">
            <div
              v-if="schema.atm"
              class="flex items-center gap-2 text-xs text-muted-foreground rounded-md border bg-muted/30 px-3 py-1.5"
            >
              <Icon name="lucide:activity" class="h-3.5 w-3.5" />
              ATM :
              <span v-if="schema.atm.bruxisme" class="text-amber-600 font-medium">Bruxisme</span>
              <span
                v-if="schema.atm.douleur_droite || schema.atm.douleur_gauche"
                class="text-red-600 font-medium"
              >
                Douleurs
              </span>
              <span
                v-if="!schema.atm.bruxisme && !schema.atm.douleur_droite && !schema.atm.douleur_gauche"
              >
                RAS
              </span>
            </div>
            <div
              v-if="schema.occlusion"
              class="flex items-center gap-2 text-xs text-muted-foreground rounded-md border bg-muted/30 px-3 py-1.5"
            >
              <Icon name="lucide:align-center" class="h-3.5 w-3.5" />
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
          class="fixed z-50 rounded-xl border bg-card p-4 shadow-xl w-72"
          :style="popoverStyle"
        >
          <!-- Header -->
          <div class="flex items-center justify-between mb-3">
            <h3 class="font-semibold text-foreground text-sm">
              Dent {{ selectedEntry.tooth.numero_fdi }}
            </h3>
            <Button
              variant="ghost"
              size="sm"
              class="h-6 w-6 p-0 text-muted-foreground hover:text-foreground"
              @click="closePopover"
            >
              <Icon name="lucide:x" class="h-3.5 w-3.5" />
            </Button>
          </div>

          <!-- Quick status buttons -->
          <div class="mb-3">
            <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wide mb-1.5">
              Statut
            </p>
            <div class="flex flex-wrap gap-1.5">
              <button
                v-for="status in quickStatuses"
                :key="status.value"
                class="px-2.5 py-1 text-xs rounded-md border transition-colors"
                :class="selectedEntry.tooth.statut === status.value
                  ? 'bg-primary text-primary-foreground border-primary'
                  : 'bg-card text-foreground border-border hover:bg-muted'"
                @click="handleQuickStatus(status.value)"
              >
                {{ status.label }}
              </button>
            </div>
          </div>

          <!-- Face toggle buttons -->
          <div class="mb-3">
            <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wide mb-1.5">
              Faces
            </p>
            <div class="flex gap-1.5">
              <button
                v-for="face in faceButtons"
                :key="face.name"
                class="w-9 h-9 text-xs font-medium rounded-md border transition-colors"
                :class="face.state !== 'saine'
                  ? 'bg-red-50 text-red-700 border-red-300'
                  : 'bg-card text-foreground border-border hover:bg-muted'"
                :title="face.label"
                @click="handleFaceToggle(face.name)"
              >
                {{ face.short }}
              </button>
            </div>
          </div>

          <!-- Notes textarea -->
          <div class="mb-3">
            <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wide mb-1.5">
              Notes
            </p>
            <Textarea
              v-model="popoverNotes"
              rows="2"
              placeholder="Observations..."
              class="text-xs resize-none"
              @blur="handleNotesBlur"
            />
          </div>

          <!-- Details link -->
          <Button
            variant="link"
            size="sm"
            class="w-full text-xs text-primary"
            @click="openFullDetail"
          >
            Details complets
          </Button>
        </div>
      </Teleport>
    </div>
  </div>

  <!-- New version modal -->
  <Dialog v-model:open="showNewVersionModal">
    <DialogContent class="max-w-sm">
      <DialogHeader>
        <DialogTitle>Nouvelle version du schema</DialogTitle>
      </DialogHeader>

      <div class="space-y-3">
        <Label>Type de dentition</Label>
        <Select v-model="newDentition">
          <SelectTrigger>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="permanente">Permanente (adulte)</SelectItem>
            <SelectItem value="lacteale">Lacteale (enfant)</SelectItem>
            <SelectItem value="mixte">Mixte</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <DialogFooter>
        <Button variant="outline" @click="showNewVersionModal = false">
          Annuler
        </Button>
        <Button
          class="bg-primary hover:bg-primary/90 text-primary-foreground"
          :disabled="loading"
          @click="handleCreateVersion"
        >
          Creer
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>

<script setup lang="ts">
import type { FaceName, ToothEntry, ToothStatus, FaceState } from '~/components/dental-chart/types'
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
  const POPOVER_WIDTH = 288
  const POPOVER_MARGIN = 8

  let left = popover.value.clientX + 12
  let top = popover.value.clientY - 20

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
  if (fdi === 0) {
    clearSelection()
    return
  }
  if (selection.value.fdi === fdi && selection.value.face === face) {
    clearSelection()
    return
  }
  selection.value = { fdi, face }
  saveError.value = null
  saveSuccess.value = false

  const entry = schema.value?.dents.find((e) => e.tooth.numero_fdi === fdi)
  popoverNotes.value = entry?.tooth.notes ?? ''
}

function handleToothClick(fdi: number, face: FaceName | null, clientX: number, clientY: number): void {
  handleToothSelected(fdi, face)
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
    setTimeout(() => { saveSuccess.value = false }, 2500)
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

function handleDocumentClick(event: MouseEvent): void {
  if (!popover.value.visible) return
  const target = event.target as HTMLElement
  if (popoverRef.value?.contains(target)) return
  if (chartContainerRef.value?.contains(target)) return
  closePopover()
}

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
