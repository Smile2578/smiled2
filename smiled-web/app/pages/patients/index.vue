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
              <p v-if="fieldErrors.nom" class="text-xs text-destructive">{{ fieldErrors.nom }}</p>
            </div>
            <div class="space-y-2">
              <Label for="prenom">Prénom *</Label>
              <Input id="prenom" v-model="form.prenom" required placeholder="Jean" />
              <p v-if="fieldErrors.prenom" class="text-xs text-destructive">{{ fieldErrors.prenom }}</p>
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
              <p v-if="fieldErrors.sexe" class="text-xs text-destructive">{{ fieldErrors.sexe }}</p>
            </div>
          </div>

          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="date_naissance">Date de naissance *</Label>
              <Input id="date_naissance" v-model="form.date_naissance" type="date" required />
              <p v-if="fieldErrors.date_naissance" class="text-xs text-destructive">{{ fieldErrors.date_naissance }}</p>
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
              <p v-if="fieldErrors.couverture" class="text-xs text-destructive">{{ fieldErrors.couverture }}</p>
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
import { z } from 'zod'
import type { Patient } from '~/types/patient'
import { couvertureLabel, couvertureBadgeVariant } from '~/utils/display'

const patientSchema = z.object({
  nom: z.string().min(1, 'Le nom est requis'),
  prenom: z.string().min(1, 'Le prénom est requis'),
  date_naissance: z.string().min(1, 'La date de naissance est requise'),
  sexe: z.enum(['M', 'F'], { required_error: 'Le sexe est requis' }),
  couverture: z.enum(['mutuelle', 'cmu_c2s', 'ame', 'aucune'], { required_error: 'La couverture est requise' }),
})

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
const fieldErrors = ref<Record<string, string>>({})

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
  fieldErrors.value = {}
  createError.value = null

  const result = patientSchema.safeParse({
    nom: form.nom,
    prenom: form.prenom,
    date_naissance: form.date_naissance,
    sexe: form.sexe || undefined,
    couverture: form.couverture || undefined,
  })

  if (!result.success) {
    for (const issue of result.error.issues) {
      const field = issue.path[0] as string
      if (!fieldErrors.value[field]) fieldErrors.value[field] = issue.message
    }
    return
  }

  creating.value = true

  try {
    const response = await createPatient({
      nom: result.data.nom,
      nom_naissance: form.nom_naissance || null,
      prenom: result.data.prenom,
      sexe: result.data.sexe,
      date_naissance: result.data.date_naissance,
      couverture: result.data.couverture,
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

watch([debouncedSearch, currentPage], fetchPatients, { immediate: true })

watch(debouncedSearch, () => {
  currentPage.value = 1
})
</script>
