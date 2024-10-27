use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetPaymentCommand {
    pub payment_id: i32,
}

#[derive(Deserialize)]
pub struct GetUserPaymentsCommand {
    pub user_id: i32,
}

#[derive(Deserialize)]
pub struct GetGroupPaymentsCommand {
    pub group_id: i32,
}

#[derive(Deserialize)]
pub struct MakePaymentCommand {
    pub user_id: i32,
    pub group_id: i32,
    pub amt: f32,
    pub description: Option<String>,
}
