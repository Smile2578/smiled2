<template>
  <div>
    <!-- Page header -->
    <div class="flex items-center justify-between mb-6">
      <div>
        <h1 class="text-2xl font-semibold tracking-tight text-foreground">Patients</h1>
        <p class="mt-1 text-sm text-muted-foreground">
          Gerez vos patients et leurs dossiers
        </p>
      </div>

      <Button @click="showCreateSheet = true">
        <Icon name="lucide:user-plus" class="mr-2 h-4 w-4" />
        Nouveau patient
      </Button>
    </div>

    <!-- Search -->
    <div class="flex gap-2 mb-4">
      <div class="relative flex-1">
        <Icon
          name="lucide:search"
          class="pointer-events-none absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground"
        />
        <Input
          v-model="searchQuery"
          placeholder="Rechercher par nom, prenom..."
          class="pl-10"
        />
      </div>
    </div>

    <!-- Error -->
    <Alert v-if="error" variant="destructive" class="mb-4">
      <AlertDescription>{{ error }}</AlertDescription>
    </Alert>

    <!-- Table -->
    <Card class="overflow-hidden border shadow-sm">
      <Table>
        <TableHeader>
          <TableRow class="bg-muted/50 hover:bg-muted/50">
            <TableHead class="w-[280px] font-semibold">Nom</TableHead>
            <TableHead class="font-semibold">Date de naissance</TableHead>
            <TableHead class="font-semibold">Telephone</TableHead>
            <TableHead class="font-semibold">Email</TableHead>
            <TableHead class="w-[60px] text-right font-semibold">
              <span class="sr-only">Actions</span>
            </TableHead>
          </TableRow>
        </TableHeader>

        <TableBody>
          <!-- Skeleton loading -->
          <template v-if="loading">
            <TableRow v-for="i in 5" :key="`skel-${i}`">
              <TableCell>
                <div class="flex items-center gap-3">
                  <Skeleton class="h-8 w-8 rounded-full" />
                  <Skeleton class="h-4 w-32" />
                </div>
              </TableCell>
              <TableCell><Skeleton class="h-4 w-24" /></TableCell>
              <TableCell><Skeleton class="h-4 w-28" /></TableCell>
              <TableCell><Skeleton class="h-4 w-36" /></TableCell>
              <TableCell><Skeleton class="ml-auto h-8 w-8 rounded-md" /></TableCell>
            </TableRow>
          </template>

          <!-- Empty state -->
          <TableEmpty v-else-if="patients.length === 0" :colspan="5">
            <div class="py-16 text-center">
              <div
                class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-muted"
              >
                <Icon name="lucide:users" class="h-7 w-7 text-muted-foreground" />
              </div>
              <p class="text-base font-medium text-foreground">Aucun patient trouve</p>
              <p class="mt-1 text-sm text-muted-foreground">
                {{
                  searchQuery
                    ? 'Aucun resultat pour cette recherche'
                    : 'Commencez par creer un premier patient'
                }}
              </p>
              <Button
                v-if="!searchQuery"
                class="mt-4"
                @click="showCreateSheet = true"
              >
                <Icon name="lucide:user-plus" class="mr-2 h-4 w-4" />
                Nouveau patient
              </Button>
            </div>
          </TableEmpty>

          <!-- Patient rows -->
          <TableRow
            v-for="patient in patients"
            :key="patient.id"
            class="cursor-pointer transition-colors hover:bg-muted/50"
            @click="navigateTo(`/patients/${patient.id}`)"
          >
            <TableCell>
              <div class="flex items-center gap-3">
                <div
                  class="inline-flex h-8 w-8 shrink-0 items-center justify-center rounded-full bg-primary/10 text-xs font-medium text-primary"
                >
                  {{ patientInitials(patient) }}
                </div>
                <span class="font-medium text-foreground">
                  {{ patient.nom }} {{ patient.prenom }}
                </span>
              </div>
            </TableCell>
            <TableCell class="text-muted-foreground">
              {{ formatDate(patient.date_naissance) }}
            </TableCell>
            <TableCell class="text-muted-foreground">
              {{ patient.telephone ?? '--' }}
            </TableCell>
            <TableCell class="text-muted-foreground">
              {{ patient.email ?? '--' }}
            </TableCell>
            <TableCell class="text-right" @click.stop>
              <DropdownMenu>
                <DropdownMenuTrigger as-child>
                  <Button
                    variant="ghost"
                    size="sm"
                    class="h-8 w-8 p-0 text-muted-foreground hover:text-foreground"
                  >
                    <Icon name="lucide:more-horizontal" class="h-4 w-4" />
                    <span class="sr-only">Actions</span>
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end">
                  <DropdownMenuItem @click="navigateTo(`/patients/${patient.id}`)">
                    <Icon name="lucide:user" class="mr-2 h-4 w-4" />
                    Voir le dossier
                  </DropdownMenuItem>
                  <DropdownMenuItem @click="navigateTo(`/patients/${patient.id}/schema`)">
                    <Icon name="lucide:scan-face" class="mr-2 h-4 w-4" />
                    Schema dentaire
                  </DropdownMenuItem>
                  <DropdownMenuSeparator />
                  <DropdownMenuItem @click="navigateTo(`/patients/${patient.id}/pdts`)">
                    <Icon name="lucide:file-text" class="mr-2 h-4 w-4" />
                    Plans de traitement
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </Card>

    <!-- Pagination -->
    <div
      v-if="totalPatients !== null && totalPatients > 0"
      class="flex items-center justify-between mt-4"
    >
      <p class="text-sm text-muted-foreground">
        {{ totalPatients }} patient{{ totalPatients !== 1 ? 's' : '' }}
      </p>
      <div v-if="totalPages > 1" class="flex items-center gap-2">
        <Button
          variant="outline"
          size="sm"
          :disabled="currentPage <= 1"
          @click="currentPage--"
        >
          Precedent
        </Button>
        <Button
          variant="outline"
          size="sm"
          :disabled="currentPage >= totalPages"
          @click="currentPage++"
        >
          Suivant
        </Button>
      </div>
    </div>

    <!-- Create Patient Sheet -->
    <Sheet v-model:open="showCreateSheet">
      <SheetContent side="right" class="w-full sm:max-w-lg overflow-y-auto">
        <SheetHeader class="mb-6">
          <SheetTitle>Nouveau patient</SheetTitle>
          <SheetDescription>
            Renseignez les informations du patient pour creer sa fiche.
          </SheetDescription>
        </SheetHeader>

        <form class="space-y-5" @submit.prevent="handleCreate">
          <!-- Identity section -->
          <div class="space-y-4">
            <p class="text-sm font-medium text-foreground">Identite</p>
            <div class="grid grid-cols-2 gap-4">
              <div class="space-y-2">
                <Label for="create-nom">Nom *</Label>
                <Input id="create-nom" v-model="form.nom" placeholder="DUPONT" />
                <p v-if="fieldErrors.nom" class="text-xs text-destructive">
                  {{ fieldErrors.nom }}
                </p>
              </div>
              <div class="space-y-2">
                <Label for="create-prenom">Prenom *</Label>
                <Input id="create-prenom" v-model="form.prenom" placeholder="Jean" />
                <p v-if="fieldErrors.prenom" class="text-xs text-destructive">
                  {{ fieldErrors.prenom }}
                </p>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div class="space-y-2">
                <Label for="create-nom_naissance">Nom de naissance</Label>
                <Input
                  id="create-nom_naissance"
                  v-model="form.nom_naissance"
                  placeholder="Nom de jeune fille"
                />
              </div>
              <div class="space-y-2">
                <Label for="create-sexe">Sexe *</Label>
                <Select v-model="form.sexe">
                  <SelectTrigger id="create-sexe">
                    <SelectValue placeholder="Selectionner" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="M">Masculin</SelectItem>
                    <SelectItem value="F">Feminin</SelectItem>
                  </SelectContent>
                </Select>
                <p v-if="fieldErrors.sexe" class="text-xs text-destructive">
                  {{ fieldErrors.sexe }}
                </p>
              </div>
            </div>

            <div class="grid grid-cols-2 gap-4">
              <div class="space-y-2">
                <Label for="create-date_naissance">Date de naissance *</Label>
                <Input id="create-date_naissance" v-model="form.date_naissance" type="date" />
                <p v-if="fieldErrors.date_naissance" class="text-xs text-destructive">
                  {{ fieldErrors.date_naissance }}
                </p>
              </div>
              <div class="space-y-2">
                <Label for="create-couverture">Couverture *</Label>
                <Select v-model="form.couverture">
                  <SelectTrigger id="create-couverture">
                    <SelectValue placeholder="Selectionner" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="mutuelle">Mutuelle</SelectItem>
                    <SelectItem value="cmu_c2s">CMU / C2S</SelectItem>
                    <SelectItem value="ame">AME</SelectItem>
                    <SelectItem value="aucune">Aucune</SelectItem>
                  </SelectContent>
                </Select>
                <p v-if="fieldErrors.couverture" class="text-xs text-destructive">
                  {{ fieldErrors.couverture }}
                </p>
              </div>
            </div>
          </div>

          <!-- Separator -->
          <div class="border-t" />

          <!-- Contact section -->
          <div class="space-y-4">
            <p class="text-sm font-medium text-foreground">Contact</p>
            <div class="grid grid-cols-2 gap-4">
              <div class="space-y-2">
                <Label for="create-telephone">Telephone</Label>
                <Input
                  id="create-telephone"
                  v-model="form.telephone"
                  placeholder="06 12 34 56 78"
                />
              </div>
              <div class="space-y-2">
                <Label for="create-email">Email</Label>
                <Input
                  id="create-email"
                  v-model="form.email"
                  type="email"
                  placeholder="patient@exemple.fr"
                />
              </div>
            </div>
          </div>

          <Alert v-if="createError" variant="destructive">
            <AlertDescription>{{ createError }}</AlertDescription>
          </Alert>

          <SheetFooter class="flex gap-3 pt-4">
            <Button type="button" variant="outline" class="flex-1" @click="showCreateSheet = false">
              Annuler
            </Button>
            <Button
              type="submit"
              :disabled="creating"
              class="flex-1"
            >
              <Icon v-if="creating" name="lucide:loader-2" class="mr-2 h-4 w-4 animate-spin" />
              Creer le patient
            </Button>
          </SheetFooter>
        </form>
      </SheetContent>
    </Sheet>
  </div>
</template>

<script setup lang="ts">
import { z } from 'zod'
import type { Patient } from '~/types/patient'

const patientSchema = z.object({
  nom: z.string().min(1, 'Le nom est requis'),
  prenom: z.string().min(1, 'Le prenom est requis'),
  date_naissance: z.string().min(1, 'La date de naissance est requise'),
  sexe: z.enum(['M', 'F'], { required_error: 'Le sexe est requis' }),
  couverture: z.enum(['mutuelle', 'cmu_c2s', 'ame', 'aucune'], {
    required_error: 'La couverture est requise',
  }),
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
const showCreateSheet = ref(false)
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

function patientInitials(p: Patient): string {
  return `${p.nom?.[0] ?? ''}${p.prenom?.[0] ?? ''}`.toUpperCase()
}

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
      showCreateSheet.value = false
      await router.push(`/patients/${response.data.id}`)
    } else {
      createError.value = response.error ?? 'Erreur lors de la creation'
    }
  } catch (err) {
    createError.value = err instanceof Error ? err.message : 'Erreur lors de la creation'
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
