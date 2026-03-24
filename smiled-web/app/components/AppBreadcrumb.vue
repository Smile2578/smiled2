<template>
  <Breadcrumb>
    <BreadcrumbList>
      <template v-for="(crumb, index) in crumbs" :key="crumb.path">
        <BreadcrumbItem>
          <BreadcrumbLink v-if="index < crumbs.length - 1" as-child>
            <NuxtLink :to="crumb.path">{{ crumb.label }}</NuxtLink>
          </BreadcrumbLink>
          <BreadcrumbPage v-else>{{ crumb.label }}</BreadcrumbPage>
        </BreadcrumbItem>
        <BreadcrumbSeparator v-if="index < crumbs.length - 1" />
      </template>
    </BreadcrumbList>
  </Breadcrumb>
</template>

<script setup lang="ts">
const route = useRoute()

const labelMap: Record<string, string> = {
  patients: 'Patients',
  actes: 'Actes',
  materiaux: 'Matériaux',
  teintes: 'Teintes',
  cabinet: 'Cabinet',
  audit: 'Audit',
  users: 'Utilisateurs',
  schema: 'Schema dentaire',
  questionnaire: 'Questionnaire',
  diagnostic: 'Diagnostic',
  pdts: 'Plans de traitement',
  paro: 'Parodontologie',
  documents: 'Documents',
  historique: 'Historique',
}

const crumbs = computed(() => {
  const segments = route.path.split('/').filter(Boolean)
  if (segments.length === 0) return [{ label: 'Dashboard', path: '/' }]

  const result = [{ label: 'Dashboard', path: '/' }]
  let currentPath = ''

  for (const segment of segments) {
    currentPath += `/${segment}`
    // Skip UUID-like segments (patient IDs) — show as "Patient" or use route meta
    if (segment.match(/^[0-9a-f]{8}-/)) {
      result.push({ label: 'Patient', path: currentPath })
    } else {
      result.push({
        label: labelMap[segment] ?? segment.charAt(0).toUpperCase() + segment.slice(1),
        path: currentPath,
      })
    }
  }

  return result
})
</script>
