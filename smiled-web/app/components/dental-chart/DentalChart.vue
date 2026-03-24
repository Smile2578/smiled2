<template>
  <div class="dental-chart-wrapper relative">
    <canvas
      ref="canvasRef"
      role="img"
      :aria-label="chartAriaLabel"
      tabindex="0"
      :style="{ width: cssWidth + 'px', height: cssHeight + 'px', cursor: cursorStyle }"
      class="block rounded border border-gray-200 bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
      @click="handleClick"
      @mousemove="handleMouseMove"
      @mouseleave="handleMouseLeave"
      @keydown="handleKeydown"
    />
    <div aria-live="polite" class="sr-only">{{ ariaAnnouncement }}</div>
  </div>
</template>

<script setup lang="ts">
import type { ToothEntry, FaceName, FaceState } from './types'
import { TOOTH_STATUS_LABELS, FACE_STATE_LABELS } from './types'
import { CANVAS_WIDTH, CANVAS_HEIGHT, GEOMETRY_BY_FDI } from './fdi-geometry'
import { renderCached } from './ToothRenderer'
import { eventToCanvasCoords, resolveHit, getCursorStyle } from './ChartInteraction'

// FDI navigation order: 11→12→...→18→28→27→...→21 (upper), 31→32→...→38→48→47→...→41 (lower)
const UPPER_NAV_ORDER = [11, 12, 13, 14, 15, 16, 17, 18, 28, 27, 26, 25, 24, 23, 22, 21]
const LOWER_NAV_ORDER = [31, 32, 33, 34, 35, 36, 37, 38, 48, 47, 46, 45, 44, 43, 42, 41]
const NAV_ORDER = [...UPPER_NAV_ORDER, ...LOWER_NAV_ORDER]

// Map upper↔lower opposites for ArrowUp/Down (e.g., 11↔41, 16↔46)
function getOppositeArchTooth(fdi: number): number {
  const quadrant = Math.floor(fdi / 10)
  const tooth = fdi % 10
  if (quadrant === 1) return 40 + tooth
  if (quadrant === 4) return 10 + tooth
  if (quadrant === 2) return 30 + tooth
  if (quadrant === 3) return 20 + tooth
  return fdi
}

interface Props {
  teeth: ToothEntry[]
  selectedFdi: number | null
  selectedFace: FaceName | null
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'tooth-selected': [fdi: number, face: FaceName | null]
  'tooth-click': [fdi: number, face: FaceName | null, clientX: number, clientY: number]
}>()

// Canvas ref and device pixel ratio
const canvasRef = ref<HTMLCanvasElement | null>(null)
const dpr = import.meta.client ? window.devicePixelRatio || 1 : 1

// CSS dimensions (logical pixels)
const cssWidth = CANVAS_WIDTH
const cssHeight = CANVAS_HEIGHT

// Cursor reactive state
const cursorStyle = ref<string>('default')

// Aria announcement for screen readers
const ariaAnnouncement = ref('')

// Build a lookup map from the teeth prop
const teethMap = computed<Map<number, ToothEntry>>(() => {
  return new Map(props.teeth.map((entry) => [entry.tooth.numero_fdi, entry]))
})

// Computed aria-label describing chart state
const chartAriaLabel = computed(() => {
  const totalTeeth = props.teeth.length
  let caries = 0
  let composites = 0
  for (const entry of props.teeth) {
    for (const face of entry.faces) {
      if (face.etat === 'carie') caries++
      if (face.etat === 'composite') composites++
    }
  }
  const parts = [`Schema dentaire: ${totalTeeth} dents`]
  if (caries > 0) parts.push(`${caries} caries`)
  if (composites > 0) parts.push(`${composites} composites`)
  return parts.join(', ')
})

// Render using the cached OffscreenCanvas approach
function render(): void {
  const canvas = canvasRef.value
  if (!canvas) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  // Scale for HiDPI
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0)

  renderCached(
    ctx,
    CANVAS_WIDTH,
    CANVAS_HEIGHT,
    teethMap.value,
    props.selectedFdi,
    props.selectedFace,
  )
}

// Set up canvas backing store size to account for dpr
function initCanvas(): void {
  const canvas = canvasRef.value
  if (!canvas) return
  canvas.width = Math.round(CANVAS_WIDTH * dpr)
  canvas.height = Math.round(CANVAS_HEIGHT * dpr)
}

function handleClick(event: MouseEvent): void {
  const canvas = canvasRef.value
  if (!canvas) return

  const { x, y } = eventToCanvasCoords(event, canvas, dpr)
  const hit = resolveHit(x, y)
  if (hit) {
    emit('tooth-selected', hit.fdi, hit.face)
    emit('tooth-click', hit.fdi, hit.face, event.clientX, event.clientY)
  }
}

function handleMouseMove(event: MouseEvent): void {
  const canvas = canvasRef.value
  if (!canvas) return

  const { x, y } = eventToCanvasCoords(event, canvas, dpr)
  cursorStyle.value = getCursorStyle(x, y)
}

function handleMouseLeave(): void {
  cursorStyle.value = 'default'
}

function announceToothSelection(fdi: number): void {
  const entry = teethMap.value.get(fdi)
  if (!entry) {
    ariaAnnouncement.value = `Dent ${fdi} selectionnee`
    return
  }
  const status = TOOTH_STATUS_LABELS[entry.tooth.statut as keyof typeof TOOTH_STATUS_LABELS] ?? entry.tooth.statut
  ariaAnnouncement.value = `Dent ${fdi} selectionnee, statut: ${status}`
}

function handleKeydown(event: KeyboardEvent): void {
  const key = event.key

  if (key === 'Escape') {
    event.preventDefault()
    emit('tooth-selected', 0, null) // 0 signals deselect — parent handles it
    ariaAnnouncement.value = 'Selection annulee'
    return
  }

  if (key === 'Enter' || key === ' ') {
    event.preventDefault()
    if (props.selectedFdi !== null) {
      // Compute approximate client coordinates from the tooth geometry for popover positioning
      const geo = GEOMETRY_BY_FDI.get(props.selectedFdi)
      const canvas = canvasRef.value
      if (geo && canvas) {
        const rect = canvas.getBoundingClientRect()
        const clientX = rect.left + (geo.x + geo.width / 2)
        const clientY = rect.top + (geo.y + geo.height / 2)
        emit('tooth-click', props.selectedFdi, props.selectedFace, clientX, clientY)
      }
    }
    return
  }

  if (key === 'ArrowRight') {
    event.preventDefault()
    navigateNext(1)
    return
  }

  if (key === 'ArrowLeft') {
    event.preventDefault()
    navigateNext(-1)
    return
  }

  if (key === 'ArrowUp' || key === 'ArrowDown') {
    event.preventDefault()
    if (props.selectedFdi !== null) {
      const opposite = getOppositeArchTooth(props.selectedFdi)
      emit('tooth-selected', opposite, null)
      announceToothSelection(opposite)
    }
    return
  }
}

function navigateNext(direction: 1 | -1): void {
  if (props.selectedFdi === null) {
    // No selection — start at first tooth
    const first = NAV_ORDER[0]
    emit('tooth-selected', first, null)
    announceToothSelection(first)
    return
  }

  const currentIndex = NAV_ORDER.indexOf(props.selectedFdi)
  if (currentIndex === -1) {
    emit('tooth-selected', NAV_ORDER[0], null)
    announceToothSelection(NAV_ORDER[0])
    return
  }

  const nextIndex = (currentIndex + direction + NAV_ORDER.length) % NAV_ORDER.length
  const nextFdi = NAV_ORDER[nextIndex]
  emit('tooth-selected', nextFdi, null)
  announceToothSelection(nextFdi)
}

// Use requestAnimationFrame to batch renders
let rafId: number | null = null

function scheduleRender(): void {
  if (rafId !== null) return
  rafId = requestAnimationFrame(() => {
    rafId = null
    render()
  })
}

// Initialize and render on mount
onMounted(() => {
  initCanvas()
  render()
})

// Re-render whenever teeth data or selection changes
watch(
  [() => props.teeth, () => props.selectedFdi, () => props.selectedFace],
  () => {
    scheduleRender()
  },
  { deep: true },
)

onUnmounted(() => {
  if (rafId !== null) {
    cancelAnimationFrame(rafId)
    rafId = null
  }
})
</script>

<style scoped>
.dental-chart-wrapper {
  display: inline-block;
}
</style>
