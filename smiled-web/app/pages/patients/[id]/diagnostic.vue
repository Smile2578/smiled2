<template>
  <div>
    <!-- Header -->
    <div class="mb-6 flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold text-foreground">Diagnostics</h2>
        <p class="text-sm text-muted-foreground">
          {{ diagnostics.length }} diagnostic{{ diagnostics.length !== 1 ? 's' : '' }}
        </p>
      </div>
      <Button
        class="bg-primary hover:bg-primary/90 text-primary-foreground"
        @click="showNewDialog = true"
      >
        <Icon name="lucide:plus" class="mr-2 h-4 w-4" />
        Nouveau diagnostic
      </Button>
    </div>

    <!-- Error -->
    <Alert v-if="loadError" variant="destructive" class="mb-4">
      <AlertDescription>{{ loadError }}</AlertDescription>
    </Alert>

    <!-- Loading skeleton -->
    <div v-if="loading" class="space-y-4">
      <Card v-for="i in 3" :key="i">
        <CardHeader>
          <div class="flex items-center justify-between">
            <div class="space-y-2">
              <Skeleton class="h-5 w-40" />
              <Skeleton class="h-4 w-28" />
            </div>
            <Skeleton class="h-4 w-4" />
          </div>
        </CardHeader>
      </Card>
    </div>

    <!-- Empty state -->
    <div v-else-if="diagnostics.length === 0" class="py-16 text-center">
      <div class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-muted">
        <Icon name="lucide:clipboard-x" class="h-7 w-7 text-muted-foreground" />
      </div>
      <p class="text-base font-medium text-foreground">Aucun diagnostic</p>
      <p class="mt-1 text-sm text-muted-foreground">
        Creez le premier diagnostic pour ce patient
      </p>
      <Button
        class="mt-4 bg-primary hover:bg-primary/90 text-primary-foreground"
        @click="showNewDialog = true"
      >
        <Icon name="lucide:plus" class="mr-2 h-4 w-4" />
        Creer le premier diagnostic
      </Button>
    </div>

    <!-- Diagnostic cards -->
    <div v-else class="space-y-4">
      <Card
        v-for="diag in diagnostics"
        :key="diag.id"
        class="cursor-pointer transition-shadow hover:shadow-md"
        @click="toggleExpand(diag.id)"
      >
        <CardHeader class="pb-3">
          <div class="flex items-start justify-between">
            <div class="min-w-0 flex-1">
              <CardTitle class="text-base">
                {{ diag.titre ?? 'Diagnostic sans titre' }}
              </CardTitle>
              <CardDescription class="mt-1">
                {{ formatDate(diag.created_at) }}
                <span class="mx-1.5">|</span>
                {{ diag.findings.length }} constat{{ diag.findings.length !== 1 ? 's' : '' }}
              </CardDescription>
            </div>
            <Icon
              :name="expanded.has(diag.id) ? 'lucide:chevron-up' : 'lucide:chevron-down'"
              class="mt-1 h-4 w-4 shrink-0 text-muted-foreground transition-transform"
            />
          </div>
        </CardHeader>

        <CardContent v-if="expanded.has(diag.id)" class="border-t pt-4">
          <p v-if="diag.notes" class="mb-4 text-sm text-muted-foreground">{{ diag.notes }}</p>

          <div v-if="diag.findings.length > 0" class="space-y-2">
            <div
              v-for="finding in diag.findings"
              :key="finding.id"
              class="flex items-center gap-3 rounded-lg border bg-muted/30 p-3"
            >
              <Badge :variant="sourceVariant(finding.source)" class="shrink-0 text-xs">
                {{ finding.source === 'ia' ? 'IA' : 'Praticien' }}
              </Badge>
              <span class="text-sm font-medium text-foreground">{{ finding.type }}</span>
              <Badge v-if="finding.dent_fdi" variant="secondary" class="text-xs">
                Dent {{ finding.dent_fdi }}
              </Badge>
              <div class="ml-auto flex items-center gap-1.5">
                <span class="text-xs text-muted-foreground">Severite</span>
                <div class="flex gap-0.5">
                  <span
                    v-for="i in 5"
                    :key="i"
                    class="h-2.5 w-2.5 rounded-sm"
                    :class="i <= finding.severite ? 'bg-orange-400' : 'bg-muted'"
                  />
                </div>
              </div>
            </div>
          </div>

          <p v-else class="text-sm text-muted-foreground">Aucun constat renseigne.</p>
        </CardContent>
      </Card>
    </div>

    <!-- New Diagnostic Dialog -->
    <Dialog v-model:open="showNewDialog">
      <DialogContent class="max-w-2xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Nouveau diagnostic</DialogTitle>
          <DialogDescription>
            Creez un diagnostic avec les constats cliniques associes.
          </DialogDescription>
        </DialogHeader>

        <form class="space-y-4" @submit.prevent="handleCreate">
          <div class="space-y-2">
            <Label for="titre">Titre du diagnostic</Label>
            <Input
              id="titre"
              v-model="form.titre"
              placeholder="Ex : Bilan initial, Controle annuel..."
            />
          </div>

          <div class="space-y-2">
            <Label for="notes">Notes generales</Label>
            <Textarea
              id="notes"
              v-model="form.notes"
              placeholder="Observations cliniques globales..."
              :rows="2"
            />
          </div>

          <!-- Findings -->
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <Label>Constats cliniques</Label>
              <Button type="button" variant="outline" size="sm" @click="addFinding">
                <Icon name="lucide:plus" class="mr-1 h-3.5 w-3.5" />
                Ajouter
              </Button>
            </div>

            <div
              v-for="(finding, idx) in form.findings"
              :key="idx"
              class="rounded-lg border p-3 space-y-3"
            >
              <div class="flex items-center justify-between">
                <span class="text-sm font-medium text-foreground">Constat {{ idx + 1 }}</span>
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  class="h-6 w-6 p-0 text-muted-foreground hover:text-foreground"
                  @click="removeFinding(idx)"
                >
                  <Icon name="lucide:x" class="h-3.5 w-3.5" />
                </Button>
              </div>

              <div class="grid grid-cols-2 gap-3">
                <div class="space-y-1">
                  <Label class="text-xs">Type de constat *</Label>
                  <Input
                    v-model="finding.type"
                    placeholder="Ex : Carie, Abces, Parodontite..."
                    required
                  />
                </div>
                <div class="space-y-1">
                  <Label class="text-xs">Dent (FDI)</Label>
                  <Input
                    v-model.number="finding.dent_fdi"
                    type="number"
                    min="11"
                    max="85"
                    placeholder="Ex : 36"
                  />
                </div>
              </div>

              <div class="grid grid-cols-2 gap-3">
                <div class="space-y-1">
                  <Label class="text-xs">Source</Label>
                  <Select v-model="finding.source">
                    <SelectTrigger>
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectItem value="praticien">Praticien</SelectItem>
                      <SelectItem value="ia">Intelligence artificielle</SelectItem>
                    </SelectContent>
                  </Select>
                </div>
                <div class="space-y-1">
                  <Label class="text-xs">Severite (1-5)</Label>
                  <div class="flex items-center gap-2 pt-1">
                    <input
                      v-model.number="finding.severite"
                      type="range"
                      min="1"
                      max="5"
                      step="1"
                      class="flex-1 accent-[hsl(var(--primary))]"
                    />
                    <span class="w-4 text-sm font-medium">{{ finding.severite }}</span>
                  </div>
                </div>
              </div>

              <div class="space-y-1">
                <Label class="text-xs">Notes</Label>
                <Input v-model="finding.notes" placeholder="Precisions..." />
              </div>
            </div>
          </div>

          <Alert v-if="createError" variant="destructive">
            <AlertDescription>{{ createError }}</AlertDescription>
          </Alert>

          <DialogFooter>
            <Button type="button" variant="outline" @click="showNewDialog = false">
              Annuler
            </Button>
            <Button
              type="submit"
              :disabled="creating"
              class="bg-primary hover:bg-primary/90 text-primary-foreground"
            >
              <Icon v-if="creating" name="lucide:loader-2" class="mr-2 h-4 w-4 animate-spin" />
              Enregistrer le diagnostic
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import type { Diagnostic } from '~/composables/useDiagnostic'
import { formatDate } from '~/utils/format'

const route = useRoute()
const patientId = route.params.id as string

const { listDiagnostics, createDiagnostic } = useDiagnostic()

const diagnostics = ref<Diagnostic[]>([])
const loading = ref(true)
const showNewDialog = ref(false)
const creating = ref(false)
const createError = ref<string | null>(null)
const expanded = ref<Set<string>>(new Set())
const loadError = ref<string | null>(null)

const form = reactive({
  titre: '',
  notes: '',
  findings: [] as Array<{
    type: string
    dent_fdi: number | null
    severite: number
    source: 'praticien' | 'ia'
    notes: string
  }>,
})

function addFinding(): void {
  form.findings = [
    ...form.findings,
    { type: '', dent_fdi: null, severite: 3, source: 'praticien', notes: '' },
  ]
}

function removeFinding(idx: number): void {
  form.findings = form.findings.filter((_, i) => i !== idx)
}

function toggleExpand(id: string): void {
  const next = new Set(expanded.value)
  if (next.has(id)) {
    next.delete(id)
  } else {
    next.add(id)
  }
  expanded.value = next
}

function sourceVariant(source: string): 'default' | 'secondary' | 'outline' {
  return source === 'ia' ? 'secondary' : 'outline'
}

async function fetchDiagnostics(): Promise<void> {
  try {
    const response = await listDiagnostics(patientId)
    if (response.success && response.data) {
      diagnostics.value = response.data
    }
  } catch (e) {
    loadError.value = e instanceof Error ? e.message : 'Erreur lors du chargement des diagnostics'
  } finally {
    loading.value = false
  }
}

async function handleCreate(): Promise<void> {
  creating.value = true
  createError.value = null

  try {
    const response = await createDiagnostic(patientId, {
      titre: form.titre || null,
      notes: form.notes || null,
      findings: form.findings.map((f) => ({
        type: f.type,
        dent_fdi: f.dent_fdi,
        severite: f.severite,
        source: f.source,
        notes: f.notes || null,
      })),
    })

    if (response.success && response.data) {
      diagnostics.value = [response.data, ...diagnostics.value]
      showNewDialog.value = false
      form.titre = ''
      form.notes = ''
      form.findings = []
    } else {
      createError.value = response.error ?? 'Erreur lors de la creation'
    }
  } catch (err) {
    createError.value = err instanceof Error ? err.message : 'Erreur lors de la creation'
  } finally {
    creating.value = false
  }
}

onMounted(fetchDiagnostics)
</script>
