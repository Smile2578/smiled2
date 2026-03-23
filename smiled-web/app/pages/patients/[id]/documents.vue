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
          <h1 class="text-2xl font-bold">Documents</h1>
          <p class="text-sm text-muted-foreground">
            {{ documents.length }} document{{ documents.length !== 1 ? 's' : '' }}
          </p>
        </div>
      </div>
    </div>

    <!-- Upload zone -->
    <Card class="mb-6">
      <CardHeader>
        <CardTitle class="text-base">Ajouter un document</CardTitle>
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
                  class="accent-primary"
                />
                <span class="text-sm">{{ t.label }}</span>
              </label>
            </div>
          </div>

          <div
            class="border-2 border-dashed rounded-lg p-8 text-center transition-colors"
            :class="isDragging ? 'border-primary bg-primary/5' : 'border-muted-foreground/30'"
            @dragover.prevent="isDragging = true"
            @dragleave.prevent="isDragging = false"
            @drop.prevent="handleDrop"
          >
            <Icon name="lucide:upload-cloud" class="w-10 h-10 mx-auto mb-3 text-muted-foreground" />
            <p class="text-sm text-muted-foreground mb-2">
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
              @click="handleUpload"
            >
              <Icon v-if="uploading" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
              <Icon v-else name="lucide:upload" class="w-4 h-4 mr-2" />
              Téléverser
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>

    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-32">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
    </div>

    <!-- Empty -->
    <div v-else-if="documents.length === 0" class="text-center py-12 text-muted-foreground">
      <Icon name="lucide:file-x" class="w-12 h-12 mx-auto mb-3 opacity-30" />
      <p>Aucun document enregistré</p>
    </div>

    <!-- Documents list -->
    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <Card v-for="doc in documents" :key="doc.id" class="overflow-hidden">
        <!-- Preview -->
        <div class="h-32 bg-muted flex items-center justify-center border-b">
          <img
            v-if="isImage(doc.mime_type)"
            :src="doc.url"
            :alt="doc.nom"
            class="h-full w-full object-cover"
          />
          <Icon
            v-else
            :name="documentIcon(doc.type)"
            class="w-12 h-12 text-muted-foreground"
          />
        </div>

        <CardContent class="p-3">
          <div class="mb-2">
            <p class="text-sm font-medium truncate" :title="doc.nom">{{ doc.nom }}</p>
            <div class="flex items-center gap-2 mt-1">
              <Badge variant="outline" class="text-xs">{{ typeLabel(doc.type) }}</Badge>
              <span class="text-xs text-muted-foreground">{{ formatSize(doc.taille) }}</span>
            </div>
          </div>

          <!-- Linked teeth -->
          <div v-if="doc.dents_liees.length > 0" class="mb-2">
            <p class="text-xs text-muted-foreground mb-1">Dents liées :</p>
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

          <div class="flex items-center justify-between mt-2">
            <span class="text-xs text-muted-foreground">
              {{ formatDate(doc.uploaded_at) }}
            </span>
            <div class="flex gap-1">
              <Button
                variant="ghost"
                size="sm"
                class="h-7 w-7 p-0"
                title="Lier à une dent"
                @click="openLinkDialog(doc.id)"
              >
                <Icon name="lucide:link" class="w-3.5 h-3.5" />
              </Button>
              <Button
                variant="ghost"
                size="sm"
                class="h-7 w-7 p-0"
                title="Ouvrir"
                @click="openDocument(doc.url)"
              >
                <Icon name="lucide:external-link" class="w-3.5 h-3.5" />
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
          <DialogTitle>Lier à une dent</DialogTitle>
          <DialogDescription>Saisissez le numéro FDI de la dent à associer.</DialogDescription>
        </DialogHeader>

        <div class="space-y-3">
          <div class="space-y-2">
            <Label for="fdi_link">Numéro FDI</Label>
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
          <Button :disabled="!linkFdi || linking" @click="handleLink">
            <Icon v-if="linking" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
            Lier
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import type { PatientDocument, DocumentType } from '~/composables/useDocument'

definePageMeta({ layout: 'default' })

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
      uploadError.value = response.error ?? 'Erreur lors du téléversement'
    }
  } catch (err) {
    uploadError.value = err instanceof Error ? err.message : 'Erreur lors du téléversement'
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

function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString('fr-FR')
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
