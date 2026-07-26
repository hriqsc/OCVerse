use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;
use crate::error::Error;
use argon2::{Argon2, Algorithm, Version, Params};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub argon2: Argon2<'static>,
    pub jwt_encoding_key: EncodingKey,
    pub jwt_decoding_key: DecodingKey,
}


impl AppState {
    pub async fn new(db_address: &str, jwt_secret: &str) -> Result<Self, Error> {
        let params = Params::new(8 * 1024, 2, 1, None)
            .map_err(|e| Error::Other(e.to_string()))?;

        let appdata = AppState {
            db: sqlx::postgres::PgPool::connect(db_address).await?,
            argon2: Argon2::new(Algorithm::Argon2id, Version::V0x13, params),
            jwt_encoding_key: EncodingKey::from_secret(jwt_secret.as_bytes()),
            jwt_decoding_key: DecodingKey::from_secret(jwt_secret.as_bytes()),
        };
        Ok(appdata)
    }
}