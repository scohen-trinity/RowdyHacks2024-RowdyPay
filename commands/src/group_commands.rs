use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetGroupCommand {
    pub group_id: u64,
}

#[derive(Deserialize)]
pub struct GetGroupsCommand {
    pub user_id: u64,
}
