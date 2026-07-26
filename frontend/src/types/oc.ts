export type Rarity = 'comum' | 'raro' | 'epico' | 'lendario'

export interface Oc {
  id: string
  name: string
  author: string
  rarity: Rarity
  /** paleta usada no avatar placeholder quando não há imageUrl */
  avatarPalette: number
  imageUrl?: string
  emoji?: string
}

export interface AuthorOption {
  id: string
  name: string
}
