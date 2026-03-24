<template>
  <div>
    <!-- Tooth selector -->
    <Card class="mb-6">
      <CardContent class="pt-6">
        <div class="flex items-end gap-4">
          <div class="space-y-2 flex-1 max-w-xs">
            <Label for="fdi_selector">Sélectionner une dent (FDI)</Label>
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
              <Button :disabled="!selectedFdi || loadingTimeline" @click="fetchTimeline">
                <Icon v-if="loadingTimeline" name="lucide:loader-2" class="w-4 h-4 animate-spin" />
                <Icon v-else name="lucide:search" class="w-4 h-4" />
              </Button>
            </div>
          </div>

          <!-- Quick tooth picker -->
          <div class="flex flex-wrap gap-1 max-w-lg">
            <button
              v-for="fdi in COMMON_TEETH"
              :key="fdi"
              class="w-8 h-8 text-xs rounded border transition-colors"
              :class="selectedFdi === fdi
                ? 'bg-primary text-primary-foreground border-primary'
                : 'border-border hover:bg-muted'"
              @click="selectTooth(fdi)"
            >
              {{ fdi }}
            </button>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Timeline -->
    <div v-if="!selectedFdi && !events.length" class="text-center py-16 text-muted-foreground">
      <Icon name="lucide:clock" class="w-12 h-12 mx-auto mb-3 opacity-30" />
      <p>Sélectionnez une dent pour afficher son historique</p>
    </div>

    <div v-else-if="loadingTimeline" class="flex items-center justify-center h-32">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
    </div>

    <div v-else-if="events.length === 0 && hasSearched" class="text-center py-12 text-muted-foreground">
      <Icon name="lucide:calendar-x" class="w-12 h-12 mx-auto mb-3 opacity-30" />
      <p>Aucun événement enregistré pour la dent {{ selectedFdi }}</p>
    </div>

    <div v-else-if="events.length > 0">
      <div class="flex items-center gap-2 mb-4">
        <h2 class="font-semibold">Dent {{ selectedFdi }}</h2>
        <Badge variant="secondary">{{ events.length }} événement{{ events.length !== 1 ? 's' : '' }}</Badge>
      </div>

      <!-- Timeline list -->
      <div class="relative">
        <!-- Vertical line -->
        <div class="absolute left-4 top-0 bottom-0 w-0.5 bg-border" />

        <div class="space-y-4 ml-10">
          <div
            v-for="event in events"
            :key="event.id"
            class="relative"
          >
            <!-- Dot -->
            <div
              class="absolute -left-[2.375rem] top-3 w-3 h-3 rounded-full border-2 border-background"
              :class="eventDotClass(event.type)"
            />

            <Card>
              <CardContent class="pt-4 pb-3">
                <div class="flex items-start justify-between gap-4">
                  <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2 mb-1">
                      <Badge :class="eventBadgeClass(event.type)" class="text-xs shrink-0">
                        {{ eventTypeLabel(event.type) }}
                      </Badge>
                      <span v-if="event.praticien_nom" class="text-xs text-muted-foreground truncate">
                        Dr {{ event.praticien_nom }}
                      </span>
                    </div>
                    <p v-if="event.description" class="text-sm">{{ event.description }}</p>
                  </div>
                  <time class="text-xs text-muted-foreground whitespace-nowrap shrink-0">
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
    note: 'bg-gray-400',
  }
  return classes[type] ?? 'bg-muted-foreground'
}

function eventBadgeClass(type: string): string {
  const classes: Record<string, string> = {
    acte: 'bg-primary/10 text-primary',
    diagnostic: 'bg-orange-100 text-orange-700',
    paro: 'bg-red-100 text-red-700',
    document: 'bg-blue-100 text-blue-700',
    note: 'bg-gray-100 text-gray-700',
  }
  return classes[type] ?? 'bg-muted text-muted-foreground'
}
</script>
