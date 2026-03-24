<template>
  <div class="p-8">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold">Teintes</h1>
        <p class="text-muted-foreground mt-1">Catalogue des teintes dentaires</p>
      </div>
      <Button @click="openNewDialog">
        <Icon name="lucide:plus" class="w-4 h-4 mr-2" />
        Nouvelle teinte
      </Button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-32">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
    </div>

    <!-- Grouped by system -->
    <div v-else class="space-y-4">
      <div v-for="[systeme, items] in groupedTeintes" :key="systeme ?? 'autre'">
        <button
          class="flex items-center gap-2 w-full text-left mb-2"
          @click="toggleGroup(systeme ?? 'autre')"
        >
          <Icon
            :name="collapsedGroups.has(systeme ?? 'autre') ? 'lucide:chevron-right' : 'lucide:chevron-down'"
            class="w-4 h-4 text-muted-foreground"
          />
          <span class="font-semibold text-sm">{{ systeme ?? 'Autre' }}</span>
          <Badge variant="secondary" class="text-xs">{{ items.length }}</Badge>
        </button>

        <Card v-if="!collapsedGroups.has(systeme ?? 'autre')">
          <div class="flex flex-wrap gap-2 p-4">
            <div
              v-for="teinte in items"
              :key="teinte.id"
              class="group relative flex items-center justify-center w-16 h-10 rounded border border-border/60 cursor-pointer hover:border-primary hover:shadow-sm transition-all"
              :title="teinte.libelle"
              :style="{ backgroundColor: shadeColor(teinte.code) }"
              @click="openEditDialog(teinte)"
            >
              <span class="text-xs font-medium drop-shadow-[0_1px_1px_rgba(255,255,255,0.6)]">{{ teinte.code }}</span>
              <div class="absolute inset-0 bg-primary/5 opacity-0 group-hover:opacity-100 rounded transition-opacity" />
            </div>
          </div>
        </Card>
      </div>

      <div v-if="teintes.length === 0" class="text-center py-12 text-muted-foreground">
        <Icon name="lucide:palette" class="w-12 h-12 mx-auto mb-3 opacity-30" />
        <p>Aucune teinte trouvée</p>
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
              <Label for="libelle_teinte">Libellé *</Label>
              <Input id="libelle_teinte" v-model="form.libelle" required placeholder="Ex : A2 Vita" />
            </div>
          </div>

          <div class="space-y-2">
            <Label for="systeme_teinte">Système</Label>
            <Input id="systeme_teinte" v-model="form.systeme" placeholder="Ex : VITA Classical" />
          </div>

          <Alert v-if="saveError" variant="destructive">
            <AlertDescription>{{ saveError }}</AlertDescription>
          </Alert>

          <DialogFooter>
            <Button type="button" variant="outline" @click="closeDialog">Annuler</Button>
            <Button type="submit" :disabled="saving">
              <Icon v-if="saving" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
              {{ editingId ? 'Enregistrer' : 'Créer' }}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import type { Teinte } from '~/composables/useTeinte'

definePageMeta({ layout: 'default' })

const { listTeintes, createTeinte, updateTeinte } = useTeinte()

const teintes = ref<Teinte[]>([])
const loading = ref(true)
const showDialog = ref(false)
const saving = ref(false)
const saveError = ref<string | null>(null)
const editingId = ref<string | null>(null)
const collapsedGroups = ref<Set<string>>(new Set())

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

const groupedTeintes = computed(() => {
  const map = new Map<string | null, Teinte[]>()
  const sorted = [...teintes.value].sort((a, b) => (a.ordre ?? 0) - (b.ordre ?? 0))
  for (const t of sorted) {
    const key = t.systeme
    const group = map.get(key) ?? []
    map.set(key, [...group, t])
  }
  return map
})

function toggleGroup(key: string): void {
  const next = new Set(collapsedGroups.value)
  if (next.has(key)) {
    next.delete(key)
  } else {
    next.add(key)
  }
  collapsedGroups.value = next
}

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
        saveError.value = response.error ?? 'Erreur lors de la mise à jour'
      }
    } else {
      const response = await createTeinte(payload)
      if (response.success && response.data) {
        teintes.value = [response.data, ...teintes.value]
        closeDialog()
      } else {
        saveError.value = response.error ?? 'Erreur lors de la création'
      }
    }
  } catch (err) {
    saveError.value = err instanceof Error ? err.message : 'Erreur inattendue'
  } finally {
    saving.value = false
  }
}

onMounted(async () => {
  try {
    const response = await listTeintes()
    if (response.success && response.data) {
      teintes.value = response.data
    }
  } catch {
    // ignore
  } finally {
    loading.value = false
  }
})
</script>
