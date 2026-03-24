<template>
  <div>
    <!-- Loading skeleton -->
    <div v-if="loading" class="space-y-6">
      <Card v-for="i in 4" :key="i">
        <CardHeader>
          <Skeleton class="h-5 w-32" />
        </CardHeader>
        <CardContent class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div v-for="j in 4" :key="j" class="space-y-2">
            <Skeleton class="h-4 w-20" />
            <Skeleton class="h-9 w-full" />
          </div>
        </CardContent>
      </Card>
    </div>

    <template v-else-if="patient">
      <form class="space-y-6" @submit.prevent="handleSave">
        <!-- Identite -->
        <Card>
          <CardHeader class="border-b bg-muted/30">
            <div class="flex items-center gap-2">
              <Icon name="lucide:user" class="h-4 w-4 text-primary" />
              <CardTitle class="text-base font-semibold">Identite</CardTitle>
            </div>
          </CardHeader>
          <CardContent class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-6">
            <div class="space-y-2">
              <Label for="edit-nom">Nom</Label>
              <Input id="edit-nom" v-model="editForm.nom" :disabled="!editing" />
              <p v-if="fieldErrors.nom" class="text-xs text-destructive">
                {{ fieldErrors.nom }}
              </p>
            </div>
            <div class="space-y-2">
              <Label for="edit-prenom">Prenom</Label>
              <Input id="edit-prenom" v-model="editForm.prenom" :disabled="!editing" />
              <p v-if="fieldErrors.prenom" class="text-xs text-destructive">
                {{ fieldErrors.prenom }}
              </p>
            </div>
            <div class="space-y-2">
              <Label for="edit-nom_naissance">Nom de naissance</Label>
              <Input
                id="edit-nom_naissance"
                v-model="editForm.nom_naissance"
                :disabled="!editing"
                placeholder="--"
              />
            </div>
            <div class="space-y-2">
              <Label for="edit-sexe">Sexe</Label>
              <Select v-model="editForm.sexe" :disabled="!editing">
                <SelectTrigger id="edit-sexe">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="M">Masculin</SelectItem>
                  <SelectItem value="F">Feminin</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div class="space-y-2">
              <Label for="edit-date_naissance">Date de naissance</Label>
              <Input
                id="edit-date_naissance"
                v-model="editForm.date_naissance"
                type="date"
                :disabled="!editing"
              />
              <p v-if="fieldErrors.date_naissance" class="text-xs text-destructive">
                {{ fieldErrors.date_naissance }}
              </p>
            </div>
            <div class="space-y-2">
              <Label for="edit-profession">Profession</Label>
              <Input
                id="edit-profession"
                v-model="editForm.profession"
                :disabled="!editing"
                placeholder="--"
              />
            </div>
          </CardContent>
        </Card>

        <!-- Contact -->
        <Card>
          <CardHeader class="border-b bg-muted/30">
            <div class="flex items-center gap-2">
              <Icon name="lucide:phone" class="h-4 w-4 text-primary" />
              <CardTitle class="text-base font-semibold">Contact</CardTitle>
            </div>
          </CardHeader>
          <CardContent class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-6">
            <div class="space-y-2">
              <Label for="edit-telephone">Telephone</Label>
              <Input
                id="edit-telephone"
                v-model="editForm.telephone"
                :disabled="!editing"
                placeholder="--"
              />
            </div>
            <div class="space-y-2">
              <Label for="edit-email">Email</Label>
              <Input
                id="edit-email"
                v-model="editForm.email"
                type="email"
                :disabled="!editing"
                placeholder="--"
              />
            </div>
            <div class="col-span-full space-y-2">
              <Label for="edit-adresse">Adresse</Label>
              <Textarea
                id="edit-adresse"
                v-model="editForm.adresse"
                :disabled="!editing"
                rows="2"
                placeholder="--"
              />
            </div>
          </CardContent>
        </Card>

        <!-- Couverture -->
        <Card>
          <CardHeader class="border-b bg-muted/30">
            <div class="flex items-center gap-2">
              <Icon name="lucide:shield" class="h-4 w-4 text-primary" />
              <CardTitle class="text-base font-semibold">Couverture sociale</CardTitle>
            </div>
          </CardHeader>
          <CardContent class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-6">
            <div class="space-y-2">
              <Label for="edit-num_ss">N. Securite Sociale</Label>
              <Input
                id="edit-num_ss"
                v-model="editForm.num_ss"
                :disabled="!editing"
                placeholder="--"
              />
            </div>
            <div class="space-y-2">
              <Label for="edit-couverture">Type de couverture</Label>
              <Select v-model="editForm.couverture" :disabled="!editing">
                <SelectTrigger id="edit-couverture">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="mutuelle">Mutuelle</SelectItem>
                  <SelectItem value="cmu_c2s">CMU / C2S</SelectItem>
                  <SelectItem value="ame">AME</SelectItem>
                  <SelectItem value="aucune">Aucune</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div class="space-y-2">
              <Label for="edit-mutuelle_nom">Nom de la mutuelle</Label>
              <Input
                id="edit-mutuelle_nom"
                v-model="editForm.mutuelle_nom"
                :disabled="!editing"
                placeholder="--"
              />
            </div>
            <div class="flex items-center gap-2 self-end pb-1">
              <Checkbox
                id="edit-mutuelle_tableau_garantie"
                v-model:checked="editForm.mutuelle_tableau_garantie"
                :disabled="!editing"
              />
              <Label for="edit-mutuelle_tableau_garantie" class="text-sm">
                Tableau de garantie recu
              </Label>
            </div>
          </CardContent>
        </Card>

        <!-- Urgence -->
        <Card>
          <CardHeader class="border-b bg-muted/30">
            <div class="flex items-center gap-2">
              <Icon name="lucide:heart-pulse" class="h-4 w-4 text-primary" />
              <CardTitle class="text-base font-semibold">Contact d'urgence</CardTitle>
            </div>
          </CardHeader>
          <CardContent class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-6">
            <div class="space-y-2">
              <Label for="edit-contact_urgence_nom">Nom</Label>
              <Input
                id="edit-contact_urgence_nom"
                v-model="editForm.contact_urgence_nom"
                :disabled="!editing"
                placeholder="--"
              />
            </div>
            <div class="space-y-2">
              <Label for="edit-contact_urgence_tel">Telephone</Label>
              <Input
                id="edit-contact_urgence_tel"
                v-model="editForm.contact_urgence_tel"
                :disabled="!editing"
                placeholder="--"
              />
            </div>
            <div class="space-y-2">
              <Label for="edit-contact_urgence_lien">Lien</Label>
              <Input
                id="edit-contact_urgence_lien"
                v-model="editForm.contact_urgence_lien"
                :disabled="!editing"
                placeholder="Conjoint, parent..."
              />
            </div>
          </CardContent>
        </Card>

        <!-- Medecin traitant -->
        <Card>
          <CardHeader class="border-b bg-muted/30">
            <div class="flex items-center gap-2">
              <Icon name="lucide:stethoscope" class="h-4 w-4 text-primary" />
              <CardTitle class="text-base font-semibold">Medecin traitant</CardTitle>
            </div>
          </CardHeader>
          <CardContent class="grid grid-cols-1 md:grid-cols-2 gap-4 pt-6">
            <div class="space-y-2">
              <Label for="edit-medecin_traitant_nom">Nom</Label>
              <Input
                id="edit-medecin_traitant_nom"
                v-model="editForm.medecin_traitant_nom"
                :disabled="!editing"
                placeholder="--"
              />
            </div>
            <div class="space-y-2">
              <Label for="edit-medecin_traitant_tel">Telephone</Label>
              <Input
                id="edit-medecin_traitant_tel"
                v-model="editForm.medecin_traitant_tel"
                :disabled="!editing"
                placeholder="--"
              />
            </div>
          </CardContent>
        </Card>

        <!-- Actions -->
        <Alert v-if="saveError" variant="destructive">
          <AlertDescription>{{ saveError }}</AlertDescription>
        </Alert>

        <div class="flex justify-end gap-3 pb-4">
          <Button v-if="!editing" type="button" variant="outline" @click="editing = true">
            <Icon name="lucide:pencil" class="mr-2 h-4 w-4" />
            Modifier
          </Button>
          <template v-else>
            <Button type="button" variant="outline" @click="cancelEditing">
              Annuler
            </Button>
            <Button type="submit" :disabled="saving">
              <Icon v-if="saving" name="lucide:loader-2" class="mr-2 h-4 w-4 animate-spin" />
              Enregistrer
            </Button>
          </template>
        </div>
      </form>
    </template>
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

const route = useRoute()
const patientId = route.params.id as string

const { updatePatient } = usePatient()

const injectedPatient = inject<Ref<Patient | null>>('patient', ref(null))
const patient = injectedPatient
const loading = computed(() => patient.value === null)
const editing = ref(false)
const saving = ref(false)
const saveError = ref<string | null>(null)
const fieldErrors = ref<Record<string, string>>({})

const editForm = reactive({
  nom: '',
  nom_naissance: '',
  prenom: '',
  sexe: '' as 'M' | 'F',
  date_naissance: '',
  num_ss: '',
  profession: '',
  telephone: '',
  email: '',
  adresse: '',
  couverture: '' as 'mutuelle' | 'cmu_c2s' | 'ame' | 'aucune',
  mutuelle_nom: '',
  mutuelle_tableau_garantie: false,
  contact_urgence_nom: '',
  contact_urgence_tel: '',
  contact_urgence_lien: '',
  medecin_traitant_nom: '',
  medecin_traitant_tel: '',
})

function syncFormFromPatient(p: Patient) {
  editForm.nom = p.nom
  editForm.nom_naissance = p.nom_naissance ?? ''
  editForm.prenom = p.prenom
  editForm.sexe = p.sexe
  editForm.date_naissance = p.date_naissance
  editForm.num_ss = p.num_ss ?? ''
  editForm.profession = p.profession ?? ''
  editForm.telephone = p.telephone ?? ''
  editForm.email = p.email ?? ''
  editForm.adresse = p.adresse ?? ''
  editForm.couverture = p.couverture
  editForm.mutuelle_nom = p.mutuelle_nom ?? ''
  editForm.mutuelle_tableau_garantie = p.mutuelle_tableau_garantie ?? false
  editForm.contact_urgence_nom = p.contact_urgence_nom ?? ''
  editForm.contact_urgence_tel = p.contact_urgence_tel ?? ''
  editForm.contact_urgence_lien = p.contact_urgence_lien ?? ''
  editForm.medecin_traitant_nom = p.medecin_traitant_nom ?? ''
  editForm.medecin_traitant_tel = p.medecin_traitant_tel ?? ''
}

function cancelEditing() {
  if (patient.value) syncFormFromPatient(patient.value)
  editing.value = false
  saveError.value = null
  fieldErrors.value = {}
}

async function handleSave() {
  fieldErrors.value = {}
  saveError.value = null

  const result = patientSchema.safeParse({
    nom: editForm.nom,
    prenom: editForm.prenom,
    date_naissance: editForm.date_naissance,
    sexe: editForm.sexe || undefined,
    couverture: editForm.couverture || undefined,
  })

  if (!result.success) {
    for (const issue of result.error.issues) {
      const field = issue.path[0] as string
      if (!fieldErrors.value[field]) fieldErrors.value[field] = issue.message
    }
    return
  }

  saving.value = true

  try {
    const response = await updatePatient(patientId, {
      nom: editForm.nom,
      nom_naissance: editForm.nom_naissance || null,
      prenom: editForm.prenom,
      sexe: editForm.sexe,
      date_naissance: editForm.date_naissance,
      num_ss: editForm.num_ss || null,
      profession: editForm.profession || null,
      telephone: editForm.telephone || null,
      email: editForm.email || null,
      adresse: editForm.adresse || null,
      couverture: editForm.couverture,
      mutuelle_nom: editForm.mutuelle_nom || null,
      mutuelle_tableau_garantie: editForm.mutuelle_tableau_garantie,
      contact_urgence_nom: editForm.contact_urgence_nom || null,
      contact_urgence_tel: editForm.contact_urgence_tel || null,
      contact_urgence_lien: editForm.contact_urgence_lien || null,
      medecin_traitant_nom: editForm.medecin_traitant_nom || null,
      medecin_traitant_tel: editForm.medecin_traitant_tel || null,
    })

    if (response.success && response.data) {
      patient.value = response.data
      syncFormFromPatient(response.data)
      editing.value = false
    } else {
      saveError.value = response.error ?? 'Erreur lors de la sauvegarde'
    }
  } catch (err) {
    saveError.value = err instanceof Error ? err.message : 'Erreur lors de la sauvegarde'
  } finally {
    saving.value = false
  }
}

// Sync form when the injected patient becomes available
watch(
  patient,
  (p) => {
    if (p) syncFormFromPatient(p)
  },
  { immediate: true },
)
</script>
