use dotenvy::dotenv;
use tokio::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use axum::{routing::{delete, get, post}, Router};

use auth_controller::{auth_user, create_user};
use user_controller::{get_user, leave_group};
use balance_controller::{get_balance, update_balances};
use group_controller::{create_group, get_group, get_groups, get_users_by_group};
use payment_controller::{get_group_payments, get_payment, get_user_payments, make_payment};

pub mod balance_controller;
pub mod hello_world_controller;
pub mod payment_controller;
pub mod user_controller;
pub mod group_controller;
pub mod auth_controller;

#[tokio::main]
async fn main() {
    // fetching environment variables
    dotenv().ok();

    // create the CORS layer to allow requests
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // create the database connection pool
    let db_connection_str: String = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    // create the database pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await
        .expect("couldn't connect to the database");

    // testing the database with a simple query
    let row: (String,) = sqlx::query_as("SELECT version()")
        .fetch_one(&pool)
        .await
        .expect("Failed to execute test query");

    println!("Connection is working, psql version: {}", row.0);

    // application routes
    let app: Router = Router::new()
        .route("/", get(root))
        .route("/api/get_user", post(get_user))
        .route("/api/get_balance", post(get_balance))
        .route("/api/update_balances", post(update_balances))
        .route("/api/get_group", post(get_group))
        .route("/api/get_groups", post(get_groups))
        .route("/api/get_users_by_group", post(get_users_by_group))
        .route("/api/get_payment", post(get_payment))
        .route("/api/get_user_payments", post(get_user_payments))
        .route("/api/get_group_payments", post(get_group_payments))
        .route("/api/make_payment", post(make_payment))
        .route("/api/create_user", post(create_user))
        .route("/api/auth_user", post(auth_user))
        .route("/api/create_group", post(create_group))
        .route("/api/leave_group", delete(leave_group))
        .with_state(pool)
        .layer(cors);

    // listen globally to port 9000
    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str { "Hello Axum!" }
