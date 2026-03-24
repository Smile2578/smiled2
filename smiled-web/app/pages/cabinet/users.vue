<template>
  <div class="max-w-5xl">
    <!-- Page Header -->
    <div class="flex justify-between items-center mb-6">
      <div>
        <h1 class="text-2xl font-semibold tracking-tight">Equipe du cabinet</h1>
        <p class="text-sm text-muted-foreground mt-1">
          Gerez les membres de votre equipe
          <span v-if="!loading" class="text-muted-foreground/60 ml-1">&middot; {{ users.length }} membre{{ users.length !== 1 ? 's' : '' }}</span>
        </p>
      </div>
    </div>

    <!-- Navigation Tabs -->
    <Tabs default-value="team" class="mb-8">
      <TabsList>
        <TabsTrigger value="info" @click="navigateTo('/cabinet')">Informations</TabsTrigger>
        <TabsTrigger value="team">Equipe</TabsTrigger>
      </TabsList>
    </Tabs>

    <!-- Loading Skeleton -->
    <Card v-if="loading" class="overflow-hidden">
      <Table>
        <TableHeader>
          <TableRow class="bg-muted/40">
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground">Nom</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground">Email</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-44">Role</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground text-center w-24">Statut</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow v-for="i in 4" :key="i">
            <TableCell>
              <div class="flex items-center gap-3">
                <Skeleton class="w-9 h-9 rounded-full" />
                <Skeleton class="h-4 w-32" />
              </div>
            </TableCell>
            <TableCell><Skeleton class="h-4 w-40" /></TableCell>
            <TableCell><Skeleton class="h-5 w-24 rounded-full" /></TableCell>
            <TableCell class="text-center"><Skeleton class="h-5 w-12 mx-auto rounded-full" /></TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </Card>

    <!-- Empty state -->
    <div v-else-if="users.length === 0" class="text-center py-16">
      <Users class="w-12 h-12 mx-auto mb-3 text-muted-foreground/40" />
      <p class="text-sm font-medium text-muted-foreground">Aucun utilisateur trouve</p>
      <p class="text-xs text-muted-foreground/60 mt-1">Les membres de l'equipe apparaitront ici</p>
    </div>

    <!-- Users Table -->
    <Card v-else class="overflow-hidden">
      <Table>
        <TableHeader>
          <TableRow class="bg-muted/40">
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground">Nom</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground">Email</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground w-44">Role</TableHead>
            <TableHead class="text-xs uppercase tracking-wider text-muted-foreground text-center w-24">Statut</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow
            v-for="user in users"
            :key="user.id"
            class="hover:bg-muted/50 transition-colors"
          >
            <TableCell>
              <div class="flex items-center gap-3">
                <div
                  :class="[
                    'w-9 h-9 rounded-full flex items-center justify-center shrink-0 text-xs font-semibold',
                    avatarColor(user.role),
                  ]"
                >
                  {{ initials(user) }}
                </div>
                <span class="text-sm font-medium">{{ user.prenom }} {{ user.nom }}</span>
              </div>
            </TableCell>
            <TableCell class="text-sm text-muted-foreground">{{ user.email }}</TableCell>
            <TableCell>
              <span
                :class="[
                  'inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium',
                  roleBadgeClass(user.role),
                ]"
              >
                {{ roleLabel(user.role) }}
              </span>
            </TableCell>
            <TableCell class="text-center">
              <span class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-emerald-50 text-emerald-700 border border-emerald-200">
                Actif
              </span>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { Users } from 'lucide-vue-next'
import type { User } from '~/types/api'

definePageMeta({ layout: 'default' })

const { getCabinetUsers } = useCabinet()

const users = ref<User[]>([])
const loading = ref(true)

const ROLE_LABELS: Record<string, string> = {
  titulaire: 'Titulaire',
  associe: 'Associe',
  collaborateur: 'Collaborateur',
  remplacant: 'Remplacant',
  assistant: 'Assistant',
  aspbd: 'ASPBD',
  specialiste_endo: 'Endodontiste',
  specialiste_implanto: 'Implantologue',
  specialiste_ortho: 'Orthodontiste',
  admin: 'Administrateur',
  secretaire: 'Secretaire',
}

const ROLE_BADGE_CLASSES: Record<string, string> = {
  titulaire: 'bg-teal-50 text-teal-700 border border-teal-200',
  associe: 'bg-teal-50 text-teal-700 border border-teal-200',
  collaborateur: 'bg-teal-50 text-teal-700 border border-teal-200',
  remplacant: 'bg-teal-50 text-teal-700 border border-teal-200',
  assistant: 'bg-blue-50 text-blue-700 border border-blue-200',
  aspbd: 'bg-blue-50 text-blue-700 border border-blue-200',
  specialiste_endo: 'bg-teal-50 text-teal-700 border border-teal-200',
  specialiste_implanto: 'bg-teal-50 text-teal-700 border border-teal-200',
  specialiste_ortho: 'bg-teal-50 text-teal-700 border border-teal-200',
  admin: 'bg-purple-50 text-purple-700 border border-purple-200',
  secretaire: 'bg-amber-50 text-amber-700 border border-amber-200',
}

const AVATAR_COLORS: Record<string, string> = {
  titulaire: 'bg-teal-100 text-teal-700',
  associe: 'bg-teal-100 text-teal-700',
  collaborateur: 'bg-teal-100 text-teal-700',
  remplacant: 'bg-teal-100 text-teal-700',
  assistant: 'bg-blue-100 text-blue-700',
  aspbd: 'bg-blue-100 text-blue-700',
  specialiste_endo: 'bg-teal-100 text-teal-700',
  specialiste_implanto: 'bg-teal-100 text-teal-700',
  specialiste_ortho: 'bg-teal-100 text-teal-700',
  admin: 'bg-purple-100 text-purple-700',
  secretaire: 'bg-amber-100 text-amber-700',
}

function initials(user: User): string {
  return `${user.prenom.charAt(0)}${user.nom.charAt(0)}`.toUpperCase()
}

function roleLabel(role: string): string {
  return ROLE_LABELS[role] ?? role
}

function roleBadgeClass(role: string): string {
  return ROLE_BADGE_CLASSES[role] ?? 'bg-gray-50 text-gray-700 border border-gray-200'
}

function avatarColor(role: string): string {
  return AVATAR_COLORS[role] ?? 'bg-gray-100 text-gray-700'
}

onMounted(async () => {
  try {
    const response = await getCabinetUsers()
    if (response.success && response.data) {
      users.value = response.data
    }
  } catch {
    // ignore
  } finally {
    loading.value = false
  }
})
</script>
