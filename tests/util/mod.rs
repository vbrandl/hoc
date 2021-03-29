use hoc::{config::Settings, telemetry};

use std::net::TcpListener;

use tempfile::{tempdir, TempDir};

lazy_static::lazy_static! {
    static ref TRACING: () = {
        let filter = if std::env::var("TEST_LOG").is_ok() { "debug" } else { "" };
        let subscriber = telemetry::get_subscriber("test", filter);
        telemetry::init_subscriber(subscriber);
    };
}

pub struct TestApp {
    pub address: String,
    repo_dir: TempDir,
    cache_dir: TempDir,
}

pub async fn spawn_app() -> TestApp {
    lazy_static::initialize(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let repo_dir = tempdir().expect("Cannot create repo_dir");
    let cache_dir = tempdir().expect("Cannot create cache_dir");

    let mut settings = Settings::load().expect("Failed to read configuration.");
    settings.repodir = repo_dir.path().to_path_buf();
    settings.cachedir = cache_dir.path().to_path_buf();
    // configuration.database.database_name = Uuid::new_v4().to_string();

    // let connection_pool = configure_database(&configuration.database).await;

    let server = hoc::run(listener, settings)
        .await
        .expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address,
        repo_dir,
        cache_dir,
    }
}
