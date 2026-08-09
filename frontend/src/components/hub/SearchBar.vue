<script setup lang="ts">
import { ref } from 'vue'
import { useOcStore } from '@/stores/oc'
import type { SearchMode } from '@/types/oc'

const store = useOcStore()
const draft = ref(store.appliedTerm)

function confirmSearch() {
  store.applySearch(draft.value)
}

function setMode(mode: SearchMode) {
  store.setSearchMode(mode)
  // re-filter immediately if a search term was already confirmed
  if (store.appliedTerm) confirmSearch()
}
</script>

<template>
  <div class="search-bar">
    <div class="search-bar__mode" role="group" aria-label="Pesquisar por">
      <button
        type="button"
        class="search-bar__mode-btn"
        :class="{ 'search-bar__mode-btn--active': store.searchMode === 'oc' }"
        @click="setMode('oc')"
      >
        OC
      </button>
      <button
        type="button"
        class="search-bar__mode-btn"
        :class="{ 'search-bar__mode-btn--active': store.searchMode === 'autor' }"
        @click="setMode('autor')"
      >
        Autor
      </button>
    </div>

    <label class="search-bar__field">
      <span class="search-bar__icon" aria-hidden="true">⌕</span>
      <input
        v-model="draft"
        type="search"
        :placeholder="store.searchMode === 'oc' ? 'Pesquisar OC…' : 'Pesquisar autor…'"
        @keyup.enter="confirmSearch"
      />
    </label>
  </div>
</template>

<style scoped>
.search-bar {
  display: flex;
  align-items: stretch;
  gap: 10px;
}

.search-bar__mode {
  display: flex;
  padding: 4px;
  gap: 4px;
  background: var(--color-surface-alt);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
}

.search-bar__mode-btn {
  border: none;
  background: transparent;
  color: var(--color-text-muted);
  font-family: var(--font-display);
  font-weight: 600;
  font-size: 14px;
  padding: 0 14px;
  border-radius: calc(var(--radius-sm) - 2px);
  cursor: pointer;
  transition:
    background var(--transition-fast),
    color var(--transition-fast);
}

.search-bar__mode-btn--active {
  background: var(--color-brand);
  color: var(--color-text-on-accent);
}

.search-bar__field {
  display: flex;
  align-items: center;
  gap: 8px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  padding: 0 14px;
  min-width: 260px;
  transition: border-color var(--transition-fast);
}

.search-bar__field:focus-within {
  border-color: var(--color-brand);
}

.search-bar__icon {
  color: var(--color-text-faint);
  font-size: 16px;
}

.search-bar__field input {
  border: none;
  background: transparent;
  color: var(--color-text);
  font-size: 15px;
  padding: 11px 0;
  width: 100%;
  outline: none;
}

.search-bar__field input::placeholder {
  color: var(--color-text-faint);
}

@media (max-width: 640px) {
  .search-bar {
    flex-direction: column;
  }
  .search-bar__field {
    min-width: 0;
  }
}
</style>
