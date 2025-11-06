use crate::{
    State,
    error::Result,
    service::FormValue,
    statics::VERSION_INFO,
    template::RepoGeneratorInfo,
    templates::{self, statics::StaticFile},
};

use std::{
    borrow::Cow,
    sync::atomic::{AtomicUsize, Ordering},
    time::{Duration, SystemTime},
};

use actix_web::{HttpResponse, get, http::header::Expires, post, web};
use serde::{Deserialize, Serialize};

#[get("/")]
#[allow(clippy::unused_async)]
pub(crate) async fn index(
    state: web::Data<State>,
    repo_count: web::Data<AtomicUsize>,
) -> Result<HttpResponse> {
    let mut buf = Vec::new();
    templates::index_html(
        &mut buf,
        VERSION_INFO,
        repo_count.load(Ordering::Relaxed),
        &state.settings.base_url,
    )?;
    Ok(HttpResponse::Ok().content_type("text/html").body(buf))
}

#[derive(Deserialize, Serialize)]
struct GeneratorForm<'a> {
    service: FormValue,
    user: Cow<'a, str>,
    repo: Cow<'a, str>,
    branch: Option<Cow<'a, str>>,
}

#[post("/generate")]
#[allow(clippy::unused_async)]
async fn generate(
    params: web::Form<GeneratorForm<'_>>,
    state: web::Data<State>,
    repo_count: web::Data<AtomicUsize>,
) -> Result<HttpResponse> {
    let mut buf = Vec::new();
    let repo_info = RepoGeneratorInfo {
        service: params.service,
        user: &params.user,
        repo: &params.repo,
        branch: params
            .branch
            .as_deref()
            .filter(|s| !s.is_empty())
            .unwrap_or("master"),
    };
    templates::generate_html(
        &mut buf,
        VERSION_INFO,
        repo_count.load(Ordering::Relaxed),
        &state.settings.base_url,
        &repo_info,
    )?;

    Ok(HttpResponse::Ok().content_type("text/html").body(buf))
}

#[get("/static/{filename}")]
#[allow(clippy::unused_async)]
async fn static_file(
    path: web::Path<String>,
    repo_count: web::Data<AtomicUsize>,
) -> Result<HttpResponse> {
    /// A duration to add to current time for a far expires header.
    static FAR: Duration = Duration::from_secs(180 * 24 * 60 * 60);

    StaticFile::get(&path)
        .map(|data| {
            let far_expires = SystemTime::now() + FAR;
            HttpResponse::Ok()
                .insert_header(Expires(far_expires.into()))
                .content_type(data.mime.clone())
                .body(data.content)
        })
        .map_or_else(|| p404(&repo_count), Result::Ok)
}

#[get("/favicon.ico")]
#[allow(clippy::unused_async)]
async fn favicon32() -> HttpResponse {
    let data = &crate::templates::statics::favicon32_png;
    HttpResponse::Ok()
        .content_type(data.mime.clone())
        .body(data.content)
}

#[get("/health_check")]
#[allow(clippy::unused_async)]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub(crate) fn p404(repo_count: &AtomicUsize) -> Result<HttpResponse> {
    let mut buf = Vec::new();
    templates::p404_html(&mut buf, VERSION_INFO, repo_count.load(Ordering::Relaxed))?;
    Ok(HttpResponse::NotFound().content_type("text/html").body(buf))
}
