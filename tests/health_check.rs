#[actix_rt::test]
async fn test_health_check() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("127.0.0.1:8000/health")
        .send()
        .await
        .expect("Health check failed.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = payments::start_server().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
