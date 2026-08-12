<script setup lang="ts">
import { computed } from 'vue'
import type { PostMinified } from '@/types/oc'

const props = defineProps<{ oc: PostMinified }>()

const avatarPalette = computed(() => (props.oc.id % 5) + 1)

const tapeRotation = computed(() => [-6, -3, 4, 2, -4][(avatarPalette.value - 1) % 5])

const tapeStyle = computed(() => ({
  '--tape-color': `var(--tape-${avatarPalette.value})`,
}))

const placeholderStyle = computed(() => ({
  background: `var(--avatar-${avatarPalette.value}-bg)`,
  color: `var(--avatar-${avatarPalette.value}-fg)`,
}))
</script>

<template>
  <router-link
    :to="`/hub/oc/${oc.id}`"
    class="oc-card"
    :style="tapeStyle"
  >
    <span
      class="oc-card__tape"
      :style="{ transform: `translateX(-50%) rotate(${tapeRotation}deg)` }"
      aria-hidden="true"
    />

    <span class="oc-card__punch" aria-hidden="true" />

    <div class="oc-card__art">
      <img
        v-if="oc.thumb"
        :src="oc.thumb"
        alt=""
        class="oc-card__image"
      />

      <div
        v-else
        class="oc-card__placeholder"
        :style="placeholderStyle"
      >
        <svg viewBox="0 0 100 100" role="img" aria-label="Avatar placeholder">
          <circle cx="50" cy="38" r="18" class="placeholder-shape" />
          <path
            d="M14 100c0-24 16-38 36-38s36 14 36 38"
            class="placeholder-shape"
          />
        </svg>
      </div>
    </div>

    <footer class="oc-card__info">
      <h3 class="oc-card__name">{{ oc.oc_name }}</h3>
      <p class="oc-card__author">por {{ oc.creator_user_name }}</p>
    </footer>
  </router-link>
</template>

<style scoped>
.oc-card {
  --tape-color: var(--tape-1);

  position: relative;
  display: block;
  overflow: visible;

  color: inherit;
  text-decoration: none;

  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-card);

  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);

  transition:
    transform var(--transition-base, .3s ease),
    opacity var(--transition-base, .3s ease),
    box-shadow var(--transition-base, .3s ease);
}

.oc-card:hover {
  transform: translateY(-7px) rotate(-0.9deg);

  opacity: 1;
  box-shadow:
    0 10px 0 rgba(50, 40, 65, 0.05),
    0 18px 30px 5px rgba(50, 32, 67, 0.32);
}


.oc-card__tape,
.oc-card__punch {
  position: absolute;
  z-index: 2;
}

.oc-card__tape {
  top: -10px;
  left: 50%;
  width: 64px;
  height: 22px;

  background: var(--tape-color);
  border-radius: 2px;
  opacity: .85;

  box-shadow: 0 2px 4px rgb(44 36 23 / .2);
}

.oc-card__punch {
  top: 12px;
  left: 12px;

  width: 10px;
  aspect-ratio: 1;

  border-radius: 50%;
  background: var(--color-bg);
  box-shadow: inset 0 1px 2px rgb(44 36 23 / .35);
}

.oc-card__art {
  aspect-ratio: 4 / 3;
  overflow: hidden;
  border-radius: var(--radius-md) var(--radius-md) 0 0;
}

.oc-card__image,
.oc-card__placeholder {
  width: 100%;
  height: 100%;
}

.oc-card__image {
  display: block;
  object-fit: cover;
}

.oc-card__placeholder {
  display: grid;
  place-items: center;
}

.oc-card__placeholder svg {
  width: 46%;
  height: 46%;
}

.placeholder-shape {
  fill: currentColor;
}

.oc-card__info {
  padding: 16px 16px 18px;
  border-top: 1px solid var(--color-border-soft);
}

.oc-card__name {
  margin: 0 0 2px;

  font: 600 17px var(--font-display);
  color: var(--color-text);
}

.oc-card__author {
  margin: 0;

  font: 18px/1 var(--font-hand);
  color: var(--color-text-muted);
}
</style>