import type { ApiResponse } from '~/types/api'

export function useApi() {
  const config = useRuntimeConfig()
  const auth = useAuthStore()

  async function apiFetch<T>(url: string, options?: Parameters<typeof $fetch>[1]): Promise<T> {
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...(options?.headers as Record<string, string> | undefined),
    }

    if (auth.accessToken) {
      headers['Authorization'] = `Bearer ${auth.accessToken}`
    }

    try {
      return await $fetch<T>(`${config.public.apiBase}${url}`, {
        ...options,
        headers,
      })
    } catch (error: unknown) {
      const fetchError = error as { statusCode?: number }

      if (fetchError.statusCode === 401 && auth.refreshToken) {
        await auth.refresh()

        const retryHeaders: Record<string, string> = {
          ...headers,
          Authorization: `Bearer ${auth.accessToken}`,
        }

        return await $fetch<T>(`${config.public.apiBase}${url}`, {
          ...options,
          headers: retryHeaders,
        })
      }

      throw error
    }
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
