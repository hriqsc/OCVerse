<script setup lang="ts">
import { useOcStore } from '@/stores/oc'

const store = useOcStore()
</script>

<template>
  <div class="search-bar">
    <label class="search-bar__field">
      <span class="search-bar__icon" aria-hidden="true">⌕</span>
      <input
        type="search"
        placeholder="Pesquisar"
        :value="store.searchTerm"
        @input="store.setSearchTerm(($event.target as HTMLInputElement).value)"
      />
    </label>

    <label class="search-bar__select">
      <select
        :value="store.selectedAuthor"
        @change="store.setSelectedAuthor(($event.target as HTMLSelectElement).value)"
      >
        <option v-for="author in store.authors" :key="author.id" :value="author.id">
          {{ author.id === 'all' ? 'Autor' : author.name }}
        </option>
      </select>
      <span class="search-bar__chevron" aria-hidden="true">⌄</span>
    </label>
  </div>
</template>

<style scoped>
.search-bar {
  display: flex;
  align-items: stretch;
  gap: 10px;
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

.search-bar__select {
  position: relative;
  display: flex;
  align-items: center;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  padding: 0 34px 0 14px;
}

.search-bar__select select {
  appearance: none;
  border: none;
  background: transparent;
  color: var(--color-text);
  font-weight: 600;
  font-size: 15px;
  padding: 11px 0;
  outline: none;
  cursor: pointer;
}

.search-bar__select select option {
  background: var(--color-surface-alt);
  color: var(--color-text);
}

.search-bar__chevron {
  position: absolute;
  right: 12px;
  color: var(--color-text-faint);
  pointer-events: none;
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
