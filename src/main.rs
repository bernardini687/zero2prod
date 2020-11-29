use std::net::TcpListener;
use zero2prod::configuration::get_app_settings;
use zero2prod::startup::run;

#[actix_web::main] // macro needed to mark binary entrypoint `main` as `async`
async fn main() -> std::io::Result<()> {
    let app_settings = get_app_settings().expect("Failed to read configuration file");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", app_settings.port))?;

    run(listener)?.await
}
