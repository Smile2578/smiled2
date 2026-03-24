export function couvertureLabel(c: string): string {
  const map: Record<string, string> = {
    mutuelle: 'Mutuelle',
    cmu_c2s: 'CMU / C2S',
    ame: 'AME',
    aucune: 'Aucune',
  }
  return map[c] || c
}

export function couvertureBadgeVariant(c: string): 'default' | 'secondary' | 'destructive' | 'outline' {
  const map: Record<string, 'default' | 'secondary' | 'destructive' | 'outline'> = {
    mutuelle: 'default',
    cmu_c2s: 'secondary',
    ame: 'secondary',
    aucune: 'outline',
  }
  return map[c] || 'outline'
}
