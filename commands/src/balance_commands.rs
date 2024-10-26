use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetBalanceCommand {
    pub user_id: u64, 
    pub group_id: u64,
}

#[derive(Deserialize)]
pub struct UpdateBalancesCommand {
    pub submitter_id: u64, 
    pub group_id: u64,
    pub user_ids: Vec<u64>,
    pub amt: f64,
}
