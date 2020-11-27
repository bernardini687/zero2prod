use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

#[derive(serde::Deserialize, Debug)]
struct Subscription {
    email: String,
    name: String,
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscriptions(subscription_form: web::Form<Subscription>) -> HttpResponse {
    println!("data: {:?}", subscription_form);

    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        // `web::get()` is a shortcut for `Route::new().guard(guard::Get())`
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
    })
    // .bind(address)? // rather than binding to an address, let's listen to an already established TCP connection
    .listen(listener)?
    .run();

    Ok(server)
}
