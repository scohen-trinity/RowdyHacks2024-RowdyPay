use axum::{routing::post, Json, Router};
use commands::group_commands::{GetGroupCommand, GetGroupsCommand};
use models::group_model::Group;

async fn get_group(Json(payload): Json<GetGroupCommand>) -> Json<Group> {
    let group: Group = Group { 
        id: payload.group_id,
        name: "test group".to_string(),
        users: vec![1, 2],
        image: "something.txt".to_string(),
    };

    Json(group)
}

async fn get_groups(Json(payload): Json<GetGroupsCommand>) -> Json<Vec<Group>> {
    let groups: Vec<Group> = vec![
        Group { 
            id: payload.user_id,
            name: "test group 1".to_string(),
            users: vec![1, 2],
            image: "something.txt".to_string(),
        },
        Group { 
            id: payload.user_id,
            name: "test group 2".to_string(),
            users: vec![1, 2, 3],
            image: "another_something.txt".to_string(),
        },
    ];

    Json(groups)
}

pub fn group_routes() -> Router {
    Router::new()
        .route("/get_group", post(get_group))
        .route("/get_groups", post(get_groups))
}
