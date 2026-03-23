<template>
  <div class="p-8">
    <!-- Header -->
    <div class="flex items-center gap-4 mb-6">
      <Button variant="ghost" size="sm" @click="navigateTo(`/patients/${patientId}`)">
        <Icon name="lucide:arrow-left" class="w-4 h-4 mr-2" />
        Retour à la fiche
      </Button>
      <Separator orientation="vertical" class="h-6" />
      <div>
        <h1 class="text-2xl font-bold">Questionnaire médical</h1>
        <p v-if="questionnaire" class="text-sm text-muted-foreground">
          Version {{ questionnaire.version }}
          · Mis à jour le {{ formatDate(questionnaire.updated_at) }}
        </p>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-64">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
    </div>

    <template v-else>
      <!-- Signature info -->
      <Card class="mb-6">
        <CardHeader>
          <CardTitle class="text-base">Informations de signature</CardTitle>
        </CardHeader>
        <CardContent class="grid grid-cols-3 gap-4">
          <div class="space-y-2">
            <Label for="date_signature">Date de signature</Label>
            <Input id="date_signature" v-model="form.date_signature" type="date" />
          </div>
          <div class="space-y-2">
            <Label for="nom_signataire">Signataire (nom)</Label>
            <Input id="nom_signataire" v-model="form.nom_signataire" placeholder="Nom du patient ou représentant" />
          </div>
          <div class="space-y-2">
            <Label for="prochaine_maj">Prochaine mise à jour</Label>
            <Input id="prochaine_maj" v-model="form.prochaine_maj" type="date" />
          </div>
        </CardContent>
      </Card>

      <!-- Sections -->
      <div class="space-y-4">
        <SectionHemoragique v-model="hemoData" />
        <SectionInfectieux v-model="infectieuxData" />
        <SectionMedicamenteux v-model="medicamenteuxData" />
        <SectionTroubles v-model="troublesData" />
        <SectionHabitudes v-model="habitudesData" />
        <SectionDentaire v-model="dentaireData" />
      </div>

      <!-- Save -->
      <Alert v-if="saveError" variant="destructive" class="mt-6">
        <AlertDescription>{{ saveError }}</AlertDescription>
      </Alert>

      <Alert v-if="saveSuccess" class="mt-6">
        <Icon name="lucide:check-circle" class="w-4 h-4" />
        <AlertDescription>Questionnaire enregistré avec succès.</AlertDescription>
      </Alert>

      <div class="flex justify-end mt-6">
        <Button :disabled="saving" @click="handleSave">
          <Icon v-if="saving" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
          <Icon v-else name="lucide:save" class="w-4 h-4 mr-2" />
          Enregistrer le questionnaire
        </Button>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import type { Questionnaire } from '~/types/patient'
import type { HemosectionData } from '~/components/questionnaire/SectionHemoragique.vue'
import type { InfectieuxSectionData } from '~/components/questionnaire/SectionInfectieux.vue'
import type { MedicamenteuxSectionData } from '~/components/questionnaire/SectionMedicamenteux.vue'
import type { TroublesSectionData } from '~/components/questionnaire/SectionTroubles.vue'
import type { HabitudesSectionData } from '~/components/questionnaire/SectionHabitudes.vue'
import type { DentaireSectionData } from '~/components/questionnaire/SectionDentaire.vue'

const route = useRoute()
const patientId = route.params.id as string
const { getQuestionnaire, saveQuestionnaire } = usePatient()

const questionnaire = ref<Questionnaire | null>(null)
const loading = ref(true)
const saving = ref(false)
const saveError = ref<string | null>(null)
const saveSuccess = ref(false)

const form = reactive({
  date_signature: '',
  nom_signataire: '',
  prochaine_maj: '',
})

const hemoData = ref<HemosectionData>({
  avk: false,
  avk_molecule: '',
  aod_molecule: '',
  antiagregants: false,
  antiagregants_molecule: '',
  hemostase: false,
  hemostase_detail: '',
})

const infectieuxData = ref<InfectieuxSectionData>({
  endocardite: false,
  immunodepression: false,
  immunodepression_detail: '',
  protheses_articulaires: false,
  protheses_articulaires_detail: '',
})

const medicamenteuxData = ref<MedicamenteuxSectionData>({
  bisphosphonates: false,
  bisphosphonates_molecule: '',
  antiresorptifs: false,
  antiresorptifs_molecule: '',
  radiotherapie: false,
  radiotherapie_zone: '',
})

const troublesData = ref<TroublesSectionData>({
  troubles: '',
  medicaments: '',
  allergies: false,
  allergies_detail: '',
})

const habitudesData = ref<HabitudesSectionData>({
  tabac: '',
  alcool: '',
  grossesse_mois: '',
  allaitement: false,
  bruxisme: '',
  sahos: false,
  sahos_traitement: '',
  rgo: false,
})

const dentaireData = ref<DentaireSectionData>({
  dernier_rdv_date: '',
  brossage_quotidien: '',
  auxiliaires: [],
  historique_connu: [],
  apprehension: '',
})

function formatDate(dateStr: string): string {
  try {
    return new Date(dateStr).toLocaleDateString('fr-FR')
  } catch {
    return dateStr
  }
}

function hydrateFromQuestionnaire(q: Questionnaire) {
  form.date_signature = q.date_signature ?? ''
  form.nom_signataire = q.nom_signataire ?? ''
  form.prochaine_maj = q.prochaine_maj ?? ''

  const avkData = q.avk as { actif?: boolean; molecule?: string } | null
  hemoData.value = {
    avk: avkData?.actif ?? false,
    avk_molecule: avkData?.molecule ?? '',
    aod_molecule: q.aod_molecule ?? '',
    antiagregants: !!(q.antiagregants as { actif?: boolean } | null)?.actif,
    antiagregants_molecule: (q.antiagregants as { molecule?: string } | null)?.molecule ?? '',
    hemostase: !!(q.hemostase as { actif?: boolean } | null)?.actif,
    hemostase_detail: (q.hemostase as { detail?: string } | null)?.detail ?? '',
  }

  infectieuxData.value = {
    endocardite: !!(q.endocardite as { actif?: boolean } | null)?.actif,
    immunodepression: !!(q.immunodepression as { actif?: boolean } | null)?.actif,
    immunodepression_detail: (q.immunodepression as { detail?: string } | null)?.detail ?? '',
    protheses_articulaires: !!(q.protheses_articulaires as { actif?: boolean } | null)?.actif,
    protheses_articulaires_detail: (q.protheses_articulaires as { detail?: string } | null)?.detail ?? '',
  }

  medicamenteuxData.value = {
    bisphosphonates: !!(q.bisphosphonates as { actif?: boolean } | null)?.actif,
    bisphosphonates_molecule: (q.bisphosphonates as { molecule?: string } | null)?.molecule ?? '',
    antiresorptifs: !!(q.antiresorptifs as { actif?: boolean } | null)?.actif,
    antiresorptifs_molecule: (q.antiresorptifs as { molecule?: string } | null)?.molecule ?? '',
    radiotherapie: !!(q.radiotherapie as { actif?: boolean } | null)?.actif,
    radiotherapie_zone: (q.radiotherapie as { zone?: string } | null)?.zone ?? '',
  }

  troublesData.value = {
    troubles: (q.troubles as { texte?: string } | null)?.texte ?? '',
    medicaments: q.medicaments?.join('\n') ?? '',
    allergies: !!(q.allergies as { actif?: boolean } | null)?.actif,
    allergies_detail: (q.allergies as { detail?: string } | null)?.detail ?? '',
  }

  habitudesData.value = {
    tabac: q.tabac ?? '',
    alcool: q.alcool ?? '',
    grossesse_mois: q.grossesse_mois != null ? String(q.grossesse_mois) : '',
    allaitement: q.allaitement ?? false,
    bruxisme: q.bruxisme ?? '',
    sahos: !!(q.sahos as { actif?: boolean } | null)?.actif,
    sahos_traitement: (q.sahos as { traitement?: string } | null)?.traitement ?? '',
    rgo: q.rgo ?? false,
  }

  dentaireData.value = {
    dernier_rdv_date: q.dernier_rdv_date ?? '',
    brossage_quotidien: q.brossage_quotidien != null ? String(q.brossage_quotidien) : '',
    auxiliaires: q.auxiliaires ?? [],
    historique_connu: q.historique_connu ?? [],
    apprehension: q.apprehension ?? '',
  }
}

function buildPayload() {
  return {
    date_signature: form.date_signature || null,
    nom_signataire: form.nom_signataire || null,
    prochaine_maj: form.prochaine_maj || null,
    avk: hemoData.value.avk
      ? { actif: true, molecule: hemoData.value.avk_molecule }
      : null,
    aod_molecule: hemoData.value.aod_molecule || null,
    antiagregants: hemoData.value.antiagregants
      ? { actif: true, molecule: hemoData.value.antiagregants_molecule }
      : null,
    hemostase: hemoData.value.hemostase
      ? { actif: true, detail: hemoData.value.hemostase_detail }
      : null,
    endocardite: infectieuxData.value.endocardite ? { actif: true } : null,
    immunodepression: infectieuxData.value.immunodepression
      ? { actif: true, detail: infectieuxData.value.immunodepression_detail }
      : null,
    protheses_articulaires: infectieuxData.value.protheses_articulaires
      ? { actif: true, detail: infectieuxData.value.protheses_articulaires_detail }
      : null,
    bisphosphonates: medicamenteuxData.value.bisphosphonates
      ? { actif: true, molecule: medicamenteuxData.value.bisphosphonates_molecule }
      : null,
    antiresorptifs: medicamenteuxData.value.antiresorptifs
      ? { actif: true, molecule: medicamenteuxData.value.antiresorptifs_molecule }
      : null,
    radiotherapie: medicamenteuxData.value.radiotherapie
      ? { actif: true, zone: medicamenteuxData.value.radiotherapie_zone }
      : null,
    troubles: troublesData.value.troubles ? { texte: troublesData.value.troubles } : null,
    medicaments: troublesData.value.medicaments
      ? troublesData.value.medicaments.split(/[\n,]/).map((m) => m.trim()).filter(Boolean)
      : null,
    allergies: troublesData.value.allergies
      ? { actif: true, detail: troublesData.value.allergies_detail }
      : null,
    tabac: habitudesData.value.tabac || null,
    alcool: habitudesData.value.alcool || null,
    grossesse_mois: habitudesData.value.grossesse_mois
      ? Number(habitudesData.value.grossesse_mois)
      : null,
    allaitement: habitudesData.value.allaitement,
    bruxisme: habitudesData.value.bruxisme || null,
    sahos: habitudesData.value.sahos
      ? { actif: true, traitement: habitudesData.value.sahos_traitement }
      : null,
    rgo: habitudesData.value.rgo,
    dernier_rdv_date: dentaireData.value.dernier_rdv_date || null,
    brossage_quotidien: dentaireData.value.brossage_quotidien
      ? Number(dentaireData.value.brossage_quotidien)
      : null,
    auxiliaires: dentaireData.value.auxiliaires.length > 0 ? dentaireData.value.auxiliaires : null,
    historique_connu: dentaireData.value.historique_connu.length > 0 ? dentaireData.value.historique_connu : null,
    apprehension: dentaireData.value.apprehension || null,
  }
}

async function handleSave() {
  saving.value = true
  saveError.value = null
  saveSuccess.value = false

  try {
    const payload = buildPayload()
    const response = await saveQuestionnaire(patientId, payload)

    if (response.success && response.data) {
      questionnaire.value = response.data
      saveSuccess.value = true
      setTimeout(() => { saveSuccess.value = false }, 3000)
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
    const response = await getQuestionnaire(patientId)
    if (response.success && response.data) {
      questionnaire.value = response.data
      hydrateFromQuestionnaire(response.data)
    }
    // No questionnaire yet — form stays with defaults, which is fine
  } catch {
    // First-time patient — questionnaire doesn't exist yet
  } finally {
    loading.value = false
  }
})
</script>
