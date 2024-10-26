use axum::{
    routing::get,
    Json,
    Router
};

async fn get_hello_world() -> Json<String> {
    Json("Hello World!".to_string())
}

pub fn hello_world_routes() -> Router {
    Router::new()
        .route("/hello_world", get(get_hello_world))
}
