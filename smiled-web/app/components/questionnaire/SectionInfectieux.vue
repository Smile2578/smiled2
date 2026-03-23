<template>
  <Card>
    <CardHeader>
      <CardTitle class="text-base flex items-center gap-2">
        <Icon name="lucide:shield-alert" class="w-4 h-4 text-orange-500" />
        Risque infectieux
      </CardTitle>
    </CardHeader>
    <CardContent class="space-y-4">
      <!-- Endocardite -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <Checkbox
            id="endocardite"
            :checked="modelValue.endocardite"
            @update:checked="emit('update:modelValue', { ...modelValue, endocardite: $event })"
          />
          <Label for="endocardite">Risque d'endocardite infectieuse (ATCD valvulopathie, prothèse valvulaire...)</Label>
        </div>
      </div>

      <Separator />

      <!-- Immunodépression -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <Checkbox
            id="immunodepression"
            :checked="modelValue.immunodepression"
            @update:checked="emit('update:modelValue', { ...modelValue, immunodepression: $event })"
          />
          <Label for="immunodepression">Immunodépression</Label>
        </div>
        <div v-if="modelValue.immunodepression" class="ml-6 space-y-2">
          <Label for="immunodepression_detail">Préciser</Label>
          <Input
            id="immunodepression_detail"
            :value="modelValue.immunodepression_detail"
            placeholder="Ex: Chimiothérapie, corticothérapie, VIH..."
            @input="emit('update:modelValue', { ...modelValue, immunodepression_detail: ($event.target as HTMLInputElement).value })"
          />
        </div>
      </div>

      <Separator />

      <!-- Prothèses articulaires -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <Checkbox
            id="protheses_articulaires"
            :checked="modelValue.protheses_articulaires"
            @update:checked="emit('update:modelValue', { ...modelValue, protheses_articulaires: $event })"
          />
          <Label for="protheses_articulaires">Prothèses articulaires</Label>
        </div>
        <div v-if="modelValue.protheses_articulaires" class="ml-6 space-y-2">
          <Label for="protheses_articulaires_detail">Préciser (hanche, genou, épaule...)</Label>
          <Input
            id="protheses_articulaires_detail"
            :value="modelValue.protheses_articulaires_detail"
            placeholder="Ex: Prothèse totale de hanche gauche (2019)"
            @input="emit('update:modelValue', { ...modelValue, protheses_articulaires_detail: ($event.target as HTMLInputElement).value })"
          />
        </div>
      </div>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
export interface InfectieuxSectionData {
  endocardite: boolean
  immunodepression: boolean
  immunodepression_detail: string
  protheses_articulaires: boolean
  protheses_articulaires_detail: string
}

const props = defineProps<{ modelValue: InfectieuxSectionData }>()
const emit = defineEmits<{ 'update:modelValue': [value: InfectieuxSectionData] }>()
</script>
