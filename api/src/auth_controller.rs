use chrono::Utc;
use sqlx::PgPool;
use axum::{extract::State, Json};

use models::{auth_db_models::UserDB, profile_model::Profile};
use commands::auth_commands::{AuthUserCommand, CreateUserCommand};

// endpoint to authenticate the user
pub async fn auth_user(
    State(pool): State<PgPool>,
    Json(payload): Json<AuthUserCommand>
) -> Json<Profile> {
    let user: UserDB = sqlx::query_as!(
        UserDB,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(&pool)
    .await
    .expect("Cannot log user in");

    let profile: Profile = Profile::new(user.user_id, user.display_name, user.email, user.img.unwrap_or_default(), user.date_created);

    Json(profile)
}

// endpoint to create a new user
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserCommand>
) -> Json<Profile> {
    // Add the user to the database
    let user: UserDB = sqlx::query_as!(
        UserDB,
        r#"
        INSERT INTO users (display_name, email, img, date_created)
        VALUES ($1, $2, $3, $4)
        RETURNING user_id, display_name, email, img, date_created
        "#,
        payload.display_name,
        payload.email,
        "https://art.ngfiles.com/images/2357000/2357939_redshibe_silly-green-critter.png?f1645057365".to_string(),
        Utc::now().timestamp() as i32
    )
    .fetch_one(&pool)
    .await
    .expect("Could not add to database");

    let profile: Profile = Profile::new(user.user_id, user.display_name, user.email, user.img.unwrap_or_default(), user.date_created);

    Json(profile)
}