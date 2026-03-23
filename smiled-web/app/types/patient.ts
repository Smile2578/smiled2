export interface Patient {
  id: string
  cabinet_id: string
  nom: string
  nom_naissance: string | null
  prenom: string
  sexe: 'M' | 'F'
  date_naissance: string
  num_ss: string | null
  email: string | null
  telephone: string | null
  adresse: string | null
  profession: string | null
  couverture: 'mutuelle' | 'cmu_c2s' | 'ame' | 'aucune'
  mutuelle_nom: string | null
  mutuelle_tableau_garantie: boolean | null
  contact_urgence_nom: string | null
  contact_urgence_tel: string | null
  contact_urgence_lien: string | null
  representant_legal_nom: string | null
  representant_legal_tel: string | null
  medecin_traitant_nom: string | null
  medecin_traitant_tel: string | null
  created_at: string
  updated_at: string
}

export interface CreatePatientInput {
  nom: string
  nom_naissance?: string | null
  prenom: string
  sexe: 'M' | 'F'
  date_naissance: string
  num_ss?: string | null
  email?: string | null
  telephone?: string | null
  adresse?: string | null
  profession?: string | null
  couverture: 'mutuelle' | 'cmu_c2s' | 'ame' | 'aucune'
  mutuelle_nom?: string | null
  mutuelle_tableau_garantie?: boolean | null
  contact_urgence_nom?: string | null
  contact_urgence_tel?: string | null
  contact_urgence_lien?: string | null
  representant_legal_nom?: string | null
  representant_legal_tel?: string | null
  medecin_traitant_nom?: string | null
  medecin_traitant_tel?: string | null
}

export interface Questionnaire {
  id: string
  patient_id: string
  cabinet_id: string
  version: number
  date_signature: string | null
  signe_par: string | null
  nom_signataire: string | null
  prochaine_maj: string | null
  // Haemostasis
  avk: unknown | null
  aod_molecule: string | null
  antiagregants: unknown | null
  hemostase: unknown | null
  // Infectious
  endocardite: unknown | null
  immunodepression: unknown | null
  protheses_articulaires: unknown | null
  // Drug risk
  bisphosphonates: unknown | null
  antiresorptifs: unknown | null
  radiotherapie: unknown | null
  // General
  troubles: unknown | null
  medicaments: string[] | null
  allergies: unknown | null
  // Habits
  tabac: string | null
  alcool: string | null
  drogues: unknown | null
  grossesse_mois: number | null
  allaitement: boolean | null
  activite_physique: string | null
  bruxisme: string | null
  sahos: unknown | null
  rgo: boolean | null
  tca: string | null
  // Dental history
  dernier_rdv_date: string | null
  brossage_quotidien: number | null
  auxiliaires: string[] | null
  historique_connu: string[] | null
  apprehension: string | null
  notice_information_date: string | null
  created_at: string
  updated_at: string
}

export interface QuestionnaireFormData {
  date_signature: string
  signe_par: string
  nom_signataire: string
  prochaine_maj: string
  // Haemostasis
  avk: boolean
  avk_molecule: string
  aod_molecule: string
  antiagregants: boolean
  antiagregants_molecule: string
  hemostase: boolean
  hemostase_detail: string
  // Infectious
  endocardite: boolean
  immunodepression: boolean
  immunodepression_detail: string
  protheses_articulaires: boolean
  protheses_articulaires_detail: string
  // Drug risk
  bisphosphonates: boolean
  bisphosphonates_molecule: string
  antiresorptifs: boolean
  antiresorptifs_molecule: string
  radiotherapie: boolean
  radiotherapie_zone: string
  // General
  troubles: string
  medicaments: string
  allergies: boolean
  allergies_detail: string
  // Habits
  tabac: string
  alcool: string
  drogues: boolean
  drogues_detail: string
  grossesse_mois: string
  allaitement: boolean
  activite_physique: string
  bruxisme: string
  sahos: boolean
  sahos_traitement: string
  rgo: boolean
  tca: string
  // Dental history
  dernier_rdv_date: string
  brossage_quotidien: string
  auxiliaires: string
  historique_connu: string
  apprehension: string
  notice_information_date: string
}
