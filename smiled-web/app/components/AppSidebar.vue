<template>
  <Sidebar collapsible="icon">
    <SidebarHeader>
      <SidebarMenu>
        <SidebarMenuItem>
          <SidebarMenuButton size="lg" as-child>
            <NuxtLink to="/">
              <img src="~/assets/img/logomini.png" alt="Smiled.IO" class="size-8 rounded-lg" />
              <div class="grid flex-1 text-left text-sm leading-tight">
                <span class="truncate font-semibold">Smiled.IO</span>
                <span class="truncate text-xs text-sidebar-foreground/60">Logiciel dentaire</span>
              </div>
            </NuxtLink>
          </SidebarMenuButton>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarHeader>

    <SidebarContent>
      <!-- Clinique -->
      <SidebarGroup v-if="canSeeClinical">
        <SidebarGroupLabel>Clinique</SidebarGroupLabel>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem v-for="item in clinicalItems" :key="item.to">
              <SidebarMenuButton
                as-child
                :tooltip="item.label"
                :is-active="isActive(item.to)"
              >
                <NuxtLink :to="item.to">
                  <component :is="item.icon" class="size-4" />
                  <span>{{ item.label }}</span>
                </NuxtLink>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>

      <!-- Administration -->
      <SidebarGroup v-if="canSeeAdmin">
        <SidebarGroupLabel>Administration</SidebarGroupLabel>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem v-for="item in adminItems" :key="item.to">
              <SidebarMenuButton
                as-child
                :tooltip="item.label"
                :is-active="isActive(item.to)"
              >
                <NuxtLink :to="item.to">
                  <component :is="item.icon" class="size-4" />
                  <span>{{ item.label }}</span>
                </NuxtLink>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>

      <!-- Parametres -->
      <SidebarGroup v-if="canSeeAdmin || canSeeAudit">
        <SidebarGroupLabel>Paramètres</SidebarGroupLabel>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem v-if="canSeeAdmin">
              <SidebarMenuButton
                as-child
                tooltip="Cabinet"
                :is-active="isActive('/cabinet')"
              >
                <NuxtLink to="/cabinet">
                  <Building2 class="size-4" />
                  <span>Cabinet</span>
                </NuxtLink>
              </SidebarMenuButton>
            </SidebarMenuItem>
            <SidebarMenuItem v-if="canSeeAudit">
              <SidebarMenuButton
                as-child
                tooltip="Journal d'audit"
                :is-active="isActive('/audit')"
              >
                <NuxtLink to="/audit">
                  <ScrollText class="size-4" />
                  <span>Journal d'audit</span>
                </NuxtLink>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>

    <SidebarFooter>
      <SidebarMenu>
        <SidebarMenuItem>
          <DropdownMenu>
            <DropdownMenuTrigger as-child>
              <SidebarMenuButton
                size="lg"
                class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
              >
                <Avatar class="size-8 rounded-lg">
                  <AvatarFallback class="rounded-lg bg-sidebar-primary/10 text-sidebar-primary text-xs">
                    {{ userInitials }}
                  </AvatarFallback>
                </Avatar>
                <div class="grid flex-1 text-left text-sm leading-tight">
                  <span class="truncate font-semibold">{{ userName }}</span>
                  <span class="truncate text-xs text-sidebar-foreground/60">{{ userEmail }}</span>
                </div>
                <ChevronsUpDown class="ml-auto size-4" />
              </SidebarMenuButton>
            </DropdownMenuTrigger>
            <DropdownMenuContent
              class="w-[--reka-popper-anchor-width] min-w-56 rounded-lg"
              side="bottom"
              align="end"
              :side-offset="4"
            >
              <DropdownMenuLabel class="p-0 font-normal">
                <div class="flex items-center gap-2 px-1 py-1.5 text-left text-sm">
                  <Avatar class="size-8 rounded-lg">
                    <AvatarFallback class="rounded-lg text-xs">
                      {{ userInitials }}
                    </AvatarFallback>
                  </Avatar>
                  <div class="grid flex-1 text-left text-sm leading-tight">
                    <span class="truncate font-semibold">{{ userName }}</span>
                    <span class="truncate text-xs text-muted-foreground">{{ userEmail }}</span>
                  </div>
                </div>
              </DropdownMenuLabel>
              <DropdownMenuSeparator />
              <DropdownMenuItem @click="handleLogout">
                <LogOut class="size-4" />
                Se déconnecter
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarFooter>

    <SidebarRail />
  </Sidebar>
</template>

<script setup lang="ts">
import {
  LayoutDashboard,
  Users,
  ClipboardList,
  FlaskConical,
  Palette,
  Building2,
  ScrollText,
  ChevronsUpDown,
  LogOut,
} from 'lucide-vue-next'

const route = useRoute()
const authStore = useAuthStore()
const { hasPermission, loadPermissions } = usePermissions()

onMounted(() => { loadPermissions() })

const permissionsLoaded = computed(() => usePermissions().permissions.value.length > 0)

const canSeeClinical = computed(() =>
  !permissionsLoaded.value || hasPermission('patient.read_admin') || hasPermission('patient.read_clinical'),
)

const canSeeAdmin = computed(() =>
  !permissionsLoaded.value || hasPermission('settings.cabinet') || hasPermission('settings.manage'),
)

const canSeeAudit = computed(() => {
  if (!permissionsLoaded.value) return true
  return hasPermission('audit.read')
})

const clinicalItems = [
  { to: '/', label: 'Dashboard', icon: LayoutDashboard },
  { to: '/patients', label: 'Patients', icon: Users },
]

const adminItems = [
  { to: '/actes', label: 'Actes', icon: ClipboardList },
  { to: '/materiaux', label: 'Matériaux', icon: FlaskConical },
  { to: '/teintes', label: 'Teintes', icon: Palette },
]

const userName = computed(() => {
  const user = authStore.user
  if (!user) return ''
  return `${user.prenom ?? ''} ${user.nom ?? ''}`.trim()
})
const userEmail = computed(() => authStore.user?.email ?? '')

const userInitials = computed(() => {
  const user = authStore.user
  if (!user) return '?'
  const p = user.prenom?.charAt(0) ?? ''
  const n = user.nom?.charAt(0) ?? ''
  return (p + n).toUpperCase() || '?'
})

function isActive(path: string): boolean {
  if (path === '/') return route.path === '/'
  return route.path.startsWith(path)
}

async function handleLogout() {
  await authStore.logout()
}
</script>
