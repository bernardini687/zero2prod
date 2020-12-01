use crate::routes::{health_check, subscriptions};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // `PgPool` is wrapped in a smart pointer `std::sync::Arc`
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        // `get()` is a shortcut for `Route::new().guard(guard::Get())`
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions))
            // Using `.data` would add another Arc pointer on top
            // of the existing one - an unnecessary indirection.
            // `.app_data` does not perform an additional layer of wrapping.
            .app_data(db_pool.clone())
    })
    // .bind(address)? // rather than binding to an address, let's listen to an already established TCP connection
    .listen(listener)?
    .run();

    Ok(server)
}
