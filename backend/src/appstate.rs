use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::SqlitePool;
use crate::{
    definitions::tabble_definitions::run_tables_defs,
    error::Error,
    shared::encrypt::fnv1a_hash
};
use argon2::{Argon2, Algorithm, Version, Params};
use std::env;

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

        let db: sqlx::Pool<sqlx::Sqlite>  = sqlx::sqlite::SqlitePool::connect(&env::var("DB_PATH")?).await?;

        run_tables_defs(&db).await?;

        let secret_code = fnv1a_hash(env::var("SECRET_CODE")?.as_bytes());

        Ok(AppState {
            db,
            argon2: Argon2::new(Algorithm::Argon2id, Version::V0x13, params),
            jwt_encoding_key: EncodingKey::from_secret(jwt_secret.as_bytes()),
            jwt_decoding_key: DecodingKey::from_secret(jwt_secret.as_bytes()),
            image_repo_path: env::var("IMAGE_REPO_PATH")?,
            secret_code
        })
    }
}