const PUBLIC_PAGES = ['/login', '/forgot-password', '/reset-password']

export default defineNuxtRouteMiddleware((to) => {
  // Auth check only runs client-side (tokens are in localStorage)
  if (!import.meta.client) return

  const auth = useAuthStore()
  auth.hydrate()

  if (PUBLIC_PAGES.includes(to.path)) return

  if (!auth.isAuthenticated) {
    return navigateTo('/login')
  }
})
