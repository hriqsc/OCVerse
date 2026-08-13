<script setup lang="ts">
import { onUnmounted, ref, watch } from 'vue'
import type { OcDraft, EditOc, Oc } from '@/types/oc'
import LoadingSpinner from '@/components/common/LoadingSpinner.vue'
import { useHeightMask } from '@/composables/useHeightMask'
import { useOcStore } from '@/stores/oc'
import { processImageFile,ImageProcessingError,parseImageSlot  } from '@/service/image'

const props = defineProps<{ open: boolean; oc: EditOc }>()
const emit = defineEmits<{ 'update:open': [value: boolean]; saved: [] }>()

const store = useOcStore()

const MAX_IMAGES = 4
const fileInput = ref<HTMLInputElement>()
const uploadingCount = ref(0)
const errorMessages = ref<string[]>([])

const name = ref(props.oc.oc_name)
const specie = ref(props.oc.specie)
const sex = ref(props.oc.sex)
const height = useHeightMask(props.oc.height)
const description = ref(props.oc.description)

interface ImageTile {
  url: string
  file?: File
  originalIndex?: number
}
const imageTiles = ref<ImageTile[]>([])

function revokeNewImageUrls() {
  imageTiles.value.forEach((tile) => {
    if (tile.file) URL.revokeObjectURL(tile.url)
  })
}

watch(
  () => props.open,
  (isOpen) => {
    if (isOpen) {
      name.value = props.oc.oc_name
      specie.value = props.oc.specie
      sex.value = props.oc.sex
      height.reset(props.oc.height)
      description.value = props.oc.description
      uploadingCount.value = 0
      errorMessages.value = []
      imageTiles.value = props.oc.images.map((url) => ({
        url,
        originalIndex: parseImageSlot(url),
      }))
    } else {
      revokeNewImageUrls()
    }
  },
)

onUnmounted(revokeNewImageUrls)

function close() {
  emit('update:open', false)
}

function triggerFilePicker() {
  fileInput.value?.click()
}

function handleFiles(event: Event) {
  const files = (event.target as HTMLInputElement).files
  if (!files) return
  const remaining = MAX_IMAGES - imageTiles.value.length
  const toProcess = Array.from(files).slice(0, remaining)

  toProcess.forEach((file) => {
    uploadingCount.value++
    processImageFile(file)
      .then((processed) => {
        const url = URL.createObjectURL(processed)
        imageTiles.value.push({ url, file: processed })
      })
      .catch((e) => {
        const message =
          e instanceof ImageProcessingError ? e.message : 'Não foi possível processar esta imagem.'
        errorMessages.value = [...errorMessages.value, `${file.name}: ${message}`]
      })
      .finally(() => {
        uploadingCount.value--
      })
  })
  ;(event.target as HTMLInputElement).value = ''
}

function removeImage(index: number) {
  const [tile] = imageTiles.value.splice(index, 1)
  if (tile?.file) URL.revokeObjectURL(tile.url)
}

async function save() {
  errorMessages.value = []

  const draft: OcDraft = {
    oc_name: name.value.trim() || 'Nome do oc',
    specie: specie.value.trim(),
    sex: sex.value.trim(),
    height: height.raw.value.trim(),
    description: description.value.trim(),
    newImages: imageTiles.value.filter((t) => t.file).map((t) => t.file as File),
    existingImageIndexes: imageTiles.value
      .filter((t) => t.originalIndex !== undefined)
      .map((t) => t.originalIndex as number),
  }

  const result = await store.updateOc(props.oc.id, draft)

  if (!result.success) {
    errorMessages.value = result.errors ?? ['Não foi possível salvar as alterações. Tente novamente.']
    return
  }

  emit('saved')
  close()
}
</script>

<template>
  <Teleport to="body">
    <Transition name="modal-fade">
      <div v-if="open" class="modal-backdrop" @mousedown.self="close">
        <form class="modal-sheet" role="dialog" aria-modal="true" @submit.prevent="save">
          <button type="button" class="modal-sheet__close" aria-label="Fechar" @click="close">
            ✕
          </button>

          <h2 class="modal-sheet__title">Editar OC</h2>
          <div class="modal-sheet__divider" />

          <div class="modal-sheet__grid">
            <label class="modal-sheet__field">
              <span>Nome:</span>
              <input v-model="name" type="text" />
            </label>
            <label class="modal-sheet__field">
              <span>Espécie:</span>
              <input v-model="specie" type="text" />
            </label>
            <label class="modal-sheet__field">
                <span>Sexo:</span>
                <select v-model="sex">
                    <option value="M">Masculino</option>
                    <option value="F">Feminino</option>
                    <option value="O">Outro</option>
                </select>
                </label>
                <label class="modal-sheet__field">
                <span>Altura:</span>
                <input
                    :value="height.display.value"
                    @input="height.onInput"
                    type="text"
                    inputmode="numeric"
                    placeholder="1,70 m"
                />
            </label>
          </div>

          <label class="modal-sheet__field modal-sheet__field--full">
            <span>Descrição:</span>
            <textarea v-model="description" rows="3" />
          </label>

          <div class="modal-sheet__field modal-sheet__field--full">
            <span>Imagens:</span>
            <div class="image-picker">
              <div v-for="(tile, index) in imageTiles" :key="tile.url" class="image-picker__tile">
                <img :src="tile.url" alt="" />
                <button
                  type="button"
                  class="image-picker__remove"
                  aria-label="Remover imagem"
                  @click="removeImage(index)"
                >
                  ✕
                </button>
              </div>
              <div v-for="n in uploadingCount" :key="`uploading-${n}`" class="image-picker__tile image-picker__tile--loading">
                <LoadingSpinner size="sm" label="" />
              </div>
              <button
                v-if="imageTiles.length + uploadingCount < MAX_IMAGES"
                type="button"
                class="image-picker__add"
                @click="triggerFilePicker"
              >
                <span aria-hidden="true">＋</span>
                Adicionar
              </button>
            </div>
            <input
              ref="fileInput"
              type="file"
              accept="image/*"
              multiple
              hidden
              @change="handleFiles"
            />
          </div>

          <ul v-if="errorMessages.length" class="modal-sheet__error-list">
            <li v-for="(msg, i) in errorMessages" :key="i">{{ msg }}</li>
          </ul>

          <div class="modal-sheet__actions">
            <button type="button" class="modal-sheet__cancel" @click="close">Cancelar</button>
            <button type="submit" class="modal-sheet__save" :disabled="store.isSaving">
              {{ store.isSaving ? 'Salvando...' : 'Salvar' }}
            </button>
          </div>
        </form>
      </div>
    </Transition>
  </Teleport>
</template>

<style scoped>
.modal-backdrop {
  position: fixed;
  inset: 0;
  background: rgba(44, 36, 23, 0.45);
  backdrop-filter: blur(3px);
  display: grid;
  place-items: center;
  z-index: 100;
  padding: 20px;
}

.modal-sheet {
  position: relative;
  width: 100%;
  max-width: 560px;
  max-height: 90vh;
  overflow-y: auto;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-popover);
  padding: 28px 32px;
}

.modal-sheet__close {
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

.modal-sheet__close:hover {
  background: var(--color-surface-alt);
  color: var(--color-text);
}

.modal-sheet__title {
  font-size: 24px;
  text-align: center;
}

.modal-sheet__divider {
  height: 1px;
  background: var(--color-border);
  margin: 14px 0 20px;
}

.modal-sheet__grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 14px 20px;
  margin-bottom: 14px;
}

.modal-sheet__field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-weight: 600;
  font-size: 14px;
  color: var(--color-text);
}

.modal-sheet__field--full {
  margin-bottom: 18px;
}

.modal-sheet__field input,
.modal-sheet__field textarea {
  font-family: var(--font-body);
  font-weight: 400;
  border: 1px solid var(--color-border);
  background: var(--color-bg-elevated);
  border-radius: var(--radius-sm);
  padding: 9px 12px;
  color: var(--color-text);
  font-size: 15px;
  outline: none;
  resize: vertical;
  transition: border-color var(--transition-fast);
}

.modal-sheet__field input:focus,
.modal-sheet__field textarea:focus {
  border-color: var(--color-brand);
}

.image-picker {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.image-picker__tile,
.image-picker__add {
  position: relative;
  width: 88px;
  height: 88px;
  border-radius: var(--radius-sm);
  overflow: hidden;
  border: 1px solid var(--color-border);
}

.image-picker__tile img {
  width: 100%;
  height: 100%;
  object-fit: cover;
  display: block;
}

.image-picker__tile--loading {
  display: grid;
  place-items: center;
  background: var(--color-bg-elevated);
  border-style: dashed;
}

.image-picker__remove {
  position: absolute;
  top: 4px;
  right: 4px;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  border: none;
  background: var(--color-surface);
  color: var(--color-text);
  font-size: 10px;
  cursor: pointer;
  display: grid;
  place-items: center;
  box-shadow: 0 1px 4px rgba(44, 36, 23, 0.3);
}

.image-picker__add {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 4px;
  background: var(--color-bg-elevated);
  border-style: dashed;
  color: var(--color-text-muted);
  font-size: 11px;
  font-weight: 600;
  cursor: pointer;
}

.image-picker__add span {
  font-size: 18px;
  color: var(--color-brand);
}

.modal-sheet__actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.modal-sheet__cancel {
  border: 1px solid var(--color-border);
  background: transparent;
  color: var(--color-text);
  font-weight: 600;
  font-size: 14px;
  padding: 10px 20px;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.modal-sheet__cancel:hover {
  background: var(--color-surface-alt);
}

.modal-sheet__save {
  border: none;
  background: var(--color-brand);
  color: var(--color-text-on-accent);
  font-family: var(--font-display);
  font-weight: 700;
  font-size: 14px;
  padding: 10px 24px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background var(--transition-fast);
}

.modal-sheet__save:hover {
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
.modal-fade-enter-active .modal-sheet,
.modal-fade-leave-active .modal-sheet {
  transition: transform var(--transition-base);
}
.modal-fade-enter-from .modal-sheet,
.modal-fade-leave-to .modal-sheet {
  transform: scale(0.96) translateY(8px);
}

.sheet__field select,
.modal-sheet__field select {
  font-family: var(--font-body);
  font-weight: 400;
  border: 1px solid var(--color-border);
  background: var(--color-bg-elevated);
  border-radius: var(--radius-sm);
  padding: 10px 12px;
  color: var(--color-text);
  font-size: 15px;
  outline: none;
}


.modal-sheet__error-list {
  margin: 0 0 16px;
  padding: 10px 14px;
  list-style: disc;
  list-style-position: inside;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-danger, #c0392b);
  background: color-mix(in srgb, var(--color-danger, #c0392b) 8%, transparent);
  border: 1px solid color-mix(in srgb, var(--color-danger, #c0392b) 30%, transparent);
  border-radius: var(--radius-sm);
}

.modal-sheet__error-list li + li {
  margin-top: 2px;
}


@media (max-width: 560px) {
  .modal-sheet__grid {
    grid-template-columns: 1fr;
  }
}
</style>