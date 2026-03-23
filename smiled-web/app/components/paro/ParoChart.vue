<template>
  <div class="overflow-x-auto">
    <table class="text-xs border-collapse w-full min-w-[700px]">
      <thead>
        <tr class="bg-muted">
          <th class="border px-2 py-1 text-left font-medium w-16">Dent</th>
          <th
            v-for="site in SITES"
            :key="site"
            class="border px-1 py-1 text-center font-medium w-20"
          >
            {{ site }}
          </th>
        </tr>
      </thead>
      <tbody>
        <template v-for="fdi in displayTeeth" :key="fdi">
          <!-- Profondeur de poche -->
          <tr class="hover:bg-muted/30">
            <td
              v-if="isFirstRowForTooth(fdi)"
              class="border px-2 py-1 font-semibold text-center align-middle bg-muted/50"
              :rowspan="4"
            >
              {{ fdi }}
            </td>
            <td
              v-for="site in SITES"
              :key="`${fdi}-${site}-poche`"
              class="border px-1 py-0.5"
            >
              <input
                type="number"
                min="0"
                max="20"
                step="0.5"
                class="w-full text-center text-xs border-0 bg-transparent focus:outline-none focus:ring-1 focus:ring-primary rounded"
                :value="getSiteValue(fdi, site, 'profondeur_poche')"
                :placeholder="'mm'"
                @change="setSiteValue(fdi, site, 'profondeur_poche', $event)"
              />
            </td>
          </tr>
          <!-- Récession -->
          <tr class="hover:bg-muted/30">
            <td
              v-for="site in SITES"
              :key="`${fdi}-${site}-recession`"
              class="border px-1 py-0.5"
            >
              <input
                type="number"
                min="-5"
                max="15"
                step="0.5"
                class="w-full text-center text-xs border-0 bg-transparent focus:outline-none focus:ring-1 focus:ring-primary rounded"
                :value="getSiteValue(fdi, site, 'recession')"
                :placeholder="'mm'"
                @change="setSiteValue(fdi, site, 'recession', $event)"
              />
            </td>
          </tr>
          <!-- BOP / Suppuration -->
          <tr class="hover:bg-muted/30 bg-red-50/30">
            <td
              v-for="site in SITES"
              :key="`${fdi}-${site}-bop`"
              class="border px-1 py-0.5 text-center"
            >
              <input
                type="checkbox"
                :checked="getSiteBool(fdi, site, 'bop')"
                class="accent-red-500"
                title="BOP"
                @change="setSiteBool(fdi, site, 'bop', $event)"
              />
            </td>
          </tr>
          <!-- Plaque -->
          <tr class="hover:bg-muted/30 bg-yellow-50/30">
            <td
              v-for="site in SITES"
              :key="`${fdi}-${site}-plaque`"
              class="border px-1 py-0.5 text-center"
            >
              <input
                type="checkbox"
                :checked="getSiteBool(fdi, site, 'plaque')"
                class="accent-yellow-500"
                title="Plaque"
                @change="setSiteBool(fdi, site, 'plaque', $event)"
              />
            </td>
          </tr>
          <!-- Separator between teeth -->
          <tr>
            <td :colspan="SITES.length + 1" class="border-t-2 border-muted" />
          </tr>
        </template>
      </tbody>
    </table>

    <!-- Legend -->
    <div class="flex items-center gap-6 mt-3 text-xs text-muted-foreground">
      <span>Lignes par dent :</span>
      <div class="flex items-center gap-1">
        <span class="font-medium">1</span> Profondeur (mm)
      </div>
      <div class="flex items-center gap-1">
        <span class="font-medium">2</span> Récession (mm)
      </div>
      <div class="flex items-center gap-1.5">
        <span class="w-3 h-3 rounded bg-red-200 inline-block" />
        <span>BOP</span>
      </div>
      <div class="flex items-center gap-1.5">
        <span class="w-3 h-3 rounded bg-yellow-200 inline-block" />
        <span>Plaque</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { ParoSite, ParoSiteUpdate } from '~/composables/useParo'

const SITES = ['MB', 'B', 'DB', 'ML', 'L', 'DL'] as const
type SiteCode = typeof SITES[number]

// Standard FDI permanent teeth order (maxillary right → left, mandibular left → right)
const TEETH_MAXILLARY = [18, 17, 16, 15, 14, 13, 12, 11, 21, 22, 23, 24, 25, 26, 27, 28]
const TEETH_MANDIBULAR = [48, 47, 46, 45, 44, 43, 42, 41, 31, 32, 33, 34, 35, 36, 37, 38]
const displayTeeth = [...TEETH_MAXILLARY, ...TEETH_MANDIBULAR]

const props = defineProps<{
  sites: ParoSite[]
}>()

const emit = defineEmits<{
  (e: 'update', sites: ParoSiteUpdate[]): void
}>()

// Local mutable state derived from props
const localSites = ref<Map<string, ParoSiteUpdate>>(new Map())

function siteKey(fdi: number, site: string): string {
  return `${fdi}-${site}`
}

function initLocalSites() {
  const map = new Map<string, ParoSiteUpdate>()

  for (const fdi of displayTeeth) {
    for (const site of SITES) {
      const key = siteKey(fdi, site)
      const existing = props.sites.find((s) => s.dent_fdi === fdi && s.site === site)
      map.set(key, {
        dent_fdi: fdi,
        site,
        profondeur_poche: existing?.profondeur_poche ?? null,
        recession: existing?.recession ?? null,
        bop: existing?.bop ?? false,
        suppuration: existing?.suppuration ?? false,
        plaque: existing?.plaque ?? false,
        mobilite: existing?.mobilite ?? null,
      })
    }
  }

  localSites.value = map
}

function getEntry(fdi: number, site: SiteCode): ParoSiteUpdate {
  return (
    localSites.value.get(siteKey(fdi, site)) ?? {
      dent_fdi: fdi,
      site,
      profondeur_poche: null,
      recession: null,
      bop: false,
      suppuration: false,
      plaque: false,
      mobilite: null,
    }
  )
}

function getSiteValue(fdi: number, site: SiteCode, field: 'profondeur_poche' | 'recession'): string {
  const val = getEntry(fdi, site)[field]
  return val !== null ? String(val) : ''
}

function getSiteBool(fdi: number, site: SiteCode, field: 'bop' | 'plaque' | 'suppuration'): boolean {
  return getEntry(fdi, site)[field]
}

function setSiteValue(
  fdi: number,
  site: SiteCode,
  field: 'profondeur_poche' | 'recession',
  event: Event,
): void {
  const input = event.target as HTMLInputElement
  const value = input.value === '' ? null : parseFloat(input.value)
  const key = siteKey(fdi, site)
  const current = localSites.value.get(key) ?? getEntry(fdi, site)
  localSites.value.set(key, { ...current, [field]: value })
  emitUpdate()
}

function setSiteBool(
  fdi: number,
  site: SiteCode,
  field: 'bop' | 'plaque' | 'suppuration',
  event: Event,
): void {
  const input = event.target as HTMLInputElement
  const key = siteKey(fdi, site)
  const current = localSites.value.get(key) ?? getEntry(fdi, site)
  localSites.value.set(key, { ...current, [field]: input.checked })
  emitUpdate()
}

function isFirstRowForTooth(_fdi: number): boolean {
  // Always true — used for rowspan logic; each fdi block's first row needs the cell
  return true
}

function emitUpdate(): void {
  emit('update', Array.from(localSites.value.values()))
}

watch(() => props.sites, initLocalSites, { immediate: true })
</script>
