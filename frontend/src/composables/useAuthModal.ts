import { useAuthModalStore } from '@/stores/authModal'

/**
 * useAuthModal
 * ------------
 * Use it from any component/page, no need to import <LoginModal /> manually —
 * it's already mounted once in App.vue:
 *
 *   const { openLogin, openRegister, closeAuthModal } = useAuthModal()
 *   openLogin() // opens the popup on the login tab
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
