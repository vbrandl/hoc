use crate::{
    http::AppState,
    platform::Platform,
    statics::VERSION_INFO,
    template::RepoGeneratorInfo,
    templates::{self, statics::StaticFile},
};

use std::{
    borrow::Cow,
    sync::{Arc, atomic::Ordering},
};

use axum::{
    Form,
    extract::{Path, State},
    http::{StatusCode, header},
    response::IntoResponse,
};
use jiff::{SignedDuration, Timestamp, fmt::rfc2822};
use serde::{Deserialize, Serialize};
use tracing::error;

pub(crate) async fn index(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    render!(
        templates::index_html,
        VERSION_INFO,
        state.repo_count.load(Ordering::Relaxed),
        &state.settings.base_url,
    )
}

#[derive(Deserialize, Serialize)]
pub(crate) struct GeneratorForm<'a> {
    service: Platform,
    user: Cow<'a, str>,
    repo: Cow<'a, str>,
    branch: Option<Cow<'a, str>>,
}

pub(crate) async fn generate(
    State(state): State<Arc<AppState>>,
    Form(params): Form<GeneratorForm<'_>>,
) -> impl IntoResponse {
    render!(
        templates::generate_html,
        VERSION_INFO,
        state.repo_count.load(Ordering::Relaxed),
        &state.settings.base_url,
        &RepoGeneratorInfo {
            platform: params.service,
            user: &params.user,
            repo: &params.repo,
            branch: params
                .branch
                .as_deref()
                .filter(|s| !s.is_empty())
                .unwrap_or("master"),
        }
    )
}

pub(crate) async fn static_file(
    Path(path): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    /// A duration to add to current time for a far expires header.
    const FAR: SignedDuration = SignedDuration::from_hours(180 * 24);

    if let Some(data) = StaticFile::get(&path) {
        let far_expires = Timestamp::now() + FAR;
        let formatter = rfc2822::DateTimePrinter::new();
        let expires = formatter.timestamp_to_string(&far_expires);
        if let Err(err) = &expires {
            error!(%err, "formatter error");
        }
        (
            StatusCode::OK,
            [
                (
                    header::EXPIRES,
                    // TODO: error handling
                    expires.unwrap(),
                ),
                (header::CONTENT_TYPE, data.mime.to_string()),
            ],
            data.content,
        )
            .into_response()
    } else {
        p404(State(state)).await.into_response()
    }
}

pub(crate) async fn favicon32() -> impl IntoResponse {
    let data = &crate::templates::statics::favicon32_png;
    (
        [(header::CONTENT_TYPE, data.mime.to_string())],
        data.content,
    )
}

pub(crate) async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub(crate) async fn p404(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    render!(
        templates::p404_html,
        VERSION_INFO,
        state.repo_count.load(Ordering::Relaxed)
    )
}
