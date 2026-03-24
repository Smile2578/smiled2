<template>
  <div>
    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-64">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
    </div>

    <template v-else-if="patient">
      <form class="space-y-6" @submit.prevent="handleSave">
        <!-- Identity -->
        <Card>
          <CardHeader>
            <CardTitle class="text-base">Identité</CardTitle>
          </CardHeader>
          <CardContent class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="nom">Nom</Label>
              <Input id="nom" v-model="editForm.nom" :disabled="!editing" />
              <p v-if="fieldErrors.nom" class="text-xs text-destructive">{{ fieldErrors.nom }}</p>
            </div>
            <div class="space-y-2">
              <Label for="prenom">Prénom</Label>
              <Input id="prenom" v-model="editForm.prenom" :disabled="!editing" />
              <p v-if="fieldErrors.prenom" class="text-xs text-destructive">{{ fieldErrors.prenom }}</p>
            </div>
            <div class="space-y-2">
              <Label for="nom_naissance">Nom de naissance</Label>
              <Input id="nom_naissance" v-model="editForm.nom_naissance" :disabled="!editing" placeholder="—" />
            </div>
            <div class="space-y-2">
              <Label for="sexe">Sexe</Label>
              <Select v-model="editForm.sexe" :disabled="!editing">
                <SelectTrigger id="sexe">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="M">Masculin</SelectItem>
                  <SelectItem value="F">Féminin</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div class="space-y-2">
              <Label for="date_naissance">Date de naissance</Label>
              <Input id="date_naissance" v-model="editForm.date_naissance" type="date" :disabled="!editing" />
              <p v-if="fieldErrors.date_naissance" class="text-xs text-destructive">{{ fieldErrors.date_naissance }}</p>
            </div>
            <div class="space-y-2">
              <Label for="num_ss">N° Sécurité Sociale</Label>
              <Input id="num_ss" v-model="editForm.num_ss" :disabled="!editing" placeholder="—" />
            </div>
            <div class="space-y-2">
              <Label for="profession">Profession</Label>
              <Input id="profession" v-model="editForm.profession" :disabled="!editing" placeholder="—" />
            </div>
          </CardContent>
        </Card>

        <!-- Contact -->
        <Card>
          <CardHeader>
            <CardTitle class="text-base">Coordonnées</CardTitle>
          </CardHeader>
          <CardContent class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="telephone">Téléphone</Label>
              <Input id="telephone" v-model="editForm.telephone" :disabled="!editing" placeholder="—" />
            </div>
            <div class="space-y-2">
              <Label for="email">Email</Label>
              <Input id="email" v-model="editForm.email" type="email" :disabled="!editing" placeholder="—" />
            </div>
            <div class="col-span-2 space-y-2">
              <Label for="adresse">Adresse</Label>
              <Textarea id="adresse" v-model="editForm.adresse" :disabled="!editing" rows="2" placeholder="—" />
            </div>
          </CardContent>
        </Card>

        <!-- Coverage -->
        <Card>
          <CardHeader>
            <CardTitle class="text-base">Couverture sociale</CardTitle>
          </CardHeader>
          <CardContent class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="couverture">Type de couverture</Label>
              <Select v-model="editForm.couverture" :disabled="!editing">
                <SelectTrigger id="couverture">
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
              <Label for="mutuelle_nom">Nom de la mutuelle</Label>
              <Input id="mutuelle_nom" v-model="editForm.mutuelle_nom" :disabled="!editing" placeholder="—" />
            </div>
            <div class="flex items-center gap-2">
              <Checkbox
                id="mutuelle_tableau_garantie"
                v-model:checked="editForm.mutuelle_tableau_garantie"
                :disabled="!editing"
              />
              <Label for="mutuelle_tableau_garantie">Tableau de garantie reçu</Label>
            </div>
          </CardContent>
        </Card>

        <!-- Emergency contact -->
        <Card>
          <CardHeader>
            <CardTitle class="text-base">Contact d'urgence</CardTitle>
          </CardHeader>
          <CardContent class="grid grid-cols-3 gap-4">
            <div class="space-y-2">
              <Label for="contact_urgence_nom">Nom</Label>
              <Input id="contact_urgence_nom" v-model="editForm.contact_urgence_nom" :disabled="!editing" placeholder="—" />
            </div>
            <div class="space-y-2">
              <Label for="contact_urgence_tel">Téléphone</Label>
              <Input id="contact_urgence_tel" v-model="editForm.contact_urgence_tel" :disabled="!editing" placeholder="—" />
            </div>
            <div class="space-y-2">
              <Label for="contact_urgence_lien">Lien</Label>
              <Input id="contact_urgence_lien" v-model="editForm.contact_urgence_lien" :disabled="!editing" placeholder="Conjoint, parent..." />
            </div>
          </CardContent>
        </Card>

        <!-- Médecin traitant -->
        <Card>
          <CardHeader>
            <CardTitle class="text-base">Médecin traitant</CardTitle>
          </CardHeader>
          <CardContent class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="medecin_traitant_nom">Nom</Label>
              <Input id="medecin_traitant_nom" v-model="editForm.medecin_traitant_nom" :disabled="!editing" placeholder="—" />
            </div>
            <div class="space-y-2">
              <Label for="medecin_traitant_tel">Téléphone</Label>
              <Input id="medecin_traitant_tel" v-model="editForm.medecin_traitant_tel" :disabled="!editing" placeholder="—" />
            </div>
          </CardContent>
        </Card>

        <!-- Actions -->
        <Alert v-if="saveError" variant="destructive">
          <AlertDescription>{{ saveError }}</AlertDescription>
        </Alert>

        <div class="flex justify-end gap-3">
          <Button v-if="!editing" type="button" variant="outline" @click="editing = true">
            <Icon name="lucide:pencil" class="w-4 h-4 mr-2" />
            Modifier
          </Button>
          <template v-else>
            <Button type="button" variant="outline" @click="cancelEditing">
              Annuler
            </Button>
            <Button type="submit" :disabled="saving">
              <Icon v-if="saving" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
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
  prenom: z.string().min(1, 'Le prénom est requis'),
  date_naissance: z.string().min(1, 'La date de naissance est requise'),
  sexe: z.enum(['M', 'F'], { required_error: 'Le sexe est requis' }),
  couverture: z.enum(['mutuelle', 'cmu_c2s', 'ame', 'aucune'], { required_error: 'La couverture est requise' }),
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
watch(patient, (p) => {
  if (p) syncFormFromPatient(p)
}, { immediate: true })
</script>
