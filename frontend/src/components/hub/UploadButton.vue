<script setup lang="ts">
const emit = defineEmits<{ click: [] }>()
</script>

<template>
  <button type="button" class="upload-btn" @click="emit('click')">
    <span class="upload-btn__icon" aria-hidden="true">＋</span>
    Upload
  </button>
</template>

<style scoped>
.upload-btn {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 11px 20px;
  border-radius: var(--radius-sm);
  border: 1px solid var(--color-border);
  background: var(--color-surface);
  color: var(--color-text);
  font-weight: 600;
  font-size: 15px;
  cursor: pointer;
  transition:
    border-color var(--transition-fast),
    transform var(--transition-fast),
    background var(--transition-fast);
}

.upload-btn:hover {
  border-color: var(--color-brand);
  background: var(--color-surface-alt);
  transform: translateY(-1px);
}

.upload-btn__icon {
  color: var(--color-brand);
  font-weight: 700;
}
</style>
