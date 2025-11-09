use crate::error::Error;

use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub enum Platform {
    #[serde(rename = "github")]
    GitHub,
    #[serde(rename = "gitlab")]
    Gitlab,
    #[serde(rename = "bitbucket")]
    Bitbucket,
    #[serde(rename = "sourcehut")]
    Sourcehut,
}

impl Platform {
    pub(crate) fn domain(self) -> &'static str {
        match self {
            Self::GitHub => "github.com",
            Self::Gitlab => "gitlab.com",
            Self::Bitbucket => "bitbucket.org",
            Self::Sourcehut => "git.sr.ht",
        }
    }

    pub(crate) fn url_path(self) -> &'static str {
        match self {
            Self::GitHub => "github",
            Self::Gitlab => "gitlab",
            Self::Bitbucket => "bitbucket",
            Self::Sourcehut => "sourcehut",
        }
    }

    pub(crate) fn commit_url(self, repo: &str, commit_ref: &str) -> String {
        match self {
            Self::GitHub | Self::Gitlab | Self::Sourcehut => {
                format!("https://{}/{repo}/commit/{commit_ref}", self.domain())
            }
            Self::Bitbucket => {
                format!("https://{}/{repo}/commits/{commit_ref}", self.domain(),)
            }
        }
    }
}

impl FromStr for Platform {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "github" => Ok(Self::GitHub),
            "gitlab" => Ok(Self::Gitlab),
            "bitbucket" => Ok(Self::Bitbucket),
            "sourcehut" => Ok(Self::Sourcehut),
            _ => Err(Error::UnknownPlatform(s.to_string())),
        }
    }
}
