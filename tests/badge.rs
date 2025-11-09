mod util;

use axum::{body::Body, http::Request};

#[tokio::test]
async fn badge_succeeds() {
    let (_test_app, handle, addr) = util::spawn_app().await;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build_http();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{addr}/github/vbrandl/hoc"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(response.status().is_success());
    handle.abort();
}
