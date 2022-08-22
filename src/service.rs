use crate::{calculate_hoc, delete_repo_and_cache, json_hoc, overview};

use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    web, App,
};

pub(crate) trait Service: Sized + 'static {
    fn domain() -> &'static str;
    fn url_path() -> &'static str;
    fn commit_url(repo: &str, commit_ref: &str) -> String;

    fn register_service<T>(app: App<T>) -> App<T>
    where
        T: ServiceFactory<ServiceRequest, Config = (), Error = actix_web::Error, InitError = ()>,
    {
        let url_path = Self::url_path();
        app.service(
            web::resource(format!("/{url_path}/{{user}}/{{repo}}")).to(calculate_hoc::<Self>),
        )
        .service(
            web::resource(format!("/{url_path}/{{user}}/{{repo}}/delete"))
                .route(web::post().to(delete_repo_and_cache::<Self>)),
        )
        .service(web::resource(format!("/{url_path}/{{user}}/{{repo}}/json")).to(json_hoc::<Self>))
        .service(web::resource(format!("/view/{url_path}/{{user}}/{{repo}}")).to(overview::<Self>))
        .service(web::resource(format!("/{url_path}/{{user}}/{{repo}}/view")).to(overview::<Self>))
    }
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum FormService {
    #[serde(rename = "github")]
    GitHub,
    #[serde(rename = "gitlab")]
    Gitlab,
    #[serde(rename = "bitbucket")]
    Bitbucket,
    #[serde(rename = "sourcehut")]
    Sourcehut,
}

impl FormService {
    pub(crate) fn url(&self) -> &str {
        match self {
            FormService::GitHub => "github.com",
            FormService::Gitlab => "gitlab.com",
            FormService::Bitbucket => "bitbucket.org",
            FormService::Sourcehut => "git.sr.ht",
        }
    }

    pub(crate) fn service(&self) -> &str {
        match self {
            FormService::GitHub => "github",
            FormService::Gitlab => "gitlab",
            FormService::Bitbucket => "bitbucket",
            FormService::Sourcehut => "sourcehut",
        }
    }
}

pub(crate) struct GitHub;

impl Service for GitHub {
    fn domain() -> &'static str {
        "github.com"
    }
    fn url_path() -> &'static str {
        "github"
    }
    fn commit_url(repo: &str, commit_ref: &str) -> String {
        format!("https://{}/{}/commit/{}", Self::domain(), repo, commit_ref)
    }
}

pub(crate) struct Gitlab;

impl Service for Gitlab {
    fn domain() -> &'static str {
        "gitlab.com"
    }
    fn url_path() -> &'static str {
        "gitlab"
    }
    fn commit_url(repo: &str, commit_ref: &str) -> String {
        format!("https://{}/{}/commit/{}", Self::domain(), repo, commit_ref)
    }
}

pub(crate) struct Bitbucket;

impl Service for Bitbucket {
    fn domain() -> &'static str {
        "bitbucket.org"
    }
    fn url_path() -> &'static str {
        "bitbucket"
    }
    fn commit_url(repo: &str, commit_ref: &str) -> String {
        format!("https://{}/{}/commits/{}", Self::domain(), repo, commit_ref)
    }
}

pub(crate) struct Sourcehut;

impl Service for Sourcehut {
    fn domain() -> &'static str {
        "git.sr.ht"
    }
    fn url_path() -> &'static str {
        "sourcehut"
    }
    fn commit_url(repo: &str, commit_ref: &str) -> String {
        format!("https://{}/{}/commit/{}", Self::domain(), repo, commit_ref)
    }
}
