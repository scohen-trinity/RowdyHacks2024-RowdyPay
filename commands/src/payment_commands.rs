use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetPaymentsCommand {
    pub id: u64,
}
