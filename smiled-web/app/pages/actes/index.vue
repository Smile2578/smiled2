<template>
  <div class="p-8">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold">Actes</h1>
        <p class="text-muted-foreground mt-1">
          {{ actes.length }} acte{{ actes.length !== 1 ? 's' : '' }} au catalogue
        </p>
      </div>
      <Button @click="showNewDialog = true">
        <Icon name="lucide:plus" class="w-4 h-4 mr-2" />
        Nouvel acte
      </Button>
    </div>

    <!-- Search & filter -->
    <div class="flex gap-3 mb-4">
      <div class="relative flex-1 max-w-sm">
        <Icon name="lucide:search" class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
        <Input
          v-model="search"
          placeholder="Rechercher par code ou libellé..."
          class="pl-9"
        />
      </div>
      <Select v-model="filterNomenclature">
        <SelectTrigger class="w-44">
          <SelectValue placeholder="Nomenclature" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="all">Toutes</SelectItem>
          <SelectItem value="ccam">CCAM</SelectItem>
          <SelectItem value="ngap">NGAP</SelectItem>
          <SelectItem value="hors_nomenclature">Hors nomenclature</SelectItem>
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

    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-32">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
    </div>

    <!-- Table -->
    <Card v-else>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead class="w-32">Code</TableHead>
            <TableHead>Libellé</TableHead>
            <TableHead>Nomenclature</TableHead>
            <TableHead class="text-right">Prix défaut</TableHead>
            <TableHead class="text-right">Prix cabinet</TableHead>
            <TableHead class="text-center">Actif</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableEmpty v-if="filteredActes.length === 0" :colspan="6">
            <p class="text-muted-foreground">Aucun acte trouvé</p>
          </TableEmpty>
          <TableRow v-for="acte in filteredActes" :key="acte.id">
            <TableCell class="font-mono text-xs whitespace-nowrap">{{ acte.code || nomenclatureLabel(acte.nomenclature) }}</TableCell>
            <TableCell>
              <div>
                <p class="font-medium text-sm">{{ acte.libelle }}</p>
                <p v-if="acte.categorie" class="text-xs text-muted-foreground">{{ acte.categorie }}</p>
              </div>
            </TableCell>
            <TableCell>
              <Badge :variant="nomenclatureBadge(acte.nomenclature)">
                {{ nomenclatureLabel(acte.nomenclature) }}
              </Badge>
            </TableCell>
            <TableCell class="text-right text-sm">
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
                <Button size="sm" class="h-7 w-7 p-0" @click="saveTarif(acte.id)">
                  <Icon name="lucide:check" class="w-3 h-3" />
                </Button>
                <Button variant="ghost" size="sm" class="h-7 w-7 p-0" @click="cancelTarifEdit">
                  <Icon name="lucide:x" class="w-3 h-3" />
                </Button>
              </div>
              <button
                v-else
                class="text-sm hover:text-primary transition-colors group flex items-center gap-1 ml-auto"
                @click="startTarifEdit(acte.id, acte.prix_cabinet)"
              >
                <span>{{ acte.prix_cabinet !== null ? formatPrice(acte.prix_cabinet) : '—' }}</span>
                <Icon name="lucide:pencil" class="w-3 h-3 opacity-0 group-hover:opacity-100" />
              </button>
            </TableCell>
            <TableCell class="text-center">
              <button
                :class="acte.actif ? 'text-green-600' : 'text-muted-foreground'"
                @click="handleToggle(acte.id)"
              >
                <Icon :name="acte.actif ? 'lucide:check-circle-2' : 'lucide:circle'" class="w-5 h-5" />
              </button>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </Card>

    <!-- New Acte Dialog -->
    <Dialog v-model:open="showNewDialog">
      <DialogContent class="max-w-lg">
        <DialogHeader>
          <DialogTitle>Nouvel acte cabinet</DialogTitle>
          <DialogDescription>Créez un acte spécifique à votre cabinet.</DialogDescription>
        </DialogHeader>

        <form class="space-y-4" @submit.prevent="handleCreate">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="libelle">Libellé *</Label>
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
                  <SelectValue placeholder="Sélectionner" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="ccam">CCAM</SelectItem>
                  <SelectItem value="ngap">NGAP</SelectItem>
                  <SelectItem value="hors_nomenclature">Hors nomenclature</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div class="space-y-2">
              <Label for="prix">Prix (€) *</Label>
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
            <Label for="categorie">Catégorie</Label>
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
            <Button type="submit" :disabled="creating">
              <Icon v-if="creating" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
              Créer l'acte
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
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
const editingTarifId = ref<string | null>(null)
const editingTarifValue = ref(0)

const form = reactive({
  libelle: '',
  code: '',
  nomenclature: '' as Nomenclature | '',
  prix_defaut: 0,
  categorie: '',
  description: '',
})

const filteredActes = computed(() => {
  return actes.value.filter((a) => {
    const matchSearch =
      !search.value ||
      a.libelle.toLowerCase().includes(search.value.toLowerCase()) ||
      (a.code ?? '').toLowerCase().includes(search.value.toLowerCase())
    const matchNomenclature =
      filterNomenclature.value === 'all' || a.nomenclature === filterNomenclature.value
    return matchSearch && matchNomenclature
  })
})

function nomenclatureLabel(n: Nomenclature): string {
  const labels: Record<Nomenclature, string> = {
    ccam: 'CCAM',
    ngap: 'NGAP',
    hors_nomenclature: 'Hors nomenclature',
  }
  return labels[n] ?? n
}

function nomenclatureBadge(n: Nomenclature): 'default' | 'secondary' | 'outline' {
  const variants: Record<Nomenclature, 'default' | 'secondary' | 'outline'> = {
    ccam: 'default',
    ngap: 'secondary',
    hors_nomenclature: 'outline',
  }
  return variants[n] ?? 'outline'
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
    tarifError.value = e instanceof Error ? e.message : 'Erreur lors de la mise à jour du tarif'
    console.error('Failed to save tarif:', e)
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
    console.error('Failed to toggle acte:', e)
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
      createError.value = response.error ?? 'Erreur lors de la création'
    }
  } catch (err) {
    createError.value = err instanceof Error ? err.message : 'Erreur lors de la création'
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
    console.error('Failed to load actes:', e)
  } finally {
    loading.value = false
  }
})
</script>
