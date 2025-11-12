mod util;

#[actix_rt::test]
async fn html_returns_success() {
    let test_app = util::spawn_app().await;

    let client = awc::Client::default();

    let mut response = client
        .get(&format!("{}/github/vbrandl/hoc/view", test_app.address))
        .send()
        .await
        .expect("Failed to execute request");
    let body_bytes = response.body().await.unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();

    assert!(response.status().is_success());
    assert!(body_str.contains("https://github.com/vbrandl/hoc/commit/020d35ff106f0d6b7d99e9a9971a4f5c8d3bd6a1"));
}
