import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

export const useUserStore = defineStore('user', () => {
  const username = ref<string | null>(null)

  const isLoggedIn = computed(() => username.value !== null)

  /** Mock: no backend yet, just stores the name entered in the login popup. */
  function login(name: string) {
    username.value = name
  }

  function logout() {
    username.value = null
  }

  return { username, isLoggedIn, login, logout }
})
