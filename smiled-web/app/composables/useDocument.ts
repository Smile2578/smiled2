import type { ApiResponse } from '~/types/api'

export type DocumentType = 'radio' | 'panoramique' | 'photo' | 'autre'

export interface PatientDocument {
  id: string
  patient_id: string
  nom: string
  type: DocumentType
  mime_type: string
  taille: number
  url: string
  dents_liees: number[]
  uploaded_at: string
  uploaded_by: string
}

export function useDocument() {
  const { apiGet, apiPost, apiFetch } = useApi()

  async function listDocuments(patientId: string): Promise<ApiResponse<PatientDocument[]>> {
    return apiGet<PatientDocument[]>(`/api/v1/patients/${patientId}/documents`)
  }

  async function uploadDocument(
    patientId: string,
    file: File,
    type: DocumentType,
  ): Promise<ApiResponse<PatientDocument>> {
    const config = useRuntimeConfig()
    const auth = useAuthStore()

    const formData = new FormData()
    formData.append('file', file)
    formData.append('type', type)

    const response = await fetch(
      `${config.public.apiBase}/api/v1/patients/${patientId}/documents/upload`,
      {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${auth.accessToken}`,
        },
        body: formData,
      },
    )

    if (!response.ok) throw new Error('Erreur lors de l\'upload du document')

    return response.json() as Promise<ApiResponse<PatientDocument>>
  }

  async function linkToTooth(
    patientId: string,
    docId: string,
    fdi: number,
  ): Promise<ApiResponse<PatientDocument>> {
    return apiPost<PatientDocument>(
      `/api/v1/patients/${patientId}/documents/${docId}/link-dent/${fdi}`,
      {},
    )
  }

  return {
    listDocuments,
    uploadDocument,
    linkToTooth,
  }
}
