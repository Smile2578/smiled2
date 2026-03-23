<template>
  <div class="w-72 border-l bg-white flex flex-col h-full overflow-y-auto">
    <!-- Empty state -->
    <div v-if="!selectedEntry" class="flex-1 flex items-center justify-center text-gray-400 p-8 text-center">
      <div>
        <div class="text-4xl mb-3">🦷</div>
        <p class="text-sm">Cliquez sur une dent pour voir ses détails</p>
      </div>
    </div>

    <!-- Tooth detail -->
    <template v-else>
      <!-- Header -->
      <div class="p-4 border-b bg-gray-50">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="font-semibold text-gray-900">Dent {{ selectedEntry.tooth.numero_fdi }}</h3>
            <p class="text-xs text-gray-500 mt-0.5">{{ faceLabel }}</p>
          </div>
          <button
            class="text-gray-400 hover:text-gray-600 transition-colors p-1"
            title="Fermer"
            @click="emit('close')"
          >
            <Icon name="lucide:x" class="w-4 h-4" />
          </button>
        </div>
      </div>

      <!-- Form body -->
      <div class="flex-1 p-4 space-y-5">
        <!-- Statut -->
        <div class="space-y-1.5">
          <label class="text-xs font-semibold text-gray-600 uppercase tracking-wide">
            Statut de la dent
          </label>
          <select
            v-model="localStatut"
            class="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option v-for="(label, value) in TOOTH_STATUS_LABELS" :key="value" :value="value">
              {{ label }}
            </option>
          </select>
        </div>

        <!-- Face état (if a face is selected) -->
        <div v-if="selectedFace" class="space-y-1.5">
          <label class="text-xs font-semibold text-gray-600 uppercase tracking-wide">
            État de la face {{ FACE_LABELS[selectedFace] }}
          </label>
          <select
            v-model="localFaceEtat"
            class="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option v-for="(label, value) in FACE_STATE_LABELS" :key="value" :value="value">
              {{ label }}
            </option>
          </select>
        </div>

        <!-- Mobilité paro -->
        <div class="space-y-1.5">
          <label class="text-xs font-semibold text-gray-600 uppercase tracking-wide">
            Mobilité (0–3)
          </label>
          <div class="flex items-center gap-3">
            <input
              v-model.number="localMobilite"
              type="range"
              min="0"
              max="3"
              step="1"
              class="flex-1 accent-blue-600"
            />
            <span class="text-sm font-medium w-4 text-center">{{ localMobilite }}</span>
          </div>
          <div class="flex justify-between text-xs text-gray-400">
            <span>Aucune</span>
            <span>Grade I</span>
            <span>Grade II</span>
            <span>Grade III</span>
          </div>
        </div>

        <!-- Endodontie -->
        <div class="space-y-1.5">
          <label class="text-xs font-semibold text-gray-600 uppercase tracking-wide">
            Endodontie
          </label>
          <div class="flex items-center gap-2">
            <input
              v-model="localEndoTraite"
              id="endo-traite"
              type="checkbox"
              class="accent-blue-600"
            />
            <label for="endo-traite" class="text-sm text-gray-700">Traitement endodontique</label>
          </div>
        </div>

        <!-- Prothèse fixe -->
        <div class="space-y-1.5">
          <label class="text-xs font-semibold text-gray-600 uppercase tracking-wide">
            Prothèse fixe
          </label>
          <select
            v-model="localProtheseType"
            class="w-full rounded-md border border-gray-300 bg-white px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="">Aucune</option>
            <option value="couronne_metallique">Couronne métallique</option>
            <option value="couronne_ceramo_metallique">Couronne céramométallique</option>
            <option value="couronne_ceramique">Couronne céramique</option>
            <option value="facette">Facette</option>
            <option value="bridge">Bridge</option>
            <option value="inlay_core">Inlay-core</option>
          </select>
        </div>

        <!-- Notes -->
        <div class="space-y-1.5">
          <label class="text-xs font-semibold text-gray-600 uppercase tracking-wide">
            Notes
          </label>
          <textarea
            v-model="localNotes"
            rows="3"
            placeholder="Observations cliniques..."
            class="w-full rounded-md border border-gray-300 px-3 py-2 text-sm resize-none focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
        </div>
      </div>

      <!-- Save button -->
      <div class="p-4 border-t bg-gray-50">
        <button
          class="w-full rounded-md bg-blue-600 text-white px-4 py-2 text-sm font-medium hover:bg-blue-700 active:bg-blue-800 transition-colors disabled:opacity-50"
          :disabled="saving"
          @click="handleSave"
        >
          <span v-if="saving">Enregistrement...</span>
          <span v-else>Enregistrer</span>
        </button>
        <p v-if="saveError" class="mt-2 text-xs text-red-600 text-center">{{ saveError }}</p>
        <p v-if="saveSuccess" class="mt-2 text-xs text-green-600 text-center">Modifications enregistrées</p>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { ToothEntry, ToothStatus, FaceName, FaceState } from './types'
import { TOOTH_STATUS_LABELS, FACE_STATE_LABELS, FACE_LABELS } from './types'

interface Props {
  selectedEntry: ToothEntry | null
  selectedFace: FaceName | null
  saving?: boolean
  saveError?: string | null
  saveSuccess?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  saving: false,
  saveError: null,
  saveSuccess: false,
})

const emit = defineEmits<{
  close: []
  save: [fdi: number, statut: ToothStatus, faceEtat: FaceState | null, mobilite: number, endoTraite: boolean, protheseType: string, notes: string]
}>()

// Local form state, synced from selectedEntry
const localStatut = ref<ToothStatus>('presente')
const localFaceEtat = ref<FaceState>('saine')
const localMobilite = ref<number>(0)
const localEndoTraite = ref<boolean>(false)
const localProtheseType = ref<string>('')
const localNotes = ref<string>('')

const faceLabel = computed(() => {
  if (!props.selectedFace) return 'Aucune face sélectionnée'
  return `Face ${FACE_LABELS[props.selectedFace]}`
})

// Sync form when selection changes
watch(
  () => props.selectedEntry,
  (entry) => {
    if (!entry) return
    localStatut.value = (entry.tooth.statut as ToothStatus) ?? 'presente'
    localMobilite.value = entry.tooth.paro_mobilite ?? 0
    localEndoTraite.value = !!(entry.tooth.endo as Record<string, unknown> | null)?.traite
    localProtheseType.value = ((entry.tooth.prothese_fixe as Record<string, unknown> | null)?.type as string) ?? ''
    localNotes.value = entry.tooth.notes ?? ''
  },
  { immediate: true },
)

// Sync face état when face selection changes
watch(
  () => [props.selectedEntry, props.selectedFace] as const,
  ([entry, face]) => {
    if (!entry || !face) return
    const faceData = entry.faces.find((f) => f.face === face)
    localFaceEtat.value = (faceData?.etat as FaceState) ?? 'saine'
  },
  { immediate: true },
)

function handleSave(): void {
  if (!props.selectedEntry) return

  const faceEtat = props.selectedFace ? localFaceEtat.value : null

  emit(
    'save',
    props.selectedEntry.tooth.numero_fdi,
    localStatut.value,
    faceEtat,
    localMobilite.value,
    localEndoTraite.value,
    localProtheseType.value,
    localNotes.value,
  )
}
</script>
