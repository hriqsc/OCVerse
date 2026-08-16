use actix_multipart::Multipart;
use actix_web::{HttpResponse, get, post, put, web,delete};
use futures_util::StreamExt;
use sqlx::{Row, sqlite::SqliteRow};
use tracing::{error, warn, instrument};
use futures::future::try_join_all;
use sqlx::QueryBuilder;


use crate::{
    api_error::ApiError, appstate::AppState, error::Error, middleware::auth::AuthUser, schemas::{
        post::{CreatePost, EditPost, PostMetadata, PostMinified}, query::PostQuery
    }, services::image::{MAX_IMAGES, delete_post_images, get_images, get_thumb, update_images}, validator::post_validator::{
        validate_post_create_post,
        validate_post_edit_post
    }
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

    let validation = validate_post_create_post(&metadata);
    if validation.len() != 0 {
        return Err(ApiError::BadRequest(validation));
    }

    if images.is_empty() {
        warn!("request contained no images");
        return Err(ApiError::BadRequest("Você precisa enviar pelo menos uma imagem".into()));
    }

    update_images(
        &auth.user_name,
        &metadata.oc_name,
        &state.image_repo_path,
        images,
        Vec::new()
    ).await?;


    let post_id: i32 = sqlx::query_scalar(
        "INSERT INTO posts (oc_name, description, creator_user_name,height,specie, sex) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
    )
        .bind(&metadata.oc_name)
        .bind(&metadata.description)
        .bind(&auth.user_name)
        .bind(&metadata.height)
        .bind(&metadata.specie)
        .bind(&metadata.sex)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!(error = %e, "failed to insert post into database");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;


    let return_json = PostMetadata{
        id : post_id,
        creator_user_name : auth.user_name.clone(),
        oc_name : metadata.oc_name.clone(),
        description : metadata.description,
        specie : metadata.specie,
        sex : metadata.sex,
        height : metadata.height,
        images : get_images(
            &auth.user_name,
            &metadata.oc_name,
            &state.image_repo_path
        ).await?,
    };

    Ok(HttpResponse::Created().json(return_json))
}

#[instrument(skip(state, payload), fields(user = %auth.user_name))]
#[put("/api/v1/post")]
pub async fn update_post(
    state: web::Data<AppState>,
    auth: AuthUser,
    payload: Multipart,
) -> Result<HttpResponse, ApiError> {

    let (metadata, images): (EditPost, Vec<Vec<u8>>) = parse_post_multipart(payload).await?;

    let validation = validate_post_edit_post(&metadata);
    if validation.len() != 0 {
        return Err(ApiError::BadRequest(validation));
    }

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

    update_images(
        &auth.user_name,
        &metadata.oc_name,
        &state.image_repo_path,
        images,
        metadata.existing_images
    ).await?;

    let result = sqlx::query(
        "UPDATE posts SET 
            oc_name = $1,
            description = $2,
            specie = $3,
            sex = $4,
            height = $5
        WHERE id = $6
        AND creator_user_name = $7 
        RETURNING id"
    )
        .bind(&metadata.oc_name)
        .bind(&metadata.description)
        .bind(&metadata.specie)
        .bind(&metadata.sex)
        .bind(&metadata.height)
        .bind(&metadata.id)
        .bind(&auth.user_name)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!(error = %e, "failed to update post in database");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    if result.rows_affected() == 0 {
        warn!(id = %metadata.id, "update matched no rows");
        return Err(ApiError::UnAuthorized("unauthorized".into()));
    }

    let return_json = PostMetadata{
        id : metadata.id,
        creator_user_name : auth.user_name.clone(),
        oc_name : metadata.oc_name.clone(),
        description : metadata.description,
        specie : metadata.specie,
        sex : metadata.sex,
        height : metadata.height,
        images : get_images(
            &auth.user_name,
            &metadata.oc_name,
            &state.image_repo_path
        ).await?,
    };

    Ok(HttpResponse::Ok().json(return_json))
}


#[instrument(skip(state), fields(user = %auth.user_name))]
#[delete("/api/v1/post/{id}")]
pub async fn delete_post(
    state: web::Data<AppState>,
    auth: AuthUser,
    post_id: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    let post_id = post_id.into_inner();

    let oc_name: String = sqlx::query_scalar(
        "SELECT oc_name FROM posts WHERE id = $1 AND creator_user_name = $2"
    )
        .bind(&post_id)
        .bind(&auth.user_name)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!(error = %e, "failed to fetch oc_name for post");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;
    
    if oc_name.is_empty() {
        return Err(ApiError::Internal(Error::Other("internal server error".into())));
    }

    delete_post_images(
        &auth.user_name,
        &oc_name,
        &state.image_repo_path
    ).await?;

    sqlx::query(
        "DELETE FROM posts WHERE id = $1 AND creator_user_name = $2"
    )
        .bind(&post_id)
        .bind(&auth.user_name)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!(error = %e, "failed to delete post in database");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    Ok(HttpResponse::Ok().json(""))
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
                    continue;
                }

                let mut bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let chunk = chunk.map_err(|e| {
                        error!(error = %e, "failed to read image chunk");
                        ApiError::BadRequest("failed to read image".into())
                    })?;

                    if bytes.len() + chunk.len() > MAX_IMAGE_BYTES {
                        warn!("image exceeds max allowed size");
                        return Err(ApiError::BadRequest("image exceeds max allowed size".into()));
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

#[get("/api/v1/posts")]
pub async fn query_posts(
    state: web::Data<AppState>,
    query_params: web::Query<PostQuery>,
) -> Result<HttpResponse, ApiError> {
    let query_params = query_params.into_inner();
    let query = query_params.query.unwrap_or_default();
    let query_type = query_params.query_type.unwrap_or_default();

    // monta a query dinamicamente, sem repetir SQL 3x
    let mut qb = QueryBuilder::new(
        "SELECT id, oc_name, creator_user_name FROM posts"
    );

    let column = match query_type.as_str() {
        "C" => Some("oc_name"),
        "U" => Some("creator_user_name"),
        _ => None,
    };

    if let Some(col) = column {
        qb.push(" WHERE ")
            .push(col)
            .push(" LIKE ")
            .push_bind(format!("{}%", query));
    }

    qb.push(" ORDER BY id DESC LIMIT ")
        .push_bind(MAX_QUERY_POSTS);

    let posts: Vec<SqliteRow> = qb
        .build()
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            error!(error = %e, "database error while querying posts");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    let query_posts: Vec<PostMinified> = try_join_all(posts.into_iter().map(|post| {
        let image_repo_path = &state.image_repo_path;
        async move {
            let id: i32 = post.try_get("id")?;
            let oc_name: String = post.try_get("oc_name")?;
            let creator_user_name: String = post.try_get("creator_user_name")?;
            let thumb = get_thumb(&creator_user_name, &oc_name, image_repo_path).await?;

            Ok::<_, ApiError>(PostMinified {
                id,
                oc_name,
                creator_user_name,
                thumb,
            })
        }
    }))
    .await?;

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
        "SELECT id, creator_user_name, oc_name, description, height , specie, sex FROM posts WHERE id = $1"
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