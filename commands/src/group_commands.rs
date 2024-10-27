use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetGroupCommand {
    pub group_id: i64,
}

#[derive(Deserialize)]
pub struct GetGroupsCommand {
    pub user_id: i64,
}

#[derive(Deserialize)]
pub struct CreateGroupCommand {
    pub name: String,
    pub img: Option<String>,
    pub user_ids: Vec<i64>,
    pub creator_id: i64,
}
