<template>
  <div class="space-y-4">
    <!-- Lines list -->
    <div v-if="lines.length > 0" class="space-y-2">
      <div
        v-for="(line, idx) in lines"
        :key="idx"
        class="flex items-start gap-2 p-3 border rounded-lg bg-muted/20"
      >
        <div class="flex-1 grid grid-cols-2 md:grid-cols-4 gap-2">
          <div class="col-span-2">
            <span class="text-xs text-muted-foreground">Acte</span>
            <p class="text-sm font-medium">{{ line.acte_libelle }}</p>
            <p v-if="line.acte_code" class="text-xs text-muted-foreground">{{ line.acte_code }}</p>
          </div>
          <div>
            <span class="text-xs text-muted-foreground">Dent / Faces</span>
            <p class="text-sm">
              {{ line.dent_fdi ?? '—' }}
              <span v-if="line.faces && line.faces.length > 0" class="text-muted-foreground">
                ({{ line.faces.join('') }})
              </span>
            </p>
          </div>
          <div>
            <span class="text-xs text-muted-foreground">Prix unitaire</span>
            <p class="text-sm font-medium">{{ formatPrice(line.prix_unitaire) }}</p>
          </div>
        </div>
        <Button
          type="button"
          variant="ghost"
          size="sm"
          class="h-7 w-7 p-0 text-muted-foreground shrink-0"
          @click="removeLine(idx)"
        >
          <Icon name="lucide:trash-2" class="w-3.5 h-3.5" />
        </Button>
      </div>

      <!-- Total -->
      <div class="flex justify-end pt-2 border-t">
        <div class="text-right">
          <p class="text-xs text-muted-foreground">Total estimé</p>
          <p class="text-xl font-bold">{{ formatPrice(totalPrice) }}</p>
        </div>
      </div>
    </div>

    <p v-else class="text-sm text-muted-foreground text-center py-4">
      Aucune ligne ajoutée. Utilisez le formulaire ci-dessous pour commencer.
    </p>

    <!-- Add line form -->
    <div class="border rounded-lg p-4 space-y-3 bg-muted/10">
      <p class="text-sm font-medium text-muted-foreground">Ajouter une ligne</p>

      <div class="grid grid-cols-2 gap-3">
        <div class="col-span-2 space-y-1">
          <Label class="text-xs">Libellé de l'acte *</Label>
          <div class="relative">
            <div class="flex gap-2">
              <Input
                v-model="acteSearch"
                placeholder="Rechercher un acte (code ou libellé)..."
                class="flex-1"
                @focus="showActeDropdown = true"
                @input="showActeDropdown = true"
              />
              <Input v-model="newLine.acte_code" placeholder="Code" class="w-28" readonly />
            </div>
            <div
              v-if="showActeDropdown && filteredActes.length > 0"
              class="absolute z-50 top-full left-0 right-0 mt-1 max-h-48 overflow-y-auto rounded-md border bg-popover shadow-md"
            >
              <button
                v-for="acte in filteredActes"
                :key="acte.id"
                type="button"
                class="w-full flex items-center justify-between px-3 py-2 text-sm hover:bg-accent text-left"
                @mousedown.prevent="selectActe(acte)"
              >
                <span class="truncate">
                  <span v-if="acte.code" class="font-mono text-xs text-muted-foreground mr-2">{{ acte.code }}</span>
                  {{ acte.libelle }}
                </span>
                <span class="text-xs text-muted-foreground shrink-0 ml-2">{{ formatPrice(acte.prix_cabinet ?? acte.prix_defaut) }}</span>
              </button>
            </div>
          </div>
        </div>

        <div class="space-y-1">
          <Label class="text-xs">Dent (FDI)</Label>
          <Input
            v-model.number="newLine.dent_fdi"
            type="number"
            min="11"
            max="85"
            placeholder="Ex : 36"
          />
        </div>

        <div class="space-y-1">
          <Label class="text-xs">Faces</Label>
          <div class="flex flex-wrap gap-1.5 pt-1">
            <label
              v-for="face in FACES"
              :key="face"
              class="flex items-center gap-1 text-xs cursor-pointer"
            >
              <input
                v-model="newLine.faces"
                type="checkbox"
                :value="face"
                class="accent-primary"
              />
              {{ face }}
            </label>
          </div>
        </div>

        <div class="space-y-1">
          <Label class="text-xs">Matériau</Label>
          <Input v-model="newLine.materiau_libelle" placeholder="Ex : Composite, Zircone..." />
        </div>

        <div class="space-y-1">
          <Label class="text-xs">Teinte</Label>
          <Input v-model="newLine.teinte_libelle" placeholder="Ex : A2, 2M2..." />
        </div>

        <div class="space-y-1">
          <Label class="text-xs">Prix unitaire (€) *</Label>
          <Input
            v-model.number="newLine.prix_unitaire"
            type="number"
            min="0"
            step="0.01"
            placeholder="0.00"
            required
          />
        </div>

        <div class="space-y-1">
          <Label class="text-xs">Quantité</Label>
          <Input
            v-model.number="newLine.quantite"
            type="number"
            min="1"
            step="1"
            placeholder="1"
          />
        </div>
      </div>

      <div class="space-y-1">
        <Label class="text-xs">Notes</Label>
        <Input v-model="newLine.notes" placeholder="Précisions sur cet acte..." />
      </div>

      <Button type="button" variant="outline" size="sm" :disabled="!canAddLine" @click="addLine">
        <Icon name="lucide:plus" class="w-3.5 h-3.5 mr-1.5" />
        Ajouter la ligne
      </Button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { CreatePdtLineInput } from '~/composables/usePdt'
import type { Acte } from '~/composables/useActe'

const FACES = ['M', 'O', 'D', 'V', 'L'] as const

const props = defineProps<{
  lines: CreatePdtLineInput[]
}>()

const emit = defineEmits<{
  (e: 'update', lines: CreatePdtLineInput[]): void
}>()

// Actes autocomplete
const { listActes } = useActe()
const allActes = ref<Acte[]>([])
const acteSearch = ref('')
const showActeDropdown = ref(false)

const filteredActes = computed(() => {
  const q = acteSearch.value.toLowerCase().trim()
  if (!q) return allActes.value.filter(a => a.actif).slice(0, 20)
  return allActes.value
    .filter(a => a.actif && (
      a.libelle.toLowerCase().includes(q) ||
      (a.code && a.code.toLowerCase().includes(q))
    ))
    .slice(0, 20)
})

function selectActe(acte: Acte) {
  newLine.acte_id = acte.id
  newLine.acte_code = acte.code ?? ''
  newLine.acte_libelle = acte.libelle
  newLine.prix_unitaire = acte.prix_cabinet ?? acte.prix_defaut
  acteSearch.value = acte.libelle
  showActeDropdown.value = false
}

// Close dropdown on click outside
if (import.meta.client) {
  const handleClickOutside = () => { showActeDropdown.value = false }
  onMounted(() => document.addEventListener('click', handleClickOutside))
  onUnmounted(() => document.removeEventListener('click', handleClickOutside))
}

onMounted(async () => {
  try {
    const res = await listActes()
    if (res.success && res.data) allActes.value = res.data
  } catch {
    // Actes list unavailable — manual input still works
  }
})

const newLine = reactive({
  acte_libelle: '',
  acte_code: '',
  acte_id: null as string | null,
  dent_fdi: null as number | null,
  faces: [] as string[],
  materiau_id: null as string | null,
  materiau_libelle: '',
  teinte_id: null as string | null,
  teinte_libelle: '',
  prix_unitaire: 0,
  quantite: 1,
  notes: '',
})

const canAddLine = computed(() => {
  const hasLibelle = newLine.acte_libelle.trim().length > 0 || acteSearch.value.trim().length > 0
  return hasLibelle && newLine.prix_unitaire >= 0
})

const totalPrice = computed(() =>
  props.lines.reduce((sum, l) => sum + l.prix_unitaire * l.quantite, 0),
)

function addLine(): void {
  if (!canAddLine.value) return

  const libelle = newLine.acte_libelle.trim() || acteSearch.value.trim()

  const line: CreatePdtLineInput = {
    acte_id: newLine.acte_id,
    acte_libelle: libelle,
    acte_code: newLine.acte_code || null,
    dent_fdi: newLine.dent_fdi,
    faces: newLine.faces.length > 0 ? [...newLine.faces] : null,
    materiau_id: newLine.materiau_id,
    teinte_id: newLine.teinte_id,
    prix_unitaire: newLine.prix_unitaire,
    quantite: newLine.quantite,
    notes: newLine.notes || null,
  }

  emit('update', [...props.lines, line])

  // Reset form
  acteSearch.value = ''
  newLine.acte_libelle = ''
  newLine.acte_code = ''
  newLine.acte_id = null
  newLine.dent_fdi = null
  newLine.faces = []
  newLine.materiau_libelle = ''
  newLine.materiau_id = null
  newLine.teinte_libelle = ''
  newLine.teinte_id = null
  newLine.prix_unitaire = 0
  newLine.quantite = 1
  newLine.notes = ''
}

function removeLine(idx: number): void {
  emit('update', props.lines.filter((_, i) => i !== idx))
}

function formatPrice(price: number): string {
  return new Intl.NumberFormat('fr-FR', { style: 'currency', currency: 'EUR' }).format(price)
}
</script>
