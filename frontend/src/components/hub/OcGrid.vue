<script setup lang="ts">
import type { Oc } from '@/types/oc'
import OcCard from '@/components/hub/OcCard.vue'

defineProps<{ items: Oc[] }>()
</script>

<template>
  <TransitionGroup v-if="items.length" tag="div" name="oc-grid" class="oc-grid">
    <OcCard
      v-for="(oc, index) in items"
      :key="oc.id"
      :oc="oc"
      :style="{ '--stagger': `${Math.min(index, 12) * 35}ms` }"
    />
  </TransitionGroup>
  <Transition v-else name="oc-grid-empty" appear>
    <div class="oc-grid__empty">
      <p>Nenhuma ficha encontrada com esses filtros.</p>
    </div>
  </Transition>
</template>

<style scoped>
.oc-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(190px, 1fr));
  gap: 24px 20px;
}

/* stagger each card's own entrance animation (defined in OcCard.vue) */
.oc-grid :deep(.oc-card) {
  animation-delay: var(--stagger, 0ms);
}

/* removal transition, e.g. when filtering the board */
.oc-grid-move,
.oc-grid-leave-active {
  transition:
    opacity var(--transition-base),
    transform var(--transition-base);
}
.oc-grid-leave-active {
  position: absolute;
}
.oc-grid-leave-to {
  opacity: 0;
  transform: scale(0.94);
}

.oc-grid__empty {
  padding: 80px 0;
  text-align: center;
  color: var(--color-text-muted);
  font-size: 15px;
}

.oc-grid-empty-enter-active {
  transition:
    opacity var(--transition-slow),
    transform var(--transition-slow);
}
.oc-grid-empty-enter-from {
  opacity: 0;
  transform: translateY(6px);
}
</style>