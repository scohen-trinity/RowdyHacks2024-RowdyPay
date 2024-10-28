use chrono::Utc;
use sqlx::PgPool;
use axum::{extract::State, Json};
use bigdecimal::{BigDecimal, FromPrimitive, ToPrimitive};

use models::{payment_db_models::PaymentDB, payment_model::Payment};
use commands::payment_commands::{
    GetGroupPaymentsCommand,
    GetPaymentCommand,
    GetUserPaymentsCommand,
    MakePaymentCommand
};

pub async fn get_payment(
    State(pool): State<PgPool>,
    Json(payload): Json<GetPaymentCommand>
) -> Json<Payment> {
    // fetch a single payment
    let payment_db: PaymentDB = sqlx::query_as!(
        PaymentDB,
        "
        SELECT pmt_id, user_id, group_id, amt, date_created, description from payments WHERE pmt_id = $1
        ",
        payload.payment_id
    )
    .fetch_one(&pool)
    .await
    .expect("Cannot fetch single payment");

    let payment: Payment = Payment {
        pmt_id: payment_db.pmt_id,
        user_id: payment_db.user_id,
        group_id: payment_db.group_id,
        amt: payment_db.amt.to_f32().unwrap(),
        date_created: payment_db.date_created,
        description: payment_db.description.unwrap_or_default(),
    };

    Json(payment)
}

pub async fn get_user_payments(
    State(pool): State<PgPool>,
    Json(payload): Json<GetUserPaymentsCommand>
) -> Json<Vec<Payment>> {
    // implement fetch to the database to grab the user with the id
    let payments_db: Vec<PaymentDB> = sqlx::query_as!(
        PaymentDB,
        "
        SELECT pmt_id, user_id, group_id, amt, date_created, description from payments WHERE user_id = $1
        ",
        payload.user_id

    )
    .fetch_all(&pool)
    .await
    .expect("could not fetch payments from a user");

    let mut payments: Vec<Payment> = Vec::new();

    for payment_db in payments_db {
        let payment: Payment = Payment {
            pmt_id: payment_db.pmt_id,
            user_id: payment_db.user_id,
            group_id: payment_db.group_id,
            amt: payment_db.amt.to_f32().unwrap(),
            date_created: payment_db.date_created,
            description: payment_db.description.unwrap_or_default(),
        };

        payments.push(payment);
    }

    Json(payments)
}

pub async fn get_group_payments(
    State(pool): State<PgPool>,
    Json(payload): Json<GetGroupPaymentsCommand>
) -> Json<Vec<Payment>> {
    let payments_db: Vec<PaymentDB> = sqlx::query_as!(
        PaymentDB,
        "
        SELECT pmt_id, user_id, group_id, amt, date_created, description
        FROM payments
        WHERE group_id = $1
        ",
        payload.group_id
    )
    .fetch_all(&pool)
    .await
    .expect("Could not retrieve payments for a group");
        
    let mut payments: Vec<Payment> = Vec::new();

    for payment_db in payments_db {
        let payment: Payment = Payment {
            pmt_id: payment_db.pmt_id,
            user_id: payment_db.user_id,
            group_id: payment_db.group_id,
            amt: payment_db.amt.to_f32().unwrap(),
            date_created: payment_db.date_created,
            description: payment_db.description.unwrap_or_default(),
        };

        payments.push(payment);
    }

    Json(payments)
}

pub async fn make_payment(
    State(pool): State<PgPool>,
    Json(payload): Json<MakePaymentCommand>
) -> Json<bool> {
    let description: String = payload.description.unwrap_or_default();
    // add a new payment once the button is pressed
    sqlx::query!(
        "
        INSERT INTO payments (user_id, group_id, amt, date_created, description)
        VALUES ($1, $2, $3, $4, $5)
        ",
        payload.user_id,
        payload.group_id,
        BigDecimal::from_f32(payload.amt),
        Utc::now().timestamp() as i32,
        description
    )
    .execute(&pool)
    .await
    .expect("failed to add payment");

    // decrement the current balance
    sqlx::query!(
        "
        UPDATE balances
        SET amt = 0
        WHERE user_id = $1 AND group_id = $2
        ",
        payload.user_id,
        payload.group_id,
    )
    .execute(&pool)
    .await
    .expect("failed to reduce current balance to zero");

    Json(true)
}
