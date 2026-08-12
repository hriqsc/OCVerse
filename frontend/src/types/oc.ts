export type SearchMode = 'C' | 'U'

/** Mirrors the backend's `PostMetadata` struct (schema/post.rs). */
export interface PostMetadata {
  id: number
  creator_user_name: string
  oc_name: string
  description: string
  specie: string
  sex: string
  height: string
  images: ImageRef[]
}

/** Mirrors the backend's `PostMinified` struct (schema/post.rs). */
export interface PostMinified{
    id: number
    creator_user_name: string
    oc_name: string
    thumb: string
}

/** Mirrors the backend's `CreatePost` struct — sent as the "metadata" part
 *  of the multipart body on POST /api/v1/post. */
export interface CreatePostMetadata {
  oc_name: string
  description: string
  specie: string
  sex: string
  height: string
}

/** Mirrors the backend's `EditPost` struct — sent as the "metadata" part
 *  of the multipart body on PUT /api/v1/post. `existing_images` holds the
 *  indices (into the OC's original `images` array) of images the user kept
 *  unchanged, so the backend doesn't need to re-save images that were
 *  already uploaded — anything not listed there and not resent as a new
 *  file is treated as removed. */
export interface EditPostMetadata {
  id: number
  oc_name: string
  description: string
  sex: string
  specie: string
  height: string
  existing_images: number[]
}

/** Frontend representation of an OC — same shape as the backend's PostMetadata. */
export type Oc = PostMetadata

/** Data collected from the create/edit form, ready to be turned into a
 *  multipart request. `newImages` are files the user just picked (need
 *  uploading). `existingImageIndexes` are the indices (into the original
 *  OC's `images` array) of images the user kept — only meaningful while
 *  editing; ignored on create. */
export interface OcDraft {
  oc_name: string
  specie: string
  sex: string
  height: string
  description: string
  newImages: File[]
  existingImageIndexes: number[]
}

export interface ImageRef {
  slot: number
  url: string
}

/** Shape handed to OcModal when editing an existing OC. */
export interface EditOc {
  id: number
  oc_name: string
  specie: string
  sex: string
  height: string
  description: string
  images: ImageRef[]
}