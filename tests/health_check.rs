use std::net::TcpListener;

// testing equivalent of `actix_rt::main`.
// you can inspect what code gets generated using:
// `cargo expand --test health_check`
#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    // port 0 is special one at the OS level:
    // binding to it will trigger a scan for an available port which will then be bound to the application
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port.");

    let port = listener.local_addr().unwrap().port(); // unwrap the port before giving ownership of `listener` to `run`
    let server = zero2prod::run(listener).expect("Failed to listen to the address.");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
