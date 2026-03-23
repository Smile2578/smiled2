import type { Patient, CreatePatientInput, Questionnaire, QuestionnaireFormData } from '~/types/patient'
import type { ApiResponse } from '~/types/api'

export function usePatient() {
  const { apiGet, apiPost, apiPut } = useApi()

  async function listPatients(params?: {
    search?: string
    page?: number
    limit?: number
  }): Promise<ApiResponse<Patient[]>> {
    const query: Record<string, unknown> = {}
    if (params?.search) query.search = params.search
    if (params?.page) query.page = params.page
    if (params?.limit) query.limit = params.limit
    return apiGet<Patient[]>('/api/v1/patients', query)
  }

  async function getPatient(id: string): Promise<ApiResponse<Patient>> {
    return apiGet<Patient>(`/api/v1/patients/${id}`)
  }

  async function createPatient(data: CreatePatientInput): Promise<ApiResponse<Patient>> {
    return apiPost<Patient>('/api/v1/patients', data as Record<string, unknown>)
  }

  async function updatePatient(
    id: string,
    data: Partial<Patient>,
  ): Promise<ApiResponse<Patient>> {
    return apiPut<Patient>(`/api/v1/patients/${id}`, data as Record<string, unknown>)
  }

  async function getQuestionnaire(patientId: string): Promise<ApiResponse<Questionnaire>> {
    return apiGet<Questionnaire>(`/api/v1/patients/${patientId}/questionnaire`)
  }

  async function saveQuestionnaire(
    patientId: string,
    data: Partial<QuestionnaireFormData>,
  ): Promise<ApiResponse<Questionnaire>> {
    return apiPut<Questionnaire>(
      `/api/v1/patients/${patientId}/questionnaire`,
      data as Record<string, unknown>,
    )
  }

  return {
    listPatients,
    getPatient,
    createPatient,
    updatePatient,
    getQuestionnaire,
    saveQuestionnaire,
  }
}
