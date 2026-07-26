<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from 'vue'
import { useAuthModalStore } from '@/stores/authModal'

const store = useAuthModalStore()
const email = ref('')
const password = ref('')

function submit() {
  // mock: sem backend ainda, só fecha o popup
  store.close()
  email.value = ''
  password.value = ''
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Escape' && store.isOpen) store.close()
}

onMounted(() => window.addEventListener('keydown', handleKeydown))
onUnmounted(() => window.removeEventListener('keydown', handleKeydown))

watch(
  () => store.isOpen,
  (open) => {
    document.body.style.overflow = open ? 'hidden' : ''
  },
)
</script>

<template>
  <Teleport to="body">
    <Transition name="modal-fade">
      <div v-if="store.isOpen" class="modal-backdrop" @mousedown.self="store.close()">
        <div class="modal-card" role="dialog" aria-modal="true">
          <button type="button" class="modal-card__close" aria-label="Fechar" @click="store.close()">
            ✕
          </button>

          <div class="modal-card__tabs">
            <button
              type="button"
              class="modal-card__tab"
              :class="{ 'modal-card__tab--active': store.initialTab === 'login' }"
              @click="store.initialTab = 'login'"
            >
              Logar
            </button>
            <button
              type="button"
              class="modal-card__tab"
              :class="{ 'modal-card__tab--active': store.initialTab === 'registrar' }"
              @click="store.initialTab = 'registrar'"
            >
              Registrar
            </button>
          </div>

          <h2 class="modal-card__title">
            {{ store.initialTab === 'login' ? 'Bem-vindo de volta' : 'Crie sua conta' }}
          </h2>
          <p class="modal-card__subtitle">
            {{
              store.initialTab === 'login'
                ? 'Entre para enviar e favoritar fichas de OC.'
                : 'Cadastre-se para publicar suas fichas no hub.'
            }}
          </p>

          <form class="modal-card__form" @submit.prevent="submit">
            <label class="modal-card__field">
              <span>E-mail</span>
              <input v-model="email" type="email" required placeholder="voce@email.com" />
            </label>
            <label class="modal-card__field">
              <span>Senha</span>
              <input v-model="password" type="password" required placeholder="••••••••" />
            </label>

            <button type="submit" class="modal-card__submit">
              {{ store.initialTab === 'login' ? 'Entrar' : 'Registrar' }}
            </button>
          </form>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(6, 6, 9, 0.72);
  backdrop-filter: blur(3px);
  display: grid;
  place-items: center;
  z-index: 100;
  padding: 20px;
}

.modal-card {
  position: relative;
  width: 100%;
  max-width: 380px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-popover);
  padding: 28px;
}

.modal-card__close {
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

.modal-card__close:hover {
  background: var(--color-surface-alt);
  color: var(--color-text);
}

.modal-card__tabs {
  display: flex;
  gap: 6px;
  background: var(--color-surface-alt);
  padding: 4px;
  border-radius: var(--radius-sm);
  margin-bottom: 20px;
}

.modal-card__tab {
  flex: 1;
  border: none;
  background: transparent;
  color: var(--color-text-muted);
  font-weight: 600;
  font-size: 14px;
  padding: 8px 0;
  border-radius: calc(var(--radius-sm) - 2px);
  cursor: pointer;
  transition: background var(--transition-fast), color var(--transition-fast);
}

.modal-card__tab--active {
  background: var(--color-brand);
  color: var(--color-text-on-accent);
}

.modal-card__title {
  font-size: 22px;
  margin-bottom: 4px;
}

.modal-card__subtitle {
  margin: 0 0 20px;
  color: var(--color-text-muted);
  font-size: 14px;
}

.modal-card__form {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.modal-card__field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-size: 13px;
  color: var(--color-text-muted);
}

.modal-card__field input {
  border: 1px solid var(--color-border);
  background: var(--color-bg-elevated);
  border-radius: var(--radius-sm);
  padding: 10px 12px;
  color: var(--color-text);
  font-size: 15px;
  outline: none;
  transition: border-color var(--transition-fast);
}

.modal-card__field input:focus {
  border-color: var(--color-brand);
}

.modal-card__submit {
  margin-top: 6px;
  border: none;
  background: var(--color-brand);
  color: var(--color-text-on-accent);
  font-weight: 700;
  font-size: 15px;
  padding: 12px 0;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background var(--transition-fast);
}

.modal-card__submit:hover {
  background: var(--color-brand-strong);
}

.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity var(--transition-base);
}
.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}
.modal-fade-enter-active .modal-card,
.modal-fade-leave-active .modal-card {
  transition: transform var(--transition-base);
}
.modal-fade-enter-from .modal-card,
.modal-fade-leave-to .modal-card {
  transform: scale(0.96) translateY(8px);
}
</style>
