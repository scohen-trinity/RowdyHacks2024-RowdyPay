use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetPaymentCommand {
    pub payment_id: i64,
}

#[derive(Deserialize)]
pub struct GetUserPaymentsCommand {
    pub user_id: i64,
}

#[derive(Deserialize)]
pub struct GetGroupPaymentsCommand {
    pub group_id: i64,
}
