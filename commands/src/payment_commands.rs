use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetPaymentCommand {
    pub payment_id: u64,
}

#[derive(Deserialize)]
pub struct GetPaymentsCommand {
    pub user_id: u64,
}
