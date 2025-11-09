use axum::{body::Body, http::Request};

mod util;

#[tokio::test]
async fn health_check_works() {
    // let test_app = util::spawn_app().await;
    let (_test_app, handle, addr) = util::spawn_app().await;

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build_http();

    let response = client
        .request(
            Request::builder()
                .uri(format!("http://{addr}/health_check"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(response.status().is_success());
    handle.abort();
}
