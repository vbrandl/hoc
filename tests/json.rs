mod util;

use axum::{body::Body, http::Request};

#[tokio::test]
async fn json_returns_success() {
    let (_test_app, handle, addr) = util::spawn_app().await;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build_http();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{addr}/github/vbrandl/hoc/json"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    handle.abort();
}
