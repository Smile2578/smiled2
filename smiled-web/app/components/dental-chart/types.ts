// TypeScript types matching the Rust backend schema dentaire types

export type ToothStatus =
  | 'presente'
  | 'absente'
  | 'implant'
  | 'couronne'
  | 'bridge'
  | 'extraite'
  | 'incluse'
  | 'lacteal'

export type FaceName = 'M' | 'D' | 'O' | 'V' | 'P_L' | 'C'

export type FaceState =
  | 'saine'
  | 'carie'
  | 'composite'
  | 'amalgame'
  | 'inlay'
  | 'onlay'
  | 'couronne_provisoire'
  | 'fracture'
  | 'absente'

export interface ToothFace {
  id: string
  dent_id: string
  face: FaceName
  etat: FaceState
  restauration: Record<string, unknown> | null
}

export interface Tooth {
  id: string
  schema_id: string
  numero_fdi: number
  statut: ToothStatus
  prothese_fixe: Record<string, unknown> | null
  endo: Record<string, unknown> | null
  paro_mobilite: number | null
  notes: string | null
}

export interface ParoSite {
  id: string
  dent_id: string
  site: string
  profondeur_sondage: number | null
  recession: number | null
  mobilite: number | null
  furcation: number | null
  saignement: boolean
  suppuration: boolean
}

export interface ToothEntry {
  tooth: Tooth
  faces: ToothFace[]
  paro_sites: ParoSite[]
}

export interface SchemaInfo {
  id: string
  patient_id: string
  version: number
  dentition: 'permanente' | 'temporaire' | 'mixte'
  created_at: string
}

export interface Occlusion {
  id: string
  schema_id: string
  classe_angle: string | null
  surplomb: number | null
  recouvrement: number | null
  notes: string | null
}

export interface Atm {
  id: string
  schema_id: string
  douleur_droite: boolean
  douleur_gauche: boolean
  bruxisme: boolean
  notes: string | null
}

export interface ParoGlobal {
  id: string
  schema_id: string
  stade: string | null
  grade: string | null
  notes: string | null
}

export interface FullSchemaResponse {
  schema: SchemaInfo
  dents: ToothEntry[]
  occlusion: Occlusion | null
  atm: Atm | null
  paro_global: ParoGlobal | null
}

export interface SchemaVersion {
  id: string
  patient_id: string
  version: number
  dentition: 'permanente' | 'temporaire' | 'mixte'
  created_at: string
}

export interface UpdateToothInput {
  statut?: ToothStatus
  prothese_fixe?: Record<string, unknown> | null
  endo?: Record<string, unknown> | null
  paro_mobilite?: number | null
  notes?: string | null
}

export interface UpdateFaceInput {
  etat?: FaceState
  restauration?: Record<string, unknown> | null
}

// Canvas-specific types
export interface FaceGeometry {
  face: FaceName
  x: number
  y: number
  width: number
  height: number
}

export interface ToothGeometry {
  fdi: number
  x: number
  y: number
  width: number
  height: number
  quadrant: 1 | 2 | 3 | 4
  isUpper: boolean
  faces: FaceGeometry[]
}

export interface HitTestResult {
  fdi: number
  face: FaceName | null
}

export interface ChartSelection {
  fdi: number | null
  face: FaceName | null
}

export const TOOTH_STATUS_COLORS: Record<ToothStatus, string> = {
  presente: '#FFFFFF',
  absente: '#9CA3AF',
  implant: '#3B82F6',
  couronne: '#F59E0B',
  bridge: '#8B5CF6',
  extraite: '#EF4444',
  incluse: '#F97316',
  lacteal: '#FDE68A',
}

export const FACE_STATE_COLORS: Record<FaceState, string> = {
  saine: '#FFFFFF',
  carie: '#DC2626',
  composite: '#2563EB',
  amalgame: '#374151',
  inlay: '#D97706',
  onlay: '#7C3AED',
  couronne_provisoire: '#F59E0B',
  fracture: '#EF4444',
  absente: '#9CA3AF',
}

export const TOOTH_STATUS_LABELS: Record<ToothStatus, string> = {
  presente: 'Présente',
  absente: 'Absente',
  implant: 'Implant',
  couronne: 'Couronne',
  bridge: 'Bridge',
  extraite: 'Extraite',
  incluse: 'Incluse',
  lacteal: 'Lactéale',
}

export const FACE_STATE_LABELS: Record<FaceState, string> = {
  saine: 'Saine',
  carie: 'Carie',
  composite: 'Composite',
  amalgame: 'Amalgame',
  inlay: 'Inlay',
  onlay: 'Onlay',
  couronne_provisoire: 'Couronne provisoire',
  fracture: 'Fracture',
  absente: 'Absente',
}

export const FACE_LABELS: Record<FaceName, string> = {
  M: 'Mésiale',
  D: 'Distale',
  O: 'Occlusale',
  V: 'Vestibulaire',
  P_L: 'Palatine/Linguale',
  C: 'Cervicale',
}
