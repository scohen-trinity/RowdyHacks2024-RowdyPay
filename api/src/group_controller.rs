use axum::{extract::State, Json};
use commands::group_commands::{CreateGroupCommand, GetGroupCommand, GetGroupsCommand};
use dtos::group_dtos::{GroupDB, GroupUserDB, PartialGroupDB};
use models::{group_model::Group, profile_model::Profile};
use sqlx::PgPool;

pub async fn get_group(
    State(pool): State<PgPool>,
    Json(payload): Json<GetGroupCommand>
) -> Json<Group> {
    // make call to database to fetch the group absed on group_id
    let group_db: GroupDB = sqlx::query_as!(
        GroupDB,
        "SELECT g.group_id, g.group_name, ARRAY(SELECT DISTINCT gp.group_id FROM group_participants gp WHERE gp.group_id = g.group_id) AS users, g.img 
        FROM groups g
        WHERE g.group_id = $1",
        payload.group_id
    )
    .fetch_one(&pool)
    .await
    .expect("Cannot fetch this group");

    let group: Group = Group {
        group_id: group_db.group_id,
        group_name: group_db.group_name,
        users: group_db.users.unwrap_or_default(),
        img: group_db.img.unwrap_or_default(),
    };

    Json(group)
}

// for returning groups associated with a user_id
pub async fn get_groups(Json(payload): Json<GetGroupsCommand>) -> Json<Vec<Group>> {
    // TODO make the fetch call with the payload.id
    println!("Getting the groups from users id: {}", payload.user_id);
    let groups: Vec<Group> = vec![
        Group { 
            group_id: 1,
            group_name: "test group 1".to_string(),
            users: vec![1, 2],
            img: "https://media.istockphoto.com/id/1332758692/photo/swimming-trunks-on-a-white-background.jpg?s=612x612&w=0&k=20&c=D2_XK7R0mSAe43Moij5jnoD__QS_koqWdmWnVyiP9Js=".to_string(),
        },
        Group { 
            group_id: 2,
            group_name: "test group 2".to_string(),
            users: vec![1, 2, 3],
            img: "https://media.istockphoto.com/id/1332758692/photo/swimming-trunks-on-a-white-background.jpg?s=612x612&w=0&k=20&c=D2_XK7R0mSAe43Moij5jnoD__QS_koqWdmWnVyiP9Js=".to_string(),
        },
    ];

    Json(groups)
}

pub async fn get_users_by_group(Json(payload): Json<GetGroupCommand>) -> Json<Vec<Profile>> {
    // TODO make the database call to fetch the users from a specific group
    println!("Fetching users by group_id: {}", payload.group_id.to_string());
    let group_participants = vec![
        Profile {
            user_id: 1,
            display_name: "Aiden".to_string(),
            email: "aiden@gmail.com".to_string(),
            img: "https://memories-matter.blog/wp-content/uploads/2018/08/sillymona.png".to_string(),
            groups: vec![],
            payments: vec![],
            date_created: 1729970177,
        },
        Profile {
            user_id: 2,
            display_name: "Sam".to_string(),
            email: "sam@gmail.com".to_string(),
            img: "https://memories-matter.blog/wp-content/uploads/2018/08/sillymona.png".to_string(),
            groups: vec![],
            payments: vec![],
            date_created: 1729970177,
        },
        Profile {
            user_id: 3,
            display_name: "Khoi".to_string(),
            email: "khoi@gmail.com".to_string(),
            img: "https://memories-matter.blog/wp-content/uploads/2018/08/sillymona.png".to_string(),
            groups: vec![],
            payments: vec![],
            date_created: 1729970177,
        },
    ];

    Json(group_participants)
}

pub async fn create_group(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateGroupCommand>
) -> Json<Group> {
    // TODO make the backend call to the database to add the group as well as the associated balance
    let img = payload.img.unwrap_or_default();
    let returned_partial_group: PartialGroupDB = sqlx::query_as!(
        PartialGroupDB,
        "INSERT INTO groups (group_name, img)
        VALUES ($1, $2)
        RETURNING group_id, group_name, img",
        payload.name,
        img
    )
    .fetch_one(&pool)
    .await
    .expect("Error inserting group");

    let mut inserted_users: Vec<i32> = Vec::new();

    for user in payload.user_ids {
        println!("{}", user);
        let inserted_user: GroupUserDB = sqlx::query_as!(
            GroupUserDB,
            "
            INSERT INTO group_participants
            VALUES ($1, $2)
            RETURNING user_id
            ",
            user,
            returned_partial_group.group_id
        )
        .fetch_one(&pool)
        .await
        .expect("Error inserting a user into group_participants");

        inserted_users.push(inserted_user.user_id);
    }

    let group: Group = Group {
        group_id: returned_partial_group.group_id,
        group_name: returned_partial_group.group_name,
        users: inserted_users,
        img: returned_partial_group.img.unwrap_or_default(),
    };

    Json(group)
}
