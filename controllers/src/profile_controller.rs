use axum::{
    routing::post,
    Router,
    Json,
};
use commands::profile_commands::GetProfileCommand;
use models::profile_model::Profile;

async fn get_profile(Json(id): Json<GetProfileCommand>) -> Json<Profile> {
    // TODO implement fetch to the database to grab the profile with the id
    let profile: Profile = Profile {
        id: id.id,
        display_name: "display_name".to_string(),
        email: "example@gmail.com".to_string(),
        image: "https://memories-matter.blog/wp-content/uploads/2018/08/sillymona.png".to_string(),
        groups: vec![],
        payments: vec![],
        date_created: 1729970177,
    };

    Json(profile)
}

pub fn profile_routes() -> Router {
    Router::new()
        .route("/get_profile", post(get_profile))
}
