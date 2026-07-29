<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import AppHeader from '@/components/layout/AppHeader.vue'
import EditOcModal from '@/components/oc/EditOcModal.vue'
import { useOcStore } from '@/stores/oc'
import { useUserStore } from '@/stores/user'
import type { OcDraft } from '@/types/oc'

const route = useRoute()
const store = useOcStore()
const userStore = useUserStore()

const oc = computed(() => store.getById(String(route.params.id)))

// só é dono se estiver logado E o nickname bater com o autor do OC
const isOwner = computed(() => {
  if (!userStore.isLoggedIn || !oc.value) return false
  return userStore.username?.trim().toLowerCase() === oc.value.author.trim().toLowerCase()
})

const activeIndex = ref(0)
const isEditOpen = ref(false)

const lightboxSrc = ref<string | null>(null)

function handleSave(draft: OcDraft) {
  if (!oc.value || !isOwner.value) return
  store.updateOc(oc.value.id, draft)
  activeIndex.value = 0
}

function openLightbox(src: string) {
  lightboxSrc.value = src
}

function closeLightbox() {
  lightboxSrc.value = null
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape') closeLightbox()
}

onMounted(() => {
  window.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown)
})
</script>

<template>
  <div class="detail-page">
    <AppHeader />

    <main v-if="oc" class="detail-page__content">
      <section class="hero">
        <div class="hero__gallery">
          <div class="hero__visual">
            <template v-if="oc.images[activeIndex]">
              <img
                  :src="oc.images[activeIndex]"
                  class="hero__visual-img"
                  role="button"
                  tabindex="0"
                  aria-label="Ampliar imagem"
                  @click="openLightbox(oc.images[activeIndex])"
                  @keydown.enter="openLightbox(oc.images[activeIndex])"
              />
            </template>
            <div v-else class="hero__placeholder" :class="`hero__placeholder--${oc.avatarPalette}`">
              <svg viewBox="0 0 100 100" role="img" aria-label="Avatar placeholder">
                <circle cx="50" cy="38" r="18" class="placeholder-shape" />
                <path d="M14 100c0-24 16-38 36-38s36 14 36 38" class="placeholder-shape" />
              </svg>
            </div>
          </div>
        </div>

        <aside class="hero__info">
          <h1 class="hero__name">{{ oc.name }}</h1>
          <p class="hero__author">por {{ oc.author }}</p>
          <div class="hero__divider" />

          <dl class="hero__facts">
            <div class="hero__fact">
              <dt>Espécie:</dt>
              <dd>{{ oc.especie || '—' }}</dd>
            </div>
            <div class="hero__fact">
              <dt>Sexo:</dt>
              <dd>{{ oc.sexo || '—' }}</dd>
            </div>
            <div class="hero__fact">
              <dt>Altura:</dt>
              <dd>{{ oc.altura || '—' }}</dd>
            </div>
            <div class="hero__fact">
              <dt>Características:</dt>
              <dd>{{ oc.caracteristicas || '—' }}</dd>
            </div>
            <div class="hero__fact hero__fact--block">
              <dt>Descrição:</dt>
              <dd>{{ oc.descricao || 'Sem descrição.' }}</dd>
            </div>
          </dl>

          <div v-if="isOwner" class="hero__actions">
            <button type="button" class="hero__edit" @click="isEditOpen = true">Editar</button>
          </div>
        </aside>
      </section>

      <section
        v-if="oc.images.length"
        class="gallery-section"
        >
        <h2 class="gallery-section__title">
            Galeria
        </h2>

        <div class="gallery-grid">
            <img
              v-for="(src, index) in oc.images"
              :key="src + index"
              :src="src"
              class="gallery-image"
              role="button"
              tabindex="0"
              aria-label="Ampliar imagem"
              @click="openLightbox(src)"
              @keydown.enter="openLightbox(src)"
            />
        </div>
        </section>
    </main>

    <main v-else class="detail-page__content">
      <p class="not-found">Essa ficha não foi encontrada.</p>
    </main>

    <EditOcModal v-if="oc && isOwner" v-model:open="isEditOpen" :oc="oc" @save="handleSave" />

    <Teleport to="body">
      <div
        v-if="lightboxSrc"
        class="lightbox"
        @click.self="closeLightbox"
      >
        <button
          type="button"
          class="lightbox__close"
          aria-label="Fechar"
          @click="closeLightbox"
        >
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <path d="M6 6l12 12M18 6L6 18" stroke="currentColor" stroke-width="2.4" stroke-linecap="round" />
          </svg>
        </button>
        <img :src="lightboxSrc" class="lightbox__img" @click.stop />
      </div>
    </Teleport>
  </div>
</template>

<style scoped>
.detail-page {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  isolation: isolate;
}

.detail-page::before {
  content: '';
  position: fixed;
  inset: 0;
  background-image: url('/background.jpg');
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  opacity: 0.05; /* ajuste esse valor até ficar do jeito que quiser */
  z-index: -1;
}

.detail-page__content {
  flex: 1;
  max-width: var(--max-content-width);
  width: 100%;
  margin: 0 auto;
  padding: 32px 32px 64px;
}

.hero {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 28px;
  align-items: start;
}

.hero__gallery {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.hero__visual {
  display: flex;
  justify-content: center;
  align-items: flex-start;

  overflow: visible;
}

.hero__visual-img {
  display: block;

  width: auto;
  height: auto;

  max-width: 100%;
  max-height: 1200px;

  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-card);

  object-fit: contain;
  cursor: pointer;
  transition: border-color var(--transition-fast);
}

.hero__visual-img:hover,
.hero__visual-img:focus-visible {
  border-color: var(--color-brand);
  outline: none;
}

.hero__placeholder {
  width: 100%;
  height: 100%;
  display: grid;
  place-items: center;
}

.placeholder-shape {
  fill: currentColor;
}

.hero__placeholder--1 {
  background: var(--avatar-1-bg);
  color: var(--avatar-1-fg);
}
.hero__placeholder--2 {
  background: var(--avatar-2-bg);
  color: var(--avatar-2-fg);
}
.hero__placeholder--3 {
  background: var(--avatar-3-bg);
  color: var(--avatar-3-fg);
}
.hero__placeholder--4 {
  background: var(--avatar-4-bg);
  color: var(--avatar-4-fg);
}
.hero__placeholder--5 {
  background: var(--avatar-5-bg);
  color: var(--avatar-5-fg);
}

.hero__placeholder svg {
  width: 40%;
  height: 40%;
}

.hero__info {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-card);
  padding: 28px 32px;
}

.hero__name {
  font-size: 30px;
}

.hero__author {
  margin: 2px 0 0;
  font-family: var(--font-hand);
  font-size: 22px;
  color: var(--color-text-muted);
}

.hero__divider {
  height: 1px;
  background: var(--color-border);
  margin: 16px 0 20px;
}

.hero__facts {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin: 0 0 24px;
}

.hero__fact {
  display: flex;
  gap: 6px;
  font-size: 15px;
}

.hero__fact--block {
  flex-direction: column;
  gap: 4px;
}

.hero__fact dt {
  font-weight: 700;
  color: var(--color-text);
}

.hero__fact dd {
  margin: 0;
  color: var(--color-text);
}

.hero__fact--block dd {
  color: var(--color-text-muted);
  line-height: 1.5;
}

.hero__actions {
  display: flex;
  justify-content: flex-end;
}

.hero__edit {
  border: 1px solid var(--color-border);
  background: var(--color-bg-elevated);
  color: var(--color-text);
  font-family: var(--font-display);
  font-weight: 600;
  font-size: 14px;
  padding: 10px 22px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition:
    border-color var(--transition-fast),
    background var(--transition-fast);
}

.hero__edit:hover {
  border-color: var(--color-brand);
  background: var(--color-brand-soft);
}

.not-found {
  text-align: center;
  padding: 80px 0;
  color: var(--color-text-muted);
}


.gallery-section {
  margin-top: 42px;
  width: 100%;
  margin-left: auto;
  margin-right: auto;
}

.gallery-section__title {
  margin-bottom: 18px;
  font-size: 24px;
}

.gallery-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.gallery-image {
  display: block;
  width: 32%;
  height: 100%;
  object-fit: cover;
  border-radius: var(--radius-md);
  border: 1px solid var(--color-border);
  box-shadow: var(--shadow-card);
  cursor: pointer;
  transition:
    transform .15s ease,
    border-color var(--transition-fast);
}

.gallery-image:hover,
.gallery-image:focus-visible {
  transform: translateY(-2px);
  border-color: var(--color-brand);
  outline: none;
}

/* ============================================================
   LIGHTBOX
============================================================ */

.lightbox {
  position: fixed;
  inset: 0;
  z-index: 1000;

  display: grid;
  place-items: center;

  background: rgba(3, 5, 12, .92);
  backdrop-filter: blur(2px);
  padding: 40px;
}

.lightbox__img {
  max-width: 92vw;
  max-height: 88vh;
  object-fit: contain;
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-popover);
}

.lightbox__close {
  position: fixed;
  top: 20px;
  right: 20px;

  display: grid;
  place-items: center;
  width: 44px;
  height: 44px;

  background: var(--color-surface-alt);
  border: 1px solid var(--color-border);
  border-radius: 50%;
  color: var(--color-text);
  cursor: pointer;

  transition:
    background var(--transition-fast),
    border-color var(--transition-fast);
}

.lightbox__close:hover {
  background: var(--color-brand);
  border-color: var(--color-brand);
  color: white;
}

.lightbox__close svg {
  width: 20px;
  height: 20px;
}

@media (max-width: 860px) {
  .hero {
    grid-template-columns: 1fr;
  }
}
</style>