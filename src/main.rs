use micro_timestamp::startup;
use std::io;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let address = format!("{}:{}", "127.0.0.1", "7070");
    let listener = TcpListener::bind(address)?;

    startup::run(listener)?.await
}
