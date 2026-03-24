import type { ApiResponse } from '~/types/api'

export function useApi() {
  const config = useRuntimeConfig()

  async function apiFetch<T>(url: string, options?: Parameters<typeof $fetch>[1]): Promise<T> {
    return await $fetch<T>(`${config.public.apiBase}${url}`, {
      ...options,
      credentials: 'include',
    })
  }

  async function apiGet<T>(url: string, query?: Record<string, unknown>): Promise<ApiResponse<T>> {
    return apiFetch<ApiResponse<T>>(url, { method: 'GET', query })
  }

  async function apiPost<T>(url: string, body: Record<string, unknown>): Promise<ApiResponse<T>> {
    return apiFetch<ApiResponse<T>>(url, { method: 'POST', body })
  }

  async function apiPut<T>(url: string, body: Record<string, unknown>): Promise<ApiResponse<T>> {
    return apiFetch<ApiResponse<T>>(url, { method: 'PUT', body })
  }

  async function apiPatch<T>(url: string, body: Record<string, unknown>): Promise<ApiResponse<T>> {
    return apiFetch<ApiResponse<T>>(url, { method: 'PATCH', body })
  }

  async function apiDelete<T>(url: string): Promise<ApiResponse<T>> {
    return apiFetch<ApiResponse<T>>(url, { method: 'DELETE' })
  }

  return {
    apiFetch,
    apiGet,
    apiPost,
    apiPut,
    apiPatch,
    apiDelete,
  }
}
