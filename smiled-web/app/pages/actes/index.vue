<template>
  <div>
    <!-- Page Header -->
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-semibold tracking-tight">Catalogue des actes</h1>
        <p class="text-sm text-muted-foreground mt-1">
          Gerez les actes et tarifs de votre cabinet
          <span v-if="!loading" class="text-muted-foreground/60 ml-1">&middot; {{ actes.length }} acte{{ actes.length !== 1 ? 's' : '' }}</span>
        </p>
      </div>
      <Button class="bg-teal-600 hover:bg-teal-700 text-white" @click="showNewDialog = true">
        <Plus class="w-4 h-4 mr-2" />
        Nouvel acte
      </Button>
    </div>

    <!-- Filter Bar -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center gap-2 mb-4">
      <div class="relative flex-1 max-w-sm">
        <Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
        <Input
          v-model="search"
          placeholder="Rechercher par code ou libelle..."
          class="pl-9 h-9 text-sm"
        />
      </div>

      <!-- Nomenclature badge buttons -->
      <div class="flex items-center gap-1.5">
        <button
          v-for="option in nomenclatureOptions"
          :key="option.value"
          :class="[
            'px-3 py-1.5 rounded-md text-xs font-medium transition-colors',
            filterNomenclature === option.value
              ? 'bg-teal-600 text-white shadow-sm'
              : 'bg-background text-muted-foreground border hover:bg-muted/50',
          ]"
          @click="filterNomenclature = option.value"
        >
          {{ option.label }}
        </button>
      </div>

      <!-- Category dropdown -->
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
    <Alert v-if="tarifError" variant="destructive" class="mb-4">
      <AlertDescription>{{ tarifError }}</AlertDescription>
    </Alert>
    <Alert v-if="toggleError" variant="destructive" class="mb-4">
      <AlertDescription>{{ toggleError }}</AlertDescription>
    </Alert>

    <!-- Loading Skeleton -->
    <Card v-if="loading" class="overflow-hidden">
      <Table>
        <TableHeader>
          <TableRow class="bg-muted/40">
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-28">Code</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground">Libelle</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-36">Nomenclature</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-44">Categorie</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground text-right w-32">Tarif conv.</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground text-right w-32">Tarif cabinet</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground text-center w-20">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow v-for="i in 8" :key="i">
            <TableCell><Skeleton class="h-4 w-16" /></TableCell>
            <TableCell><Skeleton class="h-4 w-48" /></TableCell>
            <TableCell><Skeleton class="h-5 w-16 rounded-full" /></TableCell>
            <TableCell><Skeleton class="h-4 w-32" /></TableCell>
            <TableCell class="text-right"><Skeleton class="h-4 w-16 ml-auto" /></TableCell>
            <TableCell class="text-right"><Skeleton class="h-4 w-16 ml-auto" /></TableCell>
            <TableCell class="text-center"><Skeleton class="h-5 w-5 mx-auto rounded-full" /></TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </Card>

    <!-- Table Card -->
    <Card v-else class="overflow-hidden">
      <Table>
        <TableHeader>
          <TableRow class="bg-muted/40">
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-28">Code</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground">Libelle</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-36">Nomenclature</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-44">Categorie</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground text-right w-32">Tarif conv.</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground text-right w-32">Tarif cabinet</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground text-center w-20">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableEmpty v-if="paginatedActes.length === 0" :colspan="7">
            <div class="py-12 text-center">
              <Stethoscope class="w-10 h-10 mx-auto mb-3 text-muted-foreground/40" />
              <p class="text-sm font-medium text-muted-foreground">Aucun acte trouve</p>
              <p class="text-xs text-muted-foreground/60 mt-1">Essayez de modifier vos filtres</p>
            </div>
          </TableEmpty>
          <TableRow
            v-for="acte in paginatedActes"
            :key="acte.id"
            class="hover:bg-muted/50 transition-colors"
          >
            <TableCell class="font-mono text-xs text-muted-foreground whitespace-nowrap">
              {{ acte.code || '—' }}
            </TableCell>
            <TableCell>
              <p class="text-sm font-medium">{{ acte.libelle }}</p>
            </TableCell>
            <TableCell>
              <span
                :class="[
                  'inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium',
                  nomenclatureBadgeClass(acte.nomenclature),
                ]"
              >
                {{ nomenclatureLabel(acte.nomenclature) }}
              </span>
            </TableCell>
            <TableCell class="text-sm text-muted-foreground">
              {{ acte.categorie || '—' }}
            </TableCell>
            <TableCell class="text-right text-sm tabular-nums">
              {{ formatPrice(acte.prix_defaut) }}
            </TableCell>
            <TableCell class="text-right">
              <div v-if="editingTarifId === acte.id" class="flex items-center gap-1 justify-end">
                <Input
                  v-model.number="editingTarifValue"
                  type="number"
                  min="0"
                  step="0.01"
                  class="w-24 h-7 text-xs text-right"
                  @keyup.enter="saveTarif(acte.id)"
                  @keyup.escape="cancelTarifEdit"
                />
                <Button size="sm" class="h-7 w-7 p-0 bg-teal-600 hover:bg-teal-700" @click="saveTarif(acte.id)">
                  <Check class="w-3 h-3" />
                </Button>
                <Button variant="ghost" size="sm" class="h-7 w-7 p-0" @click="cancelTarifEdit">
                  <X class="w-3 h-3" />
                </Button>
              </div>
              <button
                v-else
                class="text-sm tabular-nums hover:text-teal-600 transition-colors group flex items-center gap-1 ml-auto"
                @click="startTarifEdit(acte.id, acte.prix_cabinet)"
              >
                <span>{{ acte.prix_cabinet !== null ? formatPrice(acte.prix_cabinet) : '—' }}</span>
                <Pencil class="w-3 h-3 opacity-0 group-hover:opacity-100 text-teal-500" />
              </button>
            </TableCell>
            <TableCell class="text-center">
              <button
                :class="[
                  'transition-colors rounded-full p-0.5',
                  acte.actif ? 'text-teal-600 hover:text-teal-700' : 'text-muted-foreground/40 hover:text-muted-foreground',
                ]"
                @click="handleToggle(acte.id)"
              >
                <CheckCircle2 v-if="acte.actif" class="w-5 h-5" />
                <Circle v-else class="w-5 h-5" />
              </button>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="flex items-center justify-between px-4 py-3 border-t bg-muted/30">
        <p class="text-xs text-muted-foreground">
          {{ (currentPage - 1) * pageSize + 1 }}-{{ Math.min(currentPage * pageSize, filteredActes.length) }}
          sur {{ filteredActes.length }} actes
        </p>
        <div class="flex items-center gap-1">
          <Button
            variant="outline"
            size="sm"
            class="h-7 w-7 p-0"
            :disabled="currentPage === 1"
            @click="currentPage = currentPage - 1"
          >
            <ChevronLeft class="w-4 h-4" />
          </Button>
          <template v-for="page in visiblePages" :key="page">
            <Button
              v-if="page !== '...'"
              variant="outline"
              size="sm"
              :class="[
                'h-7 min-w-7 px-2 text-xs',
                currentPage === page
                  ? 'bg-teal-600 text-white border-teal-600 hover:bg-teal-700 hover:text-white'
                  : '',
              ]"
              @click="currentPage = page as number"
            >
              {{ page }}
            </Button>
            <span v-else class="px-1 text-muted-foreground text-xs">...</span>
          </template>
          <Button
            variant="outline"
            size="sm"
            class="h-7 w-7 p-0"
            :disabled="currentPage === totalPages"
            @click="currentPage = currentPage + 1"
          >
            <ChevronRight class="w-4 h-4" />
          </Button>
        </div>
      </div>
    </Card>

    <!-- New Acte Dialog -->
    <Dialog v-model:open="showNewDialog">
      <DialogContent class="max-w-lg">
        <DialogHeader>
          <DialogTitle>Nouvel acte cabinet</DialogTitle>
          <DialogDescription>Creez un acte specifique a votre cabinet.</DialogDescription>
        </DialogHeader>

        <form class="space-y-4" @submit.prevent="handleCreate">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="libelle">Libelle *</Label>
              <Input id="libelle" v-model="form.libelle" required placeholder="Ex : Composite classe II" />
            </div>
            <div class="space-y-2">
              <Label for="code">Code</Label>
              <Input id="code" v-model="form.code" placeholder="Ex : D-ABC" />
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="nomenclature">Nomenclature *</Label>
              <Select v-model="form.nomenclature" required>
                <SelectTrigger id="nomenclature">
                  <SelectValue placeholder="Selectionner" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="ccam">CCAM</SelectItem>
                  <SelectItem value="ngap">NGAP</SelectItem>
                  <SelectItem value="hors_nomenclature">Hors nomenclature</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div class="space-y-2">
              <Label for="prix">Prix (&euro;) *</Label>
              <Input
                id="prix"
                v-model.number="form.prix_defaut"
                type="number"
                min="0"
                step="0.01"
                required
                placeholder="0.00"
              />
            </div>
          </div>

          <div class="space-y-2">
            <Label for="categorie">Categorie</Label>
            <Input id="categorie" v-model="form.categorie" placeholder="Ex : Dentisterie restauratrice" />
          </div>

          <div class="space-y-2">
            <Label for="description">Description</Label>
            <Textarea id="description" v-model="form.description" :rows="2" placeholder="Description de l'acte..." />
          </div>

          <Alert v-if="createError" variant="destructive">
            <AlertDescription>{{ createError }}</AlertDescription>
          </Alert>

          <DialogFooter>
            <Button type="button" variant="outline" @click="showNewDialog = false">Annuler</Button>
            <Button type="submit" :disabled="creating" class="bg-teal-600 hover:bg-teal-700 text-white">
              <Loader2 v-if="creating" class="w-4 h-4 mr-2 animate-spin" />
              Creer l'acte
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import {
  Check,
  CheckCircle2,
  ChevronLeft,
  ChevronRight,
  Circle,
  Loader2,
  Pencil,
  Plus,
  Search,
  Stethoscope,
  X,
} from 'lucide-vue-next'
import type { Acte, Nomenclature } from '~/composables/useActe'
import { formatPrice } from '~/utils/format'

definePageMeta({ layout: 'default' })

const { listActes, createActe, updateTarifActe, toggleActe } = useActe()

const actes = ref<Acte[]>([])
const loading = ref(true)
const showNewDialog = ref(false)
const creating = ref(false)
const createError = ref<string | null>(null)
const search = ref('')
const filterNomenclature = ref('all')
const filterCategory = ref('all')
const editingTarifId = ref<string | null>(null)
const editingTarifValue = ref(0)
const currentPage = ref(1)
const pageSize = 25

const nomenclatureOptions = [
  { value: 'all', label: 'Tous' },
  { value: 'ccam', label: 'CCAM' },
  { value: 'ngap', label: 'NGAP' },
  { value: 'hors_nomenclature', label: 'HN' },
]

const form = reactive({
  libelle: '',
  code: '',
  nomenclature: '' as Nomenclature | '',
  prix_defaut: 0,
  categorie: '',
  description: '',
})

const categories = computed(() => {
  const cats = new Set<string>()
  for (const a of actes.value) {
    if (a.categorie) cats.add(a.categorie)
  }
  return [...cats].sort()
})

const filteredActes = computed(() => {
  return actes.value.filter((a) => {
    const matchSearch =
      !search.value ||
      a.libelle.toLowerCase().includes(search.value.toLowerCase()) ||
      (a.code ?? '').toLowerCase().includes(search.value.toLowerCase())
    const matchNomenclature =
      filterNomenclature.value === 'all' || a.nomenclature === filterNomenclature.value
    const matchCategory =
      filterCategory.value === 'all' || a.categorie === filterCategory.value
    return matchSearch && matchNomenclature && matchCategory
  })
})

const totalPages = computed(() => Math.ceil(filteredActes.value.length / pageSize))

const paginatedActes = computed(() => {
  const start = (currentPage.value - 1) * pageSize
  return filteredActes.value.slice(start, start + pageSize)
})

const visiblePages = computed(() => {
  const total = totalPages.value
  const current = currentPage.value
  if (total <= 7) return Array.from({ length: total }, (_, i) => i + 1)
  const pages: (number | string)[] = [1]
  if (current > 3) pages.push('...')
  for (let i = Math.max(2, current - 1); i <= Math.min(total - 1, current + 1); i++) {
    pages.push(i)
  }
  if (current < total - 2) pages.push('...')
  pages.push(total)
  return pages
})

// Reset page when filters change
watch([search, filterNomenclature, filterCategory], () => {
  currentPage.value = 1
})

function nomenclatureLabel(n: Nomenclature): string {
  const labels: Record<Nomenclature, string> = {
    ccam: 'CCAM',
    ngap: 'NGAP',
    hors_nomenclature: 'Hors nomenclature',
  }
  return labels[n] ?? n
}

function nomenclatureBadgeClass(n: Nomenclature): string {
  const classes: Record<Nomenclature, string> = {
    ccam: 'bg-teal-50 text-teal-700 border border-teal-200',
    ngap: 'bg-blue-50 text-blue-700 border border-blue-200',
    hors_nomenclature: 'bg-amber-50 text-amber-700 border border-amber-200',
  }
  return classes[n] ?? 'bg-gray-50 text-gray-700 border border-gray-200'
}

function startTarifEdit(id: string, current: number | null): void {
  editingTarifId.value = id
  editingTarifValue.value = current ?? 0
}

function cancelTarifEdit(): void {
  editingTarifId.value = null
}

const tarifError = ref<string | null>(null)

async function saveTarif(id: string): Promise<void> {
  tarifError.value = null
  try {
    const response = await updateTarifActe(id, editingTarifValue.value)
    if (response.success && response.data) {
      actes.value = actes.value.map((a) => (a.id === id ? response.data! : a))
    }
  } catch (e) {
    tarifError.value = e instanceof Error ? e.message : 'Erreur lors de la mise a jour du tarif'
  } finally {
    editingTarifId.value = null
  }
}

const toggleError = ref<string | null>(null)

async function handleToggle(id: string): Promise<void> {
  toggleError.value = null
  try {
    const response = await toggleActe(id)
    if (response.success && response.data) {
      actes.value = actes.value.map((a) => (a.id === id ? response.data! : a))
    }
  } catch (e) {
    toggleError.value = e instanceof Error ? e.message : 'Erreur lors du changement de statut'
  }
}

async function handleCreate(): Promise<void> {
  if (!form.nomenclature) return
  creating.value = true
  createError.value = null

  try {
    const response = await createActe({
      libelle: form.libelle,
      code: form.code || null,
      nomenclature: form.nomenclature,
      prix_defaut: form.prix_defaut,
      categorie: form.categorie || null,
      description: form.description || null,
    })

    if (response.success && response.data) {
      actes.value = [response.data, ...actes.value]
      showNewDialog.value = false
      form.libelle = ''
      form.code = ''
      form.nomenclature = ''
      form.prix_defaut = 0
      form.categorie = ''
      form.description = ''
    } else {
      createError.value = response.error ?? 'Erreur lors de la creation'
    }
  } catch (err) {
    createError.value = err instanceof Error ? err.message : 'Erreur lors de la creation'
  } finally {
    creating.value = false
  }
}

const loadError = ref<string | null>(null)

onMounted(async () => {
  try {
    const response = await listActes()
    if (response.success && response.data) {
      actes.value = response.data
    }
  } catch (e) {
    loadError.value = e instanceof Error ? e.message : 'Erreur lors du chargement des actes'
  } finally {
    loading.value = false
  }
})
</script>
