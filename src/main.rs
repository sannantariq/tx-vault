use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::SqlitePool;
use tx_vault::handlers::account::create_account;
use tx_vault::handlers::transaction::create_transaction;
use tx_vault::handlers::user::{create_user, delete_user, get_user, update_user};
use tx_vault::models::{ensure_db_exists, ensure_db_schema};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    ensure_db_exists().await.unwrap();
    ensure_db_schema().await.unwrap();
    let db_url = std::env::var("TX_DB").unwrap();

    let pool = SqlitePool::connect(db_url.as_str()).await.unwrap();

    // build our application with a route
    let routes_users = Router::new()
        .route("/", post(create_user))
        .route("/:user_id", get(get_user))
        .route("/:user_id", put(update_user))
        .route("/:user_id", delete(delete_user));

    let routes_accounts = Router::new().route("/", post(create_account));
    let routes_transactions = Router::new().route("/", post(create_transaction));

    let router = Router::new()
        .route("/", get(root))
        .nest("/api/v1/user", routes_users)
        .nest("/api/v1/account", routes_accounts)
        .nest("/api/v1/transaction", routes_transactions)
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
