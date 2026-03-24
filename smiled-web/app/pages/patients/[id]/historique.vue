<template>
  <div>
    <!-- Header -->
    <div class="mb-6">
      <h2 class="text-lg font-semibold text-foreground">Historique</h2>
      <p class="text-sm text-muted-foreground">
        Chronologie des evenements par dent
      </p>
    </div>

    <!-- Tooth selector -->
    <Card class="mb-6 shadow-sm">
      <CardContent class="pt-6">
        <div class="flex items-end gap-4">
          <div class="flex-1 max-w-xs space-y-2">
            <Label for="fdi_selector">Selectionner une dent (FDI)</Label>
            <div class="flex gap-2">
              <Input
                id="fdi_selector"
                v-model.number="selectedFdi"
                type="number"
                min="11"
                max="85"
                placeholder="Ex : 36"
                @keyup.enter="fetchTimeline"
              />
              <Button
                :disabled="!selectedFdi || loadingTimeline"
                class="bg-primary hover:bg-primary/90 text-primary-foreground"
                @click="fetchTimeline"
              >
                <Icon
                  v-if="loadingTimeline"
                  name="lucide:loader-2"
                  class="h-4 w-4 animate-spin"
                />
                <Icon v-else name="lucide:search" class="h-4 w-4" />
              </Button>
            </div>
          </div>

          <!-- Quick tooth picker -->
          <div class="hidden sm:flex flex-wrap gap-1 max-w-lg">
            <button
              v-for="fdi in COMMON_TEETH"
              :key="fdi"
              class="h-8 w-8 text-xs rounded-md border transition-colors"
              :class="selectedFdi === fdi
                ? 'bg-primary text-primary-foreground border-primary'
                : 'border-border text-foreground hover:bg-muted'"
              @click="selectTooth(fdi)"
            >
              {{ fdi }}
            </button>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Initial state -->
    <div
      v-if="!selectedFdi && !events.length"
      class="py-16 text-center"
    >
      <div class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-muted">
        <Icon name="lucide:clock" class="h-7 w-7 text-muted-foreground" />
      </div>
      <p class="text-base font-medium text-foreground">
        Selectionnez une dent pour afficher son historique
      </p>
    </div>

    <!-- Loading -->
    <div v-else-if="loadingTimeline" class="flex items-center justify-center h-32">
      <Icon name="lucide:loader-2" class="h-8 w-8 animate-spin text-muted-foreground" />
    </div>

    <!-- Empty results -->
    <div
      v-else-if="events.length === 0 && hasSearched"
      class="py-12 text-center"
    >
      <div class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-muted">
        <Icon name="lucide:calendar-x" class="h-7 w-7 text-muted-foreground" />
      </div>
      <p class="text-base font-medium text-foreground">Aucun evenement</p>
      <p class="mt-1 text-sm text-muted-foreground">
        Aucun evenement enregistre pour la dent {{ selectedFdi }}
      </p>
    </div>

    <!-- Timeline -->
    <div v-else-if="events.length > 0">
      <div class="mb-4 flex items-center gap-2">
        <h3 class="font-semibold text-foreground">Dent {{ selectedFdi }}</h3>
        <Badge variant="secondary">
          {{ events.length }} evenement{{ events.length !== 1 ? 's' : '' }}
        </Badge>
      </div>

      <div class="relative">
        <!-- Vertical line -->
        <div class="absolute left-4 top-0 bottom-0 w-0.5 bg-border" />

        <div class="ml-10 space-y-4">
          <div
            v-for="event in events"
            :key="event.id"
            class="relative"
          >
            <!-- Dot -->
            <div
              class="absolute -left-[2.375rem] top-3 h-3 w-3 rounded-full border-2 border-background"
              :class="eventDotClass(event.type)"
            />

            <Card class="shadow-sm">
              <CardContent class="pb-3 pt-4">
                <div class="flex items-start justify-between gap-4">
                  <div class="min-w-0 flex-1">
                    <div class="mb-1 flex items-center gap-2">
                      <Badge :class="eventBadgeClass(event.type)" class="shrink-0 text-xs">
                        {{ eventTypeLabel(event.type) }}
                      </Badge>
                      <span
                        v-if="event.praticien_nom"
                        class="truncate text-xs text-muted-foreground"
                      >
                        Dr {{ event.praticien_nom }}
                      </span>
                    </div>
                    <p v-if="event.description" class="text-sm text-foreground">
                      {{ event.description }}
                    </p>
                  </div>
                  <time class="shrink-0 whitespace-nowrap text-xs text-muted-foreground">
                    {{ formatDate(event.created_at) }}
                  </time>
                </div>
              </CardContent>
            </Card>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { HistoriqueEvent } from '~/composables/useHistorique'
import { formatDate } from '~/utils/format'

const COMMON_TEETH = [
  18, 17, 16, 15, 14, 13, 12, 11,
  21, 22, 23, 24, 25, 26, 27, 28,
  48, 47, 46, 45, 44, 43, 42, 41,
  31, 32, 33, 34, 35, 36, 37, 38,
]

const route = useRoute()
const patientId = route.params.id as string

const { getDentTimeline } = useHistorique()

const selectedFdi = ref<number | null>(null)
const events = ref<HistoriqueEvent[]>([])
const loadingTimeline = ref(false)
const hasSearched = ref(false)

function selectTooth(fdi: number): void {
  selectedFdi.value = fdi
  fetchTimeline()
}

async function fetchTimeline(): Promise<void> {
  if (!selectedFdi.value) return
  loadingTimeline.value = true
  hasSearched.value = true

  try {
    const response = await getDentTimeline(patientId, selectedFdi.value)
    events.value = response.success && response.data ? response.data : []
  } catch {
    events.value = []
  } finally {
    loadingTimeline.value = false
  }
}

function eventTypeLabel(type: string): string {
  const labels: Record<string, string> = {
    acte: 'Acte',
    diagnostic: 'Diagnostic',
    paro: 'Paro',
    document: 'Document',
    note: 'Note',
  }
  return labels[type] ?? type
}

function eventDotClass(type: string): string {
  const classes: Record<string, string> = {
    acte: 'bg-primary',
    diagnostic: 'bg-orange-500',
    paro: 'bg-red-500',
    document: 'bg-blue-500',
    note: 'bg-muted-foreground',
  }
  return classes[type] ?? 'bg-muted-foreground'
}

function eventBadgeClass(type: string): string {
  const classes: Record<string, string> = {
    acte: 'bg-primary/10 text-primary',
    diagnostic: 'bg-orange-100 text-orange-700',
    paro: 'bg-red-100 text-red-700',
    document: 'bg-blue-100 text-blue-700',
    note: 'bg-muted text-muted-foreground',
  }
  return classes[type] ?? 'bg-muted text-muted-foreground'
}
</script>
