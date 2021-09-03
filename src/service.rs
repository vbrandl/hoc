pub(crate) trait Service {
    fn domain() -> &'static str;
    fn url_path() -> &'static str;
    fn commit_url(repo: &str, commit_ref: &str) -> String;
}

#[derive(Deserialize, Serialize)]
pub(crate) enum FormService {
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
