export type SearchMode = 'oc' | 'autor'

export interface Oc {
  id: string
  name: string
  author: string
  especie: string
  sexo: string
  altura: string
  caracteristicas: string
  descricao: string
  /** palette used for the placeholder avatar when there are no images */
  avatarPalette: number
  /** up to 4 images for the sheet; the first one is the cover */
  images: string[]
}

export interface OcDraft {
  name: string
  author: string
  especie: string
  sexo: string
  altura: string
  caracteristicas: string
  descricao: string
  images: string[]
}
