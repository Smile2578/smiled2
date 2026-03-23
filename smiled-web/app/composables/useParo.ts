import type { ApiResponse } from '~/types/api'

export interface ParoSite {
  id: string
  patient_id: string
  dent_fdi: number
  site: 'MB' | 'B' | 'DB' | 'ML' | 'L' | 'DL'
  profondeur_poche: number | null
  recession: number | null
  bop: boolean
  suppuration: boolean
  plaque: boolean
  mobilite: number | null
}

export interface ParoGlobal {
  id: string
  patient_id: string
  staging: string | null
  grading: string | null
  indice_plaque_pct: number | null
  bop_pct: number | null
  notes: string | null
  updated_at: string
}

export interface ParoSiteUpdate {
  dent_fdi: number
  site: string
  profondeur_poche: number | null
  recession: number | null
  bop: boolean
  suppuration: boolean
  plaque: boolean
  mobilite: number | null
}

export function useParo() {
  const { apiGet, apiPut } = useApi()

  async function getParoSites(patientId: string): Promise<ApiResponse<ParoSite[]>> {
    return apiGet<ParoSite[]>(`/api/v1/patients/${patientId}/schema/paro`)
  }

  async function updateParoSites(
    patientId: string,
    sites: ParoSiteUpdate[],
  ): Promise<ApiResponse<ParoSite[]>> {
    return apiPut<ParoSite[]>(`/api/v1/patients/${patientId}/schema/paro`, {
      sites,
    } as Record<string, unknown>)
  }

  async function getParoGlobal(patientId: string): Promise<ApiResponse<ParoGlobal>> {
    return apiGet<ParoGlobal>(`/api/v1/patients/${patientId}/schema/paro-global`)
  }

  async function updateParoGlobal(
    patientId: string,
    data: Partial<ParoGlobal>,
  ): Promise<ApiResponse<ParoGlobal>> {
    return apiPut<ParoGlobal>(
      `/api/v1/patients/${patientId}/schema/paro-global`,
      data as Record<string, unknown>,
    )
  }

  return {
    getParoSites,
    updateParoSites,
    getParoGlobal,
    updateParoGlobal,
  }
}
