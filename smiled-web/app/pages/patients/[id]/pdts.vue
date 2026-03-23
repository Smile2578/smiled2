<template>
  <div class="p-8">
    <!-- Header -->
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center gap-4">
        <Button variant="ghost" size="sm" @click="navigateTo(`/patients/${patientId}`)">
          <Icon name="lucide:arrow-left" class="w-4 h-4 mr-2" />
          Retour
        </Button>
        <Separator orientation="vertical" class="h-6" />
        <div>
          <h1 class="text-2xl font-bold">Plans de traitement</h1>
          <p class="text-sm text-muted-foreground">Propositions thérapeutiques par formule</p>
        </div>
      </div>
      <Button @click="showNewDialog = true">
        <Icon name="lucide:plus" class="w-4 h-4 mr-2" />
        Nouveau plan
      </Button>
    </div>

    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-64">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
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
          <div v-if="pdtsByFormule[formule].length === 0" class="text-center py-12 text-muted-foreground">
            <Icon name="lucide:file-plus" class="w-12 h-12 mx-auto mb-3 opacity-30" />
            <p>Aucun plan de traitement pour cette formule</p>
            <Button class="mt-4" variant="outline" @click="openNewWithFormule(formule)">
              Créer un plan {{ formuleLabel(formule) }}
            </Button>
          </div>

          <div v-else class="space-y-4">
            <Card v-for="pdt in pdtsByFormule[formule]" :key="pdt.id">
              <CardHeader>
                <div class="flex items-start justify-between">
                  <div>
                    <CardTitle class="text-base">
                      {{ pdt.titre ?? formuleLabel(pdt.formule) }}
                    </CardTitle>
                    <CardDescription>
                      Créé le {{ formatDate(pdt.created_at) }}
                      · {{ pdt.lines.length }} ligne{{ pdt.lines.length !== 1 ? 's' : '' }}
                    </CardDescription>
                  </div>
                  <div class="flex items-center gap-2">
                    <Badge :class="statusClass(pdt.statut)">
                      {{ statusLabel(pdt.statut) }}
                    </Badge>
                    <Button
                      variant="ghost"
                      size="sm"
                      :disabled="downloadingId === pdt.id"
                      @click.stop="handleDownloadPdf(pdt.id)"
                    >
                      <Icon
                        :name="downloadingId === pdt.id ? 'lucide:loader-2' : 'lucide:download'"
                        class="w-4 h-4"
                        :class="{ 'animate-spin': downloadingId === pdt.id }"
                      />
                    </Button>
                  </div>
                </div>
              </CardHeader>

              <CardContent>
                <!-- Lines summary -->
                <div v-if="pdt.lines.length > 0" class="space-y-1 mb-4">
                  <div
                    v-for="line in pdt.lines"
                    :key="line.id"
                    class="flex justify-between text-sm py-1 border-b last:border-0"
                  >
                    <div class="flex items-center gap-2">
                      <span>{{ line.acte_libelle }}</span>
                      <span v-if="line.dent_fdi" class="text-xs text-muted-foreground bg-muted px-1.5 py-0.5 rounded">
                        {{ line.dent_fdi }}
                      </span>
                    </div>
                    <span class="font-medium">{{ formatPrice(line.prix_unitaire * line.quantite) }}</span>
                  </div>
                </div>

                <!-- Total & status -->
                <div class="flex items-center justify-between pt-2">
                  <div class="flex items-center gap-2">
                    <Label class="text-xs">Statut :</Label>
                    <Select
                      :model-value="pdt.statut"
                      @update:model-value="(val) => handleStatusUpdate(pdt.id, val as PdtStatus)"
                    >
                      <SelectTrigger class="h-7 text-xs w-40">
                        <SelectValue />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="brouillon">Brouillon</SelectItem>
                        <SelectItem value="presente">Présenté</SelectItem>
                        <SelectItem value="accepte">Accepté</SelectItem>
                        <SelectItem value="en_cours">En cours</SelectItem>
                        <SelectItem value="termine">Terminé</SelectItem>
                        <SelectItem value="refuse">Refusé</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  <div class="text-right">
                    <p class="text-xs text-muted-foreground">Total</p>
                    <p class="text-lg font-bold">{{ formatPrice(pdt.prix_total) }}</p>
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
          <DialogDescription>Définissez les actes et leur tarification.</DialogDescription>
        </DialogHeader>

        <form class="space-y-4" @submit.prevent="handleCreate">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="formule">Formule *</Label>
              <Select v-model="form.formule">
                <SelectTrigger id="formule">
                  <SelectValue placeholder="Sélectionner" />
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
            <Textarea id="notes_pdt" v-model="form.notes" placeholder="Remarques sur ce plan..." :rows="2" />
          </div>

          <PdtBuilder :lines="form.lines" @update="form.lines = $event" />

          <Alert v-if="createError" variant="destructive">
            <AlertDescription>{{ createError }}</AlertDescription>
          </Alert>

          <DialogFooter>
            <Button type="button" variant="outline" @click="showNewDialog = false">Annuler</Button>
            <Button type="submit" :disabled="creating || !form.formule">
              <Icon v-if="creating" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
              Créer le plan
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import type { Pdt, PdtFormule, PdtStatus, CreatePdtLineInput } from '~/composables/usePdt'
import PdtBuilder from '~/components/pdt/PdtBuilder.vue'

definePageMeta({ layout: 'default' })

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
    presente: 'Présenté',
    accepte: 'Accepté',
    en_cours: 'En cours',
    termine: 'Terminé',
    refuse: 'Refusé',
  }
  return labels[s] ?? s
}

function statusClass(s: PdtStatus): string {
  const classes: Record<PdtStatus, string> = {
    brouillon: 'bg-gray-100 text-gray-700',
    presente: 'bg-blue-100 text-blue-700',
    accepte: 'bg-green-100 text-green-700',
    en_cours: 'bg-yellow-100 text-yellow-700',
    termine: 'bg-emerald-100 text-emerald-700',
    refuse: 'bg-red-100 text-red-700',
  }
  return classes[s] ?? ''
}

function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString('fr-FR')
}

function formatPrice(price: number): string {
  return new Intl.NumberFormat('fr-FR', { style: 'currency', currency: 'EUR' }).format(price)
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
      createError.value = response.error ?? 'Erreur lors de la création'
    }
  } catch (err) {
    createError.value = err instanceof Error ? err.message : 'Erreur lors de la création'
  } finally {
    creating.value = false
  }
}

async function handleStatusUpdate(pdtId: string, statut: PdtStatus): Promise<void> {
  try {
    const response = await updatePdt(patientId, pdtId, { statut })
    if (response.success && response.data) {
      pdts.value = pdts.value.map((p) => (p.id === pdtId ? response.data! : p))
    }
  } catch {
    // silent
  }
}

async function handleDownloadPdf(pdtId: string): Promise<void> {
  downloadingId.value = pdtId
  try {
    await downloadPdf(patientId, pdtId)
  } catch {
    // silent
  } finally {
    downloadingId.value = null
  }
}

onMounted(async () => {
  try {
    const response = await listPdts(patientId)
    if (response.success && response.data) {
      pdts.value = response.data
    }
  } catch {
    // ignore
  } finally {
    loading.value = false
  }
})
</script>
