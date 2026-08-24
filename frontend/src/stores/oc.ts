import { ref, watch } from 'vue'
import { defineStore } from 'pinia'
import type {
  CreatePostMetadata,
  EditPostMetadata,
  Oc,
  PostMinified,
  OcDraft,
  PostMetadata,
  SearchMode,
} from '@/types/oc'
import { authFetch } from '@/service/authFetch';

const VALIDATION_MESSAGES: Record<string, string> = {
  'invalid description': 'A descrição deve ter no máximo 1000 caracteres.',
  'invalid oc_name': 'O nome do OC é inválido.',
  'invalid specie': 'A espécie informada é inválida.',
  'invalid sex': 'Selecione um sexo válido.',
  'invalid height': 'A altura informada é inválida.',
}

const MAX_IMAGES = 6

function translateValidationError(raw: string): string {
  return VALIDATION_MESSAGES[raw.trim().toLowerCase()] ?? raw
}

async function parseValidationErrors(response: Response): Promise<string[]> {
  let raw = ''
  try {
	raw = await response.text()
  } catch {
	return ['Ocorreu um erro inesperado. Tente novamente.']
  }
  if (!raw) return ['Ocorreu um erro inesperado. Tente novamente.']

  try {
	const parsed = JSON.parse(raw)
	if (typeof parsed === 'string') raw = parsed
	else if (Array.isArray(parsed)) return parsed.map(String).map(translateValidationError)
	else if (typeof parsed?.error === 'string') raw = parsed.error
	else if (typeof parsed?.message === 'string') raw = parsed.message
  } catch {
	// that aint no json
  }

  return raw
	.split('\n')
	.map((line) => line.trim())
	.filter(Boolean)
	.map(translateValidationError)
}

export interface SaveResult {
  success: boolean
  data?: Oc
  errors?: string[]
}

export const useOcStore = defineStore('oc', () => {
  interface OCsResponse {
	posts: PostMinified[]
	total: number
  }

  const items = ref<Oc[]>([])
  const appliedTerm = ref('')
  const searchMode = ref<SearchMode>('C')
  const total_ocs = ref(0)
  const isLoading = ref(false)
  const isSaving = ref(false)
  const minified = ref<PostMinified[]>([])

  async function LoadOcs() {
	isLoading.value = true
	try {
	  minified.value = await query_ocs()
	} finally {
	  isLoading.value = false
	}
  }

  watch(appliedTerm, async (newTerm) => {
	const term = newTerm.trim().toLowerCase()
	isLoading.value = true
	try {
	  minified.value = await query_ocs(term)
	} finally {
	  isLoading.value = false
	}
  })

  async function query_ocs(query: string = ''): Promise<PostMinified[]> {
	let ocs: OCsResponse = { posts: [], total: 0 }
	let url = "";
	if (query === '' && (searchMode.value === 'C' || searchMode.value === 'U')) {
		url = `/api/v1/posts`
	}else{
		url = `/api/v1/posts?type=${searchMode.value}&query=${encodeURIComponent(query)}`
	}

	try {
	  const response = await fetch(url)
	  if (!response.ok) {
		throw new Error(`HTTP error! Status: ${response.status}`)
	  }
	  ocs = (await response.json()) as OCsResponse
	  total_ocs.value = ocs.total
	} catch (e) {
	  console.error('Failed to fetch posts:', e)
	}

	return ocs.posts
  }

  function applySearch(term: string) {
	appliedTerm.value = term
  }

  function setSearchMode(mode: SearchMode) {
	searchMode.value = mode
  }

  async function getById(id: number, forceRefresh = false): Promise<Oc | undefined> {
	if (!forceRefresh) {
		const cached = items.value.find((oc) => oc.id === id)
		if (cached) return cached
	}

	try {
		const response = await fetch(`/api/v1/post/${id}`)
		if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`)
		const fresh = (await response.json()) as Oc
		const idx = items.value.findIndex((o) => o.id === id)
		if (idx !== -1) items.value[idx] = fresh
		else items.value.push(fresh)
		return fresh
	} catch (e) {
		console.error(`Failed to fetch oc ${id}:`, e)
		return undefined
	}
	}

  function buildFormData(
	metadata: CreatePostMetadata | EditPostMetadata,
	images: File[],
  ): FormData {
	const form = new FormData()
	form.append('metadata', new Blob([JSON.stringify(metadata)], { type: 'application/json' }))
	images.forEach((file) => form.append('images', file, file.name))
	return form
  }

	function buildUpdateFormData(metadata: EditPostMetadata, newImages: File[]): FormData {
		const existing = new Set(metadata.existing_images)
		const freeSlots = Array.from({ length: MAX_IMAGES }, (_, i) => i).filter((i) => !existing.has(i))

		if (newImages.length > freeSlots.length) {
			throw new Error('Mais imagens novas do que slots disponíveis')
		}

		const lastSlot = Math.max(-1, ...metadata.existing_images, ...freeSlots.slice(0, newImages.length))
		const slots: (File | null)[] = Array.from({ length: lastSlot + 1 }, () => null)

		newImages.forEach((file, i) => {
			const slot = freeSlots[i]
			if (slot === undefined) {
				throw new Error('invalid image slot')
			}
			slots[slot] = file
		})

		const form = new FormData()
		form.append('metadata', new Blob([JSON.stringify(metadata)], { type: 'application/json' }))
		slots.forEach((file) => {
			if (file) form.append('images', file, file.name)
			else form.append('images', new Blob([]), '') // placeholder 
		})
		return form
	}

  async function addOc(draft: OcDraft): Promise<SaveResult> {
	const metadata: CreatePostMetadata = {
	  oc_name: draft.oc_name,
	  description: draft.description,
	  specie: draft.specie,
	  sex: draft.sex,
	  height: draft.height,
	}

	isSaving.value = true
	try {
	  const response = await authFetch('/api/v1/post', {
		method: 'POST',
		body: buildFormData(metadata, draft.newImages),
	  })

	  if (response.status === 400) {
		return { success: false, errors: await parseValidationErrors(response) }
	  }
	  if (!response.ok) {
		throw new Error(`HTTP error! Status: ${response.status}`)
	  }

	  const created = (await response.json()) as PostMetadata
	  items.value = [created, ...items.value]
	  return { success: true, data: created }
	} catch (e) {
	  console.error('Failed to create oc:', e)
	  return { success: false, errors: ['Não foi possível conectar ao servidor. Tente novamente.'] }
	} finally {
	  isSaving.value = false
	}
  }


  interface DeleteResult {
      success: boolean
      error: string
  }
  async function deleteOc(id: number): Promise<DeleteResult> {
    let result : DeleteResult = {success: false, error: ''}
    try {
        const response = await authFetch(`/api/v1/post/${id}`, {
            method: 'DELETE',
        })
        result.success = response.ok
        if (response.status === 404) {
            result.error = "OC não encontrado."
        }
        if (response.status === 500) {
            result.error = "Um erro inesperado aconteceu."
        }
        if (response.status === 400) {
            result.error = "Usuário sem permissão."
        }
    }catch (e) {
      console.error('Failed to delete oc:', e)
      result.success = false
      result.error = "Um erro inesperado aconteceu."
    }

    console.log(result)
    return result;
  }

  async function updateOc(id: number, draft: OcDraft): Promise<SaveResult> {
	const metadata: EditPostMetadata = {
	  id,
	  oc_name: draft.oc_name,
	  description: draft.description,
	  specie: draft.specie,
	  sex: draft.sex,
	  height: draft.height,
	  existing_images: draft.existingImageIndexes,
	}

	isSaving.value = true
	try {
	  const response = await authFetch('/api/v1/post', {
		method: 'PUT',
		body: buildUpdateFormData(metadata, draft.newImages),
	  })

	  if (response.status === 400) {
		return { success: false, errors: await parseValidationErrors(response) }
	  }
	  if (!response.ok) {
		throw new Error(`HTTP error! Status: ${response.status}`)
	  }

	  if (response.status === 404) {
		return { success: false, errors: ['OC nao encontrada'] }
	  }

	  return { success: true}
	} catch (e) {
	  console.error(`Failed to update oc ${id}:`, e)
	  return { success: false, errors: ['Não foi possível conectar ao servidor. Tente novamente.'] }
	} finally {
	  isSaving.value = false
	}
  }

  return {
	items,
	appliedTerm,
	searchMode,
	isLoading,
	isSaving,
	total_ocs,
	applySearch,
	setSearchMode,
	getById,
	addOc,
	updateOc,
	minified,
    deleteOc,
    LoadOcs
  }
})