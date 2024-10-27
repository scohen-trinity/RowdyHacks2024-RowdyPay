use sqlx::types::BigDecimal;

pub struct BalanceDB {
    pub balance_id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub amt: BigDecimal,
}

pub struct UpdateBalanceDTO {
    pub balance_id: i32,
    pub user_id: i32, 
    pub group_id: i32,
    pub amt: f32,
}

pub struct UpdateTransactionDTO {
    pub t_id: i32,
    pub ower_id: i32,
    pub owed_id: i32,
    pub group_id: i32,
    pub amt: i32,
}
