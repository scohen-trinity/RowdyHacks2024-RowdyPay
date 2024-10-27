use sqlx::PgPool;
use axum::{extract::State, Json};

use models::{profile_db_models::GetProfileDB, profile_model::Profile};
use commands::profile_commands::{GetProfileCommand, LeaveGroupCommand};

pub async fn get_user(
    State(pool): State<PgPool>,
    Json(payload): Json<GetProfileCommand>
) -> Json<Profile> {
    let row: GetProfileDB = sqlx::query_as!(
        GetProfileDB,
        "SELECT 
            u.user_id, 
            u.display_name, 
            u.email, 
            u.img,
            COALESCE(ARRAY(SELECT DISTINCT gp.group_id FROM group_participants gp WHERE gp.user_id = u.user_id), '{}') AS groups,
            COALESCE(ARRAY(SELECT DISTINCT p.pmt_id FROM payments p WHERE p.user_id = u.user_id), '{}') AS payments
        FROM 
            users u
        WHERE 
            u.user_id = $1;",
        payload.user_id,
    )
        .fetch_one(&pool)
        .await
        .expect("Cannot fetch this user");

    let profile: Profile = Profile {
        user_id: row.user_id,
        display_name: row.display_name,
        email: row.email,
        img: row.img.unwrap_or_default(),
        groups: row.groups.unwrap_or_default(),
        payments: row.payments.unwrap_or_default(),
        date_created: 4273891,
    };

    Json(profile)
}

pub async fn leave_group(
    State(pool): State<PgPool>,
    Json(payload): Json<LeaveGroupCommand>
) -> Json<bool> {
    println!("{}, {}", payload.user_id, payload.group_id);
    // delete the user/group relation from the group_participants table
    sqlx::query!(
        "
        DELETE FROM group_participants
        WHERE user_id = $1 AND group_id = $2
        ",
        payload.user_id,
        payload.group_id,
    )
    .execute(&pool)
    .await
    .expect("could not remove user from a group");
    
    Json(true)
}
