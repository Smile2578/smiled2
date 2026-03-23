<template>
  <Card>
    <CardHeader>
      <CardTitle class="text-base flex items-center gap-2">
        <Icon name="lucide:pill" class="w-4 h-4 text-purple-500" />
        Risque médicamenteux osseux
      </CardTitle>
    </CardHeader>
    <CardContent class="space-y-4">
      <!-- Bisphosphonates -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <Checkbox
            id="bisphosphonates"
            :checked="modelValue.bisphosphonates"
            @update:checked="emit('update:modelValue', { ...modelValue, bisphosphonates: $event })"
          />
          <Label for="bisphosphonates">Bisphosphonates</Label>
        </div>
        <div v-if="modelValue.bisphosphonates" class="ml-6 space-y-2">
          <Label for="bisphosphonates_molecule">Molécule et voie d'administration</Label>
          <Input
            id="bisphosphonates_molecule"
            :value="modelValue.bisphosphonates_molecule"
            placeholder="Ex: Alendronate IV, Zométa..."
            @input="emit('update:modelValue', { ...modelValue, bisphosphonates_molecule: ($event.target as HTMLInputElement).value })"
          />
        </div>
      </div>

      <Separator />

      <!-- Antirésorptifs -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <Checkbox
            id="antiresorptifs"
            :checked="modelValue.antiresorptifs"
            @update:checked="emit('update:modelValue', { ...modelValue, antiresorptifs: $event })"
          />
          <Label for="antiresorptifs">Antirésorptifs osseux (Denosumab)</Label>
        </div>
        <div v-if="modelValue.antiresorptifs" class="ml-6 space-y-2">
          <Label for="antiresorptifs_molecule">Molécule</Label>
          <Input
            id="antiresorptifs_molecule"
            :value="modelValue.antiresorptifs_molecule"
            placeholder="Ex: Prolia, Xgeva..."
            @input="emit('update:modelValue', { ...modelValue, antiresorptifs_molecule: ($event.target as HTMLInputElement).value })"
          />
        </div>
      </div>

      <Separator />

      <!-- Radiothérapie -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <Checkbox
            id="radiotherapie"
            :checked="modelValue.radiotherapie"
            @update:checked="emit('update:modelValue', { ...modelValue, radiotherapie: $event })"
          />
          <Label for="radiotherapie">Radiothérapie cervico-faciale</Label>
        </div>
        <div v-if="modelValue.radiotherapie" class="ml-6 space-y-2">
          <Label for="radiotherapie_zone">Zone et dose</Label>
          <Input
            id="radiotherapie_zone"
            :value="modelValue.radiotherapie_zone"
            placeholder="Ex: Cavité orale, 60 Gy (2021)"
            @input="emit('update:modelValue', { ...modelValue, radiotherapie_zone: ($event.target as HTMLInputElement).value })"
          />
        </div>
      </div>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
export interface MedicamenteuxSectionData {
  bisphosphonates: boolean
  bisphosphonates_molecule: string
  antiresorptifs: boolean
  antiresorptifs_molecule: string
  radiotherapie: boolean
  radiotherapie_zone: string
}

const props = defineProps<{ modelValue: MedicamenteuxSectionData }>()
const emit = defineEmits<{ 'update:modelValue': [value: MedicamenteuxSectionData] }>()
</script>
