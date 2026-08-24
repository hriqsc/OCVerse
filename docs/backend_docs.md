# API Documentation

Base path prefix for all endpoints: `/api/v1`

All request/response bodies are JSON unless noted otherwise (the post creation/update endpoints use `multipart/form-data`).

---

## Table of Contents

- [User Endpoints](#user-endpoints)
  - [POST /api/v1/user/register](#post-apiv1userregister)
  - [POST /api/v1/user/login](#post-apiv1userlogin)
  - [POST /api/v1/user/logout](#post-apiv1userlogout)
  - [POST /api/v1/user/refresh](#post-apiv1userrefresh)
  - [GET /api/v1/user/reset/{id}](#get-apiv1userresetid)
  - [POST /api/v1/user/reset](#post-apiv1userreset)
- [Post Endpoints](#post-endpoints)
  - [POST /api/v1/post](#post-apiv1post)
  - [PUT /api/v1/post](#put-apiv1post)
  - [DELETE /api/v1/post/{id}](#delete-apiv1postid)
  - [GET /api/v1/posts](#get-apiv1posts)
  - [GET /api/v1/post/{id}](#get-apiv1postid)
- [Magma Endpoints](#magma-endpoints)
  - [POST /api/v1/magma](#post-apiv1magma)
  - [GET /api/v1/magmas](#get-apiv1magmas)
- [Schemas](#schemas)
- [Common Error Format](#common-error-format)

---

## User Endpoints

### POST /api/v1/user/register

Creates a new user account.

**Auth required:** No

**Request body** (`UserLogin`):
```json
{
  "user_name": "string",
  "password": "string"
}
```

**Behavior:**
- Rejects an empty request body.
- Checks whether `user_name` already exists; if so, returns `409 Conflict`.
- Hashes the password with Argon2 and stores the new user.

**Responses:**
| Status | Description |
|---|---|
| 201 Created | User created successfully (empty body) |
| 400 Bad Request | Empty or malformed JSON body |
| 409 Conflict | Username already taken |
| 500 Internal Server Error | Password hashing or database failure |

---

### POST /api/v1/user/login

Authenticates a user and starts a session.

**Auth required:** No

**Request body** (`UserLogin`):
```json
{
  "user_name": "string",
  "password": "string"
}
```

**Behavior:**
- Looks up the user by `user_name`.
- If the user doesn't exist, performs a dummy Argon2 verification to keep response timing consistent (mitigates user-enumeration via timing attacks), then returns `401`.
- Verifies the provided password against the stored Argon2 hash.
- On success, creates a session and issues an access token plus a `refresh_token` cookie (`HttpOnly`, `SameSite=Strict`, 7-day expiry, path `/api/v1`).

**Responses:**
| Status | Description |
|---|---|
| 200 OK | Returns `{ "access_token": "string" }` and sets `refresh_token` cookie |
| 400 Bad Request | Empty or malformed JSON body |
| 401 Unauthorized | Invalid username or password |
| 500 Internal Server Error | Database, hashing, session, or token generation failure |

---

### POST /api/v1/user/logout

Revokes the current session.

**Auth required:** No (relies on `refresh_token` cookie if present)

**Request body:** None

**Behavior:**
- If a `refresh_token` cookie is present, revokes the associated session.
- Clears the `refresh_token` cookie (sets `Max-Age=0`).

**Responses:**
| Status | Description |
|---|---|
| 200 OK | Logout completed (empty body), cookie cleared |
| 500 Internal Server Error | Failure revoking the session |

---

### POST /api/v1/user/refresh

Rotates an access token/session using the refresh token cookie.

**Auth required:** Requires a valid `refresh_token` cookie

**Request body:** None

**Behavior:**
- Reads `refresh_token` from cookies; missing cookie → `401`.
- Validates the refresh token; invalid/expired → `401`.
- Revokes the old session and creates a new one.
- Issues a new access token and a new `refresh_token` cookie.

**Responses:**
| Status | Description |
|---|---|
| 200 OK | Returns `{ "access_token": "string", "user_name": "string" }` and sets new `refresh_token` cookie |
| 401 Unauthorized | Missing, invalid, or expired refresh token |
| 500 Internal Server Error | Failure revoking/creating session or generating token |

---

### GET /api/v1/user/reset/{id}

Checks whether a password-reset request exists for a given id.

**Auth required:** No

**Path params:**
| Name | Type | Description |
|---|---|---|
| id | integer | Reset login request id |

**Responses:**
| Status | Description |
|---|---|
| 200 OK | Reset id exists (empty body) |
| 404 Not Found | No reset request found for that id |
| 500 Internal Server Error | Database failure |

---

### POST /api/v1/user/reset

Completes a password reset using a reset request id.

**Auth required:** No (authorization is implicit via the reset id, presumably issued out-of-band, e.g. email)

**Request body** (`UserResetPassword`):
```json
{
  "id": 0,
  "new_password": "string"
}
```

**Behavior:**
- Looks up the `user_name` tied to the reset request `id`; not found → `404`.
- Hashes and updates the user's password.
- Deletes the reset request row.

**Responses:**
| Status | Description |
|---|---|
| 200 OK | Password reset successfully (empty body) |
| 404 Not Found | Reset request id not found |
| 500 Internal Server Error | Hashing or database failure |

---

## Post Endpoints

> "Post" here refers to a user-created OC (original character) submission with metadata and images.

### POST /api/v1/post

Creates a new post with images.

**Auth required:** Yes (`AuthUser` — access token)

**Content-Type:** `multipart/form-data`

**Multipart fields:**
| Field | Type | Notes |
|---|---|---|
| `metadata` | JSON (`CreatePost`) | Max 64 KB |
| `images` | binary (repeatable) | Max 10 MB per image, max `MAX_IMAGES` images total (extras are silently ignored) |

**`CreatePost` schema:**
```json
{
  "oc_name": "string",
  "description": "string",
  "specie": "string",
  "sex": "string",
  "height": "string"
}
```

**Behavior:**
- `sex` must be exactly 1 character.
- Runs field validation (`validate_post_create_post`); validation errors return `400` with a message.
- At least one image is required.
- Stores images under `{image_repo_path}/{user_name}/{oc_name}/`.
- Inserts the post row and returns the created post metadata, including uploaded image URLs.

**Responses:**
| Status | Description |
|---|---|
| 201 Created | Returns `PostMetadata` |
| 400 Bad Request | Invalid `sex` length, failed validation, no images, malformed multipart |
| 401 Unauthorized | Missing/invalid auth |
| 500 Internal Server Error | Image storage or database failure |

---

### PUT /api/v1/post

Updates an existing post (metadata and/or images).

**Auth required:** Yes (`AuthUser`) — only the post's original creator may update it

**Content-Type:** `multipart/form-data`

**Multipart fields:**
| Field | Type | Notes |
|---|---|---|
| `metadata` | JSON (`EditPost`) | Max 64 KB |
| `images` | binary (repeatable) | New images to add; max 10 MB per image, max `MAX_IMAGES` total |

**`EditPost` schema:**
```json
{
  "id": 0,
  "oc_name": "string",
  "description": "string",
  "sex": "string",
  "specie": "string",
  "height": "string",
  "existing_images": [0]
}
```
`existing_images` lists the indices/ids of previously uploaded images to keep; anything not listed is removed.

**Behavior:**
- Validates the payload (`validate_post_edit_post`).
- Fetches the existing post; the requester must be the original creator, otherwise `401`.
- If `oc_name` changed, renames the image directory on disk.
- Updates stored images (adds new ones, prunes ones not in `existing_images`).
- Updates the post row; if no row matched (id + creator mismatch), returns `401`.

**Responses:**
| Status | Description |
|---|---|
| 200 OK | Returns updated `PostMetadata` |
| 400 Bad Request | Failed validation or malformed multipart |
| 401 Unauthorized | Not the post owner, or update matched no rows |
| 500 Internal Server Error | Filesystem, image, or database failure |

---

### DELETE /api/v1/post/{id}

Deletes a post and its images.

**Auth required:** Yes (`AuthUser`) — only the post's creator can delete it

**Path params:**
| Name | Type | Description |
|---|---|---|
| id | string | Post id |

**Behavior:**
- Looks up `oc_name` for the post scoped to `id` + `creator_user_name`.
- Deletes the images on disk, then deletes the post row (scoped to `id` + `creator_user_name`).

**Responses:**
| Status | Description |
|---|---|
| 200 OK | Post deleted (empty body) |
| 401 Unauthorized | Missing/invalid auth |
| 500 Internal Server Error | Post not found for that user, image deletion failure, or database failure |

---

### GET /api/v1/posts

Searches/lists posts (public listing with thumbnails).

**Auth required:** No

**Query params** (`PostQuery`):
| Name | Type | Description |
|---|---|---|
| `type` | string, optional | Search field: `"C"` = search by `oc_name`, `"U"` = search by `creator_user_name`, anything else = no filter |
| `query` | string, optional | Prefix to match (`LIKE 'query%'`) against the selected field |

**Behavior:**
- Results are ordered by `id DESC`, capped at `MAX_QUERY_POSTS` (30).
- Each result includes a generated thumbnail.

**Responses:**
| Status | Description |
|---|---|
| 200 OK | Returns `{ "posts": PostMinified[], "total": number }` |
| 500 Internal Server Error | Database or thumbnail generation failure |

---

### GET /api/v1/post/{id}

Fetches full metadata (including images) for a single post.

**Auth required:** No

**Path params:**
| Name | Type | Description |
|---|---|---|
| id | string (numeric) | Post id |

**Responses:**
| Status | Description |
|---|---|
| 200 OK | Returns `PostMetadata` |
| 400 Bad Request | Empty or non-numeric `id` |
| 404 Not Found | Post does not exist |
| 500 Internal Server Error | Database failure |

---

## Magma Endpoints

### POST /api/v1/magma

Registers a new "magma" entry. Internal/service endpoint protected by a shared secret header (not user auth).

**Auth required:** Shared secret via `secret` request header (hashed with FNV-1a and compared against `state.secret_code`)

**CORS:** Permissive (`Cors::permissive()`)

**Request body** (`MagmaInsert`):
```json
{
  "url": "string",
  "time_stamp": 0
}
```

**Behavior:**
- Missing/incorrect `secret` header → `401 Unauthorized` (generic message, to avoid leaking details).
- `url` length must be between 1 and 20 characters, otherwise `401 Unauthorized` (same generic error).
- Inserts a row `(id = url, created_at = time_stamp)` into the `magmas` table.

**Responses:**
| Status | Description |
|---|---|
| 200 OK | Magma created (empty body) |
| 400 Bad Request | Malformed JSON body |
| 401 Unauthorized | Missing/incorrect secret header, or `url` outside allowed length |
| 500 Internal Server Error | Database failure |

> Note: an invalid `url` length currently returns `401` rather than `400` — worth double-checking if that's intentional, since it's a validation error rather than an auth error.

---

### GET /api/v1/magmas

Lists all magma ids.

**Auth required:** No

**Responses:**
| Status | Description |
|---|---|
| 200 OK | Returns `{ "magmas_id": string[] }`, ordered by `created_at DESC` |
| 500 Internal Server Error | Database failure |

---

## Schemas

### `UserLogin`
```json
{ "user_name": "string", "password": "string" }
```

### `Session`
```json
{ "session_id": "string", "refresh_token": "string" }
```

### `UserResetPassword`
```json
{ "id": 0, "new_password": "string" }
```

### `CreatePost`
```json
{
  "oc_name": "string",
  "description": "string",
  "specie": "string",
  "sex": "string",
  "height": "string"
}
```

### `EditPost`
```json
{
  "id": 0,
  "oc_name": "string",
  "description": "string",
  "sex": "string",
  "specie": "string",
  "height": "string",
  "existing_images": [0]
}
```

### `PostMetadata`
```json
{
  "id": 0,
  "creator_user_name": "string",
  "oc_name": "string",
  "description": "string",
  "specie": "string",
  "sex": "string",
  "height": "string",
  "images": ["string"]
}
```

### `PostMinified`
```json
{
  "id": 0,
  "creator_user_name": "string",
  "oc_name": "string",
  "thumb": "string"
}
```

### `PostQuery`
```json
{ "type": "string | null", "query": "string | null" }
```

### `MagmaInsert`
```json
{ "url": "string", "time_stamp": 0 }
```

---

## Common Error Format

All errors are returned via `ApiError`, generally as a JSON body describing the failure, with the following status codes used across endpoints:

| Status | Meaning |
|---|---|
| 400 Bad Request | Malformed input or failed validation |
| 401 Unauthorized | Missing/invalid credentials, auth token, or ownership check failed |
| 404 Not Found | Resource does not exist |
| 409 Conflict | Resource already exists (e.g. username taken) |
| 500 Internal Server Error | Unexpected server-side failure (database, filesystem, hashing, etc.) |

Internal error details are logged server-side (via `tracing`) but never exposed in the response body — clients only receive a generic `"internal server error"` message for 500s.