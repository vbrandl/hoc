use crate::{
    cache::{Cache, CacheEntry, Excludes, HocParams, ToQuery},
    error::Result,
    http::{self, AppState},
    platform::Platform,
    statics::VERSION_INFO,
    template::RepoInfo,
    templates,
};

use std::{
    io,
    sync::{Arc, atomic::Ordering},
};

use axum::{
    Json,
    extract::{Path as ReqPath, Query, State},
    http::{
        StatusCode,
        header::{self, HeaderMap, HeaderValue},
    },
    response::{IntoResponse, Redirect},
};
use badgers::{Badge, BadgeOptions};
use jiff::{SignedDuration, Timestamp, fmt::rfc2822};
use number_prefix::NumberPrefix;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{error, info, instrument, trace};

#[derive(Serialize)]
struct JsonResponse<'a> {
    head: &'a str,
    branch: &'a str,
    count: u64,
    commits: u64,
}

#[derive(Deserialize, Debug)]
pub(crate) struct BadgeQuery {
    branch: Option<String>,
    #[serde(default = "String::new")]
    exclude: String,
    #[serde(default = "default_label")]
    label: String,
}

impl BadgeQuery {
    fn excludes(&self) -> Excludes {
        self.exclude
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    }
}

fn default_label() -> String {
    "Hits-of-Code".to_string()
}

enum HocResult {
    Hoc {
        hoc: u64,
        commits: u64,
        hoc_pretty: String,
        head: String,
        params: HocParams,
    },
    Loading,
    NotFound,
}

#[instrument(
    "deleting repository and cache",
    skip_all,
    fields(platform, owner, repo, branch)
)]
pub(crate) async fn delete_repo_and_cache(
    State(state): State<Arc<AppState>>,
    ReqPath((platform, owner, repo)): ReqPath<(Platform, String, String)>,
    Query(branch): Query<BadgeQuery>,
) -> Result<impl IntoResponse> {
    let repo = format!("{}/{owner}/{repo}", platform.domain());
    info!("Deleting cache and repository");
    let repo_dir = state.repos().join(&repo);
    std::fs::remove_dir_all(repo_dir).or_else(|e| {
        if e.kind() == io::ErrorKind::NotFound {
            Ok(())
        } else {
            Err(e)
        }
    })?;
    state.repo_count.fetch_sub(1, Ordering::Relaxed);

    state.cache.clear(platform, &owner, &repo)?;

    let branch_query = branch.branch.as_ref().map(|b| format!("branch={b}"));

    let excludes = branch.excludes();
    let exclude_query = if excludes.is_empty() {
        None
    } else {
        Some(excludes.to_query())
    };

    let label_query = Some(format!("label={}", branch.label));

    let query: Vec<_> = [branch_query, exclude_query, label_query]
        .into_iter()
        .flatten()
        .collect();
    let query = query.join("&");
    let query = if query.is_empty() {
        query
    } else {
        format!("?{query}")
    };
    Ok(Redirect::temporary(&format!(
        "{}/{}/{owner}/{repo}/view{query}",
        state.settings.base_url,
        platform.url_path()
    )))
}

#[instrument(name = "hoc calculation", skip_all, fields(platform = params.platform.domain(), params.owner, params.repo, params.branch))]
async fn handle_hoc_request(state: &AppState, params: &HocParams) -> Result<HocResult> {
    let queued = state.queue.push(params.clone());
    if queued {
        trace!("queued new calculation job");
    } else {
        trace!("job already in queue");
    }

    let cached = state.cache.load(params)?;
    Ok(
        if let Some(CacheEntry::Cached {
            head,
            count,
            commits,
        }) = cached
        {
            #[allow(clippy::cast_precision_loss)]
            let hoc_pretty = match NumberPrefix::decimal(count as f64) {
                NumberPrefix::Standalone(hoc) => hoc.to_string(),
                NumberPrefix::Prefixed(prefix, hoc) => format!("{hoc:.1}{prefix}"),
            };
            HocResult::Hoc {
                hoc: count,
                commits,
                hoc_pretty,
                head,
                params: params.clone(),
            }
        } else if matches!(cached, Some(CacheEntry::NotFound)) {
            HocResult::NotFound
        } else {
            HocResult::Loading
        },
    )
}

pub(crate) async fn json_hoc(
    State(state): State<Arc<AppState>>,
    ReqPath((platform, owner, repo)): ReqPath<(Platform, String, String)>,
    Query(query): Query<BadgeQuery>,
) -> Result<impl IntoResponse> {
    let branch = query.branch.as_deref().unwrap_or("master");
    let exclude = query.excludes();
    let params = HocParams::new(platform, owner, repo, branch.to_string(), exclude);
    let r = handle_hoc_request(&state, &params).await?;
    Ok(match r {
        HocResult::NotFound => http::routes::p404(State(state)).await.into_response(),
        HocResult::Hoc {
            hoc, head, commits, ..
        } => Json(JsonResponse {
            branch,
            head: &head,
            count: hoc,
            commits,
        })
        .into_response(),
        HocResult::Loading => Json(json!({
            "status": "loading",
        }))
        .into_response(),
    })
}

fn no_cache_headers(expires: &Timestamp) -> HeaderMap {
    const FORMATTER: rfc2822::DateTimePrinter = rfc2822::DateTimePrinter::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("image/svg+xml"),
    );
    let expires = FORMATTER.timestamp_to_rfc9110_string(expires);
    if let Err(err) = &expires {
        error!(%err, "formatting error");
    }
    // TODO: error handling
    let expires = expires.unwrap();

    let expires_value = expires.try_into();
    if let Err(err) = &expires_value {
        error!(%err, "header value error");
    }
    headers.insert(
        header::EXPIRES,
        expires_value
            // TODO: error handling
            .unwrap(),
    );
    headers.append(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));
    headers.append(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    headers.append(
        header::CACHE_CONTROL,
        HeaderValue::from_static("must-revalidate"),
    );
    headers.append(header::CACHE_CONTROL, HeaderValue::from_static("max-age=0"));
    headers
}

fn no_cache_response(body: Vec<u8>) -> impl IntoResponse {
    let expiration = Timestamp::now() + SignedDuration::from_secs(30);
    (StatusCode::OK, no_cache_headers(&expiration), body)
}

pub(crate) async fn calculate_hoc(
    State(state): State<Arc<AppState>>,
    ReqPath((platform, owner, repo)): ReqPath<(Platform, String, String)>,
    Query(query): Query<BadgeQuery>,
) -> Result<impl IntoResponse> {
    let label = query.label.clone();
    let branch = query.branch.as_deref().unwrap_or("master");
    let exclude = query.excludes();
    let params = HocParams::new(platform, owner, repo, branch.to_string(), exclude);
    if let Ok(r) = handle_hoc_request(&state, &params).await {
        Ok(match r {
            HocResult::NotFound => http::routes::p404(State(state)).await.into_response(),
            HocResult::Loading => {
                let badge_opt = BadgeOptions {
                    subject: label,
                    status: "loading".to_string(),
                    color: "#ffff00".to_string(),
                };

                let badge = Badge::new(badge_opt)?;
                // TODO: remove clone
                let body = badge.to_svg().as_bytes().to_vec();

                no_cache_response(body).into_response()
            }
            HocResult::Hoc { hoc_pretty, .. } => {
                let badge_opt = BadgeOptions {
                    subject: label,
                    color: "#007ec6".to_string(),
                    status: hoc_pretty,
                };
                let badge = Badge::new(badge_opt)?;
                // TODO: remove clone
                let body = badge.to_svg().as_bytes().to_vec();

                no_cache_response(body).into_response()
            }
        })
    } else {
        let error_badge = Badge::new(BadgeOptions {
            subject: query.label.clone(),
            color: "#ff0000".to_string(),
            status: "error".to_string(),
        })
        .unwrap();
        let body = error_badge.to_svg().as_bytes().to_vec();
        Ok(no_cache_response(body).into_response())
    }
}

pub(crate) async fn overview(
    State(state): State<Arc<AppState>>,
    ReqPath((platform, owner, repo)): ReqPath<(Platform, String, String)>,
    Query(query): Query<BadgeQuery>,
) -> Result<impl IntoResponse> {
    let branch = query.branch.as_deref().unwrap_or("master");
    let label = query.label.clone();
    let base_url = state.settings.base_url.clone();
    let exclude = query.excludes();
    let params = HocParams::new(platform, owner, repo, branch.to_string(), exclude);
    let r = handle_hoc_request(&state, &params).await?;
    match r {
        HocResult::NotFound => Ok(http::routes::p404(State(state)).await.into_response()),
        HocResult::Loading => {
            let repo_info = RepoInfo {
                commit_url: "",
                commits: 0,
                base_url: &base_url,
                head: "",
                hoc: 0,
                hoc_pretty: "",
                path: &params.service_path(),
                url: &params.url(),
                branch,
            };
            Ok(render!(
                templates::loading_html,
                VERSION_INFO,
                state.repo_count.load(Ordering::Relaxed),
                repo_info,
                &label,
                &query.exclude,
            )
            .into_response())
        }
        HocResult::Hoc {
            hoc,
            commits,
            hoc_pretty,
            head,
            params,
        } => {
            let repo_info = RepoInfo {
                commit_url: &platform.commit_url(&params.owner, &params.repo, &head),
                commits,
                base_url: &base_url,
                head: &head,
                hoc,
                hoc_pretty: &hoc_pretty,
                path: &params.service_path(),
                url: &params.url(),
                branch,
            };
            Ok(render!(
                templates::overview_html,
                VERSION_INFO,
                state.repo_count.load(Ordering::Relaxed),
                repo_info,
                &label,
                &query.exclude,
            )
            .into_response())
        }
    }
}
