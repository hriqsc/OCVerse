<script setup lang="ts">
import { computed } from 'vue'
import type { Oc, Rarity } from '@/types/oc'

const props = defineProps<{ oc: Oc }>()


const isFeatured = computed(() => Boolean(props.oc.emoji))
</script>

<template>
  <article class="oc-card" :class="`oc-card--${oc.rarity}`">
    <div class="oc-card__art">
      <div v-if="isFeatured" class="oc-card__illustration">
        <span class="oc-card__emoji" aria-hidden="true">{{ oc.emoji }}</span>
      </div>
      <div v-else class="oc-card__placeholder" :class="`oc-card__placeholder--${oc.avatarPalette}`">
        <svg viewBox="0 0 100 100" role="img" aria-label="Avatar placeholder">
          <circle cx="50" cy="38" r="18" class="placeholder-shape" />
          <path
            d="M14 100c0-24 16-38 36-38s36 14 36 38"
            class="placeholder-shape"
          />
        </svg>
      </div>
      <div class="oc-card__sheen" aria-hidden="true" />
    </div>

    <footer class="oc-card__info">
      <h3 class="oc-card__name">{{ oc.name }}</h3>
      <p class="oc-card__author">{{ oc.author }}</p>
    </footer>
  </article>
</template>

<style scoped>
.oc-card {
  --rarity-color: var(--rarity-comum);
  position: relative;
  border-radius: var(--radius-md);
  overflow: hidden;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  box-shadow: var(--shadow-card);
  transition:
    transform var(--transition-base),
    border-color var(--transition-base);
  isolation: isolate;
}

.oc-card:hover {
  transform: translateY(-4px);
  border-color: var(--rarity-color);
}

.oc-card::before {
  content: '';
  position: absolute;
  inset: 0 0 auto 0;
  height: 3px;
  background: var(--rarity-color);
}

.oc-card__ribbon {
  position: absolute;
  top: 10px;
  right: -34px;
  transform: rotate(40deg);
  background: var(--rarity-color);
  color: #0a0a0d;
  font-family: var(--font-mono);
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.04em;
  padding: 3px 40px;
  z-index: 2;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.35);
}

.oc-card__art {
  position: relative;
  aspect-ratio: 4 / 3;
  overflow: hidden;
}

.oc-card__placeholder {
  width: 100%;
  height: 100%;
  display: grid;
  place-items: center;
}

.placeholder-shape {
  fill: currentColor;
}

.oc-card__placeholder--1 {
  background: var(--avatar-1-bg);
  color: var(--avatar-1-fg);
}
.oc-card__placeholder--2 {
  background: var(--avatar-2-bg);
  color: var(--avatar-2-fg);
}
.oc-card__placeholder--3 {
  background: var(--avatar-3-bg);
  color: var(--avatar-3-fg);
}
.oc-card__placeholder--4 {
  background: var(--avatar-4-bg);
  color: var(--avatar-4-fg);
}
.oc-card__placeholder--5 {
  background: var(--avatar-5-bg);
  color: var(--avatar-5-fg);
}

.oc-card__placeholder svg {
  width: 46%;
  height: 46%;
}

.oc-card__illustration {
  width: 100%;
  height: 100%;
  display: grid;
  place-items: center;
  background: radial-gradient(circle at 50% 30%, #2c3a68 0%, #141824 75%);
}

.oc-card__emoji {
  font-size: 68px;
  filter: drop-shadow(0 6px 10px rgba(0, 0, 0, 0.4));
}

/* brilho "holográfico" que passa no hover, assinatura visual dos cards */
.oc-card__sheen {
  position: absolute;
  inset: 0;
  background: linear-gradient(
    115deg,
    transparent 20%,
    rgba(255, 255, 255, 0.16) 35%,
    rgba(255, 255, 255, 0.02) 45%,
    transparent 60%
  );
  transform: translateX(-120%);
  transition: transform 650ms ease;
  pointer-events: none;
}

.oc-card:hover .oc-card__sheen {
  transform: translateX(120%);
}

.oc-card__info {
  padding: 14px 16px 16px;
  background: var(--color-surface);
  border-top: 1px solid var(--color-border-soft);
}

.oc-card__name {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text);
  margin-bottom: 2px;
}

.oc-card__author {
  margin: 0;
  font-family: var(--font-mono);
  font-size: 12px;
  color: var(--color-text-muted);
}
</style>
