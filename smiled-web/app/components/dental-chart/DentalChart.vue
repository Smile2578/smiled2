<template>
  <div class="dental-chart-wrapper relative">
    <canvas
      ref="canvasRef"
      :style="{ width: cssWidth + 'px', height: cssHeight + 'px', cursor: cursorStyle }"
      class="block rounded border border-gray-200 bg-gray-50"
      @click="handleClick"
      @mousemove="handleMouseMove"
      @mouseleave="handleMouseLeave"
    />
  </div>
</template>

<script setup lang="ts">
import type { ToothEntry, FaceName } from './types'
import { CANVAS_WIDTH, CANVAS_HEIGHT, ALL_TOOTH_GEOMETRIES, GEOMETRY_BY_FDI } from './fdi-geometry'
import { drawTooth, drawChartBackground, drawEmptyTooth } from './ToothRenderer'
import { eventToCanvasCoords, resolveHit, getCursorStyle } from './ChartInteraction'

interface Props {
  teeth: ToothEntry[]
  selectedFdi: number | null
  selectedFace: FaceName | null
}

const props = defineProps<Props>()

const emit = defineEmits<{
  'tooth-selected': [fdi: number, face: FaceName | null]
}>()

// Canvas ref and device pixel ratio
const canvasRef = ref<HTMLCanvasElement | null>(null)
const dpr = import.meta.client ? window.devicePixelRatio || 1 : 1

// CSS dimensions (logical pixels)
const cssWidth = CANVAS_WIDTH
const cssHeight = CANVAS_HEIGHT

// Cursor reactive state
const cursorStyle = ref<string>('default')

// Build a lookup map from the teeth prop
const teethMap = computed<Map<number, ToothEntry>>(() => {
  return new Map(props.teeth.map((entry) => [entry.tooth.numero_fdi, entry]))
})

// Render the full chart on canvas
function render(): void {
  const canvas = canvasRef.value
  if (!canvas) return

  const ctx = canvas.getContext('2d')
  if (!ctx) return

  // Scale for HiDPI
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0)

  drawChartBackground(ctx, CANVAS_WIDTH, CANVAS_HEIGHT)

  for (const geo of ALL_TOOTH_GEOMETRIES) {
    const entry = teethMap.value.get(geo.fdi)
    if (entry) {
      const isSelected = props.selectedFdi === geo.fdi
      const selFace = isSelected ? props.selectedFace : null
      drawTooth(ctx, entry, isSelected, selFace)
    } else {
      drawEmptyTooth(ctx, geo)
    }
  }
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
