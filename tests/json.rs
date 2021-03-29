mod util;

use actix_web::client;

#[actix_rt::test]
async fn json_returns_success() {
    let test_app = util::spawn_app().await;

    let client = client::Client::default();

    let response = client
        .get(&format!("{}/github/vbrandl/hoc/json", test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}
