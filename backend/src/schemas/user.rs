
use serde::{Deserialize, Serialize};
use sqlx::FromRow;



#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserLogin{
    pub user_name: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Session{
    pub session_id: String,
    pub refresh_token: String
}