mod util;

#[actix_rt::test]
async fn health_check_works() {
    let test_app = util::spawn_app().await;

    let client = awc::Client::default();

    let response = client
        .get(&format!("{}/health_check", test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}
