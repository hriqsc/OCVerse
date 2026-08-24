# Frontend Views Documentation

**Location:** `frontend/src/views/`

Brief overview of each view (page-level component). Views are top-level route components that compose smaller components and connect to Pinia stores.

> Covers only the files shared so far — will be expanded as more parts of the codebase are shared.

---

## `HubView.vue`
Main/home page. Loads the OC list on mount (`useOcStore().LoadOcs()`) and shows it in a grid, with a search bar and an upload button. Clicking Upload goes to `/hub/upload` if logged in, otherwise opens the login modal.

## `MagmasView.vue`
`/magmas` page listing the user's external "Magma" boards. Fetches magma IDs from the backend, builds each board's URL and thumbnail, then fetches each page's HTML client-side just to scrape its `<title>` for the card name. No store used — all logic is local to the component.

## `OcDetailView.vue`
`/oc/:id` — detail page for a single OC. Loads the OC by id and shows its info (name, species, sex, height, description) plus an image gallery with a lightbox (click to zoom, closable via Esc/click-outside). If the logged-in user is the OC's owner, shows Edit (opens `EditOcModal`) and Delete (with confirm dialog) buttons.

## `ResetLogin.vue`
`/reset/:id` — password reset form. Validates the reset id with the backend on mount (redirects to `/404` if invalid), then lets the user submit a new password. Shows success/error feedback and redirects to `/hub` after a successful reset.

## `UploadView.vue`
`/hub/upload` — form to create a new OC (name, species, sex, height, description, up to 4 images). Image handling is delegated to the `useImagePicker` composable. On submit, calls `useOcStore().addOc(...)` and redirects to the new OC's detail page on success.

---

## Common Patterns
- Every view includes `AppHeader` for consistent navigation.
- Data comes mostly from Pinia stores (`useOcStore`, `useUserStore`), except `MagmasView`, which calls the backend/external pages directly.
- Loading/error/empty states are handled inline per view.
- Styling uses scoped CSS with shared design tokens (`--color-brand`, `--radius-lg`, etc.).
- UI text is in Portuguese; code/identifiers are in English.

---

# Types
 
**Location:** `frontend/src/types/oc.ts`
 
TypeScript types for the OC (original character) domain, used across the views and stores above.
 
- **`SearchMode`** — `'C' | 'U'`, likely search-by-**C**reation vs **U**ser filter mode.
- **`PostMetadata`** — full OC data as returned by the backend (mirrors Rust's `PostMetadata`). Used as `Oc` throughout the frontend.
- **`PostMinified`** — lightweight OC summary (id, creator, name, thumbnail) used for grid/list views like `HubView`'s `store.minified`.
- **`CreatePostMetadata`** — fields sent when creating a new OC (`POST /api/v1/post`).
- **`EditPostMetadata`** — fields sent when editing an OC (`PUT /api/v1/post`); includes `existing_images`, the indices of previously-uploaded images the user kept, so the backend knows which old images to preserve vs. drop.
- **`Oc`** — alias for `PostMetadata`; the main type used in components like `OcDetailView`.
- **`OcDraft`** — local form-state shape used while creating/editing an OC (used in `UploadView`), holding new image files plus which existing image indices were kept.
- **`EditOc`** — shape passed into `EditOcModal` when editing an existing OC.
 

---
 
# Stores
 
**Location:** `frontend/src/stores/`
 
Pinia stores holding shared app state, used by the views above.
 
## `oc.ts` (`useOcStore`)
Handles everything related to OCs: listing, searching, loading a single OC, and creating/editing/deleting.
- **State:** `items` (cached full OCs), `minified` (summary list for grids), `appliedTerm` / `searchMode` (search state), `total_ocs`, `isLoading`, `isSaving`.
- **`LoadOcs()`** — fetches the minified OC list (`GET /api/v1/posts`).
- Watches `appliedTerm` and automatically re-queries (`GET /api/v1/posts?type=...&query=...`) when it changes, using `searchMode` (`'C'` or `'U'`) to filter by creator vs. general search.
- **`getById(id, forceRefresh?)`** — returns a cached OC if available, otherwise fetches it from `GET /api/v1/post/:id` and caches it.
- **`addOc(draft)`** / **`updateOc(id, draft)`** — build a `multipart/form-data` request (metadata as JSON blob + image files) and `POST`/`PUT` to `/api/v1/post` via `authFetch` (authenticated fetch wrapper). Validation errors (HTTP 400) are parsed and translated into Portuguese user-facing messages via `VALIDATION_MESSAGES`.
- **`updateOc`** uses `buildUpdateFormData`, which reconstructs an image "slot" array so kept images (`existing_images`) stay in their original position and new images fill the remaining slots (max 6 images total), sending empty placeholder blobs for unused slots.
- **`deleteOc(id)`** — `DELETE /api/v1/post/:id`, with specific error messages for 404 (not found), 400 (no permission), and 500 (unexpected error).
## `user.ts` (`useUserStore`)
Handles authentication state.
- **State:** `username`, `accessToken` (in-memory; the refresh token itself lives in an httpOnly cookie, not in this store). `isLoggedIn` is a computed based on both being set.
- **`register(user_name, password)`** — `POST /api/v1/user/register`.
- **`login(user_name, password)`** — `POST /api/v1/user/login` (with `credentials: 'include'` for the refresh-token cookie); stores the returned `access_token` and username on success.
- **`logout()`** — `POST /api/v1/user/logout`, then clears local state regardless of the request outcome.
- **`refresh(user_name?)`** — `POST /api/v1/user/refresh` to get a new access token using the httpOnly refresh cookie; clears state and returns `false` if it fails, otherwise updates `accessToken`/`username` and returns `true`. Used to silently restore a session (e.g. on app load).
## `authModal.ts` (`useAuthModalStore`)
Tiny store controlling the visibility/state of the login/register modal.
- **State:** `isOpen`, `initialTab` (`'login' | 'registrar'`, controls which tab shows first).
- **`open(tab?)`** — opens the modal, optionally pre-selecting a tab.
- **`close()`** — closes the modal.
- This is what `HubView`'s `useAuthModal()` composable presumably wraps/exposes.
## `counter.ts` (`useCounterStore`)
Default/example Pinia store scaffolded by `create-vue` (`count`, `doubleCount`, `increment`). Not used by any real feature — likely safe to remove once no longer needed as a reference.



---
 
# Services
 
**Location:** `frontend/src/service/`
 
Standalone helper modules (not Pinia stores) used by stores/components to talk to the backend or do client-side processing.
 
## `authFetch.ts`
A `fetch` wrapper that automatically attaches the current access token (`Authorization: Bearer ...`) and includes credentials (for the httpOnly refresh cookie). If a request comes back `401 Unauthorized`, it transparently calls `useUserStore().refresh()` to get a new access token and retries the request once. Used for authenticated endpoints like creating/updating/deleting an OC (`useOcStore`).
 
## `image.ts`
Client-side image processing before upload.
- **`processImageFile(file, paletteColors?)`** — main entry point. Validates the file type (png/jpeg/webp/bmp), decodes it (`createImageBitmap`, falling back to `HTMLImageElement`), resizes it down so no dimension exceeds 1500px (preserving aspect ratio), draws it to a canvas, and re-encodes it as a compressed PNG using `upng-js` palette quantization (default 256 colors; transparency/alpha is preserved). Returns a new `File` with a `.png` extension. Throws `ImageProcessingError` for unsupported formats or canvas failures.
- **`parseImageSlot(url)`** — extracts the numeric "slot" index from an image URL (e.g. `.../3.png` → `3`), used to map an OC's stored image URLs back to their slot position (relevant to `useOcStore`'s `buildUpdateFormData` slot logic).



---
 
# Composables
 
**Location:** `frontend/src/composables/`
 
Reusable stateful logic (Vue composition functions) shared across components/views.
 
## `useAuthModal.ts`
Thin wrapper around `useAuthModalStore` giving simple named actions instead of calling `store.open()`/`store.close()` directly: `openLogin()`, `openRegister()`, `closeAuthModal()`. The modal itself (`<LoginModal />`) is mounted once globally in `App.vue`, so any component can just call this composable to trigger it — as seen in `HubView`.
 
## `useHeightMask.ts`
Manages a masked height input (e.g. `"1,70 m"`) backed by a raw digit-only value (e.g. `"170"`).
- `raw` — the underlying numeric digits (max 3), stored/sent to the backend.
- `display` — computed formatted string shown in the input (`"1,70 m"`).
- `onInput(event)` — strips non-digits from user input, updates `raw`, and re-applies the formatted mask to the input element.
- `reset(value)` — reinitializes `raw` from a new raw string (used when loading an existing OC for editing).
Used by `UploadView` and presumably `EditOcModal`.
## `useImagePicker.ts`
Manages image selection/upload state shared between the upload form and the edit modal, capped at `maxImages` (default 4).
- `imageTiles` — current images, each either an existing image (`url` + `originalIndex`, its slot in the OC's original `images` array) or a newly picked one (`url` as a local object URL + `file`).
- `uploadingCount` — number of images currently being processed (for showing loading tiles).
- `errorMessages` — per-file processing errors (e.g. unsupported format), collected via `useImagePicker`'s use of `processImageFile` from the `image` service.
- `handleFiles(event)` — reads picked files from an `<input type="file">` change event (respecting the remaining slot count), runs each through `processImageFile` (compression/resizing), and pushes a new tile on success or an error message on failure.
- `removeImage(index)` — removes a tile and revokes its object URL if it was a newly added file.
- `reset(existingImages?)` — clears state and repopulates `imageTiles` from an existing OC's image URLs (using `parseImageSlot` to recover each one's original slot) — used when opening the edit modal.
- `revokeNewImageUrls()` — cleans up object URLs for newly added (unsaved) images; also auto-called `onUnmounted`.


---
 
# Components
 
**Location:** `frontend/src/components/`
 
Reusable UI building blocks used by the views above.
 
## `auth/LoginModal.vue`
Global login/register modal, driven entirely by `useAuthModalStore` (mounted once in `App.vue`, opened via `useAuthModal`). Has two tabs (Entrar/Registrar). Validates username/password locally (non-empty, only letters/numbers/`.`/`_`) before submitting. On submit, calls `userStore.login()` or `userStore.register()` + `login()`, closes itself on success, shows a generic error on failure. Closes on Escape or backdrop click; locks page scroll while open.
 
## `common/LoadingSpinner.vue`
Small reusable loading indicator styled to match the app's "binder/corkboard" theme (animated dots + a spinning pencil icon). Props: `label` (text under the spinner, default `"Carregando…"`) and `size` (`sm | md | lg`). Used in `UploadView`, `EditOcModal`, and anywhere else a lightweight loading state is needed.
 
## `layout/AppHeader.vue`
Top navigation bar shown on every page. Shows the app logo/brand linking to `/hub`. On the right: if the user is logged in, shows their username and a "Sair" (logout) button; otherwise shows "Entrar"/"Registrar" buttons that open `LoginModal` via `useAuthModal`.
 
## `oc/EditOcModal.vue`
Modal form for editing an existing OC, opened from `OcDetailView` when the viewer is the owner. Mirrors `UploadView`'s form (name, species, sex, height via `useHeightMask`, description, images via `useImagePicker`), but pre-fills fields from the `oc` prop and resets them whenever the modal is reopened (`watch(() => props.open)`). On save, builds an `OcDraft` (splitting kept images by `originalIndex` from new files) and calls `useOcStore().updateOc(id, draft)`; emits `saved` and closes on success, or shows returned error messages on failure.
 
## `oc/ImageLightbox.vue`
*(uploaded as `ImageLightbox.vue`, referenced in the routing note as `ImagineLightbox.vue` — likely the same file, filename to confirm)*
A modal grid showing **all** images of an OC as clickable tiles; clicking one emits `select` with its index and closes. Controlled via `open`/`update:open` (v-model) and `images` props. This is distinct from the simple full-screen zoom lightbox built inline in `OcDetailView` — this component is a picker/overview grid, presumably used to jump to a specific image before zooming.
 
## `hub/OcCard.vue`
A single OC preview card shown in the hub grid. Displays the thumbnail (or a colored placeholder avatar/SVG if none exists), OC name, and author, linking to `/oc/:id`. Uses a deterministic "palette" (`id % 5`) to pick one of 5 tape/avatar color themes, giving each card a semi-random but stable decorative color (a small paper/washi-tape visual detail at the top of the card).
 
## `hub/OcGrid.vue`
Renders a responsive grid of `OcCard`s from a `items: PostMinified[]` prop, with staggered entrance animations and smooth add/remove transitions (`TransitionGroup`). Shows an empty-state message when `items` is empty. Used by `HubView` to display `store.minified`.
 
## `hub/SearchBar.vue`
Search input paired with a mode toggle (search by **OC** name vs by **Autor**/author), backed directly by `useOcStore` (`searchMode`, `applySearch`). Pressing Enter confirms the search term (`store.applySearch`); switching mode re-runs the search immediately if a term is already applied.
 
## `hub/UploadButton.vue`
Simple presentational button (icon + "Upload" label) that just emits a `click` event — the actual behavior (navigate to upload page or open login modal) is handled by the parent (`HubView`).
 