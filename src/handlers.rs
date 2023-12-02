use axum::{
    extract::{Json, Path},
    http::StatusCode,
};

use crate::models::{init_db, User};

pub async fn create_user(Json(user): Json<User>) -> axum::response::Result<Json<User>, StatusCode> {
    init_db().await.unwrap(); // Ensure the database is initialized

    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email) VALUES (?, ?) RETURNING id, username, email",
    )
    .bind(&user.username)
    .bind(&user.email)
    .fetch_one(
        &sqlx::SqlitePool::connect("sqlite:my_database.db")
            .await
            .unwrap(),
    )
    .await;

    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[axum::debug_handler]
pub async fn new_user(Json(user): Json<User>) -> Json<User> {
    init_db().await.unwrap(); // Ensure the database is initialized

    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email) VALUES (?, ?) RETURNING id, username, email",
    )
    .bind(&user.username)
    .bind(&user.email)
    .fetch_one(
        &sqlx::SqlitePool::connect("sqlite:my_database.db")
            .await
            .unwrap(),
    )
    .await;
    // Ok(Json(result.unwrap()))
    Json(result.unwrap())
}

pub async fn get_user(Path((id,)): Path<(i32,)>) -> Result<Json<User>, StatusCode> {
    init_db().await.unwrap(); // Ensure the database is initialized

    let result = sqlx::query_as::<_, User>("SELECT id, username, email FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(
            &sqlx::SqlitePool::connect("sqlite:my_database.db")
                .await
                .unwrap(),
        )
        .await;

    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}
