use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use sqlx::{Pool, Sqlite};

use crate::models::user;

#[axum::debug_handler]
pub async fn create_user(
    State(pool): State<Pool<Sqlite>>,
    Json(user): Json<user::CreateUser>,
) -> axum::response::Result<Json<user::User>, (StatusCode, String)> {
    tracing::info!("Adding User {:?}", user);

    if let Err(e) = user::can_add_user(&pool, &user).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)));
    }

    let result = sqlx::query_as::<_, user::User>(
        r#"INSERT INTO users 
        (username, email, is_main)
        VALUES (?, ?, ?)
        RETURNING user_id, username, email, is_main"#,
    )
    .bind(&user.username)
    .bind(&user.email)
    .bind(user.is_main)
    .fetch_one(&pool)
    .await;

    tracing::info!("Result {:?}", result);

    match result {
        Ok(create_user) => Ok(Json(create_user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))),
    }
}

#[axum::debug_handler]
pub async fn get_user(
    State(pool): State<Pool<Sqlite>>,
    Path((user_id,)): Path<(i32,)>,
) -> axum::response::Result<Json<user::User>, (StatusCode, String)> {
    tracing::info!("Reading User {:?}", user_id);

    let result = sqlx::query_as::<_, user::User>(
        r#"SELECT user_id, username, email, is_main FROM users WHERE user_id = ?"#,
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await;

    tracing::info!("Result {:?}", result);

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::NOT_FOUND, format!("{}", e))),
    }
}

#[axum::debug_handler]
pub async fn update_user(
    State(pool): State<Pool<Sqlite>>,
    Path((user_id,)): Path<(i64,)>,
    Json(user): Json<user::CreateUser>,
) -> axum::response::Result<Json<user::User>, (StatusCode, String)> {
    tracing::info!("Updating User {:?} to {:?}", user_id, user);

    if let Err(e) = user::can_update_user(&pool, user_id, &user).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)));
    }

    let result = sqlx::query_as::<_, user::User>(
        r#"
        UPDATE users 
        SET username = ?, email = ?, is_main = ?,
        WHERE user_id = ?
        RETURNING user_id, username, email"#,
    )
    .bind(&user.username)
    .bind(&user.email)
    .bind(user.is_main)
    .bind(user_id)
    .fetch_one(&pool)
    .await;

    tracing::info!("Result {:?}", result);

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e))),
    }
}

#[axum::debug_handler]
pub async fn delete_user(
    State(pool): State<Pool<Sqlite>>,
    Path((user_id,)): Path<(i64,)>,
) -> axum::response::Result<Json<user::User>, (StatusCode, String)> {
    tracing::info!("Deleting User {:?}", user_id);

    if let Err(e) = user::can_delete_user(&pool, user_id).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("{}", e)));
    }

    let result = sqlx::query_as::<_, user::User>(
        r#"DELETE FROM users WHERE user_id = ? RETURNING user_id, username, email, is_main"#,
    )
    .bind(user_id)
    .fetch_one(&pool)
    .await;

    tracing::info!("Result {:?}", result);

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::NOT_FOUND, format!("{}", e))),
    }
}
