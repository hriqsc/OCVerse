<script setup lang="ts">
import { computed, ref } from 'vue'
import { useRoute } from 'vue-router'
import AppHeader from '@/components/layout/AppHeader.vue'
import EditOcModal from '@/components/oc/EditOcModal.vue'
import { useOcStore } from '@/stores/oc'
import type { OcDraft } from '@/types/oc'

const route = useRoute()
const store = useOcStore()

const oc = computed(() => store.getById(String(route.params.id)))

const activeIndex = ref(0)
const isEditOpen = ref(false)

function selectImage(index: number) {
  activeIndex.value = index
}

function handleSave(draft: OcDraft) {
  if (!oc.value) return
  store.updateOc(oc.value.id, draft)
  activeIndex.value = 0
}
</script>

<template>
  <div class="detail-page">
    <AppHeader />

    <main v-if="oc" class="detail-page__content">
      <section class="hero">
        <div class="hero__gallery">
          <div class="hero__visual">
            <img
                v-if="oc.images[activeIndex]"
                :src="oc.images[activeIndex]"
                class="hero__visual-img"
                @click="isLightboxOpen = true"
            />
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

          <div class="hero__actions">
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
            />
        </div>
        </section>
    </main>

    <main v-else class="detail-page__content">
      <p class="not-found">Essa ficha não foi encontrada.</p>
    </main>

    <EditOcModal v-if="oc" v-model:open="isEditOpen" :oc="oc" @save="handleSave" />
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
  aspect-ratio: 4 / 5;
  max-height: 600px;
  width: 100%;
  margin: 0 auto;
  overflow: hidden;
  border-radius: var(--radius-lg);
}

.hero__visual-img {
  width: 100%;
  height: 100%;
  /* object-fit: contain; */
  display: block;
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
  width: 32%;
  height: 100%;
  object-fit: cover;
  border-radius: var(--radius-md);
  border: 1px solid var(--color-border);
  box-shadow: var(--shadow-card);
  transition: transform .15s ease;
}

@media (max-width: 860px) {
  .hero {
    grid-template-columns: 1fr;
  }
}
</style>
