<script setup lang="ts">
defineProps<{ open: boolean; images: string[] }>()
const emit = defineEmits<{ 'update:open': [value: boolean]; select: [index: number] }>()

function close() {
  emit('update:open', false)
}

function pick(index: number) {
  emit('select', index)
  close()
}
</script>

<template>
  <Teleport to="body">
    <Transition name="lightbox-fade">
      <div v-if="open" class="lightbox-backdrop" @mousedown.self="close">
        <div class="lightbox">
          <button type="button" class="lightbox__close" aria-label="Fechar" @click="close">
            ✕
          </button>
          <h2 class="lightbox__title">Todas as imagens</h2>
          <div class="lightbox__grid">
            <button
              v-for="(src, index) in images"
              :key="src + index"
              type="button"
              class="lightbox__tile"
              @click="pick(index)"
            >
              <img :src="src" alt="" />
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.lightbox-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(44, 36, 23, 0.6);
  backdrop-filter: blur(3px);
  display: grid;
  place-items: center;
  z-index: 110;
  padding: 24px;
}

.lightbox {
  position: relative;
  width: 100%;
  max-width: 900px;
  max-height: 85vh;
  overflow-y: auto;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-popover);
  padding: 28px 32px;
}

.lightbox__close {
  position: absolute;
  top: 16px;
  right: 16px;
  border: none;
  background: transparent;
  color: var(--color-text-muted);
  font-size: 15px;
  cursor: pointer;
  padding: 6px;
  border-radius: var(--radius-sm);
}

.lightbox__close:hover {
  background: var(--color-surface-alt);
  color: var(--color-text);
}

.lightbox__title {
  font-size: 22px;
  margin-bottom: 18px;
}

.lightbox__grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 16px;
}

.lightbox__tile {
  aspect-ratio: 4 / 3;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  overflow: hidden;
  padding: 0;
  cursor: pointer;
  background: none;
  transition: transform var(--transition-fast);
}

.lightbox__tile:hover {
  transform: translateY(-2px);
}

.lightbox__tile img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.lightbox-fade-enter-active,
.lightbox-fade-leave-active {
  transition: opacity var(--transition-base);
}
.lightbox-fade-enter-from,
.lightbox-fade-leave-to {
  opacity: 0;
}
</style>
