import type { ToothGeometry, FaceGeometry, FaceName } from './types'

// Canvas layout constants
const TOOTH_WIDTH = 34
const TOOTH_HEIGHT = 34
const GAP = 3
const FACE_SIZE = TOOTH_WIDTH / 3 // approx 11px per face cell

// Upper jaw starts at y=60, lower jaw at y=120
// Center divider between upper/lower jaws
const UPPER_Y = 60
const LOWER_Y = 120

// Quadrant 1: FDI 11–18 (upper right), drawn right-to-left from center
// Quadrant 2: FDI 21–28 (upper left), drawn left-to-right from center
// Quadrant 3: FDI 31–38 (lower left), drawn left-to-right from center
// Quadrant 4: FDI 41–48 (lower right), drawn right-to-left from center

// Standard FDI layout for a full adult chart.
// The chart is viewed from the front (patient's right = display left).
const UPPER_FDI_ORDER = [18, 17, 16, 15, 14, 13, 12, 11, 21, 22, 23, 24, 25, 26, 27, 28]
const LOWER_FDI_ORDER = [48, 47, 46, 45, 44, 43, 42, 41, 31, 32, 33, 34, 35, 36, 37, 38]

function buildFaces(x: number, y: number, isUpper: boolean): FaceGeometry[] {
  const w = TOOTH_WIDTH
  const h = TOOTH_HEIGHT
  const third = Math.floor(w / 3)
  const remaining = w - third * 2

  // Face layout (5 visible faces arranged in a cross pattern):
  //
  // Upper tooth (buccal = top, palatal = bottom):
  //   ┌────────────┐  ← V (vestibulaire/buccal)
  //   │ M │ O  │ D │  ← middle row: M, O (center), D
  //   └────────────┘  ← P_L (palatine/linguale)
  //
  // Lower tooth (lingual = top, buccal = bottom):
  //   ┌────────────┐  ← P_L (palatine/linguale)
  //   │ M │ O  │ D │
  //   └────────────┘  ← V (vestibulaire)

  const rowHeight = Math.floor(h / 3)
  const midHeight = h - rowHeight * 2

  const vFace: FaceName = 'V'
  const plFace: FaceName = 'P_L'

  const topFace = isUpper ? vFace : plFace
  const bottomFace = isUpper ? plFace : vFace

  return [
    // Top strip
    { face: topFace, x, y, width: w, height: rowHeight },
    // Middle left: M
    { face: 'M', x, y: y + rowHeight, width: third, height: midHeight },
    // Middle center: O (occlusal)
    { face: 'O', x: x + third, y: y + rowHeight, width: remaining, height: midHeight },
    // Middle right: D
    { face: 'D', x: x + third + remaining, y: y + rowHeight, width: third, height: midHeight },
    // Bottom strip
    { face: bottomFace, x, y: y + rowHeight + midHeight, width: w, height: rowHeight },
  ]
}

function getQuadrant(fdi: number): 1 | 2 | 3 | 4 {
  const hundreds = Math.floor(fdi / 10)
  if (hundreds === 1) return 1
  if (hundreds === 2) return 2
  if (hundreds === 3) return 3
  return 4
}

function buildGeometries(
  fdiOrder: number[],
  y: number,
  isUpper: boolean,
  startX: number,
): ToothGeometry[] {
  return fdiOrder.map((fdi, index) => {
    const x = startX + index * (TOOTH_WIDTH + GAP)
    return {
      fdi,
      x,
      y,
      width: TOOTH_WIDTH,
      height: TOOTH_HEIGHT,
      quadrant: getQuadrant(fdi),
      isUpper,
      faces: buildFaces(x, y, isUpper),
    }
  })
}

// Total canvas width = 16 teeth * (TOOTH_WIDTH + GAP) - GAP + 2 * margin
const CHART_MARGIN = 20
export const CANVAS_WIDTH = 16 * (TOOTH_WIDTH + GAP) - GAP + CHART_MARGIN * 2
export const CANVAS_HEIGHT = 200

// Build all 32 tooth geometries
const START_X = CHART_MARGIN

const upperGeometries = buildGeometries(UPPER_FDI_ORDER, UPPER_Y - TOOTH_HEIGHT, true, START_X)
const lowerGeometries = buildGeometries(LOWER_FDI_ORDER, LOWER_Y, false, START_X)

export const ALL_TOOTH_GEOMETRIES: ToothGeometry[] = [
  ...upperGeometries,
  ...lowerGeometries,
]

export const GEOMETRY_BY_FDI: Map<number, ToothGeometry> = new Map(
  ALL_TOOTH_GEOMETRIES.map((g) => [g.fdi, g]),
)

// Hit-test: find which tooth and face was clicked
export function hitTest(
  canvasX: number,
  canvasY: number,
): { fdi: number; face: FaceName | null } | null {
  for (const geo of ALL_TOOTH_GEOMETRIES) {
    if (
      canvasX >= geo.x &&
      canvasX < geo.x + geo.width &&
      canvasY >= geo.y &&
      canvasY < geo.y + geo.height
    ) {
      // Found the tooth — now find the face
      const hitFace = geo.faces.find(
        (f) =>
          canvasX >= f.x &&
          canvasX < f.x + f.width &&
          canvasY >= f.y &&
          canvasY < f.y + f.height,
      )
      return { fdi: geo.fdi, face: hitFace?.face ?? null }
    }
  }
  return null
}
