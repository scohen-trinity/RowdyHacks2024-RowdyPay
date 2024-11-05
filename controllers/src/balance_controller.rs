use sqlx::{PgPool, Pool, Postgres};
use axum::{extract::State, routing::post, Json, Router};
use bigdecimal::{BigDecimal, FromPrimitive};

use models::{balance_db_models::BalanceDB, balance_model::Balance};
use commands::balance_commands::{GetBalanceCommand, UpdateBalancesCommand};

// endpoint to get a single balance
async fn get_balance(
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

    let balance: Balance = Balance::from(balance_db);

    // let balance: Balance = Balance {
    //     balance_id: balance_db.balance_id,
    //     user_id: balance_db.user_id,
    //     group_id: balance_db.group_id,
    //     amt: balance_db.amt.to_f32().unwrap(),
    // };

    Json(balance.amt)
}

async fn update_balances(
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateBalancesCommand>
) -> Json<bool> {
    // assuming that the user ids list includes all users including the one requesting payment
    let amount: f32 = payload.amt / ((payload.user_ids.len()) as f32);
    for user in payload.user_ids {
        if user != payload.submitter_id {
            sqlx::query_as!(
                models::balance_db_models::UpdateBalanceDB,
                "
                INSERT INTO balances (user_id, group_id, amt)
                VALUES ($2, $3, $1)
                ON CONFLICT (user_id, group_id)
                DO UPDATE SET amt = balances.amt + $1
                ",
                BigDecimal::from_f32(amount),
                user,
                payload.group_id,
            )
            .execute(&pool)
            .await
            .expect("Could not upsert the balance");

            sqlx::query_as!(
                UpdateTransactionDB,
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
            .execute(&pool)
            .await
            .expect("Could not upsert the transaction");
        }
    }
    
    Json(true)
}

pub fn balance_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/get_balance", post(get_balance))
        .route("/update_balances", post(update_balances))
}