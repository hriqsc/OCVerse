use tokio::io::AsyncWriteExt;
use tracing::error;
use tokio::fs;

use crate::{api_error::ApiError, error::Error, shared::sanitization::sanitize_path_segment};


pub const MAX_IMAGES: usize = 6;
pub const EXT : &str = "png";
//image path design = image_repo_path/{user}/{oc_name}/{image_index}.{png}


pub async fn update_images(
    user: &str,
    oc_name: &str,
    image_repo_path: &str,
    images: Vec<Vec<u8>>
) -> Result<Vec<String>, ApiError> {

    let safe_user    = sanitize_path_segment(&user)?;
    let safe_oc_name = sanitize_path_segment(&oc_name)?;

    let dir_path = format!("{}/{}/{}",image_repo_path, safe_user, safe_oc_name);

    let mut saved_paths : Vec<String> = Vec::new();

    tokio::fs::create_dir_all(&dir_path).await.map_err(|e| {
        error!(path = %dir_path, error = %e, "failed to create upload directory");
        ApiError::Internal(Error::Other("internal server error".into()))
    })?;

    for (index, bytes) in images.into_iter().enumerate() {
        let file_path = format!("{}/{}.{}", dir_path, index, EXT);

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

    Ok(saved_paths)
}


//id -> user/oc_name/index
pub async fn get_images_ids(
    user: &str,
    oc_name: &str,
    image_repo_path: &str
) -> Result<Vec<String>, Error> {
    let mut images_ids : Vec<String> = Vec::new();

    let dir_path = format!("{}/{}/{}",image_repo_path, user, oc_name);

    for i in 0..6 {
        let full_path = format!("{}/{}{}", dir_path, i, EXT);

        if fs::try_exists(&full_path).await? {
            images_ids.push(format!("{}/{}/{}", user, oc_name, i));
        }
    }

    Ok(images_ids)
}