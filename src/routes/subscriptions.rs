use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize, Debug)]
pub struct Subscription {
    email: String,
    name: String,
}

pub async fn subscriptions(
    form: web::Form<Subscription>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, HttpResponse> {
    // if compiler complains about errors generated in the macro,
    // check if db is running and `DATABASE_URL` points to it!
    sqlx::query!(
        "INSERT INTO subscriptions (id, email, name, subscribed_at) VALUES ($1, $2, $3, $4)",
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Failed to execute query: {}", e);
        HttpResponse::InternalServerError().finish()
    })?;

    Ok(HttpResponse::Ok().finish())
}
