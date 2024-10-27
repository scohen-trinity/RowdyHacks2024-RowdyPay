use sqlx::types::BigDecimal;

pub struct BalanceDB {
    pub balance_id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub amt: BigDecimal,
}
