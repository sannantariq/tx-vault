pub mod account;
pub mod transaction;
pub mod user;

use sqlx::migrate::MigrateDatabase;
use sqlx::{Sqlite, SqlitePool};

pub async fn ensure_db_exists() -> anyhow::Result<()> {
    let db_url = std::env::var("TX_DB")?;
    tracing::info!("Ensuring Database exists at {}", db_url);
    if !Sqlite::database_exists(db_url.as_str())
        .await
        .unwrap_or(false)
    {
        tracing::info!("No database found. Creating Database..");
        Sqlite::create_database(db_url.as_str())
            .await
            .map_err(anyhow::Error::msg)
    } else {
        tracing::info!("Database Exists");
        Ok(())
    }
}

pub async fn ensure_db_schema() -> anyhow::Result<()> {
    let db_url = std::env::var("TX_DB")?;
    tracing::info!("Checking Database Schema at {}", db_url);

    tracing::info!("Checking users table");
    let db = SqlitePool::connect(db_url.as_str()).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            user_id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT UNIQUE NOT NULL,
            email TEXT NOT NULL,
            is_main BOOLEAN NOT NULL
        )
        "#,
    )
    .execute(&db)
    .await?;

    tracing::info!("Checking accounts table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS accounts (
            account_id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER,
            name TEXT NOT NULL,
            currency TEXT NOT NULL,
            principle REAL NOT NULL,
            value REAL NOT NULL,
            FOREIGN KEY (user_id)
                REFERENCES users (user_id)
        )
        "#,
    )
    .execute(&db)
    .await?;

    tracing::info!("Checking transactions table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS transactions (
            tx_id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_timestamp REAL NOT NULL,
            updated_timestamp REAL NOT NULL,
            src_username TEXT NOT NULL,
            dst_username TEXT NOT NULL,
            src_account_id INTEGER NOT NULL,
            dst_account_id INTEGER NOT NULL,
            tags TEXT NOT NULL,
            description TEXT NOT NULL,
            src_currency TEXT NOT NULL,
            dst_currency TEXT NOT NULL,
            src_debit REAL NOT NULL,
            dst_credit REAL NOT NULL,
            FOREIGN KEY (src_account_id)
                REFERENCES accounts (account_id),
            FOREIGN KEY (dst_account_id)
                REFERENCES accounts (account_id)
        )
        "#,
    )
    .execute(&db)
    .await?;

    Ok(())
}
