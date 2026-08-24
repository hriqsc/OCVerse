use actix_web::{HttpRequest, HttpResponse, get, post, web};
use tracing::{error, warn};
use crate::{
    api_error::ApiError,
    appstate::AppState,
    error::Error,
    shared::{
        encrypt::fnv1a_hash,
        helpers::get_value_from_header
    },
    schemas::magma::MagmaInsert
};
use actix_cors::Cors;

#[inline]
fn generic_response() -> ApiError {
    ApiError::UnAuthorized("unauthorized".into())
}

#[post("/api/v1/magma", wrap = "Cors::permissive()")]
pub async fn new_magma(
    state: web::Data<AppState>,
    req: HttpRequest,
    req_body: String
) -> Result<HttpResponse, ApiError> {

    let secret = fnv1a_hash(
        get_value_from_header("secret", &req)?.as_bytes()
    );

    if state.secret_code != secret {
        return Err(generic_response());
    }

    let new_magma : MagmaInsert = match serde_json::from_str(&req_body){
        Ok(magma) => magma,
        Err(e) => {
            warn!(error = %e, "malformed json received on register");
            return Err(ApiError::BadRequest("invalid request".into()));
        }
    };

    if new_magma.url.len() < 1 || new_magma.url.len() > 20 {
        return Err(generic_response());
    }

    sqlx::query(
        "INSERT INTO magmas (id, created_at) VALUES ($1, $2)"
    )
    .bind(new_magma.url)
    .bind(new_magma.time_stamp)
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!(error = %e, "database error while creating new magma");
        ApiError::Internal(
            Error::Other("internal server error".into())
        )
    })?;
    
    Ok(HttpResponse::Ok().json(""))
}


// public api ===================================================

#[get("/api/v1/magmas")]
pub async fn list_magmas(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let magmas: Vec<String> = sqlx::query_scalar(
        "SELECT id FROM magmas ORDER BY created_at DESC"
    )
    .fetch_all(&state.db)
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "magmas_id": magmas
    })))
}