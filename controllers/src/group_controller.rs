use axum::{routing::post, Json, Router};
use commands::group_commands::{CreateGroupCommand, GetGroupCommand, GetGroupsCommand};
use models::{group_model::Group, profile_model::Profile};

async fn get_group(Json(payload): Json<GetGroupCommand>) -> Json<Group> {
    let group: Group = Group { 
        id: payload.group_id,
        name: "test group".to_string(),
        users: vec![1, 2],
        image: "https://media.istockphoto.com/id/1332758692/photo/swimming-trunks-on-a-white-background.jpg?s=612x612&w=0&k=20&c=D2_XK7R0mSAe43Moij5jnoD__QS_koqWdmWnVyiP9Js=".to_string(),
    };

    Json(group)
}

async fn get_groups(Json(payload): Json<GetGroupsCommand>) -> Json<Vec<Group>> {
    // TODO make the fetch call with the payload.id
    println!("Getting the groups from users id: {}", payload.user_id);
    let groups: Vec<Group> = vec![
        Group { 
            id: 1,
            name: "test group 1".to_string(),
            users: vec![1, 2],
            image: "https://media.istockphoto.com/id/1332758692/photo/swimming-trunks-on-a-white-background.jpg?s=612x612&w=0&k=20&c=D2_XK7R0mSAe43Moij5jnoD__QS_koqWdmWnVyiP9Js=".to_string(),
        },
        Group { 
            id: 2,
            name: "test group 2".to_string(),
            users: vec![1, 2, 3],
            image: "https://media.istockphoto.com/id/1332758692/photo/swimming-trunks-on-a-white-background.jpg?s=612x612&w=0&k=20&c=D2_XK7R0mSAe43Moij5jnoD__QS_koqWdmWnVyiP9Js=".to_string(),
        },
    ];

    Json(groups)
}

async fn get_users_by_group(Json(payload): Json<GetGroupCommand>) -> Json<Vec<Profile>> {
    // TODO make the database call to fetch the users from a specific group
    println!("Fetching users by group_id: {}", payload.group_id.to_string());
    let group_participants = vec![
        Profile {
            id: 1,
            display_name: "Aiden".to_string(),
            email: "aiden@gmail.com".to_string(),
            image: "https://memories-matter.blog/wp-content/uploads/2018/08/sillymona.png".to_string(),
            groups: vec![],
            payments: vec![],
            date_created: 1729970177,
        },
        Profile {
            id: 2,
            display_name: "Sam".to_string(),
            email: "sam@gmail.com".to_string(),
            image: "https://memories-matter.blog/wp-content/uploads/2018/08/sillymona.png".to_string(),
            groups: vec![],
            payments: vec![],
            date_created: 1729970177,
        },
        Profile {
            id: 3,
            display_name: "Khoi".to_string(),
            email: "khoi@gmail.com".to_string(),
            image: "https://memories-matter.blog/wp-content/uploads/2018/08/sillymona.png".to_string(),
            groups: vec![],
            payments: vec![],
            date_created: 1729970177,
        }
    ];

    Json(group_participants)
}

async fn create_group(Json(payload): Json<CreateGroupCommand>) -> Json<Group> {
    // TODO make the backend call to the database to add the group as well as the associated balance
    let group: Group = Group::new(payload.name, payload.user_ids, payload.image);

    Json(group)
}

pub fn group_routes() -> Router {
    Router::new()
        .route("/get_group", post(get_group))
        .route("/get_groups", post(get_groups))
        .route("/get_users_by_group", post(get_users_by_group))
        .route("/create_group", post(create_group))
}
