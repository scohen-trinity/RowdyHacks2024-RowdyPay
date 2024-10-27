use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetBalanceCommand {
    pub user_id: i32,
    pub group_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateBalancesCommand {
    pub submitter_id: i32, 
    pub group_id: i32,
    pub user_ids: Vec<i32>,
    pub amt: f32,
}
