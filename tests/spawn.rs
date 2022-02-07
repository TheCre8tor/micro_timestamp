use std::net::TcpListener;

use micro_timestamp::startup;

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let server = startup::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    address
}