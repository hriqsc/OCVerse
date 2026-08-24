import { onUnmounted, ref } from 'vue'
import { processImageFile, ImageProcessingError, parseImageSlot } from '@/service/image'

export interface ImageTile {
  url: string
  file?: File
  originalIndex?: number
}

/**
 * Manages the state of image selection/upload between creation form and OC edit modal.
 */
export function useImagePicker(maxImages = 4) {
  const imageTiles = ref<ImageTile[]>([])
  const uploadingCount = ref(0)
  const errorMessages = ref<string[]>([])

  function revokeNewImageUrls() {
    imageTiles.value.forEach((tile) => {
      if (tile.file) URL.revokeObjectURL(tile.url)
    })
  }

  onUnmounted(revokeNewImageUrls)

  /** reset the state. */
  function reset(existingImages: string[] = []) {
    revokeNewImageUrls()
    imageTiles.value = existingImages.map((url) => ({
      url,
      originalIndex: parseImageSlot(url),
    }))
    uploadingCount.value = 0
    errorMessages.value = []
  }

  function handleFiles(event: Event) {
    const files = (event.target as HTMLInputElement).files
    if (!files) return

    const remaining = maxImages - imageTiles.value.length - uploadingCount.value
    const toProcess = Array.from(files).slice(0, Math.max(0, remaining))

    toProcess.forEach((file) => {
      uploadingCount.value++
      processImageFile(file)
        .then((processed) => {
          imageTiles.value.push({ url: URL.createObjectURL(processed), file: processed })
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

  function clearErrors() {
    errorMessages.value = []
  }

  return {
    imageTiles,
    uploadingCount,
    errorMessages,
    handleFiles,
    removeImage,
    reset,
    clearErrors,
    revokeNewImageUrls,
  }
}