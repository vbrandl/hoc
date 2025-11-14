use axum::{body::Body, http::Request};

mod util;

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
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/json"
    );
    handle.abort();
}
