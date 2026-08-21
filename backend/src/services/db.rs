use crate::{api_error::ApiError,error::Error, schemas::post::PostMetadata};
use sqlx::{Pool, Sqlite, sqlite::SqliteRow};
use tracing::{error};


pub async fn get_post_from_db(
    id : &i32,
    db : &Pool<Sqlite>,
    image_repo : &str,
    search_images : bool
) -> Result<PostMetadata ,ApiError>{
    let post_row : Option<SqliteRow> = sqlx::query(
        "SELECT id, creator_user_name, oc_name, description, height , specie, sex FROM posts WHERE id = $1"
    )
        .bind(&id)
        .fetch_optional(db)
        .await
        .map_err(|e| {
            error!(error = %e, "database error while fetching post metadata");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    let Some(post_row) = post_row else {
        return Err(ApiError::NotFound("Not Found".into()));
    };

    Ok(
        PostMetadata::from_row(
            post_row,
            image_repo,
            search_images
        ).await?
    )
}