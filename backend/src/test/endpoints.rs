//! src/test/endpoints.rs
//!
//! Integration tests for `endpoints::user`, `endpoints::post`,
//! `endpoints::magma`, `endpoints::images`.
//!
//! Run with: cargo test -- --test-threads=1 --nocapture

#![cfg(test)]

use actix_web::{http::StatusCode, test, web, App};
use serde_json::json;

use crate::{
    appstate::AppState,
    endpoints::{images, magma, post, user},
};

// ---------------------------------------------------------------------
// Helpers
// ---------------------

async fn setup_state() -> web::Data<AppState> {
    let _ = dotenvy::from_path("../.env");
    let state = AppState::new().await.expect("failed to build AppState");
    web::Data::new(state)
}

fn unique_username(prefix: &str) -> String {
    format!(
        "{}_{}",
        prefix,
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    )
}

fn auth_header(access_token: &str) -> (&'static str, String) {
    ("Authorization", format!("Bearer {}", access_token))
}

fn build_multipart_body(metadata_json: &str, images: &[Vec<u8>]) -> (String, Vec<u8>) {
    let boundary = "----testboundary1234567890".to_string();
    let mut body: Vec<u8> = Vec::new();

    body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
    body.extend_from_slice(b"Content-Disposition: form-data; name=\"metadata\"\r\n\r\n");
    body.extend_from_slice(metadata_json.as_bytes());
    body.extend_from_slice(b"\r\n");

    for image in images {
        body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
        body.extend_from_slice(
            b"Content-Disposition: form-data; name=\"images\"; filename=\"image.png\"\r\n",
        );
        body.extend_from_slice(b"Content-Type: application/octet-stream\r\n\r\n");
        body.extend_from_slice(image);
        body.extend_from_slice(b"\r\n");
    }

    body.extend_from_slice(format!("--{}--\r\n", boundary).as_bytes());
    (boundary, body)
}

fn dummy_png_bytes() -> Vec<u8> {
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44,
        0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F,
        0x15, 0xC4, 0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00,
        0x01, 0x00, 0x00, 0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49,
        0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ]
}

/// Registers + logs in a new user, returns (user_name, access_token, refresh_cookie).
async fn register_and_login(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    >,
) -> (String, String, String) {
    let user_name = unique_username("test_user");
    let password = "SuperSecret123!";

    let req = test::TestRequest::post()
        .uri("/api/v1/user/register")
        .set_payload(json!({ "user_name": user_name, "password": password }).to_string())
        .to_request();
    let resp = test::call_service(app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::CREATED);

    let req = test::TestRequest::post()
        .uri("/api/v1/user/login")
        .set_payload(json!({ "user_name": user_name, "password": password }).to_string())
        .to_request();
    let resp = test::call_service(app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    
    let refresh_cookie = resp
    .response()
    .cookies()
    .find(|c| c.name() == "refresh_token")
    .expect("login must set refresh_token cookie")
        .value()
        .to_string();

    let body: serde_json::Value = test::read_body_json(resp).await;
    let access_token = body["access_token"].as_str().unwrap().to_string();
    
    
    (user_name, access_token, refresh_cookie)
}

// =======================================================================
// user
// =======================================================================

#[actix_web::test]
async fn test_register_new_user_returns_201() {
    let state = setup_state().await;
    let app = test::init_service(App::new().app_data(state.clone()).service(user::create_user))
        .await;

    let payload = json!({
        "user_name": unique_username("register_ok"),
        "password": "SuperSecret123!"
    })
    .to_string();

    let req = test::TestRequest::post()
        .uri("/api/v1/user/register")
        .set_payload(payload)
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::CREATED);
    
}

#[actix_web::test]
async fn test_register_duplicate_username_returns_409() {
    let state = setup_state().await;
    let app = test::init_service(App::new().app_data(state.clone()).service(user::create_user))
        .await;

    let payload = json!({
        "user_name": unique_username("register_dup"),
        "password": "SuperSecret123!"
    })
    .to_string();

    let req = test::TestRequest::post()
        .uri("/api/v1/user/register")
        .set_payload(payload.clone())
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::CREATED);

    let req = test::TestRequest::post()
        .uri("/api/v1/user/register")
        .set_payload(payload)
        .to_request();

    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::CONFLICT);
    
}

#[actix_web::test]
async fn test_register_empty_body_returns_400() {
    let state = setup_state().await;
    let app = test::init_service(App::new().app_data(state.clone()).service(user::create_user))
        .await;

    let req = test::TestRequest::post()
        .uri("/api/v1/user/register")
        .set_payload("")
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
}

#[actix_web::test]
async fn test_register_malformed_json_returns_400() {
    let state = setup_state().await;
    let app = test::init_service(App::new().app_data(state.clone()).service(user::create_user))
        .await;

    let req = test::TestRequest::post()
        .uri("/api/v1/user/register")
        .set_payload("{not valid json")
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
}

#[actix_web::test]
async fn test_login_success_returns_token_and_sets_cookie() {
    let state = setup_state().await;
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(user::create_user)
            .service(user::login_user),
    )
    .await;

    let (_user_name, access_token, refresh_cookie) = register_and_login(&app).await;
    assert!(!access_token.is_empty());
    assert!(!refresh_cookie.is_empty());
    
}

#[actix_web::test]
async fn test_login_wrong_password_returns_401() {
    let state = setup_state().await;
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(user::create_user)
            .service(user::login_user),
    )
    .await;

    let user_name = unique_username("login_wrongpw");
    let req = test::TestRequest::post()
        .uri("/api/v1/user/register")
        .set_payload(json!({ "user_name": user_name, "password": "CorrectPassword1!" }).to_string())
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::CREATED);

    let req = test::TestRequest::post()
        .uri("/api/v1/user/login")
        .set_payload(json!({ "user_name": user_name, "password": "WrongPassword!" }).to_string())
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    
}

#[actix_web::test]
async fn test_login_nonexistent_user_returns_401() {
    let state = setup_state().await;
    let app = test::init_service(App::new().app_data(state.clone()).service(user::login_user))
        .await;

    let req = test::TestRequest::post()
        .uri("/api/v1/user/login")
        .set_payload(json!({ "user_name": unique_username("nobody"), "password": "x" }).to_string())
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    
}

#[actix_web::test]
async fn test_logout_expires_cookie() {
    let state = setup_state().await;
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(user::create_user)
            .service(user::login_user)
            .service(user::logout_user),
    )
    .await;

    let (_user_name, _access_token, refresh_cookie) = register_and_login(&app).await;

    let req = test::TestRequest::post()
        .uri("/api/v1/user/logout")
        .cookie(actix_web::cookie::Cookie::new("refresh_token", refresh_cookie))
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::OK);

    let set_cookie = resp
        .response()
        .cookies()
        .find(|c| c.name() == "refresh_token")
        .expect("logout must return expiring cookie");
    assert_eq!(set_cookie.max_age(), Some(actix_web::cookie::time::Duration::seconds(0)));
    
}

#[actix_web::test]
async fn test_refresh_rotates_session_and_invalidates_old_token() {
    let state = setup_state().await;
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(user::create_user)
            .service(user::login_user)
            .service(user::refresh_token),
    )
    .await;

    let (_user_name, _access_token, old_refresh) = register_and_login(&app).await;

    let req = test::TestRequest::post()
        .uri("/api/v1/user/refresh")
        .cookie(actix_web::cookie::Cookie::new("refresh_token", old_refresh.clone()))
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::OK);

    let new_refresh = resp
        .response()
        .cookies()
        .find(|c| c.name() == "refresh_token")
        .unwrap()
        .value()
        .to_string();
    assert_ne!(new_refresh, old_refresh);

    // old token must now be revoked
    let req = test::TestRequest::post()
        .uri("/api/v1/user/refresh")
        .cookie(actix_web::cookie::Cookie::new("refresh_token", old_refresh))
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    
}

#[actix_web::test]
async fn test_refresh_without_cookie_returns_401() {
    let state = setup_state().await;
    let app =
        test::init_service(App::new().app_data(state.clone()).service(user::refresh_token)).await;

    let req = test::TestRequest::post().uri("/api/v1/user/refresh").to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    
}

// =======================================================================
// post
// =======================================================================

#[actix_web::test]
async fn test_create_post_success_returns_201() {
    let state = setup_state().await;
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(user::create_user)
            .service(user::login_user)
            .service(post::create_post),
    )
    .await;

    let (_user_name, access_token, _refresh) = register_and_login(&app).await;

    let metadata = json!({
        "oc_name": "Test OC",
        "description": "a test character",
        "specie": "wolf",
        "sex": "m"
    })
    .to_string();
    let (boundary, body) = build_multipart_body(&metadata, &[dummy_png_bytes()]);
    let (header_name, header_value) = auth_header(&access_token);

    let req = test::TestRequest::post()
        .uri("/api/v1/post")
        .insert_header(("Content-Type", format!("multipart/form-data; boundary={}", boundary)))
        .insert_header((header_name, header_value))
        .set_payload(body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::CREATED);
    
}

#[actix_web::test]
async fn test_create_post_without_images_returns_400() {
    let state = setup_state().await;
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(user::create_user)
            .service(user::login_user)
            .service(post::create_post),
    )
    .await;

    let (_user_name, access_token, _refresh) = register_and_login(&app).await;

    let metadata = json!({
        "oc_name": "No Images OC",
        "description": "missing images",
        "specie": "cat",
        "sex": "f"
    })
    .to_string();
    let (boundary, body) = build_multipart_body(&metadata, &[]);
    let (header_name, header_value) = auth_header(&access_token);

    let req = test::TestRequest::post()
        .uri("/api/v1/post")
        .insert_header(("Content-Type", format!("multipart/form-data; boundary={}", boundary)))
        .insert_header((header_name, header_value))
        .set_payload(body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
}

#[actix_web::test]
async fn test_create_post_sex_longer_than_one_char_returns_400() {
    let state = setup_state().await;
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(user::create_user)
            .service(user::login_user)
            .service(post::create_post),
    )
    .await;

    let (_user_name, access_token, _refresh) = register_and_login(&app).await;

    let metadata = json!({
        "oc_name": "Invalid Sex OC",
        "description": "sex field too long",
        "specie": "fox",
        "sex": "ab"
    })
    .to_string();
    let (boundary, body) = build_multipart_body(&metadata, &[dummy_png_bytes()]);
    let (header_name, header_value) = auth_header(&access_token);

    let req = test::TestRequest::post()
        .uri("/api/v1/post")
        .insert_header(("Content-Type", format!("multipart/form-data; boundary={}", boundary)))
        .insert_header((header_name, header_value))
        .set_payload(body)
        .to_request();

    let resp = test::call_service(&app, req).await;
    println!("response given: {}", &resp.status());
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
}

#[actix_web::test]
async fn test_update_post_by_non_owner_returns_401() {
    let state = setup_state().await;
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(user::create_user)
            .service(user::login_user)
            .service(post::create_post)
            .service(post::update_post),
    )
    .await;

    let (_owner, owner_token, _r1) = register_and_login(&app).await;
    let metadata = json!({
        "oc_name": "Owned OC",
        "description": "owned by owner",
        "specie": "dragon",
        "sex": "m"
    })
    .to_string();
    let (boundary, body) = build_multipart_body(&metadata, &[dummy_png_bytes()]);
    let (header_name, header_value) = auth_header(&owner_token);
    let req = test::TestRequest::post()
        .uri("/api/v1/post")
        .insert_header(("Content-Type", format!("multipart/form-data; boundary={}", boundary)))
        .insert_header((header_name, header_value))
        .set_payload(body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::CREATED);
    let created: serde_json::Value = test::read_body_json(resp).await;
    let post_id = created["id"].as_i64().unwrap();

    let (_intruder, intruder_token, _r2) = register_and_login(&app).await;
    let edit_metadata = json!({
        "id": post_id,
        "oc_name": "Hijacked OC",
        "description": "not yours",
        "specie": "dragon",
        "sex": "m"
    })
    .to_string();
    let (boundary, body) = build_multipart_body(&edit_metadata, &[dummy_png_bytes()]);
    let (header_name, header_value) = auth_header(&intruder_token);
    let req = test::TestRequest::put()
        .uri("/api/v1/post")
        .insert_header(("Content-Type", format!("multipart/form-data; boundary={}", boundary)))
        .insert_header((header_name, header_value))
        .set_payload(body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    
}

#[actix_web::test]
async fn test_query_posts_by_creator_returns_200() {
    let state = setup_state().await;
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(user::create_user)
            .service(user::login_user)
            .service(post::create_post)
            .service(post::query_posts),
    )
    .await;

    let (user_name, access_token, _refresh) = register_and_login(&app).await;
    let metadata = json!({
        "oc_name": "Queryable OC",
        "description": "findable",
        "specie": "cat",
        "sex": "f"
    })
    .to_string();
    let (boundary, body) = build_multipart_body(&metadata, &[dummy_png_bytes()]);
    let (header_name, header_value) = auth_header(&access_token);
    let req = test::TestRequest::post()
        .uri("/api/v1/post")
        .insert_header(("Content-Type", format!("multipart/form-data; boundary={}", boundary)))
        .insert_header((header_name, header_value))
        .set_payload(body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::CREATED);

    let uri = format!("/api/v1/posts/U/{}", user_name);
    let req = test::TestRequest::get().uri(&uri).to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::OK);
    
}

#[actix_web::test]
async fn test_get_post_returns_created_post() {
    let state = setup_state().await;
    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(user::create_user)
            .service(user::login_user)
            .service(post::create_post)
            .service(post::get_post),
    )
    .await;

    let (_user_name, access_token, _refresh) = register_and_login(&app).await;
    let metadata = json!({
        "oc_name": "Fetchable OC",
        "description": "gettable by id",
        "specie": "cat",
        "sex": "f"
    })
    .to_string();
    let (boundary, body) = build_multipart_body(&metadata, &[dummy_png_bytes()]);
    let (header_name, header_value) = auth_header(&access_token);
    let req = test::TestRequest::post()
        .uri("/api/v1/post")
        .insert_header(("Content-Type", format!("multipart/form-data; boundary={}", boundary)))
        .insert_header((header_name, header_value))
        .set_payload(body)
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::CREATED);
    let created: serde_json::Value = test::read_body_json(resp).await;
    let post_id = created["id"].as_i64().unwrap();

    // assumes route is "/api/v1/post/{id}" — adjust if yours differs
    let req = test::TestRequest::get()
        .uri(&format!("/api/v1/post/{}", post_id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::OK);
    
}

#[actix_web::test]
async fn test_get_post_non_numeric_id_returns_400() {
    let state = setup_state().await;
    let app =
        test::init_service(App::new().app_data(state.clone()).service(post::get_post)).await;

    let req = test::TestRequest::get().uri("/api/v1/post/not-a-number").to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    
}

#[actix_web::test]
async fn test_get_post_not_found_returns_404() {
    let state = setup_state().await;
    let app =
        test::init_service(App::new().app_data(state.clone()).service(post::get_post)).await;

    let req = test::TestRequest::get().uri("/api/v1/post/999999999").to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    
}

// =======================================================================
// magma
// =======================================================================

#[actix_web::test]
async fn test_new_magma_with_valid_secret_returns_ok() {
    let state = setup_state().await;
    let secret_code_plain = std::env::var("SECRET_CODE").expect("SECRET_CODE must be set");

    let app = test::init_service(
        App::new()
            .app_data(state.clone())
            .service(magma::new_magma)
            .service(magma::list_magmas),
    )
    .await;

    let id = "TestURLdDA";
    let req = test::TestRequest::post()
        .uri("/api/v1/magma")
        .insert_header(("secret", secret_code_plain))
        .set_payload(format!("{{\"url\":\"{}\"}}", id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::OK);

    let req = test::TestRequest::get().uri("/api/v1/magmas").to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::OK);
    let body: serde_json::Value = test::read_body_json(resp).await;
    assert!(body["magmas_id"].is_array());
    
}

#[actix_web::test]
async fn test_new_magma_with_invalid_secret_returns_401() {
    let state = setup_state().await;
    let app = test::init_service(App::new().app_data(state.clone()).service(magma::new_magma))
        .await;

    let req = test::TestRequest::post()
        .uri("/api/v1/magma")
        .insert_header(("secret", "wrong-secret"))
        .set_payload(format!("{{\"url\":\"{}\"}}", unique_username("magma_bad")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    
}

#[actix_web::test]
async fn test_new_magma_missing_secret_header_returns_401() {
    let state = setup_state().await;
    let app = test::init_service(App::new().app_data(state.clone()).service(magma::new_magma))
        .await;

    let req = test::TestRequest::post()
        .uri("/api/v1/magma")
        .set_payload(format!("{{\"url\":\"{}\"}}", unique_username("magma_no_header")))
        .to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    
}

#[actix_web::test]
async fn test_new_magma_empty_body_returns_401() {
    let state = setup_state().await;
    let app = test::init_service(App::new().app_data(state.clone()).service(magma::new_magma))
        .await;

    let req = test::TestRequest::post().uri("/api/v1/magma").set_payload("").to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    
}

#[actix_web::test]
async fn test_list_magmas_returns_200() {
    let state = setup_state().await;
    let app = test::init_service(App::new().app_data(state.clone()).service(magma::list_magmas))
        .await;

    let req = test::TestRequest::get().uri("/api/v1/magmas").to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::OK);
    

}

// =======================================================================
// images
// =======================================================================

#[actix_web::test]
async fn test_get_file_path_traversal_returns_400_or_404() {
    let state = setup_state().await;
    let app =
        test::init_service(App::new().app_data(state.clone()).service(images::get_file)).await;

    let req = test::TestRequest::get().uri("/f/v1/someuser/../secrets/1").to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    let status = resp.status();
    assert!(status == StatusCode::BAD_REQUEST || status == StatusCode::NOT_FOUND);
    

}

#[actix_web::test]
async fn test_get_file_missing_file_returns_404() {
    let state = setup_state().await;
    let app =
        test::init_service(App::new().app_data(state.clone()).service(images::get_file)).await;

    let req = test::TestRequest::get().uri("/f/v1/nobody/nothing/nonexistent-id").to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", &resp.status());
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    

}