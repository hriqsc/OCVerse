<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useOcStore } from '@/stores/oc'
import { useUserStore } from '@/stores/user'
import AppHeader from '@/components/layout/AppHeader.vue'
import UploadButton from '@/components/hub/UploadButton.vue'
import SearchBar from '@/components/hub/SearchBar.vue'
import OcGrid from '@/components/hub/OcGrid.vue'
import { useAuthModal } from '@/composables/useAuthModal'

const store = useOcStore()
const userStore = useUserStore()
const router = useRouter()
const { openLogin } = useAuthModal()


onMounted(() => {
  store.LoadOcs()
})

function handleUploadClick() {
  if (userStore.isLoggedIn) {
    router.push('/hub/upload')
  } else {
    openLogin()
  }
}
</script>

<template>
  <div class="hub-page">
    <AppHeader />

    <div class="hub-page__toolbar">
      <div class="hub-page__toolbar-inner">
        <div class="hub-page__toolbar-actions">
          <UploadButton @click="handleUploadClick" />
          <RouterLink to="/magmas" class="btn hub-page__magmas-link">
            Magmas
          </RouterLink>
        </div>
        <SearchBar />
      </div>
    </div>

    <main class="hub-page__content">
      <OcGrid :items="store.minified" />
    </main>
  </div>
</template>

<style scoped>
.hub-page {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  isolation: isolate;
}

.hub-page::before {
  content: '';
  position: fixed;
  inset: 0;
  background-image: url('/background.jpg');
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  opacity: 0.15;
  z-index: -1;
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

.hub-page__toolbar-actions {
  display: flex;
  align-items: center;
  gap: 12px;
}

.hub-page__magmas-link {
  display: inline-flex;
  align-items: center;
  padding: 0.6rem 1.4rem;
  text-decoration: none;
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