import type { AuthorOption, Oc, Rarity } from '@/types/oc'

/**
 * OcMockGenerator
 * ----------------
 * Gera dados falsos de OCs para popular o hub enquanto não existe backend.
 * Quando a API real estiver pronta, basta trocar o `ocStore` para consumi-la
 * em vez de `OcMockGenerator.generateMany`, sem tocar nos componentes.
 */
const FIRST_NAMES = [
  'Luna',
  'Kael',
  'Nyx',
  'Ravi',
  'Sable',
  'Mika',
  'Thorne',
  'Ivy',
  'Zeph',
  'Coral',
  'Bruma',
  'Ash',
  'Vex',
  'Suri',
  'Onix',
  'Pixel',
  'Draco',
  'Aurora',
  'Bosque',
  'Ember',
]

const AUTHORS = [
  'Yasu',
  'Miojo_art',
  'Kotori',
  'Bel.draws',
  'Rhaya',
  'Notte',
  'Fennik',
  'Ju.k',
  'Vantablack',
  'Sopa_de_letrinhas',
]

const RARITY_WEIGHTS: Array<[Rarity, number]> = [
  ['comum', 0.45],
  ['raro', 0.32],
  ['epico', 0.18],
  ['lendario', 0.05],
]

function pickRarity(random: () => number): Rarity {
  const roll = random()
  let acc = 0
  for (const [rarity, weight] of RARITY_WEIGHTS) {
    acc += weight
    if (roll <= acc) return rarity
  }
  return 'comum'
}

/** PRNG simples com seed, para gerar sempre a mesma "massa de dados" em dev. */
function mulberry32(seed: number) {
  let a = seed
  return () => {
    a |= 0
    a = (a + 0x6d2b79f5) | 0
    let t = Math.imul(a ^ (a >>> 15), 1 | a)
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296
  }
}

export class OcMockGenerator {
  private static readonly SEED = 2026

  /** Um OC "fixo" com ilustração, igual ao card de destaque do layout original. */
  static featured(): Oc {
    return {
      id: 'oc-featured-laranja',
      name: 'Laranja',
      author: 'Edylanches',
      rarity: 'epico',
      avatarPalette: 2,
      emoji: '🦊',
    }
  }

  static generateOne(index: number, random: () => number): Oc {
    // metade dos uploads simula fichas ainda sem nome/autor preenchido
    // (igual ao placeholder "Nome do oc" / "Autor" do layout original)
    const filled = random() > 0.5
    const name = filled ? FIRST_NAMES[Math.floor(random() * FIRST_NAMES.length)] : 'Nome do oc'
    const author = filled ? AUTHORS[Math.floor(random() * AUTHORS.length)] : 'Autor'
    return {
      id: `oc-${index}`,
      name,
      author,
      rarity: pickRarity(random),
      avatarPalette: (index % 5) + 1,
    }
  }

  static generateMany(count: number): Oc[] {
    const random = mulberry32(this.SEED)
    const list: Oc[] = [this.featured()]
    for (let i = 0; i < count - 1; i++) {
      list.push(this.generateOne(i, random))
    }
    return list
  }

  static authors(): AuthorOption[] {
    return [{ id: 'all', name: 'Todos' }, ...AUTHORS.map((a) => ({ id: a, name: a }))]
  }
}
