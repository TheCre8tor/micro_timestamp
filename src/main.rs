use micro_timestamp::configurations::get_configuration;
use micro_timestamp::startup;
use micro_timestamp::telemetry::{get_subscriber, init_subscriber};
use std::io;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    /* ------- Telemetry & Logs Config Initialization ------- */
    let subscriber = get_subscriber("micro_timestamp".into(), "info".into(), io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    startup::run(listener)?.await
}
