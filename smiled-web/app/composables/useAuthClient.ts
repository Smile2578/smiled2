import { createAuthClient } from 'better-auth/vue'

export function useAuthClient() {
  const config = useRuntimeConfig()
  const baseURL = import.meta.server
    ? (process.env.NUXT_PUBLIC_APP_URL || 'http://localhost:3000')
    : window.location.origin

  return createAuthClient({
    baseURL: `${baseURL}/api/auth`,
  })
}
