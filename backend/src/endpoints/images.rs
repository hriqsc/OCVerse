use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{HttpRequest, HttpResponse, get, web};
use crate::{api_error::ApiError, appstate::AppState};



#[inline]
fn is_safe_segment(segment: &str) -> bool {
    if segment.is_empty() || segment == "." || segment == ".." {
        return false;
    }
    !segment.bytes().any(|b| matches!(b, b'/' | b'\\' | 0))
}

#[get("/f/v1/{user}/{oc_name}/{id}")]
pub async fn get_file(
    req: HttpRequest,
    path: web::Path<(String, String, String)>,
    state: web::Data<AppState>,
) -> Result<HttpResponse, ApiError> {
    let (user, oc_name, id) = path.into_inner();

    if !is_safe_segment(&user) || !is_safe_segment(&oc_name) || !is_safe_segment(&id) {
        return Ok(HttpResponse::BadRequest().finish());
    }

    //builds file_path
    let mut file_path = PathBuf::with_capacity(
        state.image_repo_path.as_str().len()
            + user.len()
            + oc_name.len()
            + id.len()
            + 8,
    );
    file_path.push(&state.image_repo_path);
    file_path.push(&user);
    file_path.push(&oc_name);
    file_path.push(id_with_png(&id));

    match NamedFile::open_async(&file_path).await {
        Ok(named_file) => {
            let mut response = named_file
                .use_etag(true)
                .use_last_modified(true)
                .into_response(&req);

            response.headers_mut().insert(
                actix_web::http::header::CACHE_CONTROL,
                actix_web::http::header::HeaderValue::from_static(
                    "public, max-age=31536000, immutable",
                ),
            );
            response.headers_mut().insert(
                actix_web::http::header::CONTENT_ENCODING,
                actix_web::http::header::HeaderValue::from_static("identity"),
            );

            Ok(response)
        }
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

fn id_with_png(id: &str) -> String {
    let mut s = String::with_capacity(id.len() + 4);
    s.push_str(id);
    s.push_str(".png");
    s
}