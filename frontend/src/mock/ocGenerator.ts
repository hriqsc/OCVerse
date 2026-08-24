// import type { Oc } from '@/types/oc'

// /**
//  * OcMockGenerator
//  * ----------------
//  * Generates fake OC data to fill the hub while there is no backend yet.
//  * Once a real API exists, just swap the calls inside `src/stores/oc.ts`
//  * for HTTP requests instead of `OcMockGenerator.generateMany` — the
//  * components don't need to change.
//  */
// const ESPECIES = ['Raposa', 'Lobo', 'Gato', 'Cervo', 'Dragão', 'Coelho', 'Corvo']
// const SEXOS = ['Masculino', 'Feminino', 'Não-binário']
// const AUTORES = [
//   'Yasu',
//   'Miojo_art',
//   'Kotori',
//   'Bel.draws',
//   'Rhaya',
//   'Notte',
//   'Fennik',
//   'Sopa_de_letrinhas',
// ]
// const CARACTERISTICAS = [
//   'Tímido, gosta de café, colecionador de bottons',
//   'Extrovertido, toca violão, adora chuva',
//   'Beta, Nyuchi loyalist, Vtuber maker',
//   'Caseiro, cozinha bem, medo de altura',
//   'Curioso, gamer, dorme tarde',
// ]

// /** Simple seeded PRNG, so dev always sees the same "fake dataset". */
// function mulberry32(seed: number) {
//   let a = seed
//   return () => {
//     a |= 0
//     a = (a + 0x6d2b79f5) | 0
//     let t = Math.imul(a ^ (a >>> 15), 1 | a)
//     t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t
//     return ((t ^ (t >>> 14)) >>> 0) / 4294967296
//   }
// }

// function pick<T>(list: T[], random: () => number): T {
//   if (list.length === 0) {
//     throw new Error("Cannot pick from an empty list");
//   }
//   return list[Math.floor(random() * list.length)]!;
// }

// export class OcMockGenerator {
//   private static readonly SEED = 2026

//   static generateOne(index: number, random: () => number): Oc {
//     // half of the entries simulate sheets not filled in yet
//     // (matches the "Nome do oc" / "Autor" placeholder from the original layout)
//     const filled = random() > 0.5
//     return {
//       id: index + 1,
//       creator_user_name: filled ? pick(AUTORES, random) : 'Autor',
//       oc_name: filled ? `OC ${index + 1}` : 'Nome do oc',
//       specie: pick(ESPECIES, random),
//       sex: pick(SEXOS, random),
//       height: `${140 + Math.floor(random() * 60)}cm`,
//       description: filled ? pick(CARACTERISTICAS, random) : '',
//       images: ["https://picsum.photos/500","https://picsum.photos/600","https://picsum.photos/700","https://picsum.photos/800","https://picsum.photos/900"],
//     }
//   }

//   static generateMany(count: number): Oc[] {
//     const random = mulberry32(this.SEED)
//     const list: Oc[] = []
//     for (let i = 0; i < count; i++) {
//       list.push(this.generateOne(i, random))
//     }
//     return list
//   }
// }


// export function MockMagmaList(){
//  return ["https://magma.com/d/K0EgWkMUDz","https://magma.com/d/K0EgWkMUDz","https://magma.com/d/K0EgWkMUDz"]
// }