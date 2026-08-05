use actix_web::{HttpRequest, HttpResponse, get, post, web};
use tracing::error;

use crate::{api_error::ApiError, appstate::AppState, error::Error, shared::{encrypt::fnv1a_hash, helpers::get_value_from_header}};


fn generic_response() -> ApiError {
    ApiError::UnAuthorized("unauthorized".into())
}

#[post("/api/v1/magma")]
pub async fn new_magma(
    state: web::Data<AppState>,
    req: HttpRequest,
    req_body: String
) -> Result<HttpResponse, ApiError> {


    if req_body.trim().is_empty() {
        return Err(generic_response());
    }

    //expected json:
    /*
        header: <secret_code (String)>
        {"url":"<String>"}
     */
    if &req_body[2..5] != "url" {
        return Err(generic_response());
    }

    let secret = fnv1a_hash(
        get_value_from_header("secret", &req)?.as_bytes()
    );

    if state.secret_code != secret {
        return Err(generic_response());
    }

    let id : String = 
            req_body.chars()
                    .skip(8)
                    .take(req_body.len() - 2).collect();

    if id.len() < 1 || id.len() > 20 {
        return Err(generic_response());
    }
    
    let time_stamp = chrono::Utc::now().timestamp();

    sqlx::query(
        "INSERT INTO magmas (id, created_at) VALUES ($1, $2)"
    )
    .bind(id)
    .bind(time_stamp)
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!(error = %e, "database error while creating new magma");
        ApiError::Internal(Error::Other("internal server error".into()))
    })?;
    
    Ok(HttpResponse::Ok().json(""))
}


// public api ===================================================

#[get("/api/v1/magmas")]
pub async fn list_magmas(
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let magmas: Vec<String> = sqlx::query_scalar(
        "SELECT id FROM magmas ORDER BY created_at DESC LIMIT ?"
    )
    .bind(20)
    .fetch_all(&state.db)
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "magmas_id": magmas
    })))
}