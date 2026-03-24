<template>
  <div class="flex flex-wrap gap-x-6 gap-y-2 p-3 border-t bg-gray-50 text-xs">
    <!-- Tooth status legend -->
    <div class="flex items-center gap-1.5 font-semibold text-gray-500 uppercase tracking-wide">
      Statut :
    </div>
    <div
      v-for="(color, status) in filteredStatusColors"
      :key="status"
      class="flex items-center gap-1.5"
    >
      <span
        class="inline-block w-4 h-4 rounded-sm border border-gray-300 shrink-0"
        :style="{ backgroundColor: color }"
      />
      <span class="text-gray-600">{{ TOOTH_STATUS_LABELS[status as ToothStatus] }}</span>
    </div>

    <div class="w-px bg-gray-300 mx-1" />

    <!-- Face état legend -->
    <div class="flex items-center gap-1.5 font-semibold text-gray-500 uppercase tracking-wide">
      Faces :
    </div>
    <div
      v-for="(color, state) in filteredFaceColors"
      :key="state"
      class="flex items-center gap-1.5"
    >
      <span
        class="inline-block w-4 h-4 rounded-sm border border-gray-300 shrink-0"
        :style="{ backgroundColor: color }"
      />
      <span class="text-gray-600">{{ FACE_STATE_LABELS[state as FaceState] }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ToothStatus, FaceState } from './dentalChartTypes'
import { TOOTH_STATUS_COLORS, FACE_STATE_COLORS, TOOTH_STATUS_LABELS, FACE_STATE_LABELS } from './dentalChartTypes'

// Exclude 'presente' (white — default) from tooth status legend to reduce noise
const EXCLUDED_STATUSES: ToothStatus[] = []
const EXCLUDED_FACE_STATES: FaceState[] = []

const filteredStatusColors = computed(() => {
  return Object.fromEntries(
    Object.entries(TOOTH_STATUS_COLORS).filter(([k]) => !EXCLUDED_STATUSES.includes(k as ToothStatus)),
  ) as Record<ToothStatus, string>
})

const filteredFaceColors = computed(() => {
  return Object.fromEntries(
    Object.entries(FACE_STATE_COLORS).filter(([k]) => !EXCLUDED_FACE_STATES.includes(k as FaceState)),
  ) as Record<FaceState, string>
})
</script>
