<template>
  <Card>
    <CardHeader>
      <CardTitle class="text-base flex items-center gap-2">
        <Icon name="lucide:droplet" class="w-4 h-4 text-red-500" />
        Risque hémorragique
      </CardTitle>
    </CardHeader>
    <CardContent class="space-y-4">
      <!-- AVK -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <Checkbox
            id="avk"
            :checked="modelValue.avk"
            @update:checked="emit('update:modelValue', { ...modelValue, avk: $event })"
          />
          <Label for="avk">Traitement par AVK (anticoagulant oral)</Label>
        </div>
        <div v-if="modelValue.avk" class="ml-6 space-y-2">
          <Label for="avk_molecule">Molécule (Warfarine, Fluindione...)</Label>
          <Input
            id="avk_molecule"
            :value="modelValue.avk_molecule"
            placeholder="Ex: Préviscan, Coumadine..."
            @input="emit('update:modelValue', { ...modelValue, avk_molecule: ($event.target as HTMLInputElement).value })"
          />
        </div>
      </div>

      <Separator />

      <!-- AOD -->
      <div class="space-y-2">
        <Label for="aod_molecule">AOD / NACO (anticoagulant oral direct)</Label>
        <Input
          id="aod_molecule"
          :value="modelValue.aod_molecule"
          placeholder="Ex: Xarelto, Eliquis, Pradaxa..."
          @input="emit('update:modelValue', { ...modelValue, aod_molecule: ($event.target as HTMLInputElement).value })"
        />
        <p class="text-xs text-muted-foreground">Laisser vide si non concerné</p>
      </div>

      <Separator />

      <!-- Antiagrégants -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <Checkbox
            id="antiagregants"
            :checked="modelValue.antiagregants"
            @update:checked="emit('update:modelValue', { ...modelValue, antiagregants: $event })"
          />
          <Label for="antiagregants">Antiagrégants plaquettaires</Label>
        </div>
        <div v-if="modelValue.antiagregants" class="ml-6 space-y-2">
          <Label for="antiagregants_molecule">Molécule</Label>
          <Input
            id="antiagregants_molecule"
            :value="modelValue.antiagregants_molecule"
            placeholder="Ex: Aspirine, Plavix, Efient..."
            @input="emit('update:modelValue', { ...modelValue, antiagregants_molecule: ($event.target as HTMLInputElement).value })"
          />
        </div>
      </div>

      <Separator />

      <!-- Hémostase -->
      <div class="space-y-2">
        <div class="flex items-center gap-2">
          <Checkbox
            id="hemostase"
            :checked="modelValue.hemostase"
            @update:checked="emit('update:modelValue', { ...modelValue, hemostase: $event })"
          />
          <Label for="hemostase">Trouble de l'hémostase connu</Label>
        </div>
        <div v-if="modelValue.hemostase" class="ml-6 space-y-2">
          <Label for="hemostase_detail">Préciser</Label>
          <Input
            id="hemostase_detail"
            :value="modelValue.hemostase_detail"
            placeholder="Ex: Hémophilie A, maladie de von Willebrand..."
            @input="emit('update:modelValue', { ...modelValue, hemostase_detail: ($event.target as HTMLInputElement).value })"
          />
        </div>
      </div>
    </CardContent>
  </Card>
</template>

<script setup lang="ts">
export interface HemosectionData {
  avk: boolean
  avk_molecule: string
  aod_molecule: string
  antiagregants: boolean
  antiagregants_molecule: string
  hemostase: boolean
  hemostase_detail: string
}

const props = defineProps<{ modelValue: HemosectionData }>()
const emit = defineEmits<{ 'update:modelValue': [value: HemosectionData] }>()
</script>
