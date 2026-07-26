
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand_core::{OsRng, RngCore};
use chrono::{Duration, Utc};
use sqlx::Row;

use crate::{
    appstate::AppState,
    error::Error,
    schemas::user::Session, 
    services::encrypt::hash_token
};

pub async fn new_session(state: &AppState, user_name: &String) -> Result<Session, Error> {
    let mut session_id_raw = [0u8; 32];
    OsRng.fill_bytes(&mut session_id_raw);

    let mut refresh_token_raw = [0u8; 32];
    OsRng.fill_bytes(&mut refresh_token_raw);

    let session = Session {
        session_id: URL_SAFE_NO_PAD.encode(session_id_raw),
        refresh_token: URL_SAFE_NO_PAD.encode(refresh_token_raw),
    };

    let hashed_session_id = hash_token(&session.session_id);
    let hashed_refresh_token = hash_token(&session.refresh_token);
    let now = Utc::now();
    let expiration = now + Duration::days(7);

    sqlx::query(
        "INSERT INTO sessions (session_id, refresh_token, user_name, created_at, expires_at)
         VALUES ($1, $2, $3, $4, $5)"
    )
        .bind(&hashed_session_id)
        .bind(&hashed_refresh_token)
        .bind(user_name)
        .bind(now)
        .bind(expiration)
        .execute(&state.db)
        .await?;

    Ok(session)
}

pub async fn validate_refresh_token(state: &AppState, refresh_token: &str) -> Result<String, Error> {
    let hashed = hash_token(refresh_token);

    let row = sqlx::query(
        "SELECT user_name FROM sessions
         WHERE refresh_token = $1 AND expires_at > NOW()"
    )
        .bind(&hashed)
        .fetch_optional(&state.db)
        .await?;

    match row {
        Some(r) => Ok(r.get("user_name")),
        None => Err(Error::Other("invalid or expired refresh token".into())),
    }
}

pub async fn revoke_session(state: &AppState, refresh_token: &str) -> Result<(), Error> {
    let hashed = hash_token(refresh_token);
    sqlx::query("DELETE FROM sessions WHERE refresh_token = $1")
        .bind(&hashed)
        .execute(&state.db)
        .await?;
    Ok(())
}