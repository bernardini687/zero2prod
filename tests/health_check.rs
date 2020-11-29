use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::get_app_settings;
use zero2prod::startup::run;

/// Start an instance of the application and
/// return its address (i.e. `"http://localhost:XXXX"`).
fn spawn_app() -> String {
    // port 0 is special one at the OS level:
    // binding to it will trigger a scan for an available port which will then be bound to the application
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port(); // unwrap the port before giving ownership of `listener` to `run`
    let server = run(listener).expect("Failed to listen to the address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

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
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribtions_returns_200_on_valid_form_data() {
    let address = spawn_app();

    let app_settings = get_app_settings().expect("Failed to read configuration file");
    let mut _db_conn = PgConnection::connect(&app_settings.db.connection_url())
        .await
        .expect("Failed to connect to database");

    let client = reqwest::Client::new();
    let params = [("email", "an@ton.io"), ("name", "antonio")];

    let response = client
        .post(&format!("{}/subscriptions", &address))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    // let subscription_record = sqlx::query!("SELECT email, name FROM subscriptions")
    //     .fetch_one(&mut db_conn)
    //     .await
    //     .expect("Failed to fetch subscription");

    // assert_eq!("an@ton.io", subscription_record.email);
    // assert_eq!("antonio", subscription_record.name);
}

#[actix_rt::test]
async fn subscribtions_returns_400_on_invalid_form_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let table_driven_test_cases = vec![
        ("email=mr_bill%40example.com", "missing the name"),
        ("name=mr%20bill", "missing the email"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, failed_test_message) in table_driven_test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}",
            failed_test_message
        );
    }
}
