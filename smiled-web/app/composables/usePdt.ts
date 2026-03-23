import type { ApiResponse } from '~/types/api'

export type PdtFormule = 'lab' | 'compromis_1' | 'compromis_2' | 'budget'
export type PdtStatus =
  | 'brouillon'
  | 'presente'
  | 'accepte'
  | 'en_cours'
  | 'termine'
  | 'refuse'

export interface PdtLine {
  id: string
  pdt_id: string
  acte_id: string | null
  acte_libelle: string
  acte_code: string | null
  dent_fdi: number | null
  faces: string[] | null
  materiau_id: string | null
  materiau_libelle: string | null
  teinte_id: string | null
  teinte_libelle: string | null
  prix_unitaire: number
  quantite: number
  notes: string | null
  ordre: number
}

export interface Pdt {
  id: string
  patient_id: string
  formule: PdtFormule
  titre: string | null
  statut: PdtStatus
  notes: string | null
  prix_total: number
  created_at: string
  updated_at: string
  lines: PdtLine[]
}

export interface CreatePdtLineInput {
  acte_id: string | null
  acte_libelle: string
  acte_code: string | null
  dent_fdi: number | null
  faces: string[] | null
  materiau_id: string | null
  teinte_id: string | null
  prix_unitaire: number
  quantite: number
  notes: string | null
}

export interface CreatePdtInput {
  formule: PdtFormule
  titre: string | null
  notes: string | null
  lines: CreatePdtLineInput[]
}

export interface UpdatePdtInput {
  statut?: PdtStatus
  titre?: string | null
  notes?: string | null
  lines?: CreatePdtLineInput[]
}

export function usePdt() {
  const { apiGet, apiPost, apiPut, apiFetch } = useApi()

  async function listPdts(patientId: string): Promise<ApiResponse<Pdt[]>> {
    return apiGet<Pdt[]>(`/api/v1/patients/${patientId}/pdts`)
  }

  async function createPdt(patientId: string, data: CreatePdtInput): Promise<ApiResponse<Pdt>> {
    return apiPost<Pdt>(`/api/v1/patients/${patientId}/pdts`, data as Record<string, unknown>)
  }

  async function updatePdt(
    patientId: string,
    pdtId: string,
    data: UpdatePdtInput,
  ): Promise<ApiResponse<Pdt>> {
    return apiPut<Pdt>(
      `/api/v1/patients/${patientId}/pdts/${pdtId}`,
      data as Record<string, unknown>,
    )
  }

  async function downloadPdf(patientId: string, pdtId: string): Promise<void> {
    const config = useRuntimeConfig()
    const auth = useAuthStore()

    const response = await fetch(
      `${config.public.apiBase}/api/v1/patients/${patientId}/pdts/${pdtId}/pdf`,
      {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${auth.accessToken}`,
        },
      },
    )

    if (!response.ok) throw new Error('Erreur lors de la génération du PDF')

    const blob = await response.blob()
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `plan-de-traitement-${pdtId}.pdf`
    a.click()
    URL.revokeObjectURL(url)
  }

  return {
    listPdts,
    createPdt,
    updatePdt,
    downloadPdf,
  }
}
