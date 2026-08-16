<script setup lang="ts">
import { useAuthModal } from '@/composables/useAuthModal'
import { useUserStore } from '@/stores/user'

const { openLogin, openRegister } = useAuthModal()
const userStore = useUserStore()
</script>

<template>
  <header class="app-header">
    <div class="app-header__inner">
      <router-link to="/hub" class="brand">
        <img src="/logo.png" alt="OC Verse logo" class="brand__mark" />
        <span class="brand__text">
          <span class="brand__name">OC Verse</span>
        </span>
      </router-link>

      <nav class="app-header__actions" aria-label="Conta">
        <template v-if="userStore.isLoggedIn">
          <span class="app-header__user">{{ userStore.username }}</span>
          <button type="button" class="link-btn" @click="userStore.logout()">Sair</button>
        </template>
        <template v-else>
          <button type="button" class="link-btn" @click="openLogin">Entrar</button>
          <button type="button" class="link-btn link-btn--accent" @click="openRegister">
            Registrar
          </button>
        </template>
      </nav>
    </div>
  </header>
</template>

<style scoped>
.app-header {
  height: var(--header-height);
  border-bottom: 1px solid var(--color-border-soft);
  background: var(--color-bg-elevated);
}

.app-header__inner {
  max-width: var(--max-content-width);
  height: 100%;
  margin: 0 auto;
  padding: 0 32px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
}

.brand {
  display: flex;
  align-items: center;
  gap: 14px;
  text-decoration: none;
  color: inherit;
}

.brand__mark {
  width: 52px;
  height: 52px;
  display: block;
}

.brand__text {
  display: flex;
  flex-direction: column;
  line-height: 1.15;
}

.brand__name {
  font-family: var(--font-display);
  font-weight: 700;
  font-size: 26px;
  color: var(--color-brand);
  letter-spacing: 0.2px;
}

.brand__tagline {
  font-family: var(--font-body);
  font-weight: 500;
  font-size: 12px;
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.08em;
}

.app-header__user {
  font-family: var(--font-hand);
  font-size: 20px;
  color: var(--color-brand-strong);
  padding: 0 6px;
}

.app-header__actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.link-btn {
  border: none;
  background: transparent;
  color: var(--color-text);
  font-weight: 600;
  font-size: 15px;
  padding: 10px 18px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background var(--transition-fast);
}

.link-btn:hover {
  background: var(--color-surface-alt);
}

.link-btn--accent {
  color: var(--color-text-on-accent);
  background: var(--color-brand);
}

.link-btn--accent:hover {
  background: var(--color-brand-strong);
}

@media (max-width: 640px) {
  .app-header__inner {
    padding: 0 18px;
  }
  .brand__tagline {
    display: none;
  }
}
</style>
