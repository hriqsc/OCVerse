import { computed, ref } from 'vue'
import { defineStore } from 'pinia'
import { OcMockGenerator } from '@/mock/ocGenerator'
import type { Oc } from '@/types/oc'

export const useOcStore = defineStore('oc', () => {
  const items = ref<Oc[]>(OcMockGenerator.generateMany(20))
  const searchTerm = ref('')
  const selectedAuthor = ref('all')

  const authors = computed(() => OcMockGenerator.authors())

  const filteredItems = computed<Oc[]>(() => {
    const term = searchTerm.value.trim().toLowerCase()
    return items.value.filter((oc) => {
      const matchesAuthor = selectedAuthor.value === 'all' || oc.author === selectedAuthor.value
      const matchesTerm = term.length === 0 || oc.name.toLowerCase().includes(term)
      return matchesAuthor && matchesTerm
    })
  })

  function setSearchTerm(term: string) {
    searchTerm.value = term
  }

  function setSelectedAuthor(authorId: string) {
    selectedAuthor.value = authorId
  }

  return {
    items,
    authors,
    searchTerm,
    selectedAuthor,
    filteredItems,
    setSearchTerm,
    setSelectedAuthor,
  }
})
