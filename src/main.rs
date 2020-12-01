use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_app_settings;
use zero2prod::startup::run;

#[actix_web::main] // macro needed to mark binary entrypoint `main` as `async`
async fn main() -> std::io::Result<()> {
    let app_settings = get_app_settings().expect("Failed to read configuration file");
    let db_pool = PgPool::connect(&app_settings.db.connection_url())
        .await
        .expect("Failed to connect to database");

    let listener = TcpListener::bind(format!("127.0.0.1:{}", app_settings.port))?;

    run(listener, db_pool)?.await
}
