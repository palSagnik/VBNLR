use std::net::TcpListener;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::test]
async fn health_check_works()
{
    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get(&format!("{}/health_check", app.address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

}

pub struct TestApp
{
    pub address: String,
    pub db_pool: PgPool
}

pub async fn spawn_app() -> TestApp
{
    let listener = TcpListener::bind("127.0.0.1:0")
            .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    let server = run(listener, connection_pool.clone())
        .expect("Failed to bind address");

    let _ = tokio::spawn(server);
    TestApp
    {
        address,
        db_pool: connection_pool,
    }
}

#[tokio::test]
async fn subscriber_200_return_for_valid()
{
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address)) 
        .header("Content-Type", "application/x-www-form-urlencoded") 
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, username FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription data");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.username, "le guin");
}

#[tokio::test]
async fn subscriber_400_return_for_invalid()
{
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")];

    for (invalid_body, error_message) in test_cases
    {
        let response = client
            .post(&format!("{}/subscriptions", &app.address)) 
            .header("Content-Type", "application/x-www-form-urlencoded") 
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400, 
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }

}
