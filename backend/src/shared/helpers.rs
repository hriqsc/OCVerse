use crate::api_error::ApiError;
use actix_web::{HttpRequest};


pub fn get_value_from_header(
    key : &str, 
    req : &HttpRequest
) -> Result<String, ApiError> {
    let header_val = req
        .headers()
        .get(key)
        .ok_or_else(|| ApiError::UnAuthorized("UnAuthorized".into()))?;

    Ok(header_val
        .to_str()
        .map_err(|_| ApiError::UnAuthorized("UnAuthorized".into()))?
        .to_string()
    )

}