use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetGroupsCommand {
    pub id: u64,
}
