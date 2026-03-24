<template>
  <div>
    <!-- Actions bar -->
    <div class="flex items-center justify-between mb-6">
      <p class="text-sm text-muted-foreground">
        {{ diagnostics.length }} diagnostic{{ diagnostics.length !== 1 ? 's' : '' }}
      </p>
      <Button @click="showNewDialog = true">
        <Icon name="lucide:plus" class="w-4 h-4 mr-2" />
        Nouveau diagnostic
      </Button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-64">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
    </div>

    <!-- Empty -->
    <div v-else-if="diagnostics.length === 0" class="text-center py-16 text-muted-foreground">
      <Icon name="lucide:clipboard-x" class="w-12 h-12 mx-auto mb-3 opacity-30" />
      <p class="text-base">Aucun diagnostic enregistré</p>
      <Button class="mt-4" variant="outline" @click="showNewDialog = true">
        Créer le premier diagnostic
      </Button>
    </div>

    <!-- List -->
    <div v-else class="space-y-4">
      <Card
        v-for="diag in diagnostics"
        :key="diag.id"
        class="cursor-pointer"
        @click="toggleExpand(diag.id)"
      >
        <CardHeader class="pb-3">
          <div class="flex items-start justify-between">
            <div>
              <CardTitle class="text-base">
                {{ diag.titre ?? 'Diagnostic sans titre' }}
              </CardTitle>
              <CardDescription>
                {{ formatDate(diag.created_at) }}
                · {{ diag.findings.length }} constat{{ diag.findings.length !== 1 ? 's' : '' }}
              </CardDescription>
            </div>
            <Icon
              :name="expanded.has(diag.id) ? 'lucide:chevron-up' : 'lucide:chevron-down'"
              class="w-4 h-4 text-muted-foreground mt-1"
            />
          </div>
        </CardHeader>

        <CardContent v-if="expanded.has(diag.id)">
          <p v-if="diag.notes" class="text-sm text-muted-foreground mb-4">{{ diag.notes }}</p>

          <div v-if="diag.findings.length > 0" class="space-y-2">
            <div
              v-for="finding in diag.findings"
              :key="finding.id"
              class="flex items-center gap-3 p-2 rounded-md border bg-muted/30"
            >
              <Badge :variant="sourceVariant(finding.source)" class="shrink-0">
                {{ finding.source === 'ia' ? 'IA' : 'Praticien' }}
              </Badge>
              <span class="font-medium text-sm">{{ finding.type }}</span>
              <span v-if="finding.dent_fdi" class="text-xs text-muted-foreground">
                Dent {{ finding.dent_fdi }}
              </span>
              <div class="flex items-center gap-1 ml-auto">
                <span class="text-xs text-muted-foreground">Sévérité</span>
                <div class="flex gap-0.5">
                  <span
                    v-for="i in 5"
                    :key="i"
                    class="w-3 h-3 rounded-sm"
                    :class="i <= finding.severite ? 'bg-orange-400' : 'bg-muted'"
                  />
                </div>
              </div>
            </div>
          </div>

          <p v-else class="text-sm text-muted-foreground">Aucun constat renseigné.</p>
        </CardContent>
      </Card>
    </div>

    <!-- New Diagnostic Dialog -->
    <Dialog v-model:open="showNewDialog">
      <DialogContent class="max-w-2xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Nouveau diagnostic</DialogTitle>
          <DialogDescription>Créez un diagnostic avec les constats cliniques associés.</DialogDescription>
        </DialogHeader>

        <form class="space-y-4" @submit.prevent="handleCreate">
          <div class="space-y-2">
            <Label for="titre">Titre du diagnostic</Label>
            <Input id="titre" v-model="form.titre" placeholder="Ex : Bilan initial, Contrôle annuel..." />
          </div>

          <div class="space-y-2">
            <Label for="notes">Notes générales</Label>
            <Textarea id="notes" v-model="form.notes" placeholder="Observations cliniques globales..." :rows="2" />
          </div>

          <!-- Findings -->
          <div class="space-y-3">
            <div class="flex items-center justify-between">
              <Label>Constats cliniques</Label>
              <Button type="button" variant="outline" size="sm" @click="addFinding">
                <Icon name="lucide:plus" class="w-3.5 h-3.5 mr-1" />
                Ajouter
              </Button>
            </div>

            <div
              v-for="(finding, idx) in form.findings"
              :key="idx"
              class="border rounded-lg p-3 space-y-3"
            >
              <div class="flex items-center justify-between">
                <span class="text-sm font-medium">Constat {{ idx + 1 }}</span>
                <Button
                  type="button"
                  variant="ghost"
                  size="sm"
                  class="h-6 w-6 p-0 text-muted-foreground"
                  @click="removeFinding(idx)"
                >
                  <Icon name="lucide:x" class="w-3.5 h-3.5" />
                </Button>
              </div>

              <div class="grid grid-cols-2 gap-3">
                <div class="space-y-1">
                  <Label class="text-xs">Type de constat *</Label>
                  <Input v-model="finding.type" placeholder="Ex : Carie, Abcès, Parodontite..." required />
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
                  <Label class="text-xs">Sévérité (1–5)</Label>
                  <div class="flex items-center gap-2 pt-1">
                    <input
                      v-model.number="finding.severite"
                      type="range"
                      min="1"
                      max="5"
                      step="1"
                      class="flex-1"
                    />
                    <span class="text-sm font-medium w-4">{{ finding.severite }}</span>
                  </div>
                </div>
              </div>

              <div class="space-y-1">
                <Label class="text-xs">Notes</Label>
                <Input v-model="finding.notes" placeholder="Précisions..." />
              </div>
            </div>
          </div>

          <Alert v-if="createError" variant="destructive">
            <AlertDescription>{{ createError }}</AlertDescription>
          </Alert>

          <DialogFooter>
            <Button type="button" variant="outline" @click="showNewDialog = false">Annuler</Button>
            <Button type="submit" :disabled="creating">
              <Icon v-if="creating" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
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
  } catch {
    // ignore
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
      createError.value = response.error ?? 'Erreur lors de la création'
    }
  } catch (err) {
    createError.value = err instanceof Error ? err.message : 'Erreur lors de la création'
  } finally {
    creating.value = false
  }
}

onMounted(fetchDiagnostics)
</script>
