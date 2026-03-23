<template>
  <div class="flex h-screen">
    <!-- Sidebar -->
    <aside class="w-64 border-r bg-sidebar text-sidebar-foreground flex flex-col">
      <!-- Logo -->
      <div class="p-4 border-b border-sidebar-border">
        <h1 class="text-xl font-bold text-sidebar-primary">Smiled.IO</h1>
        <p class="text-xs text-sidebar-foreground/60">Logiciel dentaire</p>
      </div>

      <!-- Navigation -->
      <nav class="flex-1 p-3 space-y-1 overflow-y-auto">
        <p class="text-xs font-semibold text-sidebar-foreground/50 uppercase tracking-wider px-3 mb-2">
          Clinique
        </p>
        <NuxtLink to="/patients" class="sidebar-link">
          <Icon name="lucide:users" class="w-4 h-4" />
          <span>Patients</span>
        </NuxtLink>

        <p class="text-xs font-semibold text-sidebar-foreground/50 uppercase tracking-wider px-3 mt-6 mb-2">
          Administration
        </p>
        <NuxtLink to="/actes" class="sidebar-link">
          <Icon name="lucide:clipboard-list" class="w-4 h-4" />
          <span>Actes</span>
        </NuxtLink>
        <NuxtLink to="/materiaux" class="sidebar-link">
          <Icon name="lucide:flask-conical" class="w-4 h-4" />
          <span>Matériaux</span>
        </NuxtLink>
        <NuxtLink to="/teintes" class="sidebar-link">
          <Icon name="lucide:palette" class="w-4 h-4" />
          <span>Teintes</span>
        </NuxtLink>

        <p class="text-xs font-semibold text-sidebar-foreground/50 uppercase tracking-wider px-3 mt-6 mb-2">
          Paramètres
        </p>
        <NuxtLink to="/cabinet" class="sidebar-link">
          <Icon name="lucide:building-2" class="w-4 h-4" />
          <span>Cabinet</span>
        </NuxtLink>
      </nav>

      <!-- User info at bottom -->
      <div class="p-3 border-t border-sidebar-border">
        <div class="flex items-center gap-3 px-3 py-2">
          <div class="w-8 h-8 rounded-full bg-sidebar-primary/10 flex items-center justify-center shrink-0">
            <span class="text-sm font-medium text-sidebar-primary">{{ userInitials }}</span>
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium truncate">{{ userName }}</p>
            <p class="text-xs text-sidebar-foreground/60 truncate">{{ userRoleLabel }}</p>
          </div>
          <button
            class="text-sidebar-foreground/60 hover:text-sidebar-foreground transition-colors"
            title="Se déconnecter"
            @click="handleLogout"
          >
            <Icon name="lucide:log-out" class="w-4 h-4" />
          </button>
        </div>
      </div>
    </aside>

    <!-- Main content -->
    <main class="flex-1 overflow-auto">
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
const authStore = useAuthStore()

const userName = computed(() => {
  const user = authStore.user
  if (!user) return ''
  return `${user.prenom} ${user.nom}`
})

const userInitials = computed(() => {
  const user = authStore.user
  if (!user) return '?'
  return `${user.prenom.charAt(0)}${user.nom.charAt(0)}`.toUpperCase()
})

const userRoleLabel = computed(() => {
  const roleMap: Record<string, string> = {
    titulaire: 'Titulaire',
    associe: 'Associé',
    collaborateur: 'Collaborateur',
    remplacant: 'Remplaçant',
    assistant: 'Assistant',
    admin: 'Administrateur',
  }
  return roleMap[authStore.user?.role ?? ''] ?? authStore.user?.role ?? ''
})

async function handleLogout() {
  await authStore.logout()
}
</script>

<style scoped>
.sidebar-link {
  @apply flex items-center gap-3 px-3 py-2 rounded-md text-sm transition-colors;
  @apply text-sidebar-foreground/80 hover:bg-sidebar-accent hover:text-sidebar-accent-foreground;
}

.sidebar-link.router-link-active {
  @apply bg-sidebar-accent text-sidebar-accent-foreground font-medium;
}
</style>
