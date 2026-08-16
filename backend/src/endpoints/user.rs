use actix_web::{HttpResponse, cookie::SameSite, post, web};
use sqlx::Row;
use tracing::{error, warn, info, instrument};
use crate::{
    api_error::ApiError, appstate::AppState, error::Error, schemas::user::
            UserLogin, services::{
                jwt::generate_access_token, session::{
                    new_session, revoke_session, validate_refresh_token
                }
            }
};
use argon2::{
    PasswordHash, PasswordVerifier, password_hash::{
        PasswordHasher, SaltString, rand_core::OsRng
    }
};
use actix_web::cookie::{Cookie, time::Duration as CookieDuration};


#[instrument(skip(state, req_body))]
#[post("/api/v1/user/register")]
pub async fn create_user(
    state: web::Data<AppState>,
    req_body : String
) -> Result<HttpResponse, ApiError> {

    //data validation
    if req_body.trim().is_empty() {
        warn!("empty body received on register");
        return Err(ApiError::BadRequest("invalid request".into()));
    }

    let user_req: UserLogin = match serde_json::from_str(&req_body){
        Ok(user) => user,
        Err(e) => {
            warn!(error = %e, "malformed json received on register");
            return Err(ApiError::BadRequest("invalid request".into()));
        }
    };

    //check if already exists in db
    /*
    disclaimer : this is not the best way to create users since it would be 
    better to generate an id, but since its just a simple project i won't work 
    too much architecture it. ~hriq
     */
    let exists = 
        sqlx::query("SELECT user_name FROM users WHERE user_name = $1")
            .bind(&user_req.user_name)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| {
                error!(error = %e, "database error while checking existing user");
                ApiError::Internal(Error::Other("internal server error".into()))
            })?
            .is_some();

    if exists {
        warn!(user_name = %user_req.user_name, "registration attempt with taken username");
        return Err(ApiError::Conflict("username already taken".into()));
    }

    //insert new user
    let salt = SaltString::generate(&mut OsRng);

    let hash_password = state.argon2
        .hash_password(user_req.password.as_bytes(), &salt)
        .map_err(|e| {
            error!(error = %e, "failed to hash password");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?
        .to_string();
    

    sqlx::query("INSERT INTO users (user_name, password_hash) VALUES ($1, $2)")
        .bind(&user_req.user_name)
        .bind(&hash_password)
        .execute(&state.db)
        .await
        .map_err(|e| {
            error!(error = %e, "database error while inserting new user");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    info!(user_name = %user_req.user_name, "user registered");

    Ok(HttpResponse::Created().body(""))
}



#[instrument(skip(state, req_body))]
#[post("/api/v1/user/login")]
pub async fn login_user(
    state: web::Data<AppState>,
    req_body : String
) -> Result<HttpResponse, ApiError> {

    if req_body.trim().is_empty() {
        warn!("empty body received on login");
        return Err(ApiError::BadRequest("invalid request".into()));
    }

    let login_req: UserLogin = match serde_json::from_str(&req_body){
        Ok(user) => user,
        Err(e) => {
            warn!(error = %e, "malformed json received on login");
            return Err(ApiError::BadRequest("invalid request".into()));
        }
    };

    let row = sqlx::query(
        "SELECT password_hash FROM users WHERE user_name = $1"
    )
        .bind(&login_req.user_name)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            error!(error = %e, "database error while fetching user for login");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;
    
    let password_hash: String = match row {
        Some(row) => row.get("password_hash"),
        None => {
            // valid PHC-formatted dummy hash, computed once, just to keep
            // verify_password's timing consistent with the real-user path
            if let Ok(dummy_hash) = PasswordHash::new(
                "$argon2id$v=19$m=8192,t=2,p=1$c29tZXNhbHQ$RdescudvJCsgt3ub+b+dWRWJTmaaJObG"
            ) {
                let _ = state.argon2.verify_password(b"dummy", &dummy_hash);
            }
            warn!(user_name = %login_req.user_name, "login attempt for nonexistent user");
            return Err(ApiError::UnAuthorized("invalid credentials".into()));
        }
    };

    let parsed_hash = PasswordHash::new(&password_hash)
        .map_err(|e| {
            error!(error = %e, "failed to parse stored password hash");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    let is_valid = state.argon2
        .verify_password(login_req.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        warn!(user_name = %login_req.user_name, "invalid password on login attempt");
        return Err(ApiError::UnAuthorized("invalid credentials".into()));
    }

    let session = new_session(&state, &login_req.user_name).await.map_err(|e| {
        error!(error = %e, "failed to create session");
        ApiError::Internal(Error::Other("internal server error".into()))
    })?;

    let access_token = generate_access_token(&state, &login_req.user_name)
        .map_err(|e| {
            error!(error = %e, "failed to generate access token");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    let refresh_cookie = Cookie::build("refresh_token", session.refresh_token.clone())
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(CookieDuration::days(7))
        .path("/api/v1")
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(refresh_cookie)
        .json(serde_json::json!({ "access_token": access_token })))
}


#[instrument(skip(state, req))]
#[post("/api/v1/user/logout")]
pub async fn logout_user(
    state: web::Data<AppState>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ApiError> {
    if let Some(refresh_token_cookie) = req.cookie("refresh_token") {
        revoke_session(&state, refresh_token_cookie.value())
            .await
            .map_err(|e| {
                error!(error = %e, "failed to revoke session on logout");
                ApiError::Internal(Error::Other("internal server error".into()))
            })?;
    }

    // expire client cookie
    let expired_cookie = Cookie::build("refresh_token", "")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(CookieDuration::seconds(0))
        .path("/api/v1")
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(expired_cookie)
        .finish())
}



#[instrument(skip(state, req))]
#[post("/api/v1/user/refresh")]
pub async fn refresh_token(
    state: web::Data<AppState>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let old_refresh_token = req.cookie("refresh_token")
        .ok_or_else(|| {
            warn!("refresh attempted without refresh_token cookie");
            ApiError::UnAuthorized("invalid request".into())
        })?;

    let user_name = validate_refresh_token(&state, old_refresh_token.value())
        .await
        .map_err(|e| {
            warn!(error = %e, "invalid or expired refresh token");
            ApiError::UnAuthorized("invalid request".into())
        })?;

    // revoke old session and create a new one
    revoke_session(&state, old_refresh_token.value())
        .await
        .map_err(|e| {
            error!(error = %e, "failed to revoke old session during refresh");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    let session = new_session(&state, &user_name)
        .await
        .map_err(|e| {
            error!(error = %e, "failed to create new session during refresh");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    let access_token = generate_access_token(&state, &user_name)
        .map_err(|e| {
            error!(error = %e, "failed to generate access token during refresh");
            ApiError::Internal(Error::Other("internal server error".into()))
        })?;

    let refresh_cookie = Cookie::build("refresh_token", session.refresh_token.clone())
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(CookieDuration::days(7))
        .path("/api/v1")
        .finish();

    Ok(HttpResponse::Ok()
        .cookie(refresh_cookie)
        .json(serde_json::json!({
             "access_token": access_token,
             "user_name": user_name
        })))
}





//===============================public api===============================


