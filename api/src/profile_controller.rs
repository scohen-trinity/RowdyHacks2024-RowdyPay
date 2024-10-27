use axum::{extract::State, Json};
use commands::profile_commands::{GetProfileCommand, GetProfileDB};
use models::profile_model::Profile;
use sqlx::PgPool;

pub async fn get_user(
    State(pool): State<PgPool>,
    Json(payload): Json<GetProfileCommand>
) -> Json<Profile> {
    println!("{}", payload.user_id);
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

    println!("{}, {}, {}, {}", row.user_id, row.display_name, row.email, row.img.clone().unwrap_or_default());

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
