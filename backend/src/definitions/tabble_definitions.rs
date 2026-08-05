

pub fn user_table_def() -> &'static str  {
    "CREATE TABLE IF NOT EXISTS users (
        user_name CHAR(20) PRIMARY KEY,
        password_hash VARCHAR(255) NOT NULL,
    )"
}

pub fn post_table_def() -> &'static str {
    "CREATE TABLE IF NOT EXISTS users(
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        oc_name CHAR(100) NOT NULL,
        description CHAR(1000),
        specie CHAR(100),
        sex CHAR(1),
        creator_user_name CHAR(20) NOT NULL
    )"
}

pub fn magma_table_def() -> &'static str {
    "CREATE TABLE IF NOT EXISTS magmas(
        id CHAR(20)EXT PRIMARY KEY,
        created_at INTEGER NOT NULL
    )"
}

pub async fn run_tables_defs(pool : &sqlx::Pool<sqlx::Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(user_table_def()).execute(pool).await?;
    sqlx::query(post_table_def()).execute(pool).await?;
    sqlx::query(magma_table_def()).execute(pool).await?;
    Ok(())
}