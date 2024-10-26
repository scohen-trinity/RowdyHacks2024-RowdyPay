use axum::{routing::post, Json, Router};
use commands::payment_commands::{GetGroupPaymentsCommand, GetPaymentCommand, GetUserPaymentsCommand};
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
        description: "hookers and blow".to_string(),
    };

    Json(payment)
}

async fn get_user_payments(Json(payload): Json<GetUserPaymentsCommand>) -> Json<Vec<Payment>> {
    // TODO implement fetch to the database to grab the profile with the id
    let payments: Vec<Payment> = vec![
        Payment {
            pmt_id: 1,
            user_id: payload.user_id,
            group_id: 1,
            amt: 10.00,
            date_made: Date::new(22, 10, 2024),
            description: "Red bull".to_string(),
        },
        Payment {
            pmt_id: 2,
            user_id: payload.user_id,
            group_id: 1,
            amt: 8.00,
            date_made: Date::new(24, 10, 2024),
            description: "goon sesh".to_string(),
        },
        Payment {
            pmt_id: 3,
            user_id: payload.user_id,
            group_id: 1,
            amt: 15.00,
            date_made: Date::new(26, 10, 2024),
            description: "hookers and blow".to_string(),
        }
    ];

    Json(payments)
}

async fn get_group_payments(Json(payload): Json<GetGroupPaymentsCommand>) -> Json<Vec<Payment>> {
    let payments: Vec<Payment> = vec![
        Payment {
            pmt_id: 1,
            user_id: 1,
            group_id: payload.group_id,
            amt: 10.00,
            date_made: Date::new(22, 10, 2024),
            description: "something something".to_string(),
        },
        Payment {
            pmt_id: 2,
            user_id: 2,
            group_id: payload.group_id,
            amt: 8.00,
            date_made: Date::new(24, 10, 2024),
            description: "almost there".to_string(),
        },
        Payment {
            pmt_id: 3,
            user_id: 3,
            group_id: payload.group_id,
            amt: 15.00,
            date_made: Date::new(26, 10, 2024),
            description: "goofy af".to_string(),
        }
    ];

    Json(payments)
}

pub fn payment_routes() -> Router {
    Router::new()
        .route("/get_payment", post(get_payment))
        .route("/get_user_payments", post(get_user_payments))
        .route("/get_group_payments", post(get_group_payments))
}
