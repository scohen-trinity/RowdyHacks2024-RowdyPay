use axum::{extract::State, Json};
use commands::balance_commands::{GetBalanceCommand, UpdateBalancesCommand};
use dtos::balance_dtos::BalanceDB;
use models::balance_model::Balance;
use sqlx::PgPool;
use bigdecimal::ToPrimitive;

pub async fn get_balance(
    State(pool): State<PgPool>,
    Json(payload): Json<GetBalanceCommand>) -> Json<Balance> {
    // make the backend call balances
    let balance_db: BalanceDB = sqlx::query_as!(
            BalanceDB,
            "SELECT * FROM balances WHERE user_id = $1 AND group_id = $2",
            payload.user_id,
            payload.group_id
        )
        .fetch_one(&pool)
        .await
        .expect("No balance here");

    let balance: Balance = Balance {
        balance_id: balance_db.balance_id,
        user_id: balance_db.user_id,
        group_id: balance_db.group_id,
        amt: balance_db.amt.to_f32().unwrap(),
    };

    Json(balance)
}

pub async fn update_balances(Json(payload): Json<UpdateBalancesCommand>) -> Json<bool> {
    let amount: f32 = payload.amt / (payload.user_ids.len() as f32);
    println!("number of users: {}", payload.user_ids.len() as f32);
    for user in payload.user_ids {
        if user != payload.submitter_id {
            // TODO update the balances with the user id and balance id
            println!("Add {} to user_id {} and group_1 {}", amount, user, payload.group_id);
        }
    }
    
    Json(true)
}
