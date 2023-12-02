use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
#[derive(FromRow, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
}

pub async fn init_db() -> sqlx::Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            email TEXT NOT NULL
        )
        "#,
    )
    .execute(&SqlitePool::connect("sqlite:my_database.db").await?)
    .await?;

    Ok(())
}
