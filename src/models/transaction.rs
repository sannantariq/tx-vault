use crate::models::user;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Transaction {
    pub tx_id: i64,
    pub created_timestamp: f64,
    pub updated_timestamp: f64,
    pub src_username: String,
    pub dst_username: String,
    pub src_account_id: i64,
    pub dst_account_id: i64,
    pub tags: String,
    pub description: String,
    pub src_currency: String,
    pub dst_currency: String,
    pub src_debit: f32,
    pub dst_credit: f32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct CreateTransaction {
    pub created_timestamp: f64,
    pub updated_timestamp: f64,
    pub src_username: String,
    pub dst_username: String,
    pub src_account_id: Option<i64>,
    pub dst_account_id: Option<i64>,
    pub tags: String,
    pub description: String,
    pub src_currency: String,
    pub dst_currency: String,
    pub src_debit: f32,
    pub dst_credit: f32,
}

pub async fn can_add_transaction(
    pool: &Pool<Sqlite>,
    potential_tx: &CreateTransaction,
) -> anyhow::Result<()> {
    if user::is_main_username(pool, &potential_tx.src_username).await?
        || user::is_main_username(pool, &potential_tx.dst_username).await?
    {
        Err(anyhow!(
            "Neither user is main: {:?}, {:?}",
            potential_tx.src_username,
            potential_tx.dst_username,
        ))
    } else {
        Ok(())
    }
}
