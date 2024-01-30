use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

#[tokio::main]
async fn main() -> std::io::Result<()>
{
    // Panic if not able to read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");

    // No more hardcode ports
    // Now directly from the settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)
        .expect("Failed to bind random port");
    run(listener)?.await
}
