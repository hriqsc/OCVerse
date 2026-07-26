use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Json Serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(String),

}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Database(_) | Error::Io(_) | 
            Error::Json(_) | Error::Other(_) 

            => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }


    fn error_response(&self) -> HttpResponse {
        let message = match self {
            Error::Database(e) => {
                tracing::error!("db error: {e}");
                "internal server error, Database error".to_string()
            }
            Error::Io(e) => {
                tracing::error!("io error: {e}");
                "internal server error, IO error".to_string()
            }
            Error::Json(e) => {
                tracing::error!("io error: {e}");
                "internal server error, Json parsing error".to_string()
            },
            Error::Other(e) => {
                tracing::error!("other error: {e}");
                "internal server error, error".to_string()
            }
        };

        HttpResponse::build(self.status_code()).json(json!({ "error": message }))
    }
}