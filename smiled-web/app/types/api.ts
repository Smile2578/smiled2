export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
  meta?: {
    total: number
    page: number
    limit: number
  }
}

export interface User {
  id: string
  cabinet_id: string
  role: string
  nom: string
  prenom: string
  email: string
  name: string
}

export interface LoginResponse {
  access_token: string
  refresh_token: string
  user: User
}
