import type {
  FullSchemaResponse,
  SchemaVersion,
  UpdateToothInput,
  ToothEntry,
  FaceName,
  FaceState,
  ToothStatus,
} from '~/components/dental-chart/types'

export function useDentalChart(patientId: Ref<string>) {
  const { apiGet, apiPost, apiPut } = useApi()

  const schema = ref<FullSchemaResponse | null>(null)
  const versions = ref<SchemaVersion[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  async function loadVersions(): Promise<void> {
    try {
      const res = await apiGet<SchemaVersion[]>(`/api/v1/patients/${patientId.value}/schema/versions`)
      if (res.success && res.data) {
        versions.value = res.data
      }
    } catch {
      // Silently fail — versions are non-critical
    }
  }

  async function loadSchema(version?: number): Promise<void> {
    loading.value = true
    error.value = null

    try {
      const query: Record<string, unknown> = {}
      if (version !== undefined) {
        query.version = version
      }
      const res = await apiGet<FullSchemaResponse>(
        `/api/v1/patients/${patientId.value}/schema`,
        query,
      )
      if (res.success && res.data) {
        schema.value = res.data
      } else {
        error.value = res.error ?? 'Erreur lors du chargement du schéma'
      }
    } catch (err) {
      error.value = 'Impossible de charger le schéma dentaire'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function createNewVersion(dentition: 'permanente' | 'lacteale' | 'mixte'): Promise<void> {
    loading.value = true
    error.value = null

    try {
      const res = await apiPost<FullSchemaResponse>(
        `/api/v1/patients/${patientId.value}/schema`,
        { dentition },
      )
      if (res.success && res.data) {
        schema.value = res.data
        await loadVersions()
      } else {
        error.value = res.error ?? 'Erreur lors de la création du schéma'
      }
    } catch (err) {
      error.value = 'Impossible de créer un nouveau schéma'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function updateTooth(fdi: number, data: UpdateToothInput): Promise<void> {
    try {
      const res = await apiPut<ToothEntry>(
        `/api/v1/patients/${patientId.value}/schema/dent/${fdi}`,
        data as Record<string, unknown>,
      )
      if (res.success && res.data && schema.value) {
        // Immutably update the tooth in the schema
        const updatedDents = schema.value.dents.map((entry) =>
          entry.tooth.numero_fdi === fdi ? res.data! : entry,
        )
        schema.value = { ...schema.value, dents: updatedDents }
      }
    } catch {
      throw new Error(`Erreur lors de la mise à jour de la dent ${fdi}`)
    }
  }

  async function updateFaceState(fdi: number, face: FaceName, etat: FaceState): Promise<void> {
    if (!schema.value) return

    const entry = schema.value.dents.find((e) => e.tooth.numero_fdi === fdi)
    if (!entry) return

    // Build the full faces payload with the updated face
    const updatedFaces = entry.faces.map((f) =>
      f.face === face ? { ...f, etat } : f,
    )

    // Include the face update in the tooth PUT payload
    await updateTooth(fdi, {
      faces: updatedFaces,
    } as UpdateToothInput)
  }

  return {
    schema,
    versions,
    loading,
    error,
    loadSchema,
    loadVersions,
    createNewVersion,
    updateTooth,
    updateFaceState,
  }
}
