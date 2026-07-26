use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
use thiserror::Error;
use crate::error::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),
    
    #[error("Unauthorized error: {0}")]
    UnAuthorized(String),

    #[error("Internal error: {0}")]
    Internal(#[from] Error),

}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => ApiError::NotFound("resource not found".into()),

            // Postgres unique_violation = SQLSTATE 23505
            sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
                ApiError::Conflict("resource already exists".into())
            }

            // anything else: wrap as an opaque internal error
            _ => ApiError::Internal(Error::from(err)),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::UnAuthorized(_) => StatusCode::UNAUTHORIZED
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = match self {
            ApiError::Internal(e) => {
                tracing::error!("internal error: {e}");
                "internal server error".to_string()
            }
            ApiError::BadRequest(msg) => msg.clone(),
            ApiError::NotFound(msg) => msg.clone(),
            ApiError::Conflict(msg) => msg.clone(),
            ApiError::UnAuthorized(msg) => msg.clone(),
        };

        HttpResponse::build(self.status_code()).json(json!({ "error": message }))
    }
}