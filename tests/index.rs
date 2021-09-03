mod util;

#[actix_rt::test]
async fn index_returns_success() {
    let test_app = util::spawn_app().await;

    let client = awc::Client::default();

    let response = client
        .get(&format!("{}/", test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}
