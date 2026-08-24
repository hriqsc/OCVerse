use jsonwebtoken::{encode, decode, Header, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use crate::{appstate::AppState, error::Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user_name
    pub iat: usize,
    pub exp: usize,
}

pub fn generate_access_token(state: &AppState, user_name: &str) -> Result<String, Error> {
    let now = Utc::now();
    let exp = now + Duration::minutes(15);

    let claims = Claims {
        sub: user_name.to_string(),
        iat: now.timestamp() as usize,
        exp: exp.timestamp() as usize,
    };

    encode(&Header::new(Algorithm::HS256), &claims, &state.jwt_encoding_key)
        .map_err(|e| Error::Other(e.to_string()))
}

pub fn validate_access_token(state: &AppState, token: &str) -> Result<Claims, Error> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 5; // clock skew tolerance

    decode::<Claims>(token, &state.jwt_decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|e| Error::Other(e.to_string()))
}