<script setup lang="ts">
import type { Oc } from '@/types/oc'
import OcCard from '@/components/hub/OcCard.vue'

defineProps<{ items: Oc[] }>()
</script>

<template>
  <div v-if="items.length" class="oc-grid">
    <OcCard v-for="oc in items" :key="oc.id" :oc="oc" />
  </div>
  <div v-else class="oc-grid__empty">
    <p>Nenhuma ficha encontrada com esses filtros.</p>
  </div>
</template>

<style scoped>
.oc-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(190px, 1fr));
  gap: 24px 20px;
}

.oc-grid__empty {
  padding: 80px 0;
  text-align: center;
  color: var(--color-text-muted);
  font-size: 15px;
}
</style>
