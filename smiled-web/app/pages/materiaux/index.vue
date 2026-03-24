<template>
  <div class="p-8">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold">Matériaux</h1>
        <p class="text-muted-foreground mt-1">
          Catalogue des matériaux dentaires
        </p>
      </div>
      <Button @click="showNewDialog = true">
        <Icon name="lucide:plus" class="w-4 h-4 mr-2" />
        Nouveau matériau
      </Button>
    </div>

    <!-- Search -->
    <div class="flex gap-3 mb-4">
      <div class="relative flex-1 max-w-sm">
        <Icon name="lucide:search" class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
        <Input v-model="search" placeholder="Rechercher..." class="pl-9" />
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-32">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
    </div>

    <!-- Grouped by category -->
    <div v-else class="space-y-4">
      <div
        v-for="[category, items] in groupedMateriaux"
        :key="category"
      >
        <button
          class="flex items-center gap-2 w-full text-left mb-2"
          @click="toggleCategory(category)"
        >
          <Icon
            :name="collapsedCategories.has(category) ? 'lucide:chevron-right' : 'lucide:chevron-down'"
            class="w-4 h-4 text-muted-foreground"
          />
          <span class="font-semibold text-sm">{{ category || 'Non categorise' }}</span>
          <Badge variant="secondary" class="text-xs">{{ items.length }}</Badge>
        </button>

        <Card v-if="!collapsedCategories.has(category)">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Code</TableHead>
                <TableHead>Libellé</TableHead>
                <TableHead>Sous-catégorie</TableHead>
                <TableHead>Marques</TableHead>
                <TableHead class="text-right">Résistance</TableHead>
                <TableHead class="text-center">Statut</TableHead>
                <TableHead />
              </TableRow>
            </TableHeader>
            <TableBody>
              <TableRow v-for="mat in items" :key="mat.id" :class="mat.obsolete ? 'opacity-50' : ''">
                <TableCell class="font-mono text-xs">{{ mat.code }}</TableCell>
                <TableCell class="font-medium text-sm">{{ mat.libelle }}</TableCell>
                <TableCell class="text-sm text-muted-foreground">{{ mat.sous_categorie ?? '—' }}</TableCell>
                <TableCell class="text-sm">
                  <span v-if="mat.marques && mat.marques.length > 0">
                    {{ mat.marques.slice(0, 2).join(', ') }}
                    <span v-if="mat.marques.length > 2" class="text-muted-foreground">
                      +{{ mat.marques.length - 2 }}
                    </span>
                  </span>
                  <span v-else class="text-muted-foreground">—</span>
                </TableCell>
                <TableCell class="text-right text-sm">
                  {{ mat.resistance_mpa ? `${mat.resistance_mpa} MPa` : '—' }}
                </TableCell>
                <TableCell class="text-center">
                  <Badge v-if="mat.obsolete" variant="destructive" class="text-xs">Obsolète</Badge>
                  <Badge v-else variant="default" class="text-xs">Actif</Badge>
                </TableCell>
                <TableCell>
                  <Button
                    v-if="!mat.systeme"
                    variant="ghost"
                    size="sm"
                    class="h-7 w-7 p-0"
                    @click="openEditDialog(mat)"
                  >
                    <Icon name="lucide:pencil" class="w-3.5 h-3.5" />
                  </Button>
                </TableCell>
              </TableRow>
            </TableBody>
          </Table>
        </Card>
      </div>

      <div v-if="groupedMateriaux.size === 0" class="text-center py-12 text-muted-foreground">
        <Icon name="lucide:flask-conical" class="w-12 h-12 mx-auto mb-3 opacity-30" />
        <p>Aucun matériau trouvé</p>
      </div>
    </div>

    <!-- New / Edit Dialog -->
    <Dialog v-model:open="showNewDialog">
      <DialogContent class="max-w-lg">
        <DialogHeader>
          <DialogTitle>{{ editingId ? 'Modifier le matériau' : 'Nouveau matériau' }}</DialogTitle>
          <DialogDescription>
            {{ editingId ? 'Modifiez les informations du matériau.' : 'Ajoutez un matériau au catalogue cabinet.' }}
          </DialogDescription>
        </DialogHeader>

        <form class="space-y-4" @submit.prevent="handleSave">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="libelle_mat">Libellé *</Label>
              <Input id="libelle_mat" v-model="form.libelle" required placeholder="Ex : Zircone 3Y" />
            </div>
            <div class="space-y-2">
              <Label for="code_mat">Code *</Label>
              <Input id="code_mat" v-model="form.code" required placeholder="Ex : ZIRC-3Y" />
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="categorie_mat">Catégorie *</Label>
              <Input id="categorie_mat" v-model="form.categorie" required placeholder="Ex : Céramique" />
            </div>
            <div class="space-y-2">
              <Label for="sous_categorie_mat">Sous-catégorie</Label>
              <Input id="sous_categorie_mat" v-model="form.sous_categorie" placeholder="Ex : Zircone monolithique" />
            </div>
          </div>

          <div class="space-y-2">
            <Label for="marques_mat">Marques (séparées par des virgules)</Label>
            <Input id="marques_mat" v-model="form.marques_str" placeholder="Ex : Vita, Ivoclar, 3M" />
          </div>

          <div class="space-y-2">
            <Label for="resistance_mat">Résistance (MPa)</Label>
            <Input
              id="resistance_mat"
              v-model.number="form.resistance_mpa"
              type="number"
              min="0"
              placeholder="Ex : 1200"
            />
          </div>

          <div class="space-y-2">
            <Label for="notes_mat">Notes</Label>
            <Textarea id="notes_mat" v-model="form.notes" :rows="2" placeholder="Informations complémentaires..." />
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
import type { Materiau } from '~/composables/useMateriau'

definePageMeta({ layout: 'default' })

const { listMateriaux, createMateriau, updateMateriau } = useMateriau()

const materiaux = ref<Materiau[]>([])
const loading = ref(true)
const showNewDialog = ref(false)
const saving = ref(false)
const saveError = ref<string | null>(null)
const editingId = ref<string | null>(null)
const search = ref('')
const collapsedCategories = ref<Set<string>>(new Set())

const form = reactive({
  libelle: '',
  code: '',
  categorie: '',
  sous_categorie: '',
  marques_str: '',
  resistance_mpa: null as number | null,
  notes: '',
})

const filteredMateriaux = computed(() => {
  if (!search.value) return materiaux.value
  const q = search.value.toLowerCase()
  return materiaux.value.filter(
    (m) =>
      m.libelle.toLowerCase().includes(q) ||
      m.code.toLowerCase().includes(q) ||
      m.categorie.toLowerCase().includes(q),
  )
})

const groupedMateriaux = computed(() => {
  const map = new Map<string, Materiau[]>()
  for (const m of filteredMateriaux.value) {
    const key = m.categorie || 'Non categorise'
    const group = map.get(key) ?? []
    map.set(key, [...group, m])
  }
  return map
})

function toggleCategory(cat: string): void {
  const next = new Set(collapsedCategories.value)
  if (next.has(cat)) {
    next.delete(cat)
  } else {
    next.add(cat)
  }
  collapsedCategories.value = next
}

function openEditDialog(mat: Materiau): void {
  editingId.value = mat.id
  form.libelle = mat.libelle
  form.code = mat.code
  form.categorie = mat.categorie
  form.sous_categorie = mat.sous_categorie ?? ''
  form.marques_str = mat.marques?.join(', ') ?? ''
  form.resistance_mpa = mat.resistance_mpa
  form.notes = mat.notes ?? ''
  showNewDialog.value = true
}

function closeDialog(): void {
  showNewDialog.value = false
  editingId.value = null
  saveError.value = null
  form.libelle = ''
  form.code = ''
  form.categorie = ''
  form.sous_categorie = ''
  form.marques_str = ''
  form.resistance_mpa = null
  form.notes = ''
}

async function handleSave(): Promise<void> {
  saving.value = true
  saveError.value = null

  const marques = form.marques_str
    ? form.marques_str.split(',').map((s) => s.trim()).filter(Boolean)
    : null

  const payload = {
    libelle: form.libelle,
    code: form.code,
    categorie: form.categorie,
    sous_categorie: form.sous_categorie || null,
    marques,
    resistance_mpa: form.resistance_mpa,
    notes: form.notes || null,
  }

  try {
    if (editingId.value) {
      const response = await updateMateriau(editingId.value, payload)
      if (response.success && response.data) {
        materiaux.value = materiaux.value.map((m) =>
          m.id === editingId.value ? response.data! : m,
        )
        closeDialog()
      } else {
        saveError.value = response.error ?? 'Erreur lors de la mise à jour'
      }
    } else {
      const response = await createMateriau(payload)
      if (response.success && response.data) {
        materiaux.value = [response.data, ...materiaux.value]
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
    const response = await listMateriaux()
    if (response.success && response.data) {
      materiaux.value = response.data
    }
  } catch {
    // ignore
  } finally {
    loading.value = false
  }
})
</script>
