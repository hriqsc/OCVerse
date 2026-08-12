const MAX_DIMENSION = 1500
const ALLOWED_TYPES = new Set(['image/png', 'image/jpeg', 'image/webp', 'image/bmp'])

export class ImageProcessingError extends Error {}

/**
 * Validates and processes an image file, downscaling it if necessary.
 */
export async function processImageFile(file: File): Promise<File> {
  if (file.type === 'image/gif' || file.type.startsWith('video/')) {
    throw new ImageProcessingError('GIFs e vídeos não são suportados.')
  }

  if (!ALLOWED_TYPES.has(file.type)) {
    throw new ImageProcessingError('Formato de imagem não suportado.')
  }

  // decode the image as a bitmap
  let bitmap: ImageBitmap
  try {
    bitmap = await createImageBitmap(file)
  } catch {
    throw new ImageProcessingError('Não foi possível ler esta imagem.')
  }

  try {
    const { width, height } = bitmap
    const scale = Math.min(1, MAX_DIMENSION / Math.max(width, height))
    const targetWidth = Math.round(width * scale)
    const targetHeight = Math.round(height * scale)

    const canvas = document.createElement('canvas')
    canvas.width = targetWidth
    canvas.height = targetHeight

    const ctx = canvas.getContext('2d')
    if (!ctx) throw new ImageProcessingError('Canvas não suportado neste navegador.')

    // downscale the mf
    ctx.imageSmoothingEnabled = true
    ctx.imageSmoothingQuality = 'high'
    ctx.drawImage(bitmap, 0, 0, targetWidth, targetHeight)

    const blob = await new Promise<Blob | null>((resolve) =>
      canvas.toBlob(resolve, 'image/png'),
    )

    if (!blob) throw new ImageProcessingError('Falha ao converter a imagem.')

    const newName = file.name.replace(/\.[^.]+$/, '') + '.png'
    return new File([blob], newName, { type: 'image/png' })
  } finally {
    bitmap.close()
  }
}