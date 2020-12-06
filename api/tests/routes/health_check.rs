#[actix_rt::test]
async fn test_health_check() {
    // Setup
    let client = reqwest::Client::new();
    let response = client
        .get("api:8000/health")
        .send()
        .await
        .expect("Health check failed.");

    // Test(s)
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
