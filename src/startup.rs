use crate::routes::{health_check, subscriptions};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

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
