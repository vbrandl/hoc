mod util;

use axum::{body::Body, http::Request};
use http_body_util::BodyExt;

#[tokio::test]
async fn overview_contains_commit_url() {
    let (_test_app, handle, addr) = util::spawn_app().await;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build_http();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{addr}/github/vbrandl/badgers/view"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(body.to_vec()).unwrap();

    assert!(body.contains("https://github.com/vbrandl/badgers/commit/"));

    handle.abort();
}
