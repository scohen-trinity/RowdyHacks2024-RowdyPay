use models::profile_model::Profile;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetGroupCommand {
    pub group_id: u64,
}

#[derive(Deserialize)]
pub struct GetGroupsCommand {
    pub user_id: u64,
}

#[derive(Deserialize)]
pub struct CreateGroupCommand {
    pub name: String,
    pub image: String,
    pub user_ids: Vec<Profile>,
    pub creator_id: u64,
}
