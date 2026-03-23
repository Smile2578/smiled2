import type { ApiResponse } from '~/types/api'

export interface DiagnosticFinding {
  id: string
  diagnostic_id: string
  type: string
  dent_fdi: number | null
  severite: number
  source: 'praticien' | 'ia'
  notes: string | null
}

export interface Diagnostic {
  id: string
  patient_id: string
  praticien_id: string
  titre: string | null
  notes: string | null
  created_at: string
  findings: DiagnosticFinding[]
}

export interface CreateFindingInput {
  type: string
  dent_fdi: number | null
  severite: number
  source: 'praticien' | 'ia'
  notes: string | null
}

export interface CreateDiagnosticInput {
  titre: string | null
  notes: string | null
  findings: CreateFindingInput[]
}

export function useDiagnostic() {
  const { apiGet, apiPost } = useApi()

  async function listDiagnostics(patientId: string): Promise<ApiResponse<Diagnostic[]>> {
    return apiGet<Diagnostic[]>(`/api/v1/patients/${patientId}/diagnostics`)
  }

  async function getDiagnostic(patientId: string, diagId: string): Promise<ApiResponse<Diagnostic>> {
    return apiGet<Diagnostic>(`/api/v1/patients/${patientId}/diagnostics/${diagId}`)
  }

  async function createDiagnostic(
    patientId: string,
    data: CreateDiagnosticInput,
  ): Promise<ApiResponse<Diagnostic>> {
    return apiPost<Diagnostic>(
      `/api/v1/patients/${patientId}/diagnostics`,
      data as Record<string, unknown>,
    )
  }

  return {
    listDiagnostics,
    getDiagnostic,
    createDiagnostic,
  }
}
