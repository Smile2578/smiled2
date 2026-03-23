<template>
  <div class="p-8">
    <!-- Header -->
    <div class="flex items-center gap-4 mb-6">
      <Button variant="ghost" size="sm" @click="navigateTo('/cabinet')">
        <Icon name="lucide:arrow-left" class="w-4 h-4 mr-2" />
        Cabinet
      </Button>
      <Separator orientation="vertical" class="h-6" />
      <div>
        <h1 class="text-2xl font-bold">Utilisateurs</h1>
        <p class="text-sm text-muted-foreground">
          {{ users.length }} utilisateur{{ users.length !== 1 ? 's' : '' }}
        </p>
      </div>
    </div>

    <!-- Tabs (for navigation context) -->
    <Tabs default-value="users" class="mb-6">
      <TabsList>
        <TabsTrigger value="infos" @click="navigateTo('/cabinet')">Informations</TabsTrigger>
        <TabsTrigger value="users">Utilisateurs</TabsTrigger>
      </TabsList>
    </Tabs>

    <!-- Loading -->
    <div v-if="loading" class="flex items-center justify-center h-32">
      <Icon name="lucide:loader-2" class="w-8 h-8 animate-spin text-muted-foreground" />
    </div>

    <!-- Empty -->
    <div v-else-if="users.length === 0" class="text-center py-12 text-muted-foreground">
      <Icon name="lucide:users" class="w-12 h-12 mx-auto mb-3 opacity-30" />
      <p>Aucun utilisateur trouvé</p>
    </div>

    <!-- Users table -->
    <Card v-else>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Nom</TableHead>
            <TableHead>Email</TableHead>
            <TableHead>Rôle</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow v-for="user in users" :key="user.id">
            <TableCell>
              <div class="flex items-center gap-3">
                <div class="w-8 h-8 rounded-full bg-primary/10 flex items-center justify-center shrink-0">
                  <span class="text-xs font-medium text-primary">{{ initials(user) }}</span>
                </div>
                <span class="font-medium text-sm">{{ user.prenom }} {{ user.nom }}</span>
              </div>
            </TableCell>
            <TableCell class="text-sm text-muted-foreground">{{ user.email }}</TableCell>
            <TableCell>
              <Badge variant="secondary" class="text-xs">{{ roleLabel(user.role) }}</Badge>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </Card>
  </div>
</template>

<script setup lang="ts">
import type { User } from '~/types/api'

definePageMeta({ layout: 'default' })

const { getCabinetUsers } = useCabinet()

const users = ref<User[]>([])
const loading = ref(true)

const ROLE_LABELS: Record<string, string> = {
  titulaire: 'Titulaire',
  associe: 'Associé',
  collaborateur: 'Collaborateur',
  remplacant: 'Remplaçant',
  assistant: 'Assistant',
  aspbd: 'ASPBD',
  specialiste_endo: 'Endodontiste',
  specialiste_implanto: 'Implantologue',
  specialiste_ortho: 'Orthodontiste',
  admin: 'Administrateur',
  secretaire: 'Secrétaire',
}

function initials(user: User): string {
  return `${user.prenom.charAt(0)}${user.nom.charAt(0)}`.toUpperCase()
}

function roleLabel(role: string): string {
  return ROLE_LABELS[role] ?? role
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
