use reqwest::Client;

mod spawn;
use spawn::spawn_app;

#[tokio::test]
async fn should_return_todays_date_and_timestamp_if_date_params_is_empty() {
    let app_address = spawn_app().await;
    let client = Client::new();

    let response = client
        .get(&format!("{}/api", &app_address))
        .send()
        .await
        .expect("msg");

    assert!(response.status().is_success());
    assert_eq!(200, response.status().as_u16());
    assert_eq!(Some(59), response.content_length());
}
