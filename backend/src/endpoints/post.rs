use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse};
use futures_util::StreamExt;
use tokio::io::AsyncWriteExt;
use tracing::{error, warn, info, instrument};

use crate::{
    api_error::ApiError, appstate::AppState, error::Error,
    middleware::auth::AuthUser,
    schemas::post::PostMetadata,
};

const MAX_IMAGES: usize = 4;
const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB per image
const ALLOWED_MIME: [(&str, &str); 3] = [
    ("image/png", "png"),
    ("image/jpeg", "jpg"),
    ("image/webp", "webp"),
];

#[instrument(skip(state, payload), fields(user = %auth.user_name))]
#[post("/api/v1/post")]
pub async fn create_post(
    state: web::Data<AppState>,
    auth: AuthUser,
    mut payload: Multipart,
) -> Result<HttpResponse, ApiError> {
    let mut metadata: Option<PostMetadata> = None;
    let mut images: Vec<(Vec<u8>, &'static str)> = Vec::new();

    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|e| {
            error!(error = %e, "failed to read multipart field");
            ApiError::BadRequest("invalid request".into())
        })?;

        let field_name = field.name().unwrap_or("").to_string();

        match field_name.as_str() {
            "metadados" => {
                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    bytes.extend_from_slice(&chunk.map_err(|e| {
                        error!(error = %e, "failed to read metadata field bytes");
                        ApiError::BadRequest("invalid request".into())
                    })?);
                }

                metadata = Some(
                    serde_json::from_slice::<PostMetadata>(&bytes).map_err(|e| {
                        warn!(error = %e, "malformed metadata json received");
                        ApiError::BadRequest("invalid request".into())
                    })?
                );
            }

            "images" => {
                if images.len() >= MAX_IMAGES {
                    warn!(count = images.len(), "too many images in request");
                    return Err(ApiError::BadRequest("invalid request".into()));
                }

                let content_type = field.content_type().map(|m| m.to_string()).unwrap_or_default();

                let ext = ALLOWED_MIME
                    .iter()
                    .find(|(mime, _)| *mime == content_type)
                    .map(|(_, ext)| *ext)
                    .ok_or_else(|| {
                        warn!(content_type = %content_type, "rejected disallowed mime type");
                        ApiError::BadRequest("invalid request".into())
                    })?;

                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk.map_err(|e| {
                        error!(error = %e, "failed to read image chunk");
                        ApiError::BadRequest("invalid request".into())
                    })?;
                    bytes.extend_from_slice(&chunk);

                    if bytes.len() > MAX_IMAGE_SIZE {
                        warn!(size = bytes.len(), "image exceeded max size");
                        return Err(ApiError::BadRequest("invalid request".into()));
                    }
                }

                images.push((bytes, ext));
            }

            other => {
                warn!(field = %other, "ignoring unknown multipart field");
            }
        }
    }

    let metadata = metadata.ok_or_else(|| {
        warn!("request missing required 'metadados' field");
        ApiError::BadRequest("invalid request".into())
    })?;

    if images.is_empty() {
        warn!("request contained no images");
        return Err(ApiError::BadRequest("invalid request".into()));
    }

    let safe_user = sanitize_path_segment(&auth.user_name)?;
    let safe_oc_name = sanitize_path_segment(&metadata.oc_name)?;

    let dir_path = format!("images/{}/{}", safe_user, safe_oc_name);

    tokio::fs::create_dir_all(&dir_path).await.map_err(|e| {
        error!(path = %dir_path, error = %e, "failed to create upload directory");
        ApiError::Internal(Error::Other("internal server error".into()))
    })?;

    info!(path = %dir_path, "upload directory ready");

    let mut saved_paths = Vec::new();

    for (index, (bytes, ext)) in images.into_iter().enumerate() {
        let file_path = format!("{}/{}.{}", dir_path, index, ext);

        let mut file = tokio::fs::File::create(&file_path).await.map_err(|e| {
            error!(path = %file_path, error = %e, "failed to create image file");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

        file.write_all(&bytes).await.map_err(|e| {
            error!(path = %file_path, error = %e, "failed to write image file");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

        saved_paths.push(file_path);
    }

    let post_id: i32 = sqlx::query_scalar(
        "INSERT INTO posts (oc_name, description, creator_name) VALUES ($1, $2, $3) RETURNING id"
    )
        .bind(&metadata.oc_name)
        .bind(&metadata.description)
        .bind(&auth.user_name)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!(error = %e, "failed to insert post into database");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    info!(post_id, images = saved_paths.len(), "post created successfully");

    Ok(HttpResponse::Created().json(serde_json::json!({
        "id": post_id,
        "images": saved_paths
    })))
}

fn sanitize_path_segment(input: &str) -> Result<String, ApiError> {
    let trimmed = input.trim();

    if trimmed.is_empty()
        || trimmed.contains('/')
        || trimmed.contains('\\')
        || trimmed.contains("..")
    {
        warn!(value = %input, "rejected unsafe path segment");
        return Err(ApiError::BadRequest("invalid request".into()));
    }

    Ok(trimmed.to_string())
}