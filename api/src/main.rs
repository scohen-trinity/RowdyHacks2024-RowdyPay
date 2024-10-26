use axum::{routing::get, Router};
use controllers::{hello_world_controller::hello_world_routes, payment_controller::payment_routes, profile_controller::profile_routes};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // create the CORS layer to allow requests
    let cors: CorsLayer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // create the database connection pool
    // let db_connection_str: String = std::env::var("DATABASE_URL")
    //     .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    // TODO create the pool

    // application routes
    let app: Router = Router::new()
        .route("/", get(root))
        .nest("/api", hello_world_routes())
        .nest("/api", profile_routes())
        .nest("/api", payment_routes())
        .layer(cors);

    // listen globally to port 9000
    let listener: TcpListener = tokio::net::TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str { "Hello Axum!" }
