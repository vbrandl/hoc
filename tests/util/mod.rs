use hoc::{config::Settings, http, telemetry};

use std::{net::SocketAddr, sync::LazyLock};

use tempfile::{TempDir, tempdir};
use tokio::task::JoinHandle;

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let filter = if std::env::var("TEST_LOG").is_ok() {
        "debug"
    } else {
        ""
    };
    let subscriber = telemetry::get_subscriber(filter);
    telemetry::init_subscriber(subscriber);
});

pub struct TestApp {
    // Those are unused but are hold to be dropped and deleted after the TestApp goes out of scope
    _repo_dir: TempDir,
    _cache_dir: TempDir,
}

pub async fn spawn_app() -> (TestApp, JoinHandle<()>, SocketAddr) {
    LazyLock::force(&TRACING);

    let repo_dir = tempdir().expect("Cannot create repo_dir");
    let cache_dir = tempdir().expect("Cannot create cache_dir");

    let mut settings = Settings::load().expect("Failed to read configuration.");
    settings.port = 0;
    settings.repodir = repo_dir.path().to_path_buf();
    settings.cachedir = cache_dir.path().to_path_buf();

    let listener = settings.listener().await.unwrap();
    let app = http::router(settings).into_make_service_with_connect_info::<SocketAddr>();
    let addr = listener.local_addr().unwrap();

    (
        TestApp {
            _repo_dir: repo_dir,
            _cache_dir: cache_dir,
        },
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() }),
        addr,
    )
}
