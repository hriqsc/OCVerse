import { computed, ref } from 'vue'
import { defineStore } from 'pinia'
import { OcMockGenerator } from '@/mock/ocGenerator'
import type { Oc, OcDraft, SearchMode } from '@/types/oc'

export const useOcStore = defineStore('oc', () => {
  const items = ref<Oc[]>(OcMockGenerator.generateMany(20))

  // the term actually applied to the filter — only changes once the user confirms it (Enter)
  const appliedTerm = ref('')
  const searchMode = ref<SearchMode>('oc')

  const filteredItems = computed<Oc[]>(() => {
    const term = appliedTerm.value.trim().toLowerCase()
    if (term.length === 0) return items.value
    return items.value.filter((oc) => {
      const field = searchMode.value === 'oc' ? oc.name : oc.author
      return field.toLowerCase().includes(term)
    })
  })

  /** Only called when the user confirms the search (Enter). */
  function applySearch(term: string) {
    appliedTerm.value = term
  }

  function setSearchMode(mode: SearchMode) {
    searchMode.value = mode
  }

  function getById(id: string): Oc | undefined {
    return items.value.find((oc) => oc.id === id)
  }

  function addOc(draft: OcDraft): Oc {
    const oc: Oc = {
      id: `oc-${Date.now()}`,
      avatarPalette: (items.value.length % 5) + 1,
      ...draft,
    }
    items.value = [oc, ...items.value]
    return oc
  }

  function updateOc(id: string, draft: OcDraft) {
    const index = items.value.findIndex((oc) => oc.id === id)
    if (index === -1) return
    items.value[index] = { ...items.value[index], ...draft }
  }

  return {
    items,
    appliedTerm,
    searchMode,
    filteredItems,
    applySearch,
    setSearchMode,
    getById,
    addOc,
    updateOc,
  }
})
