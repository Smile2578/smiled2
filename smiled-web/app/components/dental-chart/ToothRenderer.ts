import type { ToothEntry, ToothGeometry, FaceGeometry, FaceName, FaceState, ToothStatus } from './types'
import { TOOTH_STATUS_COLORS, FACE_STATE_COLORS } from './types'
import { GEOMETRY_BY_FDI, ALL_TOOTH_GEOMETRIES } from './fdi-geometry'

// OffscreenCanvas background layer cache
let bgCache: OffscreenCanvas | null = null
let bgCacheHash = ''

function computeTeethHash(teeth: Map<number, ToothEntry>): string {
  const parts: string[] = []
  for (const geo of ALL_TOOTH_GEOMETRIES) {
    const entry = teeth.get(geo.fdi)
    if (!entry) {
      parts.push(`${geo.fdi}:empty`)
      continue
    }
    const facePart = entry.faces
      .map((f) => `${f.face}=${f.etat}`)
      .join(',')
    parts.push(`${geo.fdi}:${entry.tooth.statut}:${entry.tooth.paro_mobilite ?? 0}:${facePart}`)
  }
  return parts.join('|')
}

const BORDER_COLOR = '#374151'
const SELECTED_BORDER_COLOR = '#2563EB'
const SELECTED_BORDER_WIDTH = 2.5
const NORMAL_BORDER_WIDTH = 1
const ABSENT_PATTERN_COLOR = '#D1D5DB'
const LABEL_COLOR = '#1F2937'
const LABEL_FONT = 'bold 8px system-ui'
const FDI_LABEL_FONT = '9px system-ui'
const FDI_LABEL_COLOR = '#6B7280'
const FDI_LABEL_OFFSET_UPPER = -6
const FDI_LABEL_OFFSET_LOWER = 12

function getFaceColor(entry: ToothEntry, face: FaceName): string {
  if (entry.tooth.statut === 'absente' || entry.tooth.statut === 'extraite') {
    return ABSENT_PATTERN_COLOR
  }

  const faceData = entry.faces.find((f) => f.face === face)
  if (!faceData) return FACE_STATE_COLORS.saine

  return FACE_STATE_COLORS[faceData.etat as FaceState] ?? FACE_STATE_COLORS.saine
}

function getToothBgColor(entry: ToothEntry): string {
  return TOOTH_STATUS_COLORS[entry.tooth.statut as ToothStatus] ?? '#FFFFFF'
}

function drawFace(
  ctx: CanvasRenderingContext2D,
  faceGeo: FaceGeometry,
  fillColor: string,
  isSelected: boolean,
): void {
  ctx.fillStyle = fillColor
  ctx.fillRect(faceGeo.x, faceGeo.y, faceGeo.width, faceGeo.height)

  ctx.strokeStyle = isSelected ? SELECTED_BORDER_COLOR : BORDER_COLOR
  ctx.lineWidth = 0.5
  ctx.strokeRect(faceGeo.x, faceGeo.y, faceGeo.width, faceGeo.height)
}

function drawAbsentStripes(
  ctx: CanvasRenderingContext2D,
  geo: ToothGeometry,
): void {
  ctx.save()
  ctx.beginPath()
  ctx.rect(geo.x, geo.y, geo.width, geo.height)
  ctx.clip()

  ctx.strokeStyle = '#9CA3AF'
  ctx.lineWidth = 1

  const step = 5
  for (let i = -geo.height; i < geo.width + geo.height; i += step) {
    ctx.beginPath()
    ctx.moveTo(geo.x + i, geo.y)
    ctx.lineTo(geo.x + i + geo.height, geo.y + geo.height)
    ctx.stroke()
  }

  ctx.restore()
}

function drawImplantMarker(
  ctx: CanvasRenderingContext2D,
  geo: ToothGeometry,
): void {
  const cx = geo.x + geo.width / 2
  const cy = geo.y + geo.height / 2
  const r = Math.min(geo.width, geo.height) / 4

  ctx.beginPath()
  ctx.arc(cx, cy, r, 0, Math.PI * 2)
  ctx.fillStyle = '#3B82F6'
  ctx.fill()
  ctx.strokeStyle = '#1D4ED8'
  ctx.lineWidth = 1
  ctx.stroke()
}

function drawFdiLabel(
  ctx: CanvasRenderingContext2D,
  geo: ToothGeometry,
): void {
  ctx.font = FDI_LABEL_FONT
  ctx.fillStyle = FDI_LABEL_COLOR
  ctx.textAlign = 'center'
  ctx.textBaseline = 'middle'

  const labelY = geo.isUpper
    ? geo.y + FDI_LABEL_OFFSET_UPPER
    : geo.y + geo.height - FDI_LABEL_OFFSET_LOWER

  ctx.fillText(String(geo.fdi), geo.x + geo.width / 2, labelY)
}

function drawMobilityBadge(
  ctx: CanvasRenderingContext2D,
  geo: ToothGeometry,
  mobilite: number,
): void {
  if (mobilite === 0) return

  const cx = geo.x + geo.width - 5
  const cy = geo.isUpper ? geo.y + 5 : geo.y + geo.height - 5
  const r = 5

  ctx.beginPath()
  ctx.arc(cx, cy, r, 0, Math.PI * 2)
  ctx.fillStyle = mobilite >= 3 ? '#EF4444' : mobilite === 2 ? '#F59E0B' : '#10B981'
  ctx.fill()

  ctx.font = LABEL_FONT
  ctx.fillStyle = '#FFFFFF'
  ctx.textAlign = 'center'
  ctx.textBaseline = 'middle'
  ctx.fillText(String(mobilite), cx, cy)
}

export function drawTooth(
  ctx: CanvasRenderingContext2D,
  entry: ToothEntry,
  isSelected: boolean,
  selectedFace: FaceName | null,
): void {
  const geo = GEOMETRY_BY_FDI.get(entry.tooth.numero_fdi)
  if (!geo) return

  const status = entry.tooth.statut as ToothStatus

  // Draw each face
  for (const faceGeo of geo.faces) {
    let fillColor: string

    if (status === 'absente' || status === 'extraite') {
      fillColor = '#F3F4F6'
    } else if (status === 'implant') {
      fillColor = '#DBEAFE'
    } else {
      fillColor = getFaceColor(entry, faceGeo.face)
    }

    const isFaceSelected = isSelected && selectedFace === faceGeo.face
    drawFace(ctx, faceGeo, fillColor, isSelected)

    // Highlight selected face with a blue overlay
    if (isFaceSelected) {
      ctx.fillStyle = 'rgba(37, 99, 235, 0.18)'
      ctx.fillRect(faceGeo.x, faceGeo.y, faceGeo.width, faceGeo.height)
    }
  }

  // Special rendering overlays
  if (status === 'absente' || status === 'extraite') {
    drawAbsentStripes(ctx, geo)
  }

  if (status === 'implant') {
    drawImplantMarker(ctx, geo)
  }

  // Outer border (selected = blue, normal = dark)
  ctx.strokeStyle = isSelected ? SELECTED_BORDER_COLOR : BORDER_COLOR
  ctx.lineWidth = isSelected ? SELECTED_BORDER_WIDTH : NORMAL_BORDER_WIDTH
  ctx.strokeRect(geo.x, geo.y, geo.width, geo.height)

  // Mobility badge
  if (entry.tooth.paro_mobilite !== null && entry.tooth.paro_mobilite > 0) {
    drawMobilityBadge(ctx, geo, entry.tooth.paro_mobilite)
  }

  // FDI number label
  drawFdiLabel(ctx, geo)
}

export function drawChartBackground(
  ctx: CanvasRenderingContext2D,
  canvasWidth: number,
  canvasHeight: number,
): void {
  ctx.clearRect(0, 0, canvasWidth, canvasHeight)
  ctx.fillStyle = '#F9FAFB'
  ctx.fillRect(0, 0, canvasWidth, canvasHeight)

  // Center divider line between upper and lower jaws
  const midY = canvasHeight / 2
  ctx.beginPath()
  ctx.setLineDash([4, 4])
  ctx.strokeStyle = '#D1D5DB'
  ctx.lineWidth = 1
  ctx.moveTo(0, midY)
  ctx.lineTo(canvasWidth, midY)
  ctx.stroke()
  ctx.setLineDash([])

  // Quadrant labels
  ctx.font = '10px system-ui'
  ctx.fillStyle = '#9CA3AF'
  ctx.textAlign = 'left'
  ctx.textBaseline = 'top'
  ctx.fillText('Q1', 4, 2)
  ctx.textAlign = 'right'
  ctx.fillText('Q2', canvasWidth - 4, 2)
  ctx.textAlign = 'left'
  ctx.fillText('Q4', 4, midY + 2)
  ctx.textAlign = 'right'
  ctx.fillText('Q3', canvasWidth - 4, midY + 2)
}

export function drawEmptyTooth(
  ctx: CanvasRenderingContext2D,
  geo: ToothGeometry,
): void {
  ctx.fillStyle = '#E5E7EB'
  ctx.fillRect(geo.x, geo.y, geo.width, geo.height)
  ctx.strokeStyle = '#D1D5DB'
  ctx.lineWidth = 1
  ctx.strokeRect(geo.x, geo.y, geo.width, geo.height)
  drawFdiLabel(ctx, geo)
}

/**
 * Renders the full chart using an OffscreenCanvas background cache.
 * The background (all 32 teeth) is only redrawn when teeth data changes.
 * Only the selection highlight is drawn per frame on top of the cached background.
 */
export function renderCached(
  ctx: CanvasRenderingContext2D,
  canvasWidth: number,
  canvasHeight: number,
  teethMap: Map<number, ToothEntry>,
  selectedFdi: number | null,
  selectedFace: FaceName | null,
): void {
  const hash = computeTeethHash(teethMap)

  // Rebuild background cache if data changed
  if (hash !== bgCacheHash || !bgCache) {
    if (!bgCache || bgCache.width !== canvasWidth || bgCache.height !== canvasHeight) {
      bgCache = new OffscreenCanvas(canvasWidth, canvasHeight)
    }
    const bgCtx = bgCache.getContext('2d')
    if (!bgCtx) return

    drawChartBackground(bgCtx, canvasWidth, canvasHeight)

    for (const geo of ALL_TOOTH_GEOMETRIES) {
      const entry = teethMap.get(geo.fdi)
      if (entry) {
        drawTooth(bgCtx, entry, false, null)
      } else {
        drawEmptyTooth(bgCtx, geo)
      }
    }

    bgCacheHash = hash
  }

  // Draw cached background
  ctx.clearRect(0, 0, canvasWidth, canvasHeight)
  ctx.drawImage(bgCache!, 0, 0)

  // Draw selection highlight on top
  if (selectedFdi !== null) {
    const entry = teethMap.get(selectedFdi)
    if (entry) {
      drawTooth(ctx, entry, true, selectedFace)
    }
  }
}
