use actix_web::{FromRequest, HttpRequest, dev::Payload, web};
use std::future::{ready, Ready};
use crate::{appstate::AppState, api_error::ApiError, error::Error, services::jwt::validate_access_token};

#[derive(Debug)]
pub struct AuthUser {
    pub user_name: String,
}

impl FromRequest for AuthUser {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let state = match req.app_data::<web::Data<AppState>>() {
            Some(s) => s,
            None => return ready(Err(ApiError::Internal(Error::Other("internal error".into())))),
        };

        let token = match req.headers().get("Authorization").and_then(|h| h.to_str().ok()) {
            Some(h) if h.starts_with("Bearer ") => h.trim_start_matches("Bearer ").to_string(),
            _ => return ready(Err(ApiError::UnAuthorized("Invalid Header".into()))),
        };

        match validate_access_token(state, &token) {
            Ok(claims) => ready(Ok(AuthUser { user_name: claims.sub })),
            Err(_) => ready(Err(ApiError::UnAuthorized("Invalid Header".into()))),
        }
    }
}