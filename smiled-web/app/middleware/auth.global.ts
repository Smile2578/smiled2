const PUBLIC_PATHS = ['/', '/login', '/forgot-password', '/reset-password']

export default defineNuxtRouteMiddleware(async (to) => {
  if (PUBLIC_PATHS.includes(to.path)) return

  // On client side, fetch session then check
  if (import.meta.client) {
    const authStore = useAuthStore()
    if (!authStore.isAuthenticated) {
      await authStore.fetchSession()
    }
    if (!authStore.isAuthenticated) {
      return navigateTo('/login')
    }
  }

  // On server side, check cookie existence as a quick gate
  if (import.meta.server) {
    const cookie = useCookie('better-auth.session_token')
    if (!cookie.value) {
      return navigateTo('/login')
    }
  }
})
