<template>
  <div>
    <!-- Page Header -->
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-semibold tracking-tight">Guide des teintes</h1>
        <p class="text-sm text-muted-foreground mt-1">
          Teintes VITA Classical et 3D-Master
          <span v-if="!loading" class="text-muted-foreground/60 ml-1">&middot; {{ teintes.length }} teinte{{ teintes.length !== 1 ? 's' : '' }}</span>
        </p>
      </div>
      <Button class="bg-teal-600 hover:bg-teal-700 text-white" @click="openNewDialog">
        <Plus class="w-4 h-4 mr-2" />
        Nouvelle teinte
      </Button>
    </div>

    <!-- Errors -->
    <Alert v-if="loadError" variant="destructive" class="mb-4">
      <AlertDescription>{{ loadError }}</AlertDescription>
    </Alert>

    <!-- System Tab Filters -->
    <div class="flex items-center gap-1.5 mb-6">
      <button
        v-for="tab in systemeTabs"
        :key="tab.value"
        :class="[
          'px-4 py-2 rounded-md text-sm font-medium transition-colors',
          activeTab === tab.value
            ? 'bg-teal-600 text-white shadow-sm'
            : 'bg-background text-muted-foreground border hover:bg-muted/50',
        ]"
        @click="activeTab = tab.value"
      >
        {{ tab.label }}
      </button>
    </div>

    <!-- Loading Skeleton -->
    <div v-if="loading" class="space-y-8">
      <div v-for="i in 2" :key="i">
        <div class="flex items-center gap-3 mb-4">
          <Skeleton class="h-4 w-32" />
          <div class="flex-1 border-t" />
          <Skeleton class="h-3 w-16" />
        </div>
        <Card class="p-5">
          <div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 xl:grid-cols-12 gap-3">
            <div v-for="j in 12" :key="j" class="flex flex-col items-center gap-1.5 p-2">
              <Skeleton class="w-10 h-10 rounded-full" />
              <Skeleton class="h-3 w-8" />
              <Skeleton class="h-2 w-12" />
            </div>
          </div>
        </Card>
      </div>
    </div>

    <!-- Empty state -->
    <div v-else-if="filteredTeintes.length === 0" class="text-center py-16">
      <Palette class="w-12 h-12 mx-auto mb-3 text-muted-foreground/40" />
      <p class="text-sm font-medium text-muted-foreground">Aucune teinte trouvee</p>
      <p class="text-xs text-muted-foreground/60 mt-1">Ajoutez une teinte pour commencer</p>
    </div>

    <!-- Teintes Grid grouped by system -->
    <div v-else class="space-y-8">
      <div v-for="[systeme, items] in groupedTeintes" :key="systeme ?? 'autre'">
        <div class="flex items-center gap-3 mb-4">
          <h2 class="text-sm font-semibold uppercase tracking-wider text-muted-foreground">{{ systeme ?? 'Autre' }}</h2>
          <div class="flex-1 border-t" />
          <span class="text-xs text-muted-foreground/60">{{ items.length }} teinte{{ items.length !== 1 ? 's' : '' }}</span>
        </div>

        <Card class="p-5">
          <div class="grid grid-cols-4 sm:grid-cols-6 md:grid-cols-8 lg:grid-cols-10 xl:grid-cols-12 gap-3">
            <button
              v-for="teinte in items"
              :key="teinte.id"
              class="group flex flex-col items-center gap-1.5 p-2 rounded-lg hover:bg-muted/50 transition-colors"
              @click="openEditDialog(teinte)"
            >
              <!-- Color swatch -->
              <div
                class="w-10 h-10 rounded-full border-2 border-white shadow-md ring-1 ring-border group-hover:ring-teal-300 transition-all"
                :style="{ backgroundColor: shadeColor(teinte.code) }"
              />
              <!-- Code -->
              <span class="text-xs font-semibold group-hover:text-teal-600 transition-colors">
                {{ teinte.code }}
              </span>
              <!-- Name -->
              <span class="text-[10px] text-muted-foreground leading-tight text-center truncate w-full">
                {{ teinte.libelle }}
              </span>
            </button>
          </div>
        </Card>
      </div>
    </div>

    <!-- New / Edit Dialog -->
    <Dialog v-model:open="showDialog">
      <DialogContent class="max-w-sm">
        <DialogHeader>
          <DialogTitle>{{ editingId ? 'Modifier la teinte' : 'Nouvelle teinte' }}</DialogTitle>
        </DialogHeader>

        <form class="space-y-4" @submit.prevent="handleSave">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="code_teinte">Code *</Label>
              <Input id="code_teinte" v-model="form.code" required placeholder="Ex : A2" />
            </div>
            <div class="space-y-2">
              <Label for="libelle_teinte">Libelle *</Label>
              <Input id="libelle_teinte" v-model="form.libelle" required placeholder="Ex : A2 Vita" />
            </div>
          </div>

          <div class="space-y-2">
            <Label for="systeme_teinte">Systeme</Label>
            <Input id="systeme_teinte" v-model="form.systeme" placeholder="Ex : VITA Classical" />
          </div>

          <!-- Preview swatch -->
          <div v-if="form.code" class="flex items-center gap-3 p-3 bg-muted/50 rounded-lg">
            <div
              class="w-8 h-8 rounded-full border-2 border-white shadow-md ring-1 ring-border"
              :style="{ backgroundColor: shadeColor(form.code) }"
            />
            <div>
              <p class="text-sm font-medium">Apercu</p>
              <p class="text-xs text-muted-foreground">{{ form.code }} &mdash; {{ form.libelle || '...' }}</p>
            </div>
          </div>

          <Alert v-if="saveError" variant="destructive">
            <AlertDescription>{{ saveError }}</AlertDescription>
          </Alert>

          <DialogFooter>
            <Button type="button" variant="outline" @click="closeDialog">Annuler</Button>
            <Button type="submit" :disabled="saving" class="bg-teal-600 hover:bg-teal-700 text-white">
              <Loader2 v-if="saving" class="w-4 h-4 mr-2 animate-spin" />
              {{ editingId ? 'Enregistrer' : 'Creer' }}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { Loader2, Palette, Plus } from 'lucide-vue-next'
import type { Teinte } from '~/composables/useTeinte'

definePageMeta({ layout: 'default' })

const { listTeintes, createTeinte, updateTeinte } = useTeinte()

const teintes = ref<Teinte[]>([])
const loading = ref(true)
const showDialog = ref(false)
const saving = ref(false)
const saveError = ref<string | null>(null)
const editingId = ref<string | null>(null)
const activeTab = ref('all')

const systemeTabs = [
  { value: 'all', label: 'Tous' },
  { value: 'VITA Classical', label: 'VITA Classical' },
  { value: 'VITA 3D-Master', label: 'VITA 3D-Master' },
]

const form = reactive({
  code: '',
  libelle: '',
  systeme: '',
})

function shadeColor(code: string): string {
  const classicalColors: Record<string, string> = {
    'A1': '#F5E6D0', 'A2': '#EDD9B9', 'A3': '#E5CC9F', 'A3.5': '#DFC28D',
    'A4': '#D4B070', 'B1': '#F2E8D4', 'B2': '#EBD9B5', 'B3': '#DFC89A',
    'B4': '#D4B67D', 'C1': '#EDE0C8', 'C2': '#DFD0AC', 'C3': '#D0BF93',
    'C4': '#C2AE7D', 'D2': '#E8D4B0', 'D3': '#DDC89A', 'D4': '#D0BA85',
  }
  if (code.match(/^\dM\d$/)) return '#F0DCC0'
  if (code.match(/^\d[LR]\d/)) return '#ECD5B5'
  return classicalColors[code] ?? '#F5F0E8'
}

const filteredTeintes = computed(() => {
  if (activeTab.value === 'all') return teintes.value
  return teintes.value.filter((t) => t.systeme === activeTab.value)
})

const groupedTeintes = computed(() => {
  const map = new Map<string | null, Teinte[]>()
  const sorted = [...filteredTeintes.value].sort((a, b) => (a.ordre ?? 0) - (b.ordre ?? 0))
  for (const t of sorted) {
    const key = t.systeme
    const group = map.get(key) ?? []
    map.set(key, [...group, t])
  }
  return map
})

function openNewDialog(): void {
  editingId.value = null
  form.code = ''
  form.libelle = ''
  form.systeme = ''
  showDialog.value = true
}

function openEditDialog(teinte: Teinte): void {
  editingId.value = teinte.id
  form.code = teinte.code
  form.libelle = teinte.libelle
  form.systeme = teinte.systeme ?? ''
  showDialog.value = true
}

function closeDialog(): void {
  showDialog.value = false
  editingId.value = null
  saveError.value = null
}

async function handleSave(): Promise<void> {
  saving.value = true
  saveError.value = null

  const payload = {
    code: form.code,
    libelle: form.libelle,
    systeme: form.systeme || null,
  }

  try {
    if (editingId.value) {
      const response = await updateTeinte(editingId.value, payload)
      if (response.success && response.data) {
        teintes.value = teintes.value.map((t) =>
          t.id === editingId.value ? response.data! : t,
        )
        closeDialog()
      } else {
        saveError.value = response.error ?? 'Erreur lors de la mise a jour'
      }
    } else {
      const response = await createTeinte(payload)
      if (response.success && response.data) {
        teintes.value = [response.data, ...teintes.value]
        closeDialog()
      } else {
        saveError.value = response.error ?? 'Erreur lors de la creation'
      }
    }
  } catch (err) {
    saveError.value = err instanceof Error ? err.message : 'Erreur inattendue'
  } finally {
    saving.value = false
  }
}

const loadError = ref<string | null>(null)

onMounted(async () => {
  try {
    const response = await listTeintes()
    if (response.success && response.data) {
      teintes.value = response.data
    }
  } catch (e) {
    loadError.value = e instanceof Error ? e.message : 'Erreur lors du chargement des teintes'
  } finally {
    loading.value = false
  }
})
</script>
