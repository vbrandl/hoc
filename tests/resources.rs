mod util;

#[actix_rt::test]
async fn favicon() {
    let test_app = util::spawn_app().await;

    let client = awc::Client::default();

    let response = client
        .get(&format!("{}/favicon.ico", test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}

#[actix_rt::test]
async fn tacit_css() {
    let test_app = util::spawn_app().await;

    let client = awc::Client::default();

    let response = client
        .get(&format!("{}/tacit-css.min.css", test_app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
}
