use axum::{routing::post, Json, Router};
use commands::payment_commands::{GetPaymentCommand, GetPaymentsCommand};
use models::payment_model::Payment;
use utils::date_util::Date;

async fn get_payment(Json(payload): Json<GetPaymentCommand>) -> Json<Payment> {
    // TODO implement fetch to the database based on a single payment id
    let payment: Payment = Payment {
        pmt_id: 1,
        user_id: payload.payment_id,
        group_id: 1,
        amt: 10.00,
        date_made: Date::new(22, 10, 2024),
        // description
    };

    Json(payment)
}

async fn get_payments(Json(payload): Json<GetPaymentsCommand>) -> Json<Vec<Payment>> {
    // TODO implement fetch to the database to grab the profile with the id
    let payments: Vec<Payment> = vec![
        Payment {
            pmt_id: 1,
            user_id: payload.user_id,
            group_id: 1,
            amt: 10.00,
            date_made: Date::new(22, 10, 2024),
            // description
        },
        Payment {
            pmt_id: 2,
            user_id: payload.user_id,
            group_id: 1,
            amt: 8.00,
            date_made: Date::new(24, 10, 2024),
        },
        Payment {
            pmt_id: 3,
            user_id: payload.user_id,
            group_id: 1,
            amt: 15.00,
            date_made: Date::new(26, 10, 2024),
        }
    ];

    Json(payments)
}

pub fn payment_routes() -> Router {
    Router::new()
        .route("/get_payment", post(get_payment))
        .route("/get_payments", post(get_payments))
}
