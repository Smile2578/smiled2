import type { ApiResponse } from '~/types/api'

export type Nomenclature = 'ccam' | 'ngap' | 'hors_nomenclature'

export interface Acte {
  id: string
  code: string | null
  libelle: string
  nomenclature: Nomenclature
  prix_defaut: number
  prix_cabinet: number | null
  actif: boolean
  systeme: boolean
  description: string | null
  categorie: string | null
}

export interface CreateActeInput {
  code: string | null
  libelle: string
  nomenclature: Nomenclature
  prix_defaut: number
  description: string | null
  categorie: string | null
}

export function useActe() {
  const { apiGet, apiPost, apiPut } = useApi()

  async function listActes(): Promise<ApiResponse<Acte[]>> {
    return apiGet<Acte[]>('/api/v1/actes')
  }

  async function createActe(data: CreateActeInput): Promise<ApiResponse<Acte>> {
    return apiPost<Acte>('/api/v1/actes', data as Record<string, unknown>)
  }

  async function updateTarifActe(id: string, prix: number): Promise<ApiResponse<Acte>> {
    return apiPut<Acte>(`/api/v1/actes/${id}/tarif`, { prix_cabinet: prix })
  }

  async function toggleActe(id: string): Promise<ApiResponse<Acte>> {
    return apiPut<Acte>(`/api/v1/actes/${id}/toggle`, {})
  }

  return {
    listActes,
    createActe,
    updateTarifActe,
    toggleActe,
  }
}
