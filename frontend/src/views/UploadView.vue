<script setup lang="ts">
import { onUnmounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import AppHeader from '@/components/layout/AppHeader.vue'
import { useOcStore } from '@/stores/oc'
import { useUserStore } from '@/stores/user'
import { useHeightMask } from '@/composables/useHeightMask'
import {useImagePicker} from '@/composables/useImagePicker'
import LoadingSpinner from '@/components/common/LoadingSpinner.vue'

const router = useRouter()
const store = useOcStore()
const userStore = useUserStore()

const name = ref('')
const specie = ref('')
const sex = ref('')
const height = useHeightMask()
const description = ref('')
const fileInput = ref<HTMLInputElement>()

const MAX_IMAGES = 4

const {
  imageTiles,
  uploadingCount,
  errorMessages,
  handleFiles,
  removeImage,
  clearErrors,
} = useImagePicker(MAX_IMAGES)

function revokeImageUrls() {
  imageTiles.value.forEach((tile) => URL.revokeObjectURL(tile.url))
}
onUnmounted(revokeImageUrls)

function triggerFilePicker() {
  fileInput.value?.click()
}

async function submit() {
  clearErrors()

  const result = await store.addOc({
    oc_name: name.value.trim() || 'Nome do oc',
    specie: specie.value.trim(),
    sex: sex.value.trim(),
    height: height.raw.value,
    description: description.value.trim(),
    newImages: imageTiles.value.map((tile) => tile.file as File),
    existingImageIndexes: [],
  })

  if (!result.success || !result.data) {
    errorMessages.value = result.errors ?? ['Não foi possível enviar o OC. Tente novamente.']
    return
  }

  router.push(`/oc/${result.data.id}`)
}
</script>

<template>
  <div class="upload-page">
    <AppHeader />

    <main class="upload-page__content">
      <form class="sheet" @submit.prevent="submit">
        <h1 class="sheet__title">Upload de OC</h1>
        <p class="sheet__author-hint">Postando como <strong>{{ userStore.username ?? 'você' }}</strong></p>
        <div class="sheet__divider" />

        <div class="sheet__grid">
            <label class="sheet__field">
                <span>Nome:</span>
                <input v-model="name" type="text" placeholder="Nome do OC" />
            </label>
            <label class="sheet__field">
                <span>Espécie:</span>
                <input v-model="specie" type="text" placeholder="Ex: Raposa" />
            </label>
            <label class="sheet__field">
                <span>Sexo:</span>
                <select v-model="sex">
                    <option value="" disabled>Selecione</option>
                    <option value="M">Masculino</option>
                    <option value="F">Feminino</option>
                    <option value="O">Outro</option>
                </select>
                </label>
                <label class="sheet__field">
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

        <label class="sheet__field sheet__field--full">
          <span>Descrição:</span>
          <textarea v-model="description" rows="4" placeholder="Conte um pouco sobre o OC" />
        </label>

        <div class="sheet__field sheet__field--full">
          <span>Upload:</span>
          <div class="image-picker">
            <div v-for="(tile, index) in imageTiles" :key="tile.url" class="image-picker__tile">
                <img :src="tile.url" alt="" />
                <button type="button" class="image-picker__remove" aria-label="Remover imagem" @click="removeImage(index)">
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
          <p class="image-picker__hint">Até {{ MAX_IMAGES }} imagens.</p>
        </div>

        <ul v-if="errorMessages.length" class="sheet__error-list">
            <li v-for="(msg, i) in errorMessages" :key="i">{{ msg }}</li>
        </ul>

        <div class="sheet__actions">
          <button type="submit" class="sheet__submit" :disabled="store.isSaving">
            {{ store.isSaving ? 'Enviando...' : 'Upload' }}
          </button>
        </div>
      </form>
    </main>
  </div>
</template>

<style scoped>
.upload-page {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.upload-page__content {
  flex: 1;
  max-width: 960px;
  width: 100%;
  margin: 0 auto;
  padding: 40px 24px 64px;
}

.sheet {
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-card);
  padding: 32px 36px 28px;
}

.sheet__title {
  font-size: 28px;
  text-align: center;
}

.sheet__author-hint {
  margin: 6px 0 0;
  text-align: center;
  font-size: 13px;
  color: var(--color-text-muted);
}

.sheet__divider {
  height: 1px;
  background: var(--color-border);
  margin: 18px 0 24px;
}

.sheet__grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px 24px;
  margin-bottom: 16px;
}

.sheet__field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  font-weight: 600;
  font-size: 14px;
  color: var(--color-text);
}

.sheet__field--full {
  margin-bottom: 20px;
}

.sheet__field input,
.sheet__field textarea {
  font-family: var(--font-body);
  font-weight: 400;
  border: 1px solid var(--color-border);
  background: var(--color-bg-elevated);
  border-radius: var(--radius-sm);
  padding: 10px 12px;
  color: var(--color-text);
  font-size: 15px;
  outline: none;
  resize: vertical;
  transition: border-color var(--transition-fast);
}

.sheet__field input:focus,
.sheet__field textarea:focus {
  border-color: var(--color-brand);
}

.image-picker {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.image-picker__tile,
.image-picker__add {
  position: relative;
  width: 110px;
  height: 110px;
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

.image-picker__remove {
  position: absolute;
  top: 6px;
  right: 6px;
  width: 22px;
  height: 22px;
  border-radius: 50%;
  border: none;
  background: var(--color-surface);
  color: var(--color-text);
  font-size: 11px;
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
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
}

.image-picker__add span {
  font-size: 20px;
  color: var(--color-brand);
}

.image-picker__hint {
  margin: 8px 0 0;
  font-size: 12px;
  font-weight: 400;
  color: var(--color-text-faint);
}

.sheet__error {
  margin: 0 0 16px;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-danger, #c0392b);
}

.sheet__actions {
  display: flex;
  justify-content: flex-end;
}

.sheet__submit {
  border: none;
  background: var(--color-brand);
  color: var(--color-text-on-accent);
  font-family: var(--font-display);
  font-weight: 700;
  font-size: 15px;
  padding: 12px 32px;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: background var(--transition-fast);
}

.sheet__submit:hover {
  background: var(--color-brand-strong);
}

.sheet__submit:disabled {
  opacity: 0.6;
  cursor: not-allowed;
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

.sheet__error-list {
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

.sheet__error-list li + li {
  margin-top: 2px;
}

@media (max-width: 640px) {
  .sheet__grid {
    grid-template-columns: 1fr;
  }
  .sheet {
    padding: 24px 20px;
  }
}

</style>