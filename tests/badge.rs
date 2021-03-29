mod util;

use actix_web::client;

#[actix_rt::test]
async fn badge_succeeds() {
    let test_app = util::spawn_app().await;

    let client = client::Client::default();

    let response = client
        .get(&format!("{}/github/vbrandl/hoc", test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}
