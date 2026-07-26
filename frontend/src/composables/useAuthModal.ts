import { useAuthModalStore } from '@/stores/authModal'

/**
 * useAuthModal
 * ------------
 * Uso em qualquer componente/página, sem precisar importar o <LoginModal />
 * manualmente — ele já fica montado uma única vez no App.vue:
 *
 *   const { openLogin, openRegister, closeAuthModal } = useAuthModal()
 *   openLogin() // abre o popup na aba de login
 */
export function useAuthModal() {
  const store = useAuthModalStore()

  function openLogin() {
    store.open('login')
  }

  function openRegister() {
    store.open('registrar')
  }

  function closeAuthModal() {
    store.close()
  }

  return { openLogin, openRegister, closeAuthModal }
}
