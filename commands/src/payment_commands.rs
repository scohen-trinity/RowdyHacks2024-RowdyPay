use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetPaymentCommand {
    pub payment_id: u64,
}

#[derive(Deserialize)]
pub struct GetUserPaymentsCommand {
    pub user_id: u64,
}

#[derive(Deserialize)]
pub struct GetGroupPaymentsCommand {
    pub group_id: u64,
}
