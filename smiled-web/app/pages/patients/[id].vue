<template>
  <div class="p-8">
    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-64">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
    </div>

    <!-- Error -->
    <Alert v-else-if="error" variant="destructive">
      <AlertDescription>{{ error }}</AlertDescription>
    </Alert>

    <template v-else-if="patient">
      <!-- Header -->
      <div class="flex items-start justify-between mb-6">
        <div class="flex items-center gap-4">
          <Button variant="ghost" size="sm" @click="navigateTo('/patients')">
            <Icon name="lucide:arrow-left" class="w-4 h-4 mr-2" />
            Retour
          </Button>
          <Separator orientation="vertical" class="h-6" />
          <div>
            <h1 class="text-2xl font-bold">
              {{ patient.nom }} {{ patient.prenom }}
            </h1>
            <div class="flex items-center gap-3 mt-1">
              <span class="text-muted-foreground text-sm">
                {{ patient.sexe === 'M' ? 'M.' : 'Mme' }}
                · {{ age }} ans
                · né{{ patient.sexe === 'F' ? 'e' : '' }} le {{ formatDate(patient.date_naissance) }}
              </span>
              <Badge :variant="couvertureBadgeVariant(patient.couverture)">
                {{ couvertureLabel(patient.couverture) }}
              </Badge>
            </div>
          </div>
        </div>
      </div>

      <!-- Tabs -->
      <Tabs v-model="activeTab">
        <TabsList>
          <TabsTrigger value="fiche">Fiche</TabsTrigger>
          <TabsTrigger value="questionnaire" @click="navigateTo(`/patients/${patient.id}/questionnaire`)">
            Questionnaire
          </TabsTrigger>
          <TabsTrigger value="schema" disabled>Schéma dentaire</TabsTrigger>
          <TabsTrigger value="documents" disabled>Documents</TabsTrigger>
        </TabsList>

        <!-- Fiche Tab -->
        <TabsContent value="fiche" class="mt-6">
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
                </div>
                <div class="space-y-2">
                  <Label for="prenom">Prénom</Label>
                  <Input id="prenom" v-model="editForm.prenom" :disabled="!editing" />
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
              <Button v-if="!editing" type="button" variant="outline" @click="startEditing">
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
        </TabsContent>
      </Tabs>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { Patient } from '~/types/patient'

const route = useRoute()
const patientId = route.params.id as string

const { getPatient, updatePatient } = usePatient()

const patient = ref<Patient | null>(null)
const loading = ref(true)
const error = ref<string | null>(null)
const editing = ref(false)
const saving = ref(false)
const saveError = ref<string | null>(null)
const activeTab = ref('fiche')

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

const age = computed(() => {
  if (!patient.value) return 0
  const today = new Date()
  const birthDate = new Date(patient.value.date_naissance)
  let years = today.getFullYear() - birthDate.getFullYear()
  const monthDiff = today.getMonth() - birthDate.getMonth()
  if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birthDate.getDate())) {
    years--
  }
  return years
})

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

function startEditing() {
  editing.value = true
}

function cancelEditing() {
  if (patient.value) syncFormFromPatient(patient.value)
  editing.value = false
  saveError.value = null
}

async function handleSave() {
  saving.value = true
  saveError.value = null

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

onMounted(async () => {
  try {
    const response = await getPatient(patientId)
    if (response.success && response.data) {
      patient.value = response.data
      syncFormFromPatient(response.data)
    } else {
      error.value = response.error ?? 'Patient introuvable'
    }
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Erreur lors du chargement'
  } finally {
    loading.value = false
  }
})
</script>
