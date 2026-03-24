<template>
  <div>
    <!-- Page Header -->
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-semibold tracking-tight">Materiaux dentaires</h1>
        <p class="text-sm text-muted-foreground mt-1">
          Catalogue des materiaux du cabinet
          <span v-if="!loading" class="text-muted-foreground/60 ml-1">&middot; {{ materiaux.length }} materiau{{ materiaux.length !== 1 ? 'x' : '' }}</span>
        </p>
      </div>
      <Button class="bg-teal-600 hover:bg-teal-700 text-white" @click="showNewDialog = true">
        <Plus class="w-4 h-4 mr-2" />
        Nouveau materiau
      </Button>
    </div>

    <!-- Filter Bar -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center gap-2 mb-4">
      <div class="relative flex-1 max-w-sm">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
        <Input v-model="search" placeholder="Rechercher par nom, code ou categorie..." class="pl-9 h-9 text-sm" />
      </div>
      <Select v-model="filterCategory">
        <SelectTrigger class="w-52 h-9 text-sm">
          <SelectValue placeholder="Toutes les categories" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">Toutes les categories</SelectItem>
          <SelectItem v-for="cat in categories" :key="cat" :value="cat">{{ cat }}</SelectItem>
        </SelectContent>
      </Select>
    </div>

    <!-- Errors -->
    <Alert v-if="loadError" variant="destructive" class="mb-4">
      <AlertDescription>{{ loadError }}</AlertDescription>
    </Alert>

    <!-- Loading Skeleton -->
    <Card v-if="loading" class="overflow-hidden">
      <Table>
        <TableHeader>
          <TableRow class="bg-muted/40">
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-36">Nom</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-44">Categorie</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground">Sous-categorie</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground">Marque</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-28">Reference</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground text-center w-20">Statut</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-12" />
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow v-for="i in 8" :key="i">
            <TableCell><Skeleton class="h-4 w-28" /></TableCell>
            <TableCell><Skeleton class="h-5 w-20 rounded-full" /></TableCell>
            <TableCell><Skeleton class="h-4 w-32" /></TableCell>
            <TableCell><Skeleton class="h-4 w-24" /></TableCell>
            <TableCell><Skeleton class="h-4 w-16" /></TableCell>
            <TableCell class="text-center"><Skeleton class="h-5 w-12 mx-auto rounded-full" /></TableCell>
            <TableCell><Skeleton class="h-4 w-4" /></TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </Card>

    <!-- Table Card -->
    <Card v-else class="overflow-hidden">
      <Table>
        <TableHeader>
          <TableRow class="bg-muted/40">
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-36">Nom</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-44">Categorie</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground">Sous-categorie</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground">Marque</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-28">Reference</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground text-center w-20">Statut</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-12" />
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableEmpty v-if="filteredMateriaux.length === 0" :colspan="7">
            <div class="py-12 text-center">
              <FlaskConical class="w-10 h-10 mx-auto mb-3 text-muted-foreground/40" />
              <p class="text-sm font-medium text-muted-foreground">Aucun materiau trouve</p>
              <p class="text-xs text-muted-foreground/60 mt-1">Essayez de modifier vos filtres</p>
            </div>
          </TableEmpty>
          <TableRow
            v-for="mat in filteredMateriaux"
            :key="mat.id"
            :class="[
              'hover:bg-muted/50 transition-colors',
              mat.obsolete ? 'opacity-50' : '',
            ]"
          >
            <TableCell>
              <p class="text-sm font-medium">{{ mat.libelle }}</p>
            </TableCell>
            <TableCell>
              <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-teal-50 text-teal-700 border border-teal-200">
                {{ mat.categorie }}
              </span>
            </TableCell>
            <TableCell class="text-sm text-muted-foreground">
              {{ mat.sous_categorie ?? '—' }}
            </TableCell>
            <TableCell class="text-sm">
              <span v-if="mat.marques && mat.marques.length > 0">
                {{ mat.marques.slice(0, 2).join(', ') }}
                <span v-if="mat.marques.length > 2" class="text-muted-foreground/60">
                  +{{ mat.marques.length - 2 }}
                </span>
              </span>
              <span v-else class="text-muted-foreground/60">—</span>
            </TableCell>
            <TableCell class="font-mono text-xs text-muted-foreground">
              {{ mat.code }}
            </TableCell>
            <TableCell class="text-center">
              <span
                :class="[
                  'inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium',
                  mat.obsolete
                    ? 'bg-red-50 text-red-600 border border-red-200'
                    : 'bg-emerald-50 text-emerald-700 border border-emerald-200',
                ]"
              >
                {{ mat.obsolete ? 'Obsolete' : 'Actif' }}
              </span>
            </TableCell>
            <TableCell>
              <Button
                v-if="!mat.systeme"
                variant="ghost"
                size="sm"
                class="h-7 w-7 p-0 text-muted-foreground hover:text-teal-600"
                @click="openEditDialog(mat)"
              >
                <Pencil class="w-3.5 h-3.5" />
              </Button>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </Card>

    <!-- New / Edit Dialog -->
    <Dialog v-model:open="showNewDialog">
      <DialogContent class="max-w-lg">
        <DialogHeader>
          <DialogTitle>{{ editingId ? 'Modifier le materiau' : 'Nouveau materiau' }}</DialogTitle>
          <DialogDescription>
            {{ editingId ? 'Modifiez les informations du materiau.' : 'Ajoutez un materiau au catalogue cabinet.' }}
          </DialogDescription>
        </DialogHeader>

        <form class="space-y-4" @submit.prevent="handleSave">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="libelle_mat">Libelle *</Label>
              <Input id="libelle_mat" v-model="form.libelle" required placeholder="Ex : Zircone 3Y" />
            </div>
            <div class="space-y-2">
              <Label for="code_mat">Code *</Label>
              <Input id="code_mat" v-model="form.code" required placeholder="Ex : ZIRC-3Y" />
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="categorie_mat">Categorie *</Label>
              <Input id="categorie_mat" v-model="form.categorie" required placeholder="Ex : Ceramique" />
            </div>
            <div class="space-y-2">
              <Label for="sous_categorie_mat">Sous-categorie</Label>
              <Input id="sous_categorie_mat" v-model="form.sous_categorie" placeholder="Ex : Zircone monolithique" />
            </div>
          </div>

          <div class="space-y-2">
            <Label for="marques_mat">Marques (separees par des virgules)</Label>
            <Input id="marques_mat" v-model="form.marques_str" placeholder="Ex : Vita, Ivoclar, 3M" />
          </div>

          <div class="space-y-2">
            <Label for="resistance_mat">Resistance (MPa)</Label>
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
            <Textarea id="notes_mat" v-model="form.notes" :rows="2" placeholder="Informations complementaires..." />
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
import { FlaskConical, Loader2, Pencil, Plus, Search } from 'lucide-vue-next'
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
const filterCategory = ref('all')

const form = reactive({
  libelle: '',
  code: '',
  categorie: '',
  sous_categorie: '',
  marques_str: '',
  resistance_mpa: null as number | null,
  notes: '',
})

const categories = computed(() => {
  const cats = new Set<string>()
  for (const m of materiaux.value) {
    if (m.categorie) cats.add(m.categorie)
  }
  return [...cats].sort()
})

const filteredMateriaux = computed(() => {
  return materiaux.value.filter((m) => {
    const q = search.value.toLowerCase()
    const matchSearch =
      !search.value ||
      m.libelle.toLowerCase().includes(q) ||
      m.code.toLowerCase().includes(q) ||
      m.categorie.toLowerCase().includes(q)
    const matchCategory =
      filterCategory.value === 'all' || m.categorie === filterCategory.value
    return matchSearch && matchCategory
  })
})

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
        saveError.value = response.error ?? 'Erreur lors de la mise a jour'
      }
    } else {
      const response = await createMateriau(payload)
      if (response.success && response.data) {
        materiaux.value = [response.data, ...materiaux.value]
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
    const response = await listMateriaux()
    if (response.success && response.data) {
      materiaux.value = response.data
    }
  } catch (e) {
    loadError.value = e instanceof Error ? e.message : 'Erreur lors du chargement des materiaux'
  } finally {
    loading.value = false
  }
})
</script>
