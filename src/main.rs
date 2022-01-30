use micro_timestamp::startup;
use std::io;
use std::net::TcpListener;
use micro_timestamp::configurations::get_configuration;

#[tokio::main]
async fn main() -> io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!("{}:{}", configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;

    startup::run(listener)?.await
}
