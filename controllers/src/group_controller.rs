use axum::{routing::post, Json, Router};
use commands::group_commands::GetGroupsCommand;
use models::group_model::Group;

async fn get_groups(Json(payload): Json<GetGroupsCommand>) -> Json<Vec<Group>> {
    let group: Group = Group { 
        id: payload.id,
        name: "test group".to_string(),
        users: vec![1, 2],
        image: "something.txt".to_string(),
    };

    Json(vec![group])
}

pub fn group_routes() -> Router {
    Router::new()
        .route("/get_groups", post(get_groups))
}
