use axum::{http::StatusCode, routing::post, Json, Router};
use commands::balance_commands::{GetBalanceCommand, UpdateBalancesCommand};
use models::balance_model::Balance;

async fn get_balance(Json(payload): Json<GetBalanceCommand>) -> Json<Balance> {
    // TODO make the backend call with join on the users and groups table to get this
    let balance: Balance = Balance {
        balance_id: 1,
        user_id: payload.user_id,
        group_id: payload.group_id,
        amt: 10.00,
    };

    Json(balance)
}

async fn update_balances(Json(payload): Json<UpdateBalancesCommand>) -> StatusCode {
    let amount: f64 = payload.amt / payload.user_ids.len() as f64;
    for user in payload.user_ids {
        if user != payload.submitter_id {
            // TODO update the balances with the user id and balance id
            println!("Add {} to user_id {} and balance_id {}", amount, user, payload.group_id);
        }
    }
    StatusCode::OK
}

pub fn balance_routes() -> Router {
    Router::new()
        .route("/get_balance", post(get_balance))
        .route("/update_balances", post(update_balances))
}
