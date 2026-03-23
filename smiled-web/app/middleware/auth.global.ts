const PUBLIC_PAGES = ['/login', '/forgot-password', '/reset-password']

export default defineNuxtRouteMiddleware((to) => {
  const auth = useAuthStore()

  // Hydrate tokens from localStorage on client-side navigation
  if (import.meta.client) {
    auth.hydrate()
  }

  if (PUBLIC_PAGES.includes(to.path)) return

  if (!auth.isAuthenticated) {
    return navigateTo('/login')
  }
})
