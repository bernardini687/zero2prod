use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        // `web::get()` is a shortcut for `Route::new().guard(guard::Get())`
        App::new().route("/health_check", web::get().to(health_check))
    })
    // .bind(address)? // rather than binding to an address, let's listen to an already established TCP connection
    .listen(listener)?
    .run();

    Ok(server)
}
