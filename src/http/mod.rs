#[macro_use]
mod render;

mod hoc;
mod routes;

use crate::{
    cache::{HocParams, Persist},
    config::Settings,
    error::Error,
    platform::Platform,
    statics::VERSION_INFO,
    templates,
    worker::{Queue, worker},
};

use std::sync::{Arc, atomic::AtomicUsize};

use axum::{
    Router,
    body::Body,
    extract::{Path, Request, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use tower_http::{
    compression::CompressionLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    trace::{DefaultMakeSpan, MakeSpan, TraceLayer},
};
use tracing::{error, instrument, warn};

pub struct AppState {
    pub settings: Settings,
    pub repo_count: AtomicUsize,
    pub cache: Persist,
    pub queue: Queue<HocParams>,
}

impl AppState {
    fn repos(&self) -> &std::path::Path {
        &self.settings.repodir
    }
}

#[instrument(skip(state))]
async fn redirect_old_overview(
    State(state): State<Arc<AppState>>,
    Path((platform, owner, repo)): Path<(Platform, String, String)>,
) -> impl IntoResponse {
    warn!("request to deprecated endpoint");
    Redirect::permanent(&format!(
        "{}/{}/{owner}/{repo}/view",
        state.settings.base_url,
        platform.url_path()
    ))
}

pub fn router(state: Arc<AppState>) -> Router {
    {
        let state = state.clone();
        tokio::spawn(async move {
            worker(state).await;
        });
    }

    Router::new()
        .route("/", get(routes::index))
        .route("/health", get(routes::health_check))
        .route("/favicon.ico", get(routes::favicon32))
        .route("/generate", get(routes::generate))
        .route("/static/{filename}", get(routes::static_file))
        .route("/view/{platform}/{user}/{repo}", get(redirect_old_overview))
        .nest(
            "/{platform}/{user}/{repo}",
            Router::new()
                .route("/", get(hoc::calculate_hoc))
                .route("/json", get(hoc::json_hoc))
                .route("/view", get(hoc::overview))
                .route("/delete", post(hoc::delete_repo_and_cache)),
        )
        .fallback(routes::p404)
        .layer(
            TraceLayer::new_for_http()
                // add request-id to trace span
                .make_span_with(|request: &Request<Body>| {
                    let default_span = DefaultMakeSpan::default().make_span(request);
                    let requestid = if let Some(req_id) = request
                        .extensions()
                        .get::<RequestId>()
                        .map(RequestId::header_value)
                    {
                        req_id.to_str().unwrap_or("")
                    } else {
                        error!("cannot extract request-id");
                        ""
                    }
                    .to_string();
                    tracing::info_span!(parent: &default_span, env!("CARGO_CRATE_NAME"), %requestid)
                }),
        )
        // PropagateRequestIdLayer must be before SetRequestIdLayer
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(CompressionLayer::new().gzip(true).deflate(true))
        .with_state(state)
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        if matches!(self, Self::BranchNotFound) || matches!(self, Self::UnknownPlatform(_)) {
            (
                StatusCode::NOT_FOUND,
                render!(templates::p404_no_master_html, VERSION_INFO, 0),
            )
                .into_response()
        } else {
            error!(err=%self, "error");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                render!(templates::p500_html, VERSION_INFO, 0),
            )
                .into_response()
        }
    }
}
