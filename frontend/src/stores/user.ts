import { computed, ref } from 'vue'
import { defineStore } from 'pinia'

interface UserCredentials {
  user_name: string
  password: string
}

interface LoginResponse {
  access_token: string
}

interface RefreshResponse {
  access_token: string
  user_name: string
}

async function parseErrorMessage(res: Response): Promise<string> {
  try {
    const body = await res.json()
    if (typeof body?.message === 'string') return body.message
  } catch {
  }
  return `request failed with status ${res.status}`
}

export const useUserStore = defineStore('user', () => {
  const username = ref<string | null>(null)
  const accessToken = ref<string | null>(null) // access_token is a httpOnly cookie

  const isLoggedIn = computed(() => username.value !== null && accessToken.value !== null)

  async function register(user_name: string, password: string) {
    if (!user_name || !password) {
      throw new Error('user_name and password are required')
    }
    const res = await fetch(`/api/v1/user/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ user_name, password } satisfies UserCredentials)
    })

    if (!res.ok) {
      throw new Error(await parseErrorMessage(res))
    }
  }

  async function login(user_name: string, password: string) {
    const res = await fetch(`/api/v1/user/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include', //include is required since refresh_token is a httpOnly cookie
      body: JSON.stringify({ user_name, password } satisfies UserCredentials)
    })
    if (!user_name || !password) {
      throw new Error('user_name and password are required')
    }

    if (!res.ok) {
      throw new Error(await parseErrorMessage(res))
    }

    const data: LoginResponse = await res.json()
    accessToken.value = data.access_token
    username.value = user_name
  }

  async function logout() {
    try {
      await fetch(`/api/v1/user/logout`, {
        method: 'POST',
        credentials: 'include'
      })
    } finally {
      username.value = null
      accessToken.value = null
    }
  }

  async function refresh(user_name?: string): Promise<boolean> {
    const res = await fetch(`/api/v1/user/refresh`, {
      method: 'POST',
      credentials: 'include'
    })

    if (!res.ok) {
      username.value = null
      accessToken.value = null
      return false
    }

    const data: RefreshResponse = await res.json()
    accessToken.value = data.access_token
    username.value = data.user_name
    return true
  }

  return { username, accessToken, isLoggedIn, register, login, logout, refresh }
})