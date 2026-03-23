<template>
  <Card>
    <CardHeader>
      <CardTitle class="text-base flex items-center gap-2">
        <Icon name="lucide:activity" class="w-4 h-4 text-blue-500" />
        Troubles médicaux, médicaments, allergies
      </CardTitle>
    </CardHeader>
    <CardContent class="space-y-4">
      <!-- Troubles -->
      <div class="space-y-2">
        <Label for="troubles">Antécédents médicaux et chirurgicaux</Label>
        <Textarea
          id="troubles"
          :value="modelValue.troubles"
          placeholder="Ex: HTA, diabète type 2, infarctus 2018, appendicectomie..."
          rows="3"
          @input="emit('update:modelValue', { ...modelValue, troubles: ($event.target as HTMLTextAreaElement).value })"
        />
      </div>

      <Separator />

      <!-- Médicaments -->
      <div class="space-y-2">
        <Label for="medicaments">Médicaments en cours</Label>
        <Textarea
          id="medicaments"
          :value="modelValue.medicaments"
          placeholder="Ex: Amlodipine 5mg, Metformine 1000mg, Lévothyrox 75µg..."
          rows="3"
          @input="emit('update:modelValue', { ...modelValue, medicaments: ($event.target as HTMLTextAreaElement).value })"
        />
        <p class="text-xs text-muted-foreground">Un médicament par ligne ou séparés par des virgules</p>
      </div>

      <Separator />

      <!-- Allergies -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <Checkbox
            id="allergies"
            :checked="modelValue.allergies"
            @update:checked="emit('update:modelValue', { ...modelValue, allergies: $event })"
          />
          <Label for="allergies">Allergies connues</Label>
        </div>
        <div v-if="modelValue.allergies" class="ml-6 space-y-2">
          <Label for="allergies_detail">Préciser les allergènes</Label>
          <Textarea
            id="allergies_detail"
            :value="modelValue.allergies_detail"
            placeholder="Ex: Pénicilline, latex, nickel, AINS..."
            rows="2"
            @input="emit('update:modelValue', { ...modelValue, allergies_detail: ($event.target as HTMLTextAreaElement).value })"
          />
        </div>
      </div>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
export interface TroublesSectionData {
  troubles: string
  medicaments: string
  allergies: boolean
  allergies_detail: string
}

const props = defineProps<{ modelValue: TroublesSectionData }>()
const emit = defineEmits<{ 'update:modelValue': [value: TroublesSectionData] }>()
</script>
