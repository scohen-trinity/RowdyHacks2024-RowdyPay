use axum::{
    extract::State, routing::post, Json, Router
};
use commands::profile_commands::GetProfileCommand;
use models::profile_model::Profile;
use sqlx::PgPool;

pub fn profile_routes() -> Router {
    Router::new()
        .route("/get_profile", post(get_profile))
}

async fn get_profile(Json(payload): Json<GetProfileCommand>) -> Json<Profile> {
    // State(pool): State<PgPool>, 
    // TODO implement fetch to the database to grab the profile with the id
    // let db_profile = sqlx::query_as!(
    //     Profile,
    //     "SELECT user_id, display_name, email, img FROM users WHERE user_id = $1",
    //     payload.id
    // )
    // .fetch_one(&pool)
    // .await
    // .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));

    // println!("Id: {}, name: {}, email: {}, img: {}", profile.unwrap().user_id, profile.unwrap().display_name, profile.unwrap().email, profile.unwrap().img);
    let profile: Profile = Profile {
        user_id: payload.id,
        display_name: "display_name".to_string(),
        email: "example@gmail.com".to_string(),
        img: "https://memories-matter.blog/wp-content/uploads/2018/08/sillymona.png".to_string(),
        groups: vec![],
        payments: vec![],
        date_created: 1729970177,
    };

    Json(profile)
}
