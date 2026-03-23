import { defineStore } from 'pinia'
import type { User, LoginResponse, ApiResponse } from '~/types/api'

const ACCESS_TOKEN_KEY = 'smiled_access_token'
const REFRESH_TOKEN_KEY = 'smiled_refresh_token'

export const useAuthStore = defineStore('auth', () => {
  const config = useRuntimeConfig()
  const router = useRouter()

  const accessToken = ref<string | null>(null)
  const refreshToken = ref<string | null>(null)
  const user = ref<User | null>(null)

  const isAuthenticated = computed(() => accessToken.value !== null && user.value !== null)

  function hydrate() {
    if (!import.meta.client) return
    accessToken.value = localStorage.getItem(ACCESS_TOKEN_KEY)
    refreshToken.value = localStorage.getItem(REFRESH_TOKEN_KEY)
  }

  function persistTokens(access: string, refresh: string) {
    accessToken.value = access
    refreshToken.value = refresh
    if (import.meta.client) {
      localStorage.setItem(ACCESS_TOKEN_KEY, access)
      localStorage.setItem(REFRESH_TOKEN_KEY, refresh)
    }
  }

  function clearTokens() {
    accessToken.value = null
    refreshToken.value = null
    user.value = null
    if (import.meta.client) {
      localStorage.removeItem(ACCESS_TOKEN_KEY)
      localStorage.removeItem(REFRESH_TOKEN_KEY)
    }
  }

  async function login(email: string, password: string): Promise<void> {
    const response = await $fetch<ApiResponse<LoginResponse>>(
      `${config.public.apiBase}/api/v1/auth/login`,
      {
        method: 'POST',
        body: { email, password },
        headers: { 'Content-Type': 'application/json' },
      },
    )

    if (!response.success || !response.data) {
      throw new Error(response.error ?? 'Identifiants invalides')
    }

    const { access_token, refresh_token, user: loggedUser } = response.data
    persistTokens(access_token, refresh_token)
    user.value = loggedUser
  }

  async function logout(): Promise<void> {
    try {
      if (accessToken.value) {
        await $fetch(`${config.public.apiBase}/api/v1/auth/logout`, {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            Authorization: `Bearer ${accessToken.value}`,
          },
        })
      }
    } catch {
      // Proceed with local logout even if server call fails
    } finally {
      clearTokens()
      await router.push('/login')
    }
  }

  async function refresh(): Promise<void> {
    if (!refreshToken.value) {
      clearTokens()
      throw new Error('Aucun token de rafraîchissement disponible')
    }

    try {
      const response = await $fetch<ApiResponse<LoginResponse>>(
        `${config.public.apiBase}/api/v1/auth/refresh`,
        {
          method: 'POST',
          body: { refresh_token: refreshToken.value },
          headers: { 'Content-Type': 'application/json' },
        },
      )

      if (!response.success || !response.data) {
        clearTokens()
        throw new Error(response.error ?? 'Impossible de renouveler la session')
      }

      const { access_token, refresh_token, user: refreshedUser } = response.data
      persistTokens(access_token, refresh_token)
      user.value = refreshedUser
    } catch (error) {
      clearTokens()
      throw error
    }
  }

  return {
    accessToken,
    refreshToken,
    user,
    isAuthenticated,
    hydrate,
    login,
    logout,
    refresh,
  }
})
