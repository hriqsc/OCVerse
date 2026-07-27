<script setup lang="ts">
// Reusable loading state, styled to match the binder/corkboard theme.
// Use it for the login screen and for image upload feedback:
//
//   <LoadingSpinner label="Entrando…" />
//   <LoadingSpinner label="Enviando imagem…" size="sm" />
withDefaults(
  defineProps<{
    label?: string
    size?: 'sm' | 'md' | 'lg'
  }>(),
  { label: 'Carregando…', size: 'md' },
)
</script>

<template>
  <div class="oc-loader" :class="`oc-loader--${size}`" role="status" :aria-label="label">
    <span class="oc-loader__ring" aria-hidden="true">
      <span class="oc-loader__dot oc-loader__dot--1" />
      <span class="oc-loader__dot oc-loader__dot--2" />
      <span class="oc-loader__dot oc-loader__dot--3" />
      <span class="oc-loader__pencil" />
    </span>
    <p v-if="label" class="oc-loader__label">{{ label }}</p>
  </div>
</template>

<style scoped>
.oc-loader {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 14px;
  padding: 8px;
}

.oc-loader__ring {
  position: relative;
  display: grid;
  place-items: center;
}

.oc-loader--sm .oc-loader__ring {
  width: 32px;
  height: 32px;
}
.oc-loader--md .oc-loader__ring {
  width: 52px;
  height: 52px;
}
.oc-loader--lg .oc-loader__ring {
  width: 76px;
  height: 76px;
}

/* three washi-tape colored dots orbiting like a hand-drawn spinner */
.oc-loader__dot {
  position: absolute;
  top: 0;
  left: 50%;
  width: 22%;
  height: 22%;
  margin-left: -11%;
  border-radius: 50%;
  transform-origin: 50% 250%;
  animation: oc-orbit 1.1s linear infinite;
}

.oc-loader__dot--1 {
  background: var(--tape-1);
  animation-delay: 0s;
}
.oc-loader__dot--2 {
  background: var(--tape-3);
  animation-delay: -0.37s;
}
.oc-loader__dot--3 {
  background: var(--tape-2);
  animation-delay: -0.74s;
}

/* small pencil tip that spins in the center, nodding to the sketch/binder theme */
.oc-loader__pencil {
  width: 34%;
  height: 34%;
  border-radius: 3px 3px 40% 40%;
  background: linear-gradient(180deg, var(--tape-4) 0%, var(--tape-4) 70%, var(--color-text-faint) 70%);
  animation: oc-wobble 1.6s ease-in-out infinite;
}

.oc-loader__label {
  margin: 0;
  font-family: var(--font-hand);
  font-size: 19px;
  color: var(--color-text-muted);
}

@keyframes oc-orbit {
  from {
    transform: rotate(0deg) translateY(0);
  }
  to {
    transform: rotate(360deg) translateY(0);
  }
}

@keyframes oc-wobble {
  0%,
  100% {
    transform: rotate(-8deg);
  }
  50% {
    transform: rotate(8deg);
  }
}

@media (prefers-reduced-motion: reduce) {
  .oc-loader__dot,
  .oc-loader__pencil {
    animation: none;
  }
}
</style>