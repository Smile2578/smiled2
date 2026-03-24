<template>
  <div>
    <!-- Header -->
    <div class="mb-6 flex items-center justify-between">
      <div>
        <h2 class="text-lg font-semibold text-foreground">Plans de traitement</h2>
        <p class="text-sm text-muted-foreground">Propositions therapeutiques par formule</p>
      </div>
      <Button
        class="bg-primary hover:bg-primary/90 text-primary-foreground"
        @click="showNewDialog = true"
      >
        <Icon name="lucide:plus" class="mr-2 h-4 w-4" />
        Nouveau plan
      </Button>
    </div>

    <!-- Errors -->
    <Alert v-if="loadError" variant="destructive" class="mb-4">
      <AlertDescription>{{ loadError }}</AlertDescription>
    </Alert>
    <Alert v-if="statusError" variant="destructive" class="mb-4">
      <AlertDescription>{{ statusError }}</AlertDescription>
    </Alert>
    <Alert v-if="downloadError" variant="destructive" class="mb-4">
      <AlertDescription>{{ downloadError }}</AlertDescription>
    </Alert>

    <!-- Loading skeleton -->
    <div v-if="loading" class="space-y-4">
      <Skeleton class="h-10 w-96 rounded-lg" />
      <Card v-for="i in 2" :key="i">
        <CardHeader>
          <div class="flex items-center justify-between">
            <div class="space-y-2">
              <Skeleton class="h-5 w-48" />
              <Skeleton class="h-4 w-32" />
            </div>
            <Skeleton class="h-6 w-20 rounded-full" />
          </div>
        </CardHeader>
        <CardContent>
          <div class="space-y-2">
            <Skeleton class="h-4 w-full" />
            <Skeleton class="h-4 w-3/4" />
          </div>
        </CardContent>
      </Card>
    </div>

    <template v-else>
      <!-- Formule tabs -->
      <Tabs v-model="activeFormule">
        <TabsList>
          <TabsTrigger value="lab">Lab optimal</TabsTrigger>
          <TabsTrigger value="compromis_1">Compromis 1</TabsTrigger>
          <TabsTrigger value="compromis_2">Compromis 2</TabsTrigger>
          <TabsTrigger value="budget">Budget</TabsTrigger>
        </TabsList>

        <TabsContent
          v-for="formule in FORMULES"
          :key="formule"
          :value="formule"
          class="mt-4"
        >
          <!-- Empty formule -->
          <div v-if="pdtsByFormule[formule].length === 0" class="py-12 text-center">
            <div
              class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-muted"
            >
              <Icon name="lucide:file-plus" class="h-7 w-7 text-muted-foreground" />
            </div>
            <p class="text-base font-medium text-foreground">
              Aucun plan pour cette formule
            </p>
            <Button
              class="mt-4 bg-primary hover:bg-primary/90 text-primary-foreground"
              @click="openNewWithFormule(formule)"
            >
              Creer un plan {{ formuleLabel(formule) }}
            </Button>
          </div>

          <!-- PDT cards -->
          <div v-else class="space-y-4">
            <Card v-for="pdt in pdtsByFormule[formule]" :key="pdt.id" class="shadow-sm">
              <CardHeader>
                <div class="flex items-start justify-between">
                  <div>
                    <CardTitle class="text-base">
                      {{ pdt.titre ?? formuleLabel(pdt.formule) }}
                    </CardTitle>
                    <CardDescription class="mt-1">
                      Cree le {{ formatDate(pdt.created_at) }}
                      <span class="mx-1.5">|</span>
                      {{ pdt.lines.length }} ligne{{ pdt.lines.length !== 1 ? 's' : '' }}
                    </CardDescription>
                  </div>
                  <div class="flex items-center gap-2">
                    <Badge :class="statusClass(pdt.statut)">
                      {{ statusLabel(pdt.statut) }}
                    </Badge>
                    <Button
                      variant="ghost"
                      size="sm"
                      class="text-muted-foreground hover:text-foreground"
                      :disabled="downloadingId === pdt.id"
                      @click.stop="handleDownloadPdf(pdt.id)"
                    >
                      <Icon
                        :name="downloadingId === pdt.id ? 'lucide:loader-2' : 'lucide:download'"
                        class="h-4 w-4"
                        :class="{ 'animate-spin': downloadingId === pdt.id }"
                      />
                    </Button>
                  </div>
                </div>
              </CardHeader>

              <CardContent class="border-t pt-4">
                <!-- Lines summary -->
                <div v-if="pdt.lines.length > 0" class="mb-4 space-y-1">
                  <div
                    v-for="line in pdt.lines"
                    :key="line.id"
                    class="flex justify-between border-b border-border/50 py-1.5 text-sm last:border-0"
                  >
                    <div class="flex items-center gap-2">
                      <span class="text-foreground">{{ line.acte_libelle }}</span>
                      <Badge v-if="line.dent_fdi" variant="secondary" class="text-xs">
                        {{ line.dent_fdi }}
                      </Badge>
                    </div>
                    <span class="font-medium text-foreground">
                      {{ formatPrice(line.prix_unitaire * line.quantite) }}
                    </span>
                  </div>
                </div>

                <!-- Total & status -->
                <div class="flex items-center justify-between pt-2">
                  <div class="flex items-center gap-2">
                    <Label class="text-xs text-muted-foreground">Statut :</Label>
                    <Select
                      :model-value="pdt.statut"
                      @update:model-value="(val) => handleStatusUpdate(pdt.id, val as PdtStatus)"
                    >
                      <SelectTrigger class="h-7 w-40 text-xs">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="brouillon">Brouillon</SelectItem>
                        <SelectItem value="presente">Presente</SelectItem>
                        <SelectItem value="accepte">Accepte</SelectItem>
                        <SelectItem value="en_cours">En cours</SelectItem>
                        <SelectItem value="termine">Termine</SelectItem>
                        <SelectItem value="refuse">Refuse</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  <div class="text-right">
                    <p class="text-xs text-muted-foreground">Total</p>
                    <p class="text-lg font-bold text-foreground">
                      {{ formatPrice(pdt.prix_total) }}
                    </p>
                  </div>
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>
      </Tabs>
    </template>

    <!-- New PDT Dialog -->
    <Dialog v-model:open="showNewDialog">
      <DialogContent class="max-w-3xl max-h-[90vh] overflow-y-auto">
        <DialogHeader>
          <DialogTitle>Nouveau plan de traitement</DialogTitle>
          <DialogDescription>Definissez les actes et leur tarification.</DialogDescription>
        </DialogHeader>

        <form class="space-y-4" @submit.prevent="handleCreate">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="formule">Formule *</Label>
              <Select v-model="form.formule">
                <SelectTrigger id="formule">
                  <SelectValue placeholder="Selectionner" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="lab">Lab optimal</SelectItem>
                  <SelectItem value="compromis_1">Compromis 1</SelectItem>
                  <SelectItem value="compromis_2">Compromis 2</SelectItem>
                  <SelectItem value="budget">Budget</SelectItem>
                </SelectContent>
              </Select>
            </div>
            <div class="space-y-2">
              <Label for="titre_pdt">Titre (optionnel)</Label>
              <Input id="titre_pdt" v-model="form.titre" placeholder="Ex : Plan complet 2025" />
            </div>
          </div>

          <div class="space-y-2">
            <Label for="notes_pdt">Notes</Label>
            <Textarea
              id="notes_pdt"
              v-model="form.notes"
              placeholder="Remarques sur ce plan..."
              :rows="2"
            />
          </div>

          <PdtBuilder :lines="form.lines" @update="form.lines = $event" />

          <Alert v-if="createError" variant="destructive">
            <AlertDescription>{{ createError }}</AlertDescription>
          </Alert>

          <DialogFooter>
            <Button type="button" variant="outline" @click="showNewDialog = false">
              Annuler
            </Button>
            <Button
              type="submit"
              :disabled="creating || !form.formule"
              class="bg-primary hover:bg-primary/90 text-primary-foreground"
            >
              <Icon v-if="creating" name="lucide:loader-2" class="mr-2 h-4 w-4 animate-spin" />
              Creer le plan
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import type { Pdt, PdtFormule, PdtStatus, CreatePdtLineInput } from '~/composables/usePdt'
import { formatDate, formatPrice } from '~/utils/format'
import PdtBuilder from '~/components/pdt/PdtBuilder.vue'

const FORMULES: PdtFormule[] = ['lab', 'compromis_1', 'compromis_2', 'budget']

const route = useRoute()
const patientId = route.params.id as string

const { listPdts, createPdt, updatePdt, downloadPdf } = usePdt()

const pdts = ref<Pdt[]>([])
const loading = ref(true)
const showNewDialog = ref(false)
const creating = ref(false)
const createError = ref<string | null>(null)
const activeFormule = ref<PdtFormule>('lab')
const downloadingId = ref<string | null>(null)

const form = reactive({
  formule: '' as PdtFormule | '',
  titre: '',
  notes: '',
  lines: [] as CreatePdtLineInput[],
})

const pdtsByFormule = computed(() =>
  Object.fromEntries(
    FORMULES.map((f) => [f, pdts.value.filter((p) => p.formule === f)]),
  ) as Record<PdtFormule, Pdt[]>,
)

function formuleLabel(f: PdtFormule | string): string {
  const labels: Record<string, string> = {
    lab: 'Lab optimal',
    compromis_1: 'Compromis 1',
    compromis_2: 'Compromis 2',
    budget: 'Budget',
  }
  return labels[f] ?? f
}

function statusLabel(s: PdtStatus): string {
  const labels: Record<PdtStatus, string> = {
    brouillon: 'Brouillon',
    presente: 'Presente',
    accepte: 'Accepte',
    en_cours: 'En cours',
    termine: 'Termine',
    refuse: 'Refuse',
  }
  return labels[s] ?? s
}

function statusClass(s: PdtStatus): string {
  const classes: Record<PdtStatus, string> = {
    brouillon: 'bg-muted text-muted-foreground',
    presente: 'bg-blue-50 text-blue-700 border-blue-200',
    accepte: 'bg-emerald-50 text-emerald-700 border-emerald-200',
    en_cours: 'bg-amber-50 text-amber-700 border-amber-200',
    termine: 'bg-green-50 text-green-700 border-green-200',
    refuse: 'bg-red-50 text-red-700 border-red-200',
  }
  return classes[s] ?? ''
}

function openNewWithFormule(formule: PdtFormule): void {
  form.formule = formule
  showNewDialog.value = true
}

async function handleCreate(): Promise<void> {
  if (!form.formule) return
  creating.value = true
  createError.value = null

  try {
    const response = await createPdt(patientId, {
      formule: form.formule,
      titre: form.titre || null,
      notes: form.notes || null,
      lines: form.lines,
    })

    if (response.success && response.data) {
      pdts.value = [response.data, ...pdts.value]
      showNewDialog.value = false
      form.formule = ''
      form.titre = ''
      form.notes = ''
      form.lines = []
    } else {
      createError.value = response.error ?? 'Erreur lors de la creation'
    }
  } catch (err) {
    createError.value = err instanceof Error ? err.message : 'Erreur lors de la creation'
  } finally {
    creating.value = false
  }
}

const statusError = ref<string | null>(null)

async function handleStatusUpdate(pdtId: string, statut: PdtStatus): Promise<void> {
  statusError.value = null
  try {
    const response = await updatePdt(patientId, pdtId, { statut })
    if (response.success && response.data) {
      pdts.value = pdts.value.map((p) => (p.id === pdtId ? response.data! : p))
    }
  } catch (e) {
    statusError.value = e instanceof Error ? e.message : 'Erreur lors de la mise a jour du statut'
  }
}

const downloadError = ref<string | null>(null)

async function handleDownloadPdf(pdtId: string): Promise<void> {
  downloadingId.value = pdtId
  downloadError.value = null
  try {
    await downloadPdf(patientId, pdtId)
  } catch (e) {
    downloadError.value = e instanceof Error ? e.message : 'Erreur lors du telechargement du PDF'
  } finally {
    downloadingId.value = null
  }
}

const loadError = ref<string | null>(null)

onMounted(async () => {
  try {
    const response = await listPdts(patientId)
    if (response.success && response.data) {
      pdts.value = response.data
    }
  } catch (e) {
    loadError.value = e instanceof Error ? e.message : 'Erreur lors du chargement des plans'
  } finally {
    loading.value = false
  }
})
</script>
