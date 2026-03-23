import type { ApiResponse } from '~/types/api'

export interface Materiau {
  id: string
  code: string
  libelle: string
  categorie: string
  sous_categorie: string | null
  marques: string[] | null
  resistance_mpa: number | null
  translucidite: string | null
  obsolete: boolean
  systeme: boolean
  notes: string | null
}

export interface CreateMateriauInput {
  code: string
  libelle: string
  categorie: string
  sous_categorie: string | null
  marques: string[] | null
  resistance_mpa: number | null
  notes: string | null
}

export function useMateriau() {
  const { apiGet, apiPost, apiPut } = useApi()

  async function listMateriaux(): Promise<ApiResponse<Materiau[]>> {
    return apiGet<Materiau[]>('/api/v1/materiaux')
  }

  async function getCategories(): Promise<ApiResponse<string[]>> {
    return apiGet<string[]>('/api/v1/materiaux/categories')
  }

  async function createMateriau(data: CreateMateriauInput): Promise<ApiResponse<Materiau>> {
    return apiPost<Materiau>('/api/v1/materiaux', data as Record<string, unknown>)
  }

  async function updateMateriau(id: string, data: Partial<CreateMateriauInput>): Promise<ApiResponse<Materiau>> {
    return apiPut<Materiau>(`/api/v1/materiaux/${id}`, data as Record<string, unknown>)
  }

  return {
    listMateriaux,
    getCategories,
    createMateriau,
    updateMateriau,
  }
}
