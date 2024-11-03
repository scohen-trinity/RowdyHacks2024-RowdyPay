use sqlx::{PgPool, Pool, Postgres};
use axum::{extract::State, routing::post, Json, Router};

use commands::group_commands::{CreateGroupCommand, GetGroupCommand, GetGroupsCommand};
use models::{group_db_models::{GroupDB, GroupUserDB, PartialGroupDB, ParticipantsDB}, group_model::Group, user_model::User};

async fn get_group(
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
async fn get_groups(
    State(pool): State<PgPool>,
    Json(payload): Json<GetGroupsCommand>
) -> Json<Vec<Group>> {
    // make the fetch call with the payload.id
    let groups_db: Vec<GroupDB> = sqlx::query_as!(
        GroupDB,
        "
        SELECT g.group_id, 
           g.group_name, 
           COALESCE(ARRAY(
               SELECT DISTINCT gp.user_id 
               FROM group_participants gp 
               WHERE gp.group_id = g.group_id
           ), '{}') AS users, 
           g.img
        FROM groups g
        JOIN group_participants gp ON g.group_id = gp.group_id
        WHERE gp.user_id = $1
        ",
        payload.user_id
    )
    .fetch_all(&pool)
    .await
    .expect("Could not fetch groups");

    let mut groups: Vec<Group> = Vec::new();

    for group_db in groups_db {
        let group: Group = Group {
            group_id: group_db.group_id,
            group_name: group_db.group_name,
            users: group_db.users.unwrap_or_default(),
            img: group_db.img.unwrap_or_default(),
        };

        groups.push(group);
    }

    println!("{:?}", groups);

    Json(groups)
}

async fn get_users_by_group(
    State(pool): State<PgPool>,
    Json(payload): Json<GetGroupCommand>
) -> Json<Vec<User>> {
    // make the database call to fetch the users from a specific group
    let participants_db: Vec<ParticipantsDB> = sqlx::query_as!(
        ParticipantsDB,
        "
        SELECT 
            u.user_id,
            u.display_name,
            u.email,
            u.img,
            COALESCE(ARRAY(
                SELECT DISTINCT gp.group_id 
                FROM group_participants gp 
                WHERE gp.user_id = u.user_id
            ), '{}') AS groups,
            COALESCE(ARRAY(
                SELECT DISTINCT p.pmt_id 
                FROM payments p 
                WHERE p.user_id = u.user_id
            ), '{}') AS payments,
            u.date_created
        FROM users u
        JOIN group_participants gp ON u.user_id = gp.user_id
        WHERE gp.group_id = $1
        ",
        payload.group_id
    )
    .fetch_all(&pool)
    .await
    .expect("Could not fetch participants from a group");

    let mut group_participants: Vec<User> = Vec::new();

    for participant_db in participants_db {
        let participant: User = User {
            user_id: participant_db.user_id,
            display_name: participant_db.display_name,
            email: participant_db.email,
            password: "".to_string(),
            img: participant_db.img.unwrap_or_default(),
            groups: participant_db.groups.unwrap_or_default(),
            payments: participant_db.payments.unwrap_or_default(),
            date_created: participant_db.date_created,
        };

        group_participants.push(participant);
    }

    Json(group_participants)
}

async fn create_group(
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

pub fn group_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/get_group", post(get_group))
        .route("/get_groups", post(get_groups))
        .route("/get_users_by_group", post(get_users_by_group))
        .route("/create_group", post(create_group))
}
