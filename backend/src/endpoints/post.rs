use actix_multipart::Multipart;
use actix_web::{HttpResponse, get, post, put, web};
use futures_util::StreamExt;
use sqlx::{Row, sqlite::SqliteRow};
use tracing::{error, warn, instrument};

use crate::{
    api_error::ApiError, appstate::AppState, error::Error, middleware::auth::AuthUser, schemas::{
        post::{CreatePost, EditPost, PostMetadata, PostMinified}, query::{PostQuery, is_query_valid}
    }, services::image::{update_images,MAX_IMAGES}
};

use serde::de::DeserializeOwned;

const MAX_METADATA_BYTES: usize = 64 * 1024;
const MAX_QUERY_POSTS : u32 = 30;
const MAX_IMAGE_BYTES: usize = 10 * 1024 * 1024;
//image path design = {user}/{oc_name}/{image_index}.{png}

#[instrument(skip(state, payload), fields(user = %auth.user_name))]
#[post("/api/v1/post")]
pub async fn create_post(
    state: web::Data<AppState>,
    auth: AuthUser,
    payload: Multipart,
) -> Result<HttpResponse, ApiError> {
    
    let (metadata, images): (CreatePost, Vec<Vec<u8>>) = parse_post_multipart(payload).await?;

    if metadata.sex.len() != 1 {
        return Err(ApiError::BadRequest("invalid request".into()));
    }

    if images.is_empty() {
        warn!("request contained no images");
        return Err(ApiError::BadRequest("invalid request".into()));
    }

    let saved_paths : Vec<String> = update_images(
        &auth.user_name,
        &metadata.oc_name,
        &state.image_repo_path,
        images
    ).await?;


    let post_id: i32 = sqlx::query_scalar(
        "INSERT INTO posts (oc_name, description, creator_user_name,specie, sex) VALUES ($1, $2, $3, $4, $5) RETURNING id"
    )
        .bind(&metadata.oc_name)
        .bind(&metadata.description)
        .bind(&auth.user_name)
        .bind(&metadata.specie)
        .bind(&metadata.sex)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!(error = %e, "failed to insert post into database");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "id": post_id,
        "images": saved_paths
    })))
}

#[instrument(skip(state, payload), fields(user = %auth.user_name))]
#[put("/api/v1/post")]
pub async fn update_post(
    state: web::Data<AppState>,
    auth: AuthUser,
    payload: Multipart,
) -> Result<HttpResponse, ApiError> {

    let (metadata, images): (EditPost, Vec<Vec<u8>>) = parse_post_multipart(payload).await?;

    //checks if the user is the creator of the post
    let post_id: Option<i32> = sqlx::query_scalar(
        "SELECT id FROM posts WHERE id = $1 AND creator_user_name = $2"
    )
        .bind(&metadata.id)
        .bind(&auth.user_name)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!(error = %e, "failed to check if user is creator of post");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    let Some(_post_id) = post_id else {
        return Err(ApiError::UnAuthorized("unauthorized".into()));
    };

    if images.is_empty() {
        warn!("request contained no images");
        return Err(ApiError::BadRequest("invalid request".into()));
    }

    let saved_paths : Vec<String> = update_images(
        &auth.user_name,
        &metadata.oc_name,
        &state.image_repo_path,
        images
    ).await?;

    sqlx::query(
        "UPDATE posts SET oc_name = $1, description = $2, specie = $3, sex = $4 WHERE id = $5 AND creator_user_name = $6 RETURNING id"
    )
        .bind(&metadata.oc_name)
        .bind(&metadata.description)
        .bind(&metadata.specie)
        .bind(&metadata.sex)
        .bind(&metadata.id)
        .bind(&auth.user_name)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!(error = %e, "failed to update post in database");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;


    Ok(HttpResponse::Created().json(serde_json::json!({
        "id": metadata.id,
        "images": saved_paths
    })))
}


async fn parse_post_multipart<T>(
    mut payload: Multipart,
) -> Result<(T, Vec<Vec<u8>>), ApiError>
where
    T: DeserializeOwned,
{
    let mut metadata: Option<T> = None;
    let mut images: Vec<Vec<u8>> = Vec::new();

    while let Some(field) = payload.next().await {
        let mut field = field.map_err(|e| {
            error!(error = %e, "failed to read multipart field");
            ApiError::BadRequest("invalid request".into())
        })?;

        match field.name().unwrap_or("") {
            "metadata" => {
                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk.map_err(|e| {
                        error!(error = %e, "failed to read metadata field bytes");
                        ApiError::BadRequest("invalid request".into())
                    })?;

                    if bytes.len() + chunk.len() > MAX_METADATA_BYTES {
                        warn!("metadata exceeds max allowed size");
                        return Err(ApiError::BadRequest("invalid request".into()));
                    }

                    bytes.extend_from_slice(&chunk);
                }

                metadata = Some(
                    serde_json::from_slice::<T>(&bytes).map_err(|e| {
                        warn!(error = %e, "malformed metadata json received");
                        ApiError::BadRequest("invalid request".into())
                    })?
                );
            }
            //will read through the chunks of data and build the images
            "images" => {
                if images.len() >= MAX_IMAGES {
                    warn!(count = images.len(), "too many images in request");
                    return Err(ApiError::BadRequest("invalid request".into()));
                }

                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk.map_err(|e| {
                        error!(error = %e, "failed to read image chunk");
                        ApiError::BadRequest("invalid request".into())
                    })?;

                    if bytes.len() + chunk.len() > MAX_IMAGE_BYTES {
                        warn!("image exceeds max allowed size");
                        return Err(ApiError::BadRequest("invalid request".into()));
                    }

                    bytes.extend_from_slice(&chunk);
                }

                images.push(bytes);
            }

            other => {
                warn!(field = %other, "ignoring unknown multipart field");
            }
        }
    }

    let metadata = metadata.ok_or_else(|| {
        warn!("request missing required 'metadata' field");
        ApiError::BadRequest("invalid request".into())
    })?;

    Ok((metadata, images))
}




//===============================public api===============================



#[get("/api/v1/posts/{type}/{query}")]
pub async fn query_posts(
    state: web::Data<AppState>,
    query_params : web::Path<PostQuery>
) -> Result<HttpResponse, ApiError> {
    let query_params = query_params.into_inner();

    let query : String = if is_query_valid(&query_params) {
        query_params.query
    } else {
        "".into()
    };

    let mut query_posts : Vec<PostMinified> = Vec::new();

    //if is searching by oc_name or creator_name
    let posts : Vec<SqliteRow> = 
    if query_params.query_type == "C"{
        sqlx::query(
            "SELECT id, oc_name, creator_user_name FROM posts WHERE oc_name LIKE $1 LIMIT $2"
        )
            .bind(format!("{}%", &query))
            .bind(MAX_QUERY_POSTS)
            .fetch_all(&state.db)
            .await
            .map_err(|e| {
                error!(error = %e, "database error while querying posts");
                ApiError::Internal(Error::Other("internal server error".into()))
            })?
    }else{
        sqlx::query(
            "SELECT id, oc_name, creator_user_name FROM posts WHERE creator_user_name LIKE $1 LIMIT $2"
        )
            .bind(format!("{}%", &query))
            .bind(MAX_QUERY_POSTS)
            .fetch_all(&state.db)
            .await
            .map_err(|e| {
                error!(error = %e, "database error while querying posts");
                ApiError::Internal(Error::Other("internal server error".into()))
            })?
    };

    for post in posts {
        query_posts.push(PostMinified {
            id: post.get("id"),
            oc_name: post.get("oc_name"),
            creator_user_name: post.get("creator_user_name")
        });
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "posts": query_posts,
        "total": query_posts.len()
    })))
}

#[get("/api/v1/post/{id}")]
pub async fn get_post(
    state: web::Data<AppState>,
    post_id : web::Path<String>
) -> Result<HttpResponse, ApiError>{

    let post_id = post_id.into_inner();

    if post_id.is_empty() {
        return Err(ApiError::BadRequest("invalid request".into()));
    }

    //if post_id is not numerical
    if post_id.parse::<u32>().is_err() {
        return Err(ApiError::BadRequest("invalid request".into()));
    }

    let post : Option<SqliteRow> = sqlx::query(
        "SELECT id, creator_user_name, oc_name, description , specie, sex FROM posts WHERE id = $1"
    )
        .bind(post_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!(error = %e, "database error while fetching post metadata");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;


    let post_metadata = match post{
        Some(row) => PostMetadata::from_row(row, &state.image_repo_path).await?,
        None => return Err(ApiError::NotFound("post not found".into()))
    };
    
    Ok(HttpResponse::Ok().json(post_metadata))
}