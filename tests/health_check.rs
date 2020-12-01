use sqlx::Executor;
use sqlx::{Connection, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;
use zero2prod::configuration::{get_app_settings, DatabaseSettings};
use zero2prod::startup::run;

struct TestApp {
    address: String,
    db_pool: PgPool,
}

/// Start an instance of the application and
/// return a struct with its address (i.e. `"http://localhost:XXXX"`)
/// and the pool of connections pointing to its database.
async fn spawn_app() -> TestApp {
    let mut app_settings = get_app_settings().expect("Failed to read configuration file");
    app_settings.db.name = Uuid::new_v4().to_string();
    let db_pool = configure_db(&app_settings.db).await;

    // port 0 is special one at the OS level:
    // binding to it will trigger a scan for an available port which will then be bound to the application
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");

    let port = listener.local_addr().unwrap().port(); // unwrap the port before giving ownership of `listener` to `run`
    let server = run(listener, db_pool.clone()).expect("Failed to listen to the address");
    let _ = tokio::spawn(server);

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_pool,
    }
}

async fn configure_db(db_settings: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect(&db_settings.connection_url_without_db())
        .await
        .expect("Failed to connect to default database");

    conn.execute(&*format!(r#"CREATE DATABASE "{}""#, db_settings.name))
        .await
        .expect("Failed to create database");

    let db_pool = PgPool::connect(&db_settings.connection_url())
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate database");

    db_pool
}

// testing equivalent of `actix_rt::main`.
// you can inspect what code gets generated using:
// `cargo expand --test health_check`
#[actix_rt::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn subscribtions_returns_200_on_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let params = [("email", "an@ton.io"), ("name", "antonio")];

    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .form(&params)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let record = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch subscription");

    assert_eq!("an@ton.io", record.email);
    assert_eq!("antonio", record.name);
}

#[actix_rt::test]
async fn subscribtions_returns_400_on_invalid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let table_driven_test_cases = vec![
        ("email=mr_bill%40example.com", "missing the name"),
        ("name=mr%20bill", "missing the email"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, failed_test_message) in table_driven_test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
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
