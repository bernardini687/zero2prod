use std::net::TcpListener;

#[actix_web::main] // macro needed to mark binary entrypoint `main` as `async`
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    zero2prod::run(listener)?.await
}
