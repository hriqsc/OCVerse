<script setup lang="ts">
// ResetPasswordView.vue
import { computed, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import AppHeader from '@/components/layout/AppHeader.vue'

const route = useRoute()
const router = useRouter()

const id = computed(() => Number(route.params.id))
const isValidId = computed(() => !Number.isNaN(id.value))

const checkingId = ref(true)

const newPassword = ref('')

const loading = ref(false)
const errorMessage = ref('')
const successMessage = ref('')

async function checkIdExists() {
  if (!isValidId.value) {
    router.replace('/404')
    return
  }

  try {
    const response = await fetch(`/api/v1/user/reset/${id.value}`, {
      method: 'GET',
    })

    if (!response.ok) {
      router.replace('/404')
      return
    }
  } catch {
    router.replace('/404')
    return
  } finally {
    checkingId.value = false
  }
}

onMounted(checkIdExists)

async function handleSubmit() {
  errorMessage.value = ''
  successMessage.value = ''

  loading.value = true

  try {
    const response = await fetch('/api/v1/user/reset', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        id: id.value,
        new_password: newPassword.value,
      }),
    })

    if (!response.ok) {
      const data = await response.json().catch(() => null)
      throw new Error(data?.message || 'Não foi possível resetar a senha.')
    }

    successMessage.value = 'Senha resetada com sucesso. Você já pode entrar com a nova senha.'
    newPassword.value = ''

    setTimeout(() => {
      router.push('/hub')
    }, 1800)
  } catch (err) {
    errorMessage.value = err instanceof Error ? err.message : 'Erro ao conectar com o servidor.'
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="reset-page">
    <AppHeader />

    <main v-if="checkingId" class="reset-page__content">
      <p class="not-found">Verificando...</p>
    </main>

    <main v-else class="reset-page__content">
      <section class="reset">
        <span class="reset__tape reset__tape--a" aria-hidden="true"></span>
        <span class="reset__tape reset__tape--b" aria-hidden="true"></span>

        <p class="reset__eyebrow">// recuperação de acesso</p>
        <h1 class="reset__title">Resetar login</h1>
        <p class="reset__hint">Digite a nova senha.</p>

        <form class="reset__form" @submit.prevent="handleSubmit" novalidate>
          <label class="reset__field">
            <span class="reset__label">Nova senha</span>
            <input
              v-model="newPassword"
              type="password"
              name="new_password"
              autocomplete="new-password"
              placeholder="••••••••"
              :disabled="loading"
              required
              minlength="8"
            />
          </label>

          <p v-if="errorMessage" class="reset__feedback reset__feedback--danger" role="alert">
            {{ errorMessage }}
          </p>
          <p v-if="successMessage" class="reset__feedback reset__feedback--success" role="status">
            {{ successMessage }}
          </p>

          <div class="reset__actions">
            <button type="submit" class="reset__btn reset__btn--primary" :disabled="loading">
              {{ loading ? 'Enviando...' : 'Resetar senha' }}
            </button>
            <RouterLink to="/login" class="reset__btn reset__btn--ghost">Voltar</RouterLink>
          </div>
        </form>
      </section>
    </main>
  </div>
</template>

<style scoped>
.reset-page {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  isolation: isolate;
}

.reset-page::before {
  content: '';
  position: fixed;
  inset: 0;
  background-image: url('/background.jpg');
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  opacity: 0.03;
  z-index: -1;
}

.reset-page__content {
  flex: 1;
  display: grid;
  place-items: center;
  max-width: var(--max-content-width);
  width: 100%;
  margin: 0 auto;
  padding: 32px;
}

.not-found {
  text-align: center;
  padding: 80px 0;
  color: var(--color-text-muted);
}

.reset {
  position: relative;
  width: 100%;
  max-width: 440px;

  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-card);

  padding: 40px 36px 34px;
}

.reset__tape {
  position: absolute;
  top: -14px;
  width: 70px;
  height: 26px;
  opacity: 0.85;
  border-radius: 3px;
  box-shadow: 0 4px 10px rgba(0, 0, 0, 0.25);
}

.reset__tape--a {
  left: 32px;
  transform: rotate(-6deg);
  background: var(--tape-3);
}

.reset__tape--b {
  right: 28px;
  transform: rotate(8deg);
  background: var(--tape-4);
}

.reset__eyebrow {
  margin: 0 0 4px;
  font-family: var(--font-hand);
  font-size: 20px;
  color: var(--color-secondary);
}

.reset__title {
  font-size: 28px;
}

.reset__hint {
  margin: 8px 0 26px;
  color: var(--color-text-muted);
  font-size: 15px;
  line-height: 1.5;
}

.reset__form {
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.reset__field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.reset__label {
  font-size: 13px;
  font-weight: 700;
  color: var(--color-text-muted);
}

.reset__field input {
  width: 100%;
  font-size: 15px;
}

.reset__feedback {
  margin: -6px 0 0;
  font-size: 14px;
  padding: 10px 14px;
  border-radius: var(--radius-sm);
}

.reset__feedback--danger {
  color: var(--color-danger);
  background: rgba(232, 91, 91, 0.12);
  border: 1px solid rgba(232, 91, 91, 0.35);
}

.reset__feedback--success {
  color: var(--color-success);
  background: rgba(114, 214, 177, 0.12);
  border: 1px solid rgba(114, 214, 177, 0.35);
}

.reset__actions {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-top: 4px;
}

.reset__btn {
  border: 1px solid var(--color-border);
  font-family: var(--font-display);
  font-weight: 600;
  font-size: 14px;
  padding: 11px 24px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  text-align: center;
  transition:
    border-color var(--transition-fast),
    background var(--transition-fast),
    color var(--transition-fast);
}

.reset__btn--primary {
  flex: 1;
  background: var(--color-brand-strong);
  border-color: var(--color-brand-strong);
  color: var(--color-text-on-accent);
}

.reset__btn--primary:hover:not(:disabled) {
  background: var(--color-brand);
  border-color: var(--color-brand);
}

.reset__btn--primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.reset__btn--ghost {
  background: var(--color-bg-elevated);
  color: var(--color-text);
}

.reset__btn--ghost:hover {
  border-color: var(--color-brand);
  background: var(--color-brand-soft);
}

@media (max-width: 480px) {
  .reset {
    padding: 32px 24px 28px;
  }
}
</style>