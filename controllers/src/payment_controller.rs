use axum::{routing::get, Json, Router};
use commands::payment_commands::GetPaymentsCommand;
use models::payment_model::Payment;
use utils::date_util::Date;

async fn get_payments(Json(payload): Json<GetPaymentsCommand>) -> Json<Vec<Payment>> {
    // TODO implement fetch to the database to grab the profile with the id
    let payments: Vec<Payment> = vec![
        Payment {
            pmt_id: 1,
            user_id: payload.id,
            group_id: 1,
            amt: 10.00,
            date_made: Date::new(26, 10, 2024),
        }
    ];

    Json(payments)
}

pub fn payment_routes() -> Router {
    Router::new()
        .route("/get_payments", get(get_payments))
}
