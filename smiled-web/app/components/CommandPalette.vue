<template>
  <CommandDialog v-model:open="open">
    <CommandInput v-model="searchQuery" placeholder="Rechercher un patient, une page..." />
    <CommandList>
      <CommandEmpty>Aucun résultat.</CommandEmpty>

      <CommandGroup heading="Navigation">
        <CommandItem
          v-for="page in pages"
          :key="page.to"
          :value="page.label"
          @select="() => goTo(page.to)"
        >
          <component :is="page.icon" class="mr-2 size-4" />
          {{ page.label }}
        </CommandItem>
      </CommandGroup>

      <CommandSeparator />

      <CommandGroup heading="Actions">
        <CommandItem value="Nouveau patient" @select="() => goTo('/patients?create=true')">
          <UserPlus class="mr-2 size-4" />
          Nouveau patient
        </CommandItem>
      </CommandGroup>

      <CommandSeparator v-if="patientResults.length > 0" />

      <CommandGroup v-if="patientResults.length > 0" heading="Patients">
        <CommandItem
          v-for="p in patientResults"
          :key="p.id"
          :value="`patient-${p.nom}-${p.prenom}`"
          @select="() => goTo(`/patients/${p.id}`)"
        >
          <Users class="mr-2 size-4" />
          {{ p.nom }} {{ p.prenom }}
        </CommandItem>
      </CommandGroup>
    </CommandList>
  </CommandDialog>
</template>

<script setup lang="ts">
import {
  LayoutDashboard,
  Users,
  ClipboardList,
  FlaskConical,
  Palette,
  Building2,
  UserPlus,
} from 'lucide-vue-next'
import { useDebounceFn } from '@vueuse/core'
import type { Patient } from '~/types/patient'

const open = ref(false)
const searchQuery = ref('')
const patientResults = ref<Patient[]>([])
const { apiGet } = useApi()

const pages = [
  { to: '/', label: 'Dashboard', icon: LayoutDashboard },
  { to: '/patients', label: 'Patients', icon: Users },
  { to: '/actes', label: 'Actes', icon: ClipboardList },
  { to: '/materiaux', label: 'Matériaux', icon: FlaskConical },
  { to: '/teintes', label: 'Teintes', icon: Palette },
  { to: '/cabinet', label: 'Cabinet', icon: Building2 },
]

const searchPatients = useDebounceFn(async (query: string) => {
  if (query.length < 2) {
    patientResults.value = []
    return
  }
  try {
    const res = await apiGet<Patient[]>('/api/v1/patients', { search: query, limit: 5 })
    patientResults.value = res.data ?? []
  } catch {
    patientResults.value = []
  }
}, 300)

watch(searchQuery, (val) => {
  searchPatients(val)
})

watch(open, (val) => {
  if (!val) {
    searchQuery.value = ''
    patientResults.value = []
  }
})

onMounted(() => {
  const handler = (e: KeyboardEvent) => {
    if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
      e.preventDefault()
      open.value = !open.value
    }
  }
  window.addEventListener('keydown', handler)
  onUnmounted(() => window.removeEventListener('keydown', handler))
})

function goTo(path: string) {
  open.value = false
  navigateTo(path)
}

defineExpose({ open })
</script>
