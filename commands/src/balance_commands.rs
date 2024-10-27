use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetBalanceCommand {
    pub user_id: i64, 
    pub group_id: i64,
}

#[derive(Deserialize)]
pub struct UpdateBalancesCommand {
    pub submitter_id: i64, 
    pub group_id: i64,
    pub user_ids: Vec<i64>,
    pub amt: f64,
}
