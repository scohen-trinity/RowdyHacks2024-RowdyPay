use sqlx::{PgPool, Pool, Postgres};
use axum::{extract::State, routing::post, Json, Router};
use bigdecimal::{BigDecimal, FromPrimitive};

use models::{balance_db_models::BalanceDB, balance_model::Balance};
use commands::balance_commands::{GetBalanceCommand, UpdateBalancesCommand};

// endpoint to get a single balance
async fn get_balance(
    State(pool): State<PgPool>,
    Json(payload): Json<GetBalanceCommand>
) -> Json<f32> {
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

    Json(balance.amt)
}

async fn update_balances(
    State(pool): State<PgPool>,
    Json(payload): Json<UpdateBalancesCommand>
) -> Json<bool> {
    // TODO: assuming that the user ids list includes all users including the one requesting payment
    let amount: f32 = payload.amt / ((payload.user_ids.len()) as f32);
    for user in payload.user_ids {
        if user != payload.submitter_id {
            // TODO test this, looks good I am just testing it way too late
            sqlx::query_as!(
                UpdateBalanceDB,
                "
                UPDATE balances
                SET amt = balances.amt + $1
                ",
                BigDecimal::from_f32(amount),
            )
            .execute(&pool)
            .await
            .expect("Could not upsert the balance");

            // TODO test this
            sqlx::query_as!(
                UpdateTransactionDB,
                "
                UPDATE transactions
                SET amt = transactions.amt + $1
                ",
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
