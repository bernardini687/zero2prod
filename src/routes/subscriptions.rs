use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct Subscription {
    email: String,
    name: String,
}

pub async fn subscriptions(subscription_form: web::Form<Subscription>) -> HttpResponse {
    println!("data: {:?}", subscription_form);

    HttpResponse::Ok().finish()
}
