use axum::{extract::State, Json};
use commands::balance_commands::{GetBalanceCommand, UpdateBalancesCommand};
use dtos::balance_dtos::BalanceDB;
use models::balance_model::Balance;
use sqlx::PgPool;
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};

pub async fn get_balance(
    State(pool): State<PgPool>,
    Json(payload): Json<GetBalanceCommand>) -> Json<f32> {
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

    Json(balance.amt)
}

pub async fn update_balances(
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateBalancesCommand>
) -> Json<bool> {
    let amount: f32 = payload.amt / (payload.user_ids.len() as f32);
    println!("number of users: {}", payload.user_ids.len() as f32);
    for user in payload.user_ids {
        if user != payload.submitter_id {
            // TODO update the balances with the user id and balance id
            sqlx::query_as!(
                UpdateBalanceDTO,
                "
                INSERT INTO balances (user_id, group_id, amt)
                VALUES ($1, $2, $3)
                ON CONFLICT (user_id, group_id)
                DO UPDATE SET amt = balances.amt + EXCLUDED.amt
                ",
                user,
                payload.group_id,
                BigDecimal::from_f32(amount)
            )
            .fetch_one(&pool)
            .await
            .expect("Could not upsert the balance");
            // TODO update the transaction with the ower id and the owed id
            sqlx::query_as!(
                UpdateTransactionDTO,
                "
                INSERT INTO transactions (ower_id, owed_id, group_id, amt)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (ower_id, owed_id, group_id)
                DO UPDATE SET amt = transactions.amt + EXCLUDED.amt
                ",
                user,
                payload.submitter_id,
                payload.group_id,
                BigDecimal::from_f32(amount)
            )
            .fetch_one(&pool)
            .await
            .expect("Could not upsert the transaction");

            println!("Add {} to user_id {} and group_1 {}", amount, user, payload.group_id);
        }
    }
    
    Json(true)
}
