<template>
  <Card>
    <CardHeader>
      <CardTitle class="text-base flex items-center gap-2">
        <Icon name="lucide:smile" class="w-4 h-4 text-cyan-500" />
        Historique dentaire
      </CardTitle>
    </CardHeader>
    <CardContent class="space-y-4">
      <!-- Dernier RDV -->
      <div class="space-y-2">
        <Label for="dernier_rdv_date">Date du dernier rendez-vous dentaire</Label>
        <Input
          id="dernier_rdv_date"
          type="date"
          :value="modelValue.dernier_rdv_date"
          @input="emit('update:modelValue', { ...modelValue, dernier_rdv_date: ($event.target as HTMLInputElement).value })"
        />
      </div>

      <Separator />

      <!-- Brossage -->
      <div class="space-y-2">
        <Label for="brossage_quotidien">Nombre de brossages par jour</Label>
        <Select
          :model-value="modelValue.brossage_quotidien"
          @update:model-value="emit('update:modelValue', { ...modelValue, brossage_quotidien: $event })"
        >
          <SelectTrigger id="brossage_quotidien">
            <SelectValue placeholder="Sélectionner" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="">Non renseigné</SelectItem>
            <SelectItem value="0">0 — Jamais</SelectItem>
            <SelectItem value="1">1 fois par jour</SelectItem>
            <SelectItem value="2">2 fois par jour</SelectItem>
            <SelectItem value="3">3 fois par jour ou plus</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <Separator />

      <!-- Auxiliaires -->
      <div class="space-y-2">
        <Label>Auxiliaires d'hygiène utilisés</Label>
        <div class="grid grid-cols-2 gap-2">
          <div v-for="aux in auxiliairesOptions" :key="aux.value" class="flex items-center gap-2">
            <Checkbox
              :id="`aux-${aux.value}`"
              :checked="modelValue.auxiliaires.includes(aux.value)"
              @update:checked="toggleAuxiliaire(aux.value, $event)"
            />
            <Label :for="`aux-${aux.value}`" class="font-normal">{{ aux.label }}</Label>
          </div>
        </div>
      </div>

      <Separator />

      <!-- Historique connu -->
      <div class="space-y-2">
        <Label>Traitements dentaires antérieurs</Label>
        <div class="grid grid-cols-2 gap-2">
          <div v-for="hist in historiqueOptions" :key="hist.value" class="flex items-center gap-2">
            <Checkbox
              :id="`hist-${hist.value}`"
              :checked="modelValue.historique_connu.includes(hist.value)"
              @update:checked="toggleHistorique(hist.value, $event)"
            />
            <Label :for="`hist-${hist.value}`" class="font-normal">{{ hist.label }}</Label>
          </div>
        </div>
      </div>

      <Separator />

      <!-- Appréhension -->
      <div class="space-y-2">
        <Label for="apprehension">Appréhension / anxiété dentaire</Label>
        <Select
          :model-value="modelValue.apprehension"
          @update:model-value="emit('update:modelValue', { ...modelValue, apprehension: $event })"
        >
          <SelectTrigger id="apprehension">
            <SelectValue placeholder="Sélectionner" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="">Non renseigné</SelectItem>
            <SelectItem value="aucune">Aucune</SelectItem>
            <SelectItem value="legere">Légère</SelectItem>
            <SelectItem value="moderee">Modérée</SelectItem>
            <SelectItem value="severe">Sévère (phobie)</SelectItem>
          </SelectContent>
        </Select>
      </div>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
export interface DentaireSectionData {
  dernier_rdv_date: string
  brossage_quotidien: string
  auxiliaires: string[]
  historique_connu: string[]
  apprehension: string
}

const props = defineProps<{ modelValue: DentaireSectionData }>()
const emit = defineEmits<{ 'update:modelValue': [value: DentaireSectionData] }>()

const auxiliairesOptions = [
  { value: 'fil_dentaire', label: 'Fil dentaire' },
  { value: 'brossettes', label: 'Brossettes interdentaires' },
  { value: 'hydropulseur', label: 'Hydropulseur' },
  { value: 'grattoir_lingual', label: 'Grattoir lingual' },
  { value: 'bain_bouche', label: 'Bain de bouche' },
  { value: 'brosse_electrique', label: 'Brosse électrique' },
]

const historiqueOptions = [
  { value: 'orthodontie', label: 'Orthodontie' },
  { value: 'implants', label: 'Implants' },
  { value: 'protheses', label: 'Prothèses' },
  { value: 'paro', label: 'Parodontie' },
  { value: 'endodontie', label: 'Endodontie (dévitalisation)' },
  { value: 'chirurgie', label: 'Chirurgie buccale' },
]

function toggleAuxiliaire(value: string, checked: boolean) {
  const current = [...props.modelValue.auxiliaires]
  if (checked) {
    emit('update:modelValue', { ...props.modelValue, auxiliaires: [...current, value] })
  } else {
    emit('update:modelValue', { ...props.modelValue, auxiliaires: current.filter((v) => v !== value) })
  }
}

function toggleHistorique(value: string, checked: boolean) {
  const current = [...props.modelValue.historique_connu]
  if (checked) {
    emit('update:modelValue', { ...props.modelValue, historique_connu: [...current, value] })
  } else {
    emit('update:modelValue', { ...props.modelValue, historique_connu: current.filter((v) => v !== value) })
  }
}
</script>
