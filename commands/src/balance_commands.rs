use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetBalanceCommand {
    pub user_id: u64, 
    pub group_id: u64,
}
