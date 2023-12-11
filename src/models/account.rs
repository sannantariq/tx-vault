use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};

use crate::models::user;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Account {
    pub account_id: i64,
    pub user_id: i64,
    pub name: String,
    pub currency: String,
    pub principle: f32,
    pub value: f32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct CreateAccount {
    pub user_id: i64,
    pub name: String,
    pub currency: String,
    pub principle: f32,
    pub value: f32,
}

fn validate_currency(currency: &str) -> anyhow::Result<()> {
    if currency.len() != 3 || currency.chars().any(|c| c.is_lowercase()) {
        return Err(anyhow!("Invalid Currency String: {}", currency));
    }
    Ok(())
}

pub async fn can_add_account(
    _pool: &Pool<Sqlite>,
    potential_account: &CreateAccount,
) -> anyhow::Result<()> {
    validate_currency(&potential_account.currency)
}

pub async fn can_update_account(
    _pool: &Pool<Sqlite>,
    _account_id: i64,
    potential_account: &CreateAccount,
) -> anyhow::Result<()> {
    validate_currency(&potential_account.currency)
}

pub async fn can_delete_account(_pool: &Pool<Sqlite>, _account_id: i64) -> anyhow::Result<()> {
    Ok(())
}

pub async fn is_main_account(pool: &Pool<Sqlite>, account_id: i64) -> anyhow::Result<bool> {
    let main_account_opt = sqlx::query_as::<_, Account>(
        r#"
        SELECT * FROM accounts
        WHERE account_id = ?
        "#,
    )
    .bind(account_id)
    .fetch_optional(pool)
    .await?;

    match main_account_opt {
        Some(main_account) => user::is_main_user(pool, main_account.user_id).await,
        None => Err(anyhow!("Requested account_id not found")),
    }
}
