import type { ApiResponse } from '~/types/api'

export interface Teinte {
  id: string
  code: string
  libelle: string
  systeme: string | null
  ordre: number | null
}

export interface CreateTeinteInput {
  code: string
  libelle: string
  systeme: string | null
}

export function useTeinte() {
  const { apiGet, apiPost, apiPut } = useApi()

  async function listTeintes(): Promise<ApiResponse<Teinte[]>> {
    return apiGet<Teinte[]>('/api/v1/teintes')
  }

  async function createTeinte(data: CreateTeinteInput): Promise<ApiResponse<Teinte>> {
    return apiPost<Teinte>('/api/v1/teintes', data as Record<string, unknown>)
  }

  async function updateTeinte(id: string, data: Partial<CreateTeinteInput>): Promise<ApiResponse<Teinte>> {
    return apiPut<Teinte>(`/api/v1/teintes/${id}`, data as Record<string, unknown>)
  }

  return {
    listTeintes,
    createTeinte,
    updateTeinte,
  }
}
