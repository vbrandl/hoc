use hoc::{config::Settings, telemetry};

use std::{net::TcpListener, sync::LazyLock};

use tempfile::{tempdir, TempDir};

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let filter = if std::env::var("TEST_LOG").is_ok() {
        "debug"
    } else {
        ""
    };
    let subscriber = telemetry::get_subscriber("test", filter);
    telemetry::init_subscriber(subscriber);
});

pub struct TestApp {
    pub address: String,
    // Those are unused but are hold to be dropped and deleted after the TestApp goes out of scope
    _repo_dir: TempDir,
    _cache_dir: TempDir,
}

pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{port}");

    let repo_dir = tempdir().expect("Cannot create repo_dir");
    let cache_dir = tempdir().expect("Cannot create cache_dir");

    let mut settings = Settings::load().expect("Failed to read configuration.");
    settings.repodir = repo_dir.path().to_path_buf();
    settings.cachedir = cache_dir.path().to_path_buf();

    let server = hoc::run(listener, settings)
        .await
        .expect("Failed to bind address");

    #[allow(clippy::let_underscore_future)]
    // don't await so the test server runs in the background
    let _ = tokio::spawn(server);

    TestApp {
        address,
        _repo_dir: repo_dir,
        _cache_dir: cache_dir,
    }
}
