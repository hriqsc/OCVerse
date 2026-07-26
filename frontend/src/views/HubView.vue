<script setup lang="ts">
import { useOcStore } from '@/stores/oc'
import AppHeader from '@/components/layout/AppHeader.vue'
import UploadButton from '@/components/hub/UploadButton.vue'
import SearchBar from '@/components/hub/SearchBar.vue'
import OcGrid from '@/components/hub/OcGrid.vue'
import { useAuthModal } from '@/composables/useAuthModal'

const store = useOcStore()
const { openLogin } = useAuthModal()

function handleUploadClick() {
  // por enquanto o upload exige login — troque por navegação real quando existir a tela
  openLogin()
}
</script>

<template>
  <div class="hub-page">
    <AppHeader />

    <div class="hub-page__toolbar">
      <div class="hub-page__toolbar-inner">
        <UploadButton @click="handleUploadClick" />
        <SearchBar />
      </div>
    </div>

    <main class="hub-page__content">
      <OcGrid :items="store.filteredItems" />
    </main>
  </div>
</template>

<style scoped>
.hub-page {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.hub-page__toolbar {
  border-bottom: 1px solid var(--color-border-soft);
  background: var(--color-bg);
}

.hub-page__toolbar-inner {
  max-width: var(--max-content-width);
  margin: 0 auto;
  padding: 16px 32px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
}

.hub-page__content {
  flex: 1;
  max-width: var(--max-content-width);
  width: 100%;
  margin: 0 auto;
  padding: 28px 32px 64px;
}

@media (max-width: 640px) {
  .hub-page__toolbar-inner,
  .hub-page__content {
    padding-left: 18px;
    padding-right: 18px;
  }
}
</style>
