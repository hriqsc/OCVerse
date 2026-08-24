import { ref } from 'vue'
import { defineStore } from 'pinia'

export const useAuthModalStore = defineStore('authModal', () => {
  const isOpen = ref(false)
  /** 'login' | 'registrar' — controls which popup tab opens first */
  const initialTab = ref<'login' | 'registrar'>('login')

  function open(tab: 'login' | 'registrar' = 'login') {
    initialTab.value = tab
    isOpen.value = true
  }

  function close() {
    isOpen.value = false
  }

  return { isOpen, initialTab, open, close }
})
