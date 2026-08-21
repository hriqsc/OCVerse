import UPNG from "upng-js";

const MAX_DIMENSION = 1500
const ALLOWED_TYPES = new Set(['image/png', 'image/jpeg', 'image/webp', 'image/bmp'])
export class ImageProcessingError extends Error {}

/** Number of colors used for PNG palette quantization (lossy compression). Lower = smaller file, more quality loss. Use 0 for lossless. */
const PALETTE_COLORS = 256;


const MAX_IMAGES = 4


/**
 * Processes an image File on the client side:
 * 1. Loads the image.
 * 2. Resizes it (if needed) so neither dimension exceeds MAX_DIMENSION,
 *    preserving the original aspect ratio.
 * 3. Applies aggressive PNG compression via color quantization,
 *    while fully preserving the alpha (transparency) channel.
 * 4. Returns a new File encoded as PNG.
 *
 * @param file - The original image File (any format supported by the browser: jpg, png, webp, etc).
 * @param paletteColors - Number of colors for palette quantization (default 256). Use 0 for lossless PNG.
 * @returns A Promise that resolves to a new compressed PNG File.
 */
export async function processImageFile(
  file: File,
  paletteColors: number = PALETTE_COLORS
): Promise<File> {

  if (!ALLOWED_TYPES.has(file.type)) {
	throw new ImageProcessingError('Formato de imagem não suportado.')
  }
  // 1. Decode the file into a drawable image source
  const imageBitmap = await loadImage(file);

  // 2. Compute new dimensions, preserving aspect ratio
  const { width, height } = getScaledDimensions(
	imageBitmap.width,
	imageBitmap.height,
	MAX_DIMENSION
  );

  // 3. Draw the (possibly resized) image onto a canvas
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;

  const ctx = canvas.getContext("2d");
  if (!ctx) {
	throw new ImageProcessingError("Could not get 2D context from canvas.");
  }
  ctx.drawImage(imageBitmap, 0, 0, width, height);

  // 4. Extract raw RGBA pixel data (alpha channel included)
  const imageData = ctx.getImageData(0, 0, width, height);

  // 5. Encode as PNG with aggressive quantization (keeps alpha channel)
  const pngArrayBuffer = UPNG.encode(
	[imageData.data.buffer],
	width,
	height,
	paletteColors // 0 = lossless, >0 = lossy palette quantization
  );

  const pngBlob = new Blob([pngArrayBuffer], { type: "image/png" });

  // 6. Wrap the blob into a new File
  const newFileName = replaceExtension(file.name, "png");
  const processedFile = new File([pngBlob], newFileName, {
	type: "image/png",
	lastModified: Date.now(),
  });

  // Free memory if using ImageBitmap
  if ("close" in imageBitmap && typeof imageBitmap.close === "function") {
	imageBitmap.close();
  }

  return processedFile;
}

/**
 * Decodes a File into a drawable image source.
 * Prefers createImageBitmap (faster, off-main-thread decoding),
 * falling back to HTMLImageElement for broader compatibility.
 *
 * @param file - Image file to decode.
 * @returns A Promise resolving to an ImageBitmap or HTMLImageElement.
 */
async function loadImage(file: File): Promise<ImageBitmap | HTMLImageElement> {
  if (typeof createImageBitmap === "function") {
	try {
	  return await createImageBitmap(file);
	} catch {
	  // Fall through to the HTMLImageElement fallback below
	}
  }

  return new Promise((resolve, reject) => {
	const img = new Image();
	const url = URL.createObjectURL(file);
	img.onload = () => {
	  URL.revokeObjectURL(url);
	  resolve(img);
	};
	img.onerror = (err) => {
	  URL.revokeObjectURL(url);
	  reject(err);
	};
	img.src = url;
  });
}

/**
 * Calculates new dimensions that fit within maxDimension while
 * preserving the original aspect ratio. Returns the original
 * dimensions unchanged if they already fit.
 *
 * @param originalWidth - Original image width in pixels.
 * @param originalHeight - Original image height in pixels.
 * @param maxDimension - Maximum allowed width or height.
 * @returns The scaled { width, height }.
 */
function getScaledDimensions(
  originalWidth: number,
  originalHeight: number,
  maxDimension: number
): { width: number; height: number } {
  if (originalWidth <= maxDimension && originalHeight <= maxDimension) {
	return { width: originalWidth, height: originalHeight };
  }

  const aspectRatio = originalWidth / originalHeight;

  if (originalWidth > originalHeight) {
	return {
	  width: maxDimension,
	  height: Math.round(maxDimension / aspectRatio),
	};
  } else {
	return {
	  width: Math.round(maxDimension * aspectRatio),
	  height: maxDimension,
	};
  }
}

/**
 * Replaces the file extension of a filename with a new one.
 *
 * @param fileName - Original filename (e.g. "photo.jpg").
 * @param newExtension - New extension without the dot (e.g. "png").
 * @returns The filename with the new extension (e.g. "photo.png").
 */
function replaceExtension(fileName: string, newExtension: string): string {
  const nameWithoutExtension = fileName.replace(/\.[^/.]+$/, "");
  return `${nameWithoutExtension}.${newExtension}`;
}


export function parseImageSlot(url: string): number {
  const match = url.match(/\/(\d+)\.[a-zA-Z0-9]+$/)
  if (!match) {
    throw new Error(`URL de imagem em formato inesperado: ${url}`)
  }
  return Number(match[1])
}