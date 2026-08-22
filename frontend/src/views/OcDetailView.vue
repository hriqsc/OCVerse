<script setup lang="ts">
//OcDetailView.vue
import { computed, onMounted, onUnmounted, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import AppHeader from '@/components/layout/AppHeader.vue'
import EditOcModal from '@/components/oc/EditOcModal.vue'
import { useOcStore } from '@/stores/oc'
import { useUserStore } from '@/stores/user'
import type { Oc, OcDraft } from '@/types/oc'

const route = useRoute()
const store = useOcStore()
const userStore = useUserStore()

const oc = ref<Oc | undefined>(undefined)
const isLoading = ref(true)

async function loadOc() {
  const id = Number(route.params.id)
  if (Number.isNaN(id)) {
    oc.value = undefined
    isLoading.value = false
    return
  }
  isLoading.value = true
  oc.value = await store.getById(id)
  isLoading.value = false
}

onMounted(loadOc)
watch(() => route.params.id, loadOc)

// the backend doesn't send an avatarPalette, so derive a stable 1-5 value
// from the OC's id, same as OcCard
const avatarPalette = computed(() => (oc.value ? (oc.value.id % 5) + 1 : 1))

// só é dono se estiver logado E o nickname bater com o autor do OC
const isOwner = computed(() => {
  if (!userStore.isLoggedIn || !oc.value) return false
  return userStore.username?.trim().toLowerCase() === oc.value.creator_user_name.trim().toLowerCase()
})

const activeIndex = ref(0)
const isEditOpen = ref(false)

const lightboxSrc = ref<string | null>(null)
async function handleSaved() {
  if (!oc.value) return
  oc.value = await store.getById(oc.value.id, true)
  activeIndex.value = 0
}

function openLightbox(src: string) {
  lightboxSrc.value = src
}

function closeLightbox() {
  lightboxSrc.value = null
}

async function confirmDelete() {
  if (!oc.value) return
  if (!window.confirm(`Tem certeza que deseja excluir "${oc.value.oc_name}"? Essa ação não pode ser desfeita.`)) return
  const result = await store.deleteOc(oc.value!.id)
  
  if (result.success) {
    window.location.href = '/hub'
  }else{
    console.error(result.error)
  }
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

function WriteHeight(height: string | undefined): string {
    if (!height) return "Indefinida";
    return `${height[0]},${height[1]}${height[2]} M`;
}
</script>

<template>
  <div class="detail-page">
    <AppHeader />

    <main v-if="isLoading" class="detail-page__content">
      <p class="not-found">Carregando...</p>
    </main>

    <main v-else-if="oc" class="detail-page__content">
      <section class="oc">
        <div class="oc__gallery">
          <div class="oc__visual">
            <template v-if="oc.images[activeIndex]">
                <img
                    :src="oc.images[activeIndex]"
                    class="oc__visual-img"
                    role="button"
                    tabindex="0"
                    aria-label="Ampliar imagem"
                    @click="openLightbox(oc.images[activeIndex]!)"
                    @keydown.enter="openLightbox(oc.images[activeIndex]!)"
                />
            </template>
            <div v-else class="oc__placeholder" :class="`oc__placeholder--${avatarPalette}`">
              <svg viewBox="0 0 100 100" role="img" aria-label="Avatar placeholder">
                <circle cx="50" cy="38" r="18" class="placeholder-shape" />
                <path d="M14 100c0-24 16-38 36-38s36 14 36 38" class="placeholder-shape" />
              </svg>
            </div>
          </div>
        </div>

        <aside class="oc__info">
          <h1 class="oc__name">{{ oc.oc_name }}</h1>
          <p class="oc__author">por {{ oc.creator_user_name }}</p>
          <div class="oc__divider" />

          <dl class="oc__facts">
            <div class="oc__fact">
              <dt>Espécie:</dt>
              <dd>{{ oc.specie || 'desconhecida' }}</dd>
            </div>
            <div class="oc__fact">
              <dt>Sexo:</dt>
              <dd>{{ oc.sex === "M" ? "Masculino" : oc.sex === "F" ? "Feminino" : "Outro" }}</dd>
            </div>
            <div class="oc__fact">
              <dt>Altura:</dt>
              <dd>{{ WriteHeight(oc.height) }}</dd>
            </div>
            <div class="oc__fact oc__fact--block">
              <dt>Descrição:</dt>
              <dd>{{ oc.description || 'Sem descrição.' }}</dd>
            </div>
          </dl>

          <div v-if="isOwner" class="oc__actions">
            <button type="button" class="oc__btn" @click="isEditOpen = true">Editar</button>
            <button type="button" class="oc__btn" @click="confirmDelete">Excluir</button>
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

    <EditOcModal v-if="oc && isOwner" v-model:open="isEditOpen" :oc="oc" @saved="handleSaved" />

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
  opacity: 0.03;
  z-index: -1;
}

.detail-page__content {
  flex: 1;
  max-width: var(--max-content-width);
  width: 100%;
  margin: 0 auto;
  padding: 32px 32px 64px;
}

.oc {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 28px;
  align-items: start;
}

.oc__gallery {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.oc__visual {
  display: flex;
  justify-content: center;
  align-items: flex-start;

  overflow: visible;
}

.oc__visual-img {
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

.oc__visual-img:hover,
.oc__visual-img:focus-visible {
  border-color: var(--color-brand);
  outline: none;
}

.oc__placeholder {
  width: 100%;
  height: 100%;
  display: grid;
  place-items: center;
}

.placeholder-shape {
  fill: currentColor;
}

.oc__placeholder--1 {
  background: var(--avatar-1-bg);
  color: var(--avatar-1-fg);
}
.oc__placeholder--2 {
  background: var(--avatar-2-bg);
  color: var(--avatar-2-fg);
}
.oc__placeholder--3 {
  background: var(--avatar-3-bg);
  color: var(--avatar-3-fg);
}
.oc__placeholder--4 {
  background: var(--avatar-4-bg);
  color: var(--avatar-4-fg);
}
.oc__placeholder--5 {
  background: var(--avatar-5-bg);
  color: var(--avatar-5-fg);
}

.oc__placeholder svg {
  width: 40%;
  height: 40%;
}

.oc__info {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-card);
  padding: 28px 32px;
}

.oc__name {
  font-size: 30px;
}

.oc__author {
  margin: 2px 0 0;
  font-family: var(--font-hand);
  font-size: 22px;
  color: var(--color-text-muted);
}

.oc__divider {
  height: 1px;
  background: var(--color-border);
  margin: 16px 0 20px;
}

.oc__facts {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin: 0 0 24px;
}

.oc__fact {
  display: flex;
  gap: 6px;
  font-size: 15px;
}

.oc__fact--block {
  flex-direction: column;
  gap: 4px;
}

.oc__fact dt {
  font-weight: 700;
  color: var(--color-text);
}

.oc__fact dd {
  margin: 0;
  color: var(--color-text);
}

.oc__fact--block dd {
  color: var(--color-text-muted);
  line-height: 1.5;
}

.oc__actions {
  display: flex;
  justify-content: flex-end;
}

.oc__actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.oc__btn {
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

.oc__btn:hover {
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
  .oc {
    grid-template-columns: 1fr;
  }
}
</style>