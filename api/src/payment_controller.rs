use axum::{routing::post, Json, Router};
use commands::payment_commands::{GetGroupPaymentsCommand, GetPaymentCommand, GetUserPaymentsCommand, MakePaymentCommand};
use models::payment_model::Payment;
use utils::date_util::Date;

pub async fn get_payment(Json(payload): Json<GetPaymentCommand>) -> Json<Payment> {
    // TODO implement fetch to the database based on a single payment id
    let payment: Payment = Payment {
        pmt_id: 1,
        user_id: payload.payment_id,
        group_id: 1,
        amt: 10.00,
        date_created: Date::new(22, 10, 2024),
        description: "RowdyHacks".to_string(),
    };

    Json(payment)
}

pub async fn get_user_payments(Json(payload): Json<GetUserPaymentsCommand>) -> Json<Vec<Payment>> {
    // TODO implement fetch to the database to grab the profile with the id
    let payments: Vec<Payment> = vec![
        Payment {
            pmt_id: 1,
            user_id: payload.user_id,
            group_id: 1,
            amt: 10.00,
            date_created: Date::new(22, 10, 2024),
            description: "Red bull".to_string(),
        },
        Payment {
            pmt_id: 2,
            user_id: payload.user_id,
            group_id: 1,
            amt: 8.00,
            date_created: Date::new(24, 10, 2024),
            description: "goon sesh".to_string(),
        },
        Payment {
            pmt_id: 3,
            user_id: payload.user_id,
            group_id: 1,
            amt: 15.00,
            date_created: Date::new(26, 10, 2024),
            description: "hookers and blow".to_string(),
        }
    ];

    Json(payments)
}

pub async fn get_group_payments(Json(payload): Json<GetGroupPaymentsCommand>) -> Json<Vec<Payment>> {
    let payments: Vec<Payment> = vec![
        Payment {
            pmt_id: 1,
            user_id: 1,
            group_id: payload.group_id,
            amt: 10.00,
            date_created: Date::new(22, 10, 2024),
            description: "something something".to_string(),
        },
        Payment {
            pmt_id: 2,
            user_id: 2,
            group_id: payload.group_id,
            amt: 8.00,
            date_created: Date::new(24, 10, 2024),
            description: "almost there".to_string(),
        },
        Payment {
            pmt_id: 3,
            user_id: 3,
            group_id: payload.group_id,
            amt: 15.00,
            date_created: Date::new(26, 10, 2024),
            description: "goofy af".to_string(),
        }
    ];

    Json(payments)
}

pub async fn make_payment(Json(payload): Json<MakePaymentCommand>) -> Json<bool> {
    let description: String = "".to_string();
    // TODO create a payment
    let payment: Payment = Payment::new(1, payload.user_id, payload.user_id, payload.amt, description);
    println!("{:?}", payment);
    // TODO decremement the current balance
    Json(true)
}

pub fn payment_routes() -> Router {
    Router::new()
        .route("/get_payment", post(get_payment))
        .route("/get_user_payments", post(get_user_payments))
        .route("/get_group_payments", post(get_group_payments))
        .route("/make_payment", post(make_payment))
}
