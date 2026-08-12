import { useUserStore } from '@/stores/user'

export async function authFetch(url: string, options: RequestInit = {}): Promise<Response> {
  const userStore = useUserStore()

  const doFetch = () =>
    fetch(url, {
      ...options,
      credentials: 'include',
      headers: {
        ...options.headers,
        Authorization: `Bearer ${userStore.accessToken}`,
      },
    })

  let response = await doFetch()

  // access token expirado -> tenta refresh e refaz a request uma vez
  if (response.status === 401) {
    const refreshed = await userStore.refresh()
    if (refreshed) {
      response = await doFetch()
    }
  }

  return response
}