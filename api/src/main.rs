use axum::{routing::get, Router};
use controllers::{balance_controller::balance_routes, group_controller::group_routes, hello_world_controller::hello_world_routes, payment_controller::payment_routes, profile_controller::profile_routes};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

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
        .with_state(pool)
        .route("/", get(root))
        .nest("/api", hello_world_routes())
        .nest("/api", balance_routes())
        .nest("/api", group_routes())
        .nest("/api", payment_routes())
        .nest("/api", profile_routes())
        .layer(cors);

    // listen globally to port 9000
    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str { "Hello Axum!" }
