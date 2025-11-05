mod cache;
pub mod config;
pub mod count;
mod error;
mod hoc;
mod http;
mod service;
mod statics;
pub mod telemetry;
mod template;

use crate::{
    cache::Persist,
    config::Settings,
    service::{Bitbucket, GitHub, Gitlab, Service, Sourcehut},
};

use std::{net::TcpListener, path::Path, sync::atomic::AtomicUsize};

use actix_web::{
    App, HttpServer,
    dev::Server,
    middleware::{self, TrailingSlash},
    web,
};
use tracing::{Instrument, info_span};

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

#[derive(Debug)]
pub(crate) struct State {
    settings: Settings,
}

impl State {
    fn repos(&self) -> &Path {
        &self.settings.repodir
    }

    fn cache(&self) -> &Path {
        &self.settings.cachedir
    }
}

#[allow(clippy::unused_async)]
async fn start_server(listener: TcpListener, settings: Settings) -> std::io::Result<Server> {
    let workers = settings.workers;
    let repo_count =
        // TODO: errorhandling
        web::Data::new(AtomicUsize::new(count::count_repositories(&settings.repodir).unwrap()));
    let state = web::Data::new(State {
        settings: settings.clone(),
    });
    let cache = web::Data::new(Persist::new(settings));
    Ok(HttpServer::new(move || {
        let app = App::new()
            .app_data(state.clone())
            .app_data(repo_count.clone())
            .app_data(cache.clone())
            .wrap(tracing_actix_web::TracingLogger::default())
            .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
            .service(http::index)
            .service(http::health_check)
            .service(http::static_file)
            .service(http::favicon32)
            .service(http::generate)
            .default_service(web::to(|repo_count: web::Data<AtomicUsize>| async move {
                http::p404(&repo_count)
            }));
        let app = GitHub::register_service(app);
        let app = Gitlab::register_service(app);
        let app = Bitbucket::register_service(app);
        Sourcehut::register_service(app)
    })
    .workers(workers)
    .listen(listener)?
    .run())
}

/// Start the server.
///
/// # Errors
///
/// * server cannot bind to `listener`
pub async fn run(listener: TcpListener, settings: Settings) -> std::io::Result<Server> {
    let span = info_span!("hoc", version = env!("CARGO_PKG_VERSION"));
    let _ = span.enter();
    start_server(listener, settings).instrument(span).await
}
