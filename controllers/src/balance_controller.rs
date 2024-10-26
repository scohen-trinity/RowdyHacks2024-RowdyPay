use axum::{routing::post, Json, Router};
use commands::balance_commands::GetBalanceCommand;
use models::balance_model::Balance;

async fn get_balance(Json(payload): Json<GetBalanceCommand>) -> Json<Balance> {
    // TODO make the backend call with join on the users and groups table to get this
    let balance: Balance = Balance {
        user_id: payload.user_id,
        group_id: payload.group_id,
        amt: 10.00,
    };

    Json(balance)
}

pub fn balance_routes() -> Router {
    Router::new()
        .route("/get_balance", post(get_balance))
}
