import { defineStore } from 'pinia'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<Record<string, unknown> | null>(null)
  const isAuthenticated = computed(() => !!user.value)

  async function fetchSession(): Promise<void> {
    if (!import.meta.client) return
    try {
      const authClient = useAuthClient()
      const session = await authClient.getSession()
      user.value = session.data?.user ?? null
    } catch {
      user.value = null
    }
  }

  async function login(email: string, password: string): Promise<void> {
    const authClient = useAuthClient()
    const result = await authClient.signIn.email({ email, password })
    if (result.error) {
      throw new Error(result.error.message || 'Identifiants invalides')
    }
    user.value = result.data?.user ?? null
    await navigateTo('/dashboard')
  }

  async function logout(): Promise<void> {
    const authClient = useAuthClient()
    await authClient.signOut()
    user.value = null
    await navigateTo('/login')
  }

  return {
    user,
    isAuthenticated,
    fetchSession,
    login,
    logout,
  }
})
