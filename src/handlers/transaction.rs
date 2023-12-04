use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use sqlx::{Pool, Sqlite};

use crate::models::transaction;

#[axum::debug_handler]
pub async fn create_transaction(
    State(pool): State<Pool<Sqlite>>,
    Json(transaction): Json<transaction::CreateTransaction>,
) -> axum::response::Result<Json<transaction::Transaction>, (StatusCode, String)> {
    tracing::info!("Adding Transaction {:?}", transaction);

    if let Err(e) = transaction::can_add_transaction(&pool, &transaction).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)));
    }

    let result = sqlx::query_as::<_, transaction::Transaction>(
        r#"
        INSERT INTO transactions (
            created_timestamp,
            updated_timestamp,
            src_username,
            dst_username,
            src_account_id,
            dst_account_id,
            tags,
            description,
            src_currency,
            dst_currency,
            src_debit,
            dst_credit,
            ) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?) 
        RETURNING (
            tx_id,
            created_timestamp,
            updated_timestamp,
            src_username,
            dst_username,
            src_account_id,
            dst_account_id,
            tags,
            description,
            src_currency,
            dst_currency,
            src_debit,
            dst_credit,
            )
        "#,
    )
    .bind(transaction.created_timestamp)
    .bind(transaction.updated_timestamp)
    .bind(&transaction.src_username)
    .bind(&transaction.dst_username)
    .bind(transaction.src_account_id)
    .bind(transaction.dst_account_id)
    .bind(&transaction.tags)
    .bind(&transaction.description)
    .bind(&transaction.src_currency)
    .bind(&transaction.dst_currency)
    .bind(transaction.src_debit)
    .bind(transaction.dst_credit)
    .fetch_one(&pool)
    .await;

    tracing::info!("Result {:?}", result);

    match result {
        Ok(create_user) => Ok(Json(create_user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))),
    }
}

#[axum::debug_handler]
pub async fn get_transaction(
    State(pool): State<Pool<Sqlite>>,
    Path((transaction_id,)): Path<(i64,)>,
) -> axum::response::Result<Json<transaction::Transaction>, (StatusCode, String)> {
    tracing::info!("Reading Transaction {:?}", transaction_id);

    let result = sqlx::query_as::<_, transaction::Transaction>(
        r#"
        SELECT * FROM transactions
        WHERE transaction_id = ?
        "#,
    )
    .bind(transaction_id)
    .fetch_one(&pool)
    .await;

    tracing::info!("Result {:?}", result);

    match result {
        Ok(create_user) => Ok(Json(create_user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))),
    }
}
