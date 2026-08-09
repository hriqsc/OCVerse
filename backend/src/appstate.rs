use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::{SqlitePool, sqlite::{SqliteConnectOptions, SqlitePoolOptions}};
use crate::{
    definitions::tabble_definitions::run_tables_defs,
    error::Error,
    shared::encrypt::fnv1a_hash
};
use argon2::{Argon2, Algorithm, Version, Params};
use std::{env, str::FromStr};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub argon2: Argon2<'static>,
    pub jwt_encoding_key: EncodingKey,
    pub jwt_decoding_key: DecodingKey,
    pub image_repo_path: String,
    pub secret_code : u64
}

impl AppState {
    pub async fn new() 
    -> Result<Self, Error> {
        let params = Params::new(
            8 * 1024, 
            2,
            1,
            None
        )
            .map_err(|e| Error::Other(e.to_string()))?;

        let jwt_secret = env::var("JWT_SECRET")?;

        std::fs::create_dir_all(env::var("DB_DIR")?)?;
        let db_path = env::var("DB_PATH")?;
        
        let options = SqliteConnectOptions::from_str(&db_path)
        .map_err(|e| Error::Other(e.to_string()))?
        .create_if_missing(true);
    
        let db = SqlitePoolOptions::new()
            .connect_with(options)
            .await?;

        run_tables_defs(&db).await?;

        let image_repo_path = env::var("IMAGE_REPO_PATH")?;

        std::fs::create_dir_all(&image_repo_path)?;

        let secret_code = fnv1a_hash(env::var("SECRET_CODE")?.as_bytes());

        Ok(AppState {
            db,
            argon2: Argon2::new(Algorithm::Argon2id, Version::V0x13, params),
            jwt_encoding_key: EncodingKey::from_secret(jwt_secret.as_bytes()),
            jwt_decoding_key: DecodingKey::from_secret(jwt_secret.as_bytes()),
            image_repo_path,
            secret_code
        })
    }
}