<template>
  <div class="p-8">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-bold">Patients</h1>
        <p class="text-muted-foreground mt-1">
          {{ totalPatients !== null ? `${totalPatients} patient${totalPatients !== 1 ? 's' : ''}` : 'Chargement...' }}
        </p>
      </div>
      <Button @click="showCreateDialog = true">
        <Icon name="lucide:user-plus" class="w-4 h-4 mr-2" />
        Nouveau patient
      </Button>
    </div>

    <!-- Search -->
    <div class="mb-4">
      <div class="relative max-w-sm">
        <Icon name="lucide:search" class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
        <Input
          v-model="searchQuery"
          placeholder="Rechercher un patient..."
          class="pl-9"
        />
      </div>
    </div>

    <!-- Error -->
    <Alert v-if="error" variant="destructive" class="mb-4">
      <AlertDescription>{{ error }}</AlertDescription>
    </Alert>

    <!-- Table -->
    <Card>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Nom</TableHead>
            <TableHead>Prénom</TableHead>
            <TableHead>Date de naissance</TableHead>
            <TableHead>Téléphone</TableHead>
            <TableHead>Couverture</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableEmpty v-if="loading" :colspan="5">
            <div class="flex items-center justify-center gap-2 text-muted-foreground">
              <Icon name="lucide:loader-2" class="w-4 h-4 animate-spin" />
              Chargement...
            </div>
          </TableEmpty>
          <TableEmpty v-else-if="patients.length === 0" :colspan="5">
            <p class="text-muted-foreground">Aucun patient trouvé</p>
          </TableEmpty>
          <TableRow
            v-for="patient in patients"
            :key="patient.id"
            class="cursor-pointer hover:bg-muted/50"
            @click="navigateTo(`/patients/${patient.id}`)"
          >
            <TableCell class="font-medium">{{ patient.nom }}</TableCell>
            <TableCell>{{ patient.prenom }}</TableCell>
            <TableCell>{{ formatDate(patient.date_naissance) }}</TableCell>
            <TableCell>{{ patient.telephone ?? '—' }}</TableCell>
            <TableCell>
              <Badge :variant="couvertureBadgeVariant(patient.couverture)">
                {{ couvertureLabel(patient.couverture) }}
              </Badge>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </Card>

    <!-- Pagination -->
    <div v-if="totalPages > 1" class="flex items-center justify-between mt-4">
      <p class="text-sm text-muted-foreground">
        Page {{ currentPage }} sur {{ totalPages }}
      </p>
      <div class="flex gap-2">
        <Button
          variant="outline"
          size="sm"
          :disabled="currentPage <= 1"
          @click="currentPage--"
        >
          <Icon name="lucide:chevron-left" class="w-4 h-4" />
          Précédent
        </Button>
        <Button
          variant="outline"
          size="sm"
          :disabled="currentPage >= totalPages"
          @click="currentPage++"
        >
          Suivant
          <Icon name="lucide:chevron-right" class="w-4 h-4 ml-1" />
        </Button>
      </div>
    </div>

    <!-- Create Patient Dialog -->
    <Dialog v-model:open="showCreateDialog">
      <DialogContent class="max-w-2xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Nouveau patient</DialogTitle>
          <DialogDescription>
            Renseignez les informations du patient pour créer sa fiche.
          </DialogDescription>
        </DialogHeader>

        <form class="space-y-4" @submit.prevent="handleCreate">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="nom">Nom *</Label>
              <Input id="nom" v-model="form.nom" required placeholder="DUPONT" />
            </div>
            <div class="space-y-2">
              <Label for="prenom">Prénom *</Label>
              <Input id="prenom" v-model="form.prenom" required placeholder="Jean" />
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="nom_naissance">Nom de naissance</Label>
              <Input id="nom_naissance" v-model="form.nom_naissance" placeholder="Nom de jeune fille" />
            </div>
            <div class="space-y-2">
              <Label for="sexe">Sexe *</Label>
              <Select v-model="form.sexe" required>
                <SelectTrigger id="sexe">
                  <SelectValue placeholder="Sélectionner" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="M">Masculin</SelectItem>
                  <SelectItem value="F">Féminin</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="date_naissance">Date de naissance *</Label>
              <Input id="date_naissance" v-model="form.date_naissance" type="date" required />
            </div>
            <div class="space-y-2">
              <Label for="couverture">Couverture *</Label>
              <Select v-model="form.couverture" required>
                <SelectTrigger id="couverture">
                  <SelectValue placeholder="Sélectionner" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="mutuelle">Mutuelle</SelectItem>
                  <SelectItem value="cmu_c2s">CMU / C2S</SelectItem>
                  <SelectItem value="ame">AME</SelectItem>
                  <SelectItem value="aucune">Aucune</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="telephone">Téléphone</Label>
              <Input id="telephone" v-model="form.telephone" placeholder="06 12 34 56 78" />
            </div>
            <div class="space-y-2">
              <Label for="email">Email</Label>
              <Input id="email" v-model="form.email" type="email" placeholder="patient@exemple.fr" />
            </div>
          </div>

          <Alert v-if="createError" variant="destructive">
            <AlertDescription>{{ createError }}</AlertDescription>
          </Alert>

          <DialogFooter>
            <Button type="button" variant="outline" @click="showCreateDialog = false">
              Annuler
            </Button>
            <Button type="submit" :disabled="creating">
              <Icon v-if="creating" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
              Créer le patient
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import type { Patient } from '~/types/patient'

const { listPatients, createPatient } = usePatient()
const router = useRouter()

const patients = ref<Patient[]>([])
const totalPatients = ref<number | null>(null)
const currentPage = ref(1)
const pageLimit = 20
const loading = ref(false)
const error = ref<string | null>(null)
const searchQuery = ref('')
const showCreateDialog = ref(false)
const creating = ref(false)
const createError = ref<string | null>(null)

const totalPages = computed(() =>
  totalPatients.value !== null ? Math.ceil(totalPatients.value / pageLimit) : 1,
)

const form = reactive({
  nom: '',
  nom_naissance: '',
  prenom: '',
  sexe: '' as 'M' | 'F' | '',
  date_naissance: '',
  couverture: '' as 'mutuelle' | 'cmu_c2s' | 'ame' | 'aucune' | '',
  telephone: '',
  email: '',
})

const debouncedSearch = useDebounce(searchQuery, 300)

async function fetchPatients() {
  loading.value = true
  error.value = null

  try {
    const response = await listPatients({
      search: debouncedSearch.value || undefined,
      page: currentPage.value,
      limit: pageLimit,
    })

    if (response.success && response.data) {
      patients.value = response.data
      totalPatients.value = response.meta?.total ?? response.data.length
    } else {
      error.value = response.error ?? 'Erreur lors du chargement des patients'
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Erreur lors du chargement des patients'
  } finally {
    loading.value = false
  }
}

async function handleCreate() {
  if (!form.sexe || !form.couverture) return
  creating.value = true
  createError.value = null

  try {
    const response = await createPatient({
      nom: form.nom,
      nom_naissance: form.nom_naissance || null,
      prenom: form.prenom,
      sexe: form.sexe as 'M' | 'F',
      date_naissance: form.date_naissance,
      couverture: form.couverture as 'mutuelle' | 'cmu_c2s' | 'ame' | 'aucune',
      telephone: form.telephone || null,
      email: form.email || null,
    })

    if (response.success && response.data) {
      showCreateDialog.value = false
      await router.push(`/patients/${response.data.id}`)
    } else {
      createError.value = response.error ?? 'Erreur lors de la création'
    }
  } catch (err) {
    createError.value = err instanceof Error ? err.message : 'Erreur lors de la création'
  } finally {
    creating.value = false
  }
}

function formatDate(dateStr: string): string {
  try {
    return new Date(dateStr).toLocaleDateString('fr-FR')
  } catch {
    return dateStr
  }
}

function couvertureLabel(couverture: string): string {
  const labels: Record<string, string> = {
    mutuelle: 'Mutuelle',
    cmu_c2s: 'CMU / C2S',
    ame: 'AME',
    aucune: 'Aucune',
  }
  return labels[couverture] ?? couverture
}

function couvertureBadgeVariant(couverture: string): 'default' | 'secondary' | 'destructive' | 'outline' {
  const variants: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    mutuelle: 'default',
    cmu_c2s: 'secondary',
    ame: 'secondary',
    aucune: 'outline',
  }
  return variants[couverture] ?? 'outline'
}

watch([debouncedSearch, currentPage], fetchPatients, { immediate: true })

watch(debouncedSearch, () => {
  currentPage.value = 1
})
</script>
