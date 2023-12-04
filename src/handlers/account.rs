use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use sqlx::{Pool, Sqlite};

use crate::models::account;

#[axum::debug_handler]
pub async fn create_account(
    State(pool): State<Pool<Sqlite>>,
    Json(account): Json<account::CreateAccount>,
) -> axum::response::Result<Json<account::Account>, (StatusCode, String)> {
    tracing::info!("Adding Account {:?}", account);

    if let Err(e) = account::can_add_account(&pool, &account).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)));
    }

    let result = sqlx::query_as::<_, account::Account>(
        r#"
        INSERT INTO accounts (user_id, name, currency, principle, value) 
        VALUES (?, ?, ?, ?, ?) 
        RETURNING account_id, user_id, name, currency, principle, value
        "#,
    )
    .bind(account.user_id)
    .bind(&account.name)
    .bind(&account.currency)
    .bind(account.principle)
    .bind(account.value)
    .fetch_one(&pool)
    .await;

    tracing::info!("Result {:?}", result);

    match result {
        Ok(create_user) => Ok(Json(create_user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))),
    }
}

#[axum::debug_handler]
pub async fn get_account(
    State(pool): State<Pool<Sqlite>>,
    Path((account_id,)): Path<(i64,)>,
) -> axum::response::Result<Json<account::Account>, (StatusCode, String)> {
    tracing::info!("Reading Account {:?}", account_id);

    let result = sqlx::query_as::<_, account::Account>(
        r#"
        SELECT * FROM accounts
        WHERE account_id = ?
        "#,
    )
    .bind(account_id)
    .fetch_one(&pool)
    .await;

    tracing::info!("Result {:?}", result);

    match result {
        Ok(create_user) => Ok(Json(create_user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))),
    }
}

#[axum::debug_handler]
pub async fn update_account(
    State(pool): State<Pool<Sqlite>>,
    Path((account_id,)): Path<(i64,)>,
    Json(account): Json<account::CreateAccount>,
) -> axum::response::Result<Json<account::Account>, (StatusCode, String)> {
    tracing::info!("Updating Account {:?}", account);

    if let Err(e) = account::can_update_account(&pool, account_id, &account).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)));
    }

    let result = sqlx::query_as::<_, account::Account>(
        r#"
        UPDATE accounts
        SET user_id = ?, name = ?, currency = ?, principle = ?, value = ?,
        WHERE account_id = ?
        RETURNING account_id, user_id, name, currency, principle, value
        "#,
    )
    .bind(account.user_id)
    .bind(&account.name)
    .bind(&account.currency)
    .bind(account.principle)
    .bind(account.value)
    .bind(account_id)
    .fetch_one(&pool)
    .await;

    tracing::info!("Result {:?}", result);

    match result {
        Ok(create_user) => Ok(Json(create_user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))),
    }
}

#[axum::debug_handler]
pub async fn delete_account(
    State(pool): State<Pool<Sqlite>>,
    Path((account_id,)): Path<(i64,)>,
) -> axum::response::Result<Json<account::Account>, (StatusCode, String)> {
    tracing::info!("Deleting Account {:?}", account_id);

    if let Err(e) = account::can_delete_account(&pool, account_id).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)));
    }

    let result = sqlx::query_as::<_, account::Account>(
        r#"
        DELETE FROM accounts
        WHERE account_id = ?
        RETURNING account_id, user_id, name, currency, principle, value
        "#,
    )
    .bind(account_id)
    .fetch_one(&pool)
    .await;

    tracing::info!("Result {:?}", result);

    match result {
        Ok(create_user) => Ok(Json(create_user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))),
    }
}
