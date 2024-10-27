use sqlx::types::BigDecimal;

pub struct PaymentDB {
    pub pmt_id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub amt: BigDecimal,
    pub date_created: i32,
    pub description: Option<String>,
}

pub struct CreatePaymentDB {
    pub user_id: i32,
    pub group_id: i32,
    pub amt: BigDecimal,
    pub date_created: i32,
    pub description: String,
}
