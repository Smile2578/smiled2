import type { FaceName, HitTestResult } from './dentalChartTypes'
import { hitTest } from './fdi-geometry'

export interface ClickEvent {
  canvasX: number
  canvasY: number
}

export interface InteractionResult {
  hit: HitTestResult | null
}

/**
 * Converts a DOM MouseEvent on the canvas to canvas-local coordinates,
 * accounting for devicePixelRatio scaling and canvas bounding rect offset.
 */
export function eventToCanvasCoords(
  event: MouseEvent,
  canvas: HTMLCanvasElement,
  dpr: number,
): { x: number; y: number } {
  const rect = canvas.getBoundingClientRect()
  // The canvas CSS size may differ from its logical pixel size when dpr > 1
  const scaleX = (canvas.width / dpr) / rect.width
  const scaleY = (canvas.height / dpr) / rect.height

  return {
    x: (event.clientX - rect.left) * scaleX,
    y: (event.clientY - rect.top) * scaleY,
  }
}

/**
 * Given canvas-local coordinates, returns the hit tooth/face or null.
 */
export function resolveHit(x: number, y: number): HitTestResult | null {
  const result = hitTest(x, y)
  if (!result) return null
  return { fdi: result.fdi, face: result.face }
}

/**
 * Returns cursor style based on whether the pointer is over a tooth.
 */
export function getCursorStyle(x: number, y: number): string {
  const result = hitTest(x, y)
  return result ? 'pointer' : 'default'
}
