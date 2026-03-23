import type { ApiResponse, User } from '~/types/api'

export interface Cabinet {
  id: string
  nom: string
  adresse: string | null
  siret: string | null
  finess: string | null
  telephone: string | null
  email: string | null
}

export interface UpdateCabinetInput {
  nom: string
  adresse: string | null
  siret: string | null
  finess: string | null
  telephone: string | null
  email: string | null
}

export function useCabinet() {
  const { apiGet, apiPut } = useApi()

  async function getCabinet(): Promise<ApiResponse<Cabinet>> {
    return apiGet<Cabinet>('/api/v1/cabinet')
  }

  async function updateCabinet(data: UpdateCabinetInput): Promise<ApiResponse<Cabinet>> {
    return apiPut<Cabinet>('/api/v1/cabinet', data as Record<string, unknown>)
  }

  async function getCabinetUsers(): Promise<ApiResponse<User[]>> {
    return apiGet<User[]>('/api/v1/cabinet/users')
  }

  return {
    getCabinet,
    updateCabinet,
    getCabinetUsers,
  }
}
