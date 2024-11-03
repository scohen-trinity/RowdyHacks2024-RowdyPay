use chrono::Utc;
use sqlx::{PgPool, Pool, Postgres};
use axum::{extract::State, routing::post, Json, Router};

use models::{auth_db_models::UserDB, user_model::User};
use commands::auth_commands::{AuthUserCommand, CreateUserCommand};

// endpoint to authenticate the user
async fn auth_user(
    State(pool): State<PgPool>,
    Json(payload): Json<AuthUserCommand>
) -> Json<User> {
    let user: UserDB = sqlx::query_as!(
        UserDB,
        "SELECT * FROM users WHERE email = $1",
        payload.email
    )
    .fetch_one(&pool)
    .await
    .expect("Cannot log user in");

    let user: User = User::new(user.user_id, user.display_name, user.email, "".to_string(), user.img.unwrap_or_default(), user.date_created);

    Json(user)
}

// endpoint to create a new user
async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserCommand>
) -> Json<User> {
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

    let user: User = User::new(user.user_id, user.display_name, user.email, "".to_string(), user.img.unwrap_or_default(), user.date_created);

    Json(user)
}

pub fn auth_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/create_user", post(create_user))
        .route("/auth_user", post(auth_user))
}
