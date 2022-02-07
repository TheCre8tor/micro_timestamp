use reqwest::Client;

mod spawn;
use spawn::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app().await;
    let client = Client::new();

    let response = client
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
}
