<template>
  <div>
    <!-- Header -->
    <div class="mb-6">
      <h2 class="text-lg font-semibold text-foreground">Documents</h2>
      <p class="text-sm text-muted-foreground">
        Radiographies, photos et fichiers associes au patient
      </p>
    </div>

    <!-- Upload zone -->
    <Card class="mb-6 shadow-sm">
      <CardHeader class="pb-4">
        <div class="flex items-center gap-2">
          <Icon name="lucide:upload-cloud" class="h-4 w-4 text-muted-foreground" />
          <CardTitle class="text-base">Ajouter un document</CardTitle>
        </div>
      </CardHeader>
      <CardContent>
        <div class="space-y-4">
          <div class="space-y-2">
            <Label>Type de document</Label>
            <div class="flex flex-wrap gap-3">
              <label
                v-for="t in DOCUMENT_TYPES"
                :key="t.value"
                class="flex items-center gap-2 cursor-pointer"
              >
                <input
                  v-model="uploadForm.type"
                  type="radio"
                  :value="t.value"
                  class="accent-[hsl(var(--primary))]"
                />
                <span class="text-sm">{{ t.label }}</span>
              </label>
            </div>
          </div>

          <div
            class="border-2 border-dashed rounded-lg p-8 text-center transition-colors"
            :class="isDragging ? 'border-primary bg-primary/5' : 'border-border'"
            @dragover.prevent="isDragging = true"
            @dragleave.prevent="isDragging = false"
            @drop.prevent="handleDrop"
          >
            <Icon name="lucide:upload-cloud" class="mx-auto mb-3 h-10 w-10 text-muted-foreground" />
            <p class="mb-2 text-sm text-muted-foreground">
              Glissez un fichier ici ou
            </p>
            <label class="cursor-pointer">
              <Button type="button" variant="outline" size="sm" @click="fileInput?.click()">
                Choisir un fichier
              </Button>
              <input
                ref="fileInput"
                type="file"
                class="hidden"
                accept="image/*,.pdf,.dcm"
                @change="handleFileSelect"
              />
            </label>
            <p v-if="uploadForm.file" class="mt-3 text-sm font-medium text-primary">
              {{ uploadForm.file.name }} ({{ formatSize(uploadForm.file.size) }})
            </p>
          </div>

          <Alert v-if="uploadError" variant="destructive">
            <AlertDescription>{{ uploadError }}</AlertDescription>
          </Alert>

          <div class="flex justify-end">
            <Button
              :disabled="!uploadForm.file || uploading"
              class="bg-primary hover:bg-primary/90 text-primary-foreground"
              @click="handleUpload"
            >
              <Icon v-if="uploading" name="lucide:loader-2" class="mr-2 h-4 w-4 animate-spin" />
              <Icon v-else name="lucide:upload" class="mr-2 h-4 w-4" />
              Televerser
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Loading -->
    <div v-if="loading" class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
      <Card v-for="i in 3" :key="i">
        <Skeleton class="h-32 w-full" />
        <CardContent class="p-3 space-y-2">
          <Skeleton class="h-4 w-3/4" />
          <Skeleton class="h-3 w-1/2" />
        </CardContent>
      </Card>
    </div>

    <!-- Empty -->
    <div v-else-if="documents.length === 0" class="py-12 text-center">
      <div class="mx-auto mb-4 flex h-14 w-14 items-center justify-center rounded-full bg-muted">
        <Icon name="lucide:file-x" class="h-7 w-7 text-muted-foreground" />
      </div>
      <p class="text-base font-medium text-foreground">Aucun document</p>
      <p class="mt-1 text-sm text-muted-foreground">
        Televersez un premier document pour ce patient
      </p>
    </div>

    <!-- Documents grid -->
    <div v-else class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
      <Card v-for="doc in documents" :key="doc.id" class="overflow-hidden shadow-sm">
        <!-- Preview -->
        <div class="flex h-32 items-center justify-center border-b bg-muted/30">
          <img
            v-if="isImage(doc.mime_type)"
            :src="doc.url"
            :alt="doc.nom"
            class="h-full w-full object-cover"
          />
          <Icon
            v-else
            :name="documentIcon(doc.type)"
            class="h-12 w-12 text-muted-foreground"
          />
        </div>

        <CardContent class="p-3">
          <div class="mb-2">
            <p class="truncate text-sm font-medium text-foreground" :title="doc.nom">
              {{ doc.nom }}
            </p>
            <div class="mt-1 flex items-center gap-2">
              <Badge variant="outline" class="text-xs">{{ typeLabel(doc.type) }}</Badge>
              <span class="text-xs text-muted-foreground">{{ formatSize(doc.taille) }}</span>
            </div>
          </div>

          <!-- Linked teeth -->
          <div v-if="doc.dents_liees.length > 0" class="mb-2">
            <p class="mb-1 text-xs text-muted-foreground">Dents liees :</p>
            <div class="flex flex-wrap gap-1">
              <Badge
                v-for="fdi in doc.dents_liees"
                :key="fdi"
                variant="secondary"
                class="text-xs"
              >
                {{ fdi }}
              </Badge>
            </div>
          </div>

          <div class="mt-2 flex items-center justify-between">
            <span class="text-xs text-muted-foreground">
              {{ formatDate(doc.uploaded_at) }}
            </span>
            <div class="flex gap-1">
              <Button
                variant="ghost"
                size="sm"
                class="h-7 w-7 p-0 text-muted-foreground hover:text-foreground"
                title="Lier a une dent"
                @click="openLinkDialog(doc.id)"
              >
                <Icon name="lucide:link" class="h-3.5 w-3.5" />
              </Button>
              <Button
                variant="ghost"
                size="sm"
                class="h-7 w-7 p-0 text-muted-foreground hover:text-foreground"
                title="Ouvrir"
                @click="openDocument(doc.url)"
              >
                <Icon name="lucide:external-link" class="h-3.5 w-3.5" />
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>

    <!-- Link to tooth dialog -->
    <Dialog v-model:open="showLinkDialog">
      <DialogContent class="max-w-sm">
        <DialogHeader>
          <DialogTitle>Lier a une dent</DialogTitle>
          <DialogDescription>
            Saisissez le numero FDI de la dent a associer.
          </DialogDescription>
        </DialogHeader>

        <div class="space-y-3">
          <div class="space-y-2">
            <Label for="fdi_link">Numero FDI</Label>
            <Input
              id="fdi_link"
              v-model.number="linkFdi"
              type="number"
              min="11"
              max="85"
              placeholder="Ex : 36"
            />
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" @click="showLinkDialog = false">Annuler</Button>
          <Button
            :disabled="!linkFdi || linking"
            class="bg-primary hover:bg-primary/90 text-primary-foreground"
            @click="handleLink"
          >
            <Icon v-if="linking" name="lucide:loader-2" class="mr-2 h-4 w-4 animate-spin" />
            Lier
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import type { PatientDocument, DocumentType } from '~/composables/useDocument'
import { formatDate } from '~/utils/format'

const DOCUMENT_TYPES: { value: DocumentType; label: string }[] = [
  { value: 'radio', label: 'Radiographie' },
  { value: 'panoramique', label: 'Panoramique' },
  { value: 'photo', label: 'Photo' },
  { value: 'autre', label: 'Autre' },
]

const route = useRoute()
const patientId = route.params.id as string

const { listDocuments, uploadDocument, linkToTooth } = useDocument()

const documents = ref<PatientDocument[]>([])
const loading = ref(true)
const uploading = ref(false)
const uploadError = ref<string | null>(null)
const isDragging = ref(false)
const fileInput = ref<HTMLInputElement | null>(null)
const showLinkDialog = ref(false)
const linkDocId = ref<string | null>(null)
const linkFdi = ref<number | null>(null)
const linking = ref(false)

const uploadForm = reactive({
  file: null as File | null,
  type: 'radio' as DocumentType,
})

function handleFileSelect(event: Event): void {
  const input = event.target as HTMLInputElement
  if (input.files?.[0]) {
    uploadForm.file = input.files[0]
  }
}

function handleDrop(event: DragEvent): void {
  isDragging.value = false
  const file = event.dataTransfer?.files[0]
  if (file) {
    uploadForm.file = file
  }
}

function openLinkDialog(docId: string): void {
  linkDocId.value = docId
  linkFdi.value = null
  showLinkDialog.value = true
}

async function handleUpload(): Promise<void> {
  if (!uploadForm.file) return
  uploading.value = true
  uploadError.value = null

  try {
    const response = await uploadDocument(patientId, uploadForm.file, uploadForm.type)
    if (response.success && response.data) {
      documents.value = [response.data, ...documents.value]
      uploadForm.file = null
      if (fileInput.value) fileInput.value.value = ''
    } else {
      uploadError.value = response.error ?? 'Erreur lors du televersement'
    }
  } catch (err) {
    uploadError.value = err instanceof Error ? err.message : 'Erreur lors du televersement'
  } finally {
    uploading.value = false
  }
}

async function handleLink(): Promise<void> {
  if (!linkDocId.value || !linkFdi.value) return
  linking.value = true

  try {
    const response = await linkToTooth(patientId, linkDocId.value, linkFdi.value)
    if (response.success && response.data) {
      documents.value = documents.value.map((d) =>
        d.id === linkDocId.value ? response.data! : d,
      )
      showLinkDialog.value = false
    }
  } catch {
    // silent
  } finally {
    linking.value = false
  }
}

function isImage(mimeType: string): boolean {
  return mimeType.startsWith('image/')
}

function documentIcon(type: DocumentType): string {
  const icons: Record<DocumentType, string> = {
    radio: 'lucide:scan-line',
    panoramique: 'lucide:image',
    photo: 'lucide:camera',
    autre: 'lucide:file',
  }
  return icons[type] ?? 'lucide:file'
}

function typeLabel(type: DocumentType): string {
  return DOCUMENT_TYPES.find((t) => t.value === type)?.label ?? type
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} o`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(0)} Ko`
  return `${(bytes / (1024 * 1024)).toFixed(1)} Mo`
}

function openDocument(url: string): void {
  window.open(url, '_blank')
}

onMounted(async () => {
  try {
    const response = await listDocuments(patientId)
    if (response.success && response.data) {
      documents.value = response.data
    }
  } catch {
    // ignore
  } finally {
    loading.value = false
  }
})
</script>
