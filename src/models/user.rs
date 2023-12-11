use anyhow::anyhow;
use anyhow::bail;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::Pool;
use sqlx::Sqlite;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct User {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub is_main: bool,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub is_main: bool,
}

pub async fn can_add_user(pool: &Pool<Sqlite>, potential_user: &CreateUser) -> anyhow::Result<()> {
    if !potential_user.is_main {
        Ok(())
    } else {
        let main_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(pool)
            .await?;

        if main_count != 0 {
            bail!("Main User already exists, cannot create another one")
        } else {
            Ok(())
        }
    }
}

pub async fn can_update_user(
    pool: &Pool<Sqlite>,
    user_id: i64,
    potential_user: &CreateUser,
) -> anyhow::Result<()> {
    let current_user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    if current_user.is_main != potential_user.is_main {
        bail!("Invalid is_main property")
    }

    Ok(())
}

pub async fn can_delete_user(pool: &Pool<Sqlite>, user_id: i64) -> anyhow::Result<()> {
    let current_user = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users WHERE user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    if current_user.is_main {
        Err(anyhow!("Cannot delete main user"))
    } else {
        Ok(())
    }
}

pub async fn is_main_user(pool: &Pool<Sqlite>, user_id: i64) -> anyhow::Result<bool> {
    let main_user_opt = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users
        WHERE user_id = ?
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    match main_user_opt {
        Some(main_user) => Ok(main_user.is_main),
        None => Err(anyhow!("Requested user_id not found")),
    }
}

pub async fn is_main_username(pool: &Pool<Sqlite>, username: &str) -> anyhow::Result<bool> {
    let main_user_opt = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users
        WHERE username = ?
        "#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    match main_user_opt {
        Some(main_user) => Ok(main_user.is_main),
        None => Err(anyhow!("Requested username not found")),
    }
}
