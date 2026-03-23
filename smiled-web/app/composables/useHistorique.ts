import type { ApiResponse } from '~/types/api'

export interface HistoriqueEvent {
  id: string
  patient_id: string
  dent_fdi: number | null
  type: string
  description: string | null
  praticien_nom: string | null
  created_at: string
}

export function useHistorique() {
  const { apiGet } = useApi()

  async function getDentTimeline(
    patientId: string,
    fdi: number,
  ): Promise<ApiResponse<HistoriqueEvent[]>> {
    return apiGet<HistoriqueEvent[]>(`/api/v1/patients/${patientId}/dent/${fdi}/timeline`)
  }

  return {
    getDentTimeline,
  }
}
