use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::{
    error::Error,
    services::image::{get_images_ids},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMetadata {
    pub id : i32,
    pub creator_user_name: String,
    pub oc_name: String,
    pub description: String,
    pub specie : String,
    pub sex : String,
    pub images: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMinified {
    pub id : i32,
    pub creator_user_name: String,
    pub oc_name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePost{
    pub oc_name: String,
    pub description: String,
    pub specie : String,
    pub sex : String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EditPost{
    pub id : i32,
    pub oc_name: String,
    pub description: String,
    pub sex : String,
    pub specie : String,
}



impl PostMetadata{
    pub async fn from_row(
        row : sqlx::sqlite::SqliteRow, image_repo_path : &str
    ) -> Result<PostMetadata, Error> {
        let user_name : String = row.get("creator_user_name");
        let oc_name : String = row.get("oc_name");
        let images = get_images_ids(
            &user_name,
            &oc_name,
            image_repo_path
        ).await?;

        Ok(PostMetadata {
            id : row.get("id"),
            creator_user_name : user_name,
            oc_name : oc_name,
            description : row.get("description"),
            specie : row.get("specie"),
            sex : row.get("sex"),
            images : images,
        })

    }
}