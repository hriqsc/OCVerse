use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::{
    error::Error,
    services::image::{get_images},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMetadata {
    pub id : i32,
    pub creator_user_name: String,
    pub oc_name: String,
    pub description: String,
    pub specie : String,
    pub sex : String,
    pub height : String,
    pub images: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMinified {
    pub id : i32,
    pub creator_user_name: String,
    pub oc_name: String,
    pub thumb : String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePost{
    pub oc_name: String,
    pub description: String,
    pub specie : String,
    pub sex : String,
    pub height : String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct EditPost{
    pub id : i32,
    pub oc_name: String,
    pub description: String,
    pub sex : String,
    pub specie : String,
    pub height : String,
    pub existing_images : Vec<i32>
}



impl PostMetadata{
    pub async fn from_row(
        row : sqlx::sqlite::SqliteRow, 
        image_repo_path : &str,
        search_images : bool
    ) -> Result<PostMetadata, Error> {

        let user_name : String = row.try_get("creator_user_name")?;
        let oc_name : String = row.try_get("oc_name")?;
        let images = if search_images{ 
            get_images(
                &user_name,
                &oc_name,
                image_repo_path
            ).await?
        } else {Vec::new()}
        
        ;

        Ok(PostMetadata {
            id : row.try_get("id")?,
            creator_user_name : user_name,
            oc_name : oc_name,
            description : row.try_get("description")?,
            specie : row.try_get("specie")?,
            sex : row.try_get("sex")?,
            height : row.try_get("height")?,
            images : images,
        })

    }
}