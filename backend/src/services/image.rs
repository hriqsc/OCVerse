use tracing::{error, warn};
use tokio::fs;
use futures::future::{join_all, try_join_all};

use crate::{api_error::ApiError, error::Error, shared::sanitization::sanitize_path_segment};


pub const MAX_IMAGES: usize = 6;
pub const EXT : &str = "png";



//image path design = image_repo_path/{user}/{oc_name}/{image_index}.{png}
pub async fn update_images(
    user: &str,
    oc_name: &str,
    image_repo_path: &str,
    images: Vec<Vec<u8>>,
    mut existing_images: Vec<i32>,
) -> Result<Vec<String>, ApiError> {
    if images.len() > MAX_IMAGES {
        warn!(count = images.len(), "too many image slots in request");
        return Err(ApiError::BadRequest("invalid request".into()));
    }

    existing_images.sort_unstable();
    let is_existing: [bool; MAX_IMAGES] =
        std::array::from_fn(|i| existing_images.binary_search(&(i as i32)).is_ok());

    let safe_user    = sanitize_path_segment(user)?;
    let safe_oc_name = sanitize_path_segment(oc_name)?;
    let dir_path = format!("{}/{}/{}", image_repo_path, safe_user, safe_oc_name);

    tokio::fs::create_dir_all(&dir_path).await.map_err(|e| {
        error!(path = %dir_path, error = %e, "failed to create upload directory");
        ApiError::Internal(Error::Other("internal server error".into()))
    })?;

    let mut saved_paths = Vec::new();
    let mut write_futs = Vec::new();
    let mut delete_futs = Vec::new();

    for slot in 0..MAX_IMAGES {
        let file_path = format!("{}/{}.{}", dir_path, slot, EXT);
        let bytes = images.get(slot).cloned().unwrap_or_default();

        if !bytes.is_empty() {
            saved_paths.push(file_path.clone());
            write_futs.push(async move {
                tokio::fs::write(&file_path, &bytes).await.map_err(|e| {
                    error!(path = %file_path, error = %e, "failed to write image file");
                    ApiError::Internal(Error::Other("internal server error".into()))
                })
            });
        } else if !is_existing[slot] {
            delete_futs.push(async move {
                let _ = tokio::fs::remove_file(&file_path).await;
            });
        }
    }

    try_join_all(write_futs).await.map_err(|e: ApiError| e)?;
    join_all(delete_futs).await;

    Ok(saved_paths)
}

pub async fn delete_post_images(
    user: &str,
    oc_name: &str,
    image_repo_path: &str
) -> Result<(), ApiError> {
    let safe_user = sanitize_path_segment(user)
        .map_err(|_| ApiError::BadRequest("invalid user".into()))?;

    let safe_oc_name = sanitize_path_segment(oc_name)
        .map_err(|_| ApiError::BadRequest("invalid oc_name".into()))?;

    let dir_path = format!("{}/{}/{}", image_repo_path, safe_user, safe_oc_name);

    if let Err(e) = tokio::fs::remove_dir_all(&dir_path).await {
        error!(path = %dir_path, error = %e, "failed to delete post images");
        return Err(ApiError::Internal(
            Error::Other("internal server error".into()).into(),
        ));
    }

    Ok(())
}


//id -> /f/v1/user/oc_name/index.png
pub async fn get_images(
    user: &str,
    oc_name: &str,
    image_repo_path: &str
) -> Result<Vec<String>, Error> {
    let mut images_ids : Vec<String> = Vec::new();

    let safe_user = sanitize_path_segment(user)
        .map_err(|_| Error::Other("invalid user".into()))?;

    let safe_oc_name = sanitize_path_segment(oc_name)
        .map_err(|_| Error::Other("invalid oc_name".into()))?;

    let dir_path = format!("{}/{}/{}",image_repo_path, safe_user, safe_oc_name);

    for i in 0..6 {
        let full_path = format!("{}/{}.{}", dir_path, i, EXT);

        match fs::try_exists(&full_path).await{
            Ok(true) => images_ids.push(format!("/f/v1/{}/{}/{}.{}", safe_user, safe_oc_name, i,EXT)),
            Ok(false) | Err(_) => continue
        }

    }

    Ok(images_ids)
}

//get first images in the file dir
pub async fn get_thumb(
    user: &str,
    oc_name: &str,
    image_repo_path: &str
) -> Result<String, Error> {
    let safe_user = sanitize_path_segment(user)
        .map_err(|_| Error::Other("invalid user".into()))?;

    let safe_oc_name = sanitize_path_segment(oc_name)
        .map_err(|_| Error::Other("invalid oc_name".into()))?;

    let dir_path = format!("{}/{}/{}",image_repo_path, safe_user, safe_oc_name);

    for i in 0..6 {
        let full_path = format!("{}/{}.{}", dir_path, i, EXT);

        if fs::try_exists(&full_path).await? {
            return Ok(format!("/f/v1/{}/{}/{}.{}", safe_user, safe_oc_name, i,EXT));
        }
    }

    Err(Error::Other("Image Not Found".into()))
}