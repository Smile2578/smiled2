<template>
  <Card>
    <CardHeader>
      <CardTitle class="text-base flex items-center gap-2">
        <Icon name="lucide:heart-pulse" class="w-4 h-4 text-green-500" />
        Habitudes de vie
      </CardTitle>
    </CardHeader>
    <CardContent class="space-y-4">
      <!-- Tabac -->
      <div class="space-y-2">
        <Label for="tabac">Tabac</Label>
        <Select
          :model-value="modelValue.tabac"
          @update:model-value="emit('update:modelValue', { ...modelValue, tabac: $event })"
        >
          <SelectTrigger id="tabac">
            <SelectValue placeholder="Sélectionner" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="">Non renseigné</SelectItem>
            <SelectItem value="non">Non-fumeur</SelectItem>
            <SelectItem value="ex">Ex-fumeur</SelectItem>
            <SelectItem value="oui">Fumeur actif</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <Separator />

      <!-- Alcool -->
      <div class="space-y-2">
        <Label for="alcool">Consommation d'alcool</Label>
        <Select
          :model-value="modelValue.alcool"
          @update:model-value="emit('update:modelValue', { ...modelValue, alcool: $event })"
        >
          <SelectTrigger id="alcool">
            <SelectValue placeholder="Sélectionner" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="">Non renseigné</SelectItem>
            <SelectItem value="aucune">Aucune</SelectItem>
            <SelectItem value="occasionnelle">Occasionnelle</SelectItem>
            <SelectItem value="reguliere">Régulière</SelectItem>
            <SelectItem value="excessive">Excessive</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <Separator />

      <!-- Grossesse / Allaitement -->
      <div class="grid grid-cols-2 gap-4">
        <div class="space-y-2">
          <Label for="grossesse_mois">Grossesse (mois)</Label>
          <Input
            id="grossesse_mois"
            type="number"
            min="0"
            max="9"
            :value="modelValue.grossesse_mois"
            placeholder="Laisser vide si non"
            @input="emit('update:modelValue', { ...modelValue, grossesse_mois: ($event.target as HTMLInputElement).value })"
          />
        </div>
        <div class="space-y-2 flex flex-col justify-end">
          <div class="flex items-center gap-2 pb-2">
            <Checkbox
              id="allaitement"
              :checked="modelValue.allaitement"
              @update:checked="emit('update:modelValue', { ...modelValue, allaitement: $event })"
            />
            <Label for="allaitement">Allaitement en cours</Label>
          </div>
        </div>
      </div>

      <Separator />

      <!-- Bruxisme -->
      <div class="space-y-2">
        <Label for="bruxisme">Bruxisme / serrement</Label>
        <Select
          :model-value="modelValue.bruxisme"
          @update:model-value="emit('update:modelValue', { ...modelValue, bruxisme: $event })"
        >
          <SelectTrigger id="bruxisme">
            <SelectValue placeholder="Sélectionner" />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value="">Non renseigné</SelectItem>
            <SelectItem value="non">Non</SelectItem>
            <SelectItem value="diurne">Diurne (serrement)</SelectItem>
            <SelectItem value="nocturne">Nocturne (bruxisme)</SelectItem>
            <SelectItem value="mixte">Mixte</SelectItem>
          </SelectContent>
        </Select>
      </div>

      <Separator />

      <!-- SAHOS -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <Checkbox
            id="sahos"
            :checked="modelValue.sahos"
            @update:checked="emit('update:modelValue', { ...modelValue, sahos: $event })"
          />
          <Label for="sahos">Syndrome d'apnée du sommeil (SAHOS)</Label>
        </div>
        <div v-if="modelValue.sahos" class="ml-6 space-y-2">
          <Label for="sahos_traitement">Traitement</Label>
          <Input
            id="sahos_traitement"
            :value="modelValue.sahos_traitement"
            placeholder="Ex: PPC, OAM, sans traitement..."
            @input="emit('update:modelValue', { ...modelValue, sahos_traitement: ($event.target as HTMLInputElement).value })"
          />
        </div>
      </div>

      <!-- RGO -->
      <div class="flex items-center gap-2">
        <Checkbox
          id="rgo"
          :checked="modelValue.rgo"
          @update:checked="emit('update:modelValue', { ...modelValue, rgo: $event })"
        />
        <Label for="rgo">Reflux gastro-œsophagien (RGO)</Label>
      </div>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
export interface HabitudesSectionData {
  tabac: string
  alcool: string
  grossesse_mois: string
  allaitement: boolean
  bruxisme: string
  sahos: boolean
  sahos_traitement: string
  rgo: boolean
}

const props = defineProps<{ modelValue: HabitudesSectionData }>()
const emit = defineEmits<{ 'update:modelValue': [value: HabitudesSectionData] }>()
</script>
