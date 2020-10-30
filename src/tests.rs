use crate::{
    calculate_hoc, index, json_hoc,
    service::{Bitbucket, GitHub, Gitlab, Service},
    State,
};

use actix_web::{http, test, web, App};
use tempfile::tempdir;

macro_rules! test_app {
    ($path: expr) => {
        test::init_service(App::new().service($path)).await
    };
    ($state: expr, $path: expr) => {
        test::init_service(App::new().data($state).service($path)).await
    };
}

macro_rules! test_service {
    ($name: ident, $path: tt, $what: ident) => {
        async fn $name<T: 'static + Service>(req_path: &str) {
            let repo_dir = dbg!(tempdir().unwrap());
            let cache_dir = dbg!(tempdir().unwrap());
            let repos = format!("{}/", repo_dir.path().display());
            let cache = format!("{}/", cache_dir.path().display());
            let state = dbg!(State {
                repos,
                cache,
                logger: crate::config::init(),
            });

            let mut app = test_app!(state, web::resource($path).to($what::<T>));

            let req = dbg!(test::TestRequest::with_uri(req_path).to_request());
            let resp = dbg!(test::call_service(&mut app, req).await);

            assert_eq!(resp.status(), http::StatusCode::OK);
        }
    };
}

#[actix_rt::test]
async fn test_index() {
    let mut app = test::init_service(App::new().service(index)).await;

    let req = dbg!(test::TestRequest::with_uri("/").to_request());
    let resp = dbg!(test::call_service(&mut app, req).await);

    assert_eq!(resp.status(), http::StatusCode::OK);
}

// TODO: fix this test
// #[actix_rt::test]
async fn test_json() {
    test_service!(test_json_service, "/service/{user}/{repo}/json", json_hoc);

    test_json_service::<Gitlab>("/service/vbrandl/hoc/json").await;
    test_json_service::<GitHub>("/service/vbrandl/hoc/json").await;
    test_json_service::<Bitbucket>("/service/vbrandl/hoc/json").await;
}

// TODO: fix this test
// #[actix_rt::test]
async fn test_badge() {
    test_service!(test_badge_service, "/service/{user}/{repo}", calculate_hoc);

    test_badge_service::<Gitlab>("/service/vbrandl/hoc").await;
    test_badge_service::<GitHub>("/service/vbrandl/hoc").await;
    test_badge_service::<Bitbucket>("/service/vbrandl/hoc").await;
}
