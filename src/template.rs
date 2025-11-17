use crate::platform::Platform;

#[derive(Clone, Copy)]
pub struct RepoInfo<'a> {
    pub commit_url: &'a str,
    pub commits: u64,
    pub base_url: &'a str,
    pub head: &'a str,
    pub hoc: u64,
    pub hoc_pretty: &'a str,
    pub path: &'a str,
    pub url: &'a str,
    pub branch: &'a str,
    pub query: &'a str,
}

pub struct RepoGeneratorInfo<'a> {
    pub platform: Platform,
    pub user: &'a str,
    pub repo: &'a str,
    pub branch: Option<&'a str>,
    pub exclude: Option<&'a str>,
}

impl RepoGeneratorInfo<'_> {
    pub fn query(&self) -> String {
        let branch = self.branch.map(|b| format!("branch={b}"));
        let exclude = self.exclude.map(|e| format!("exclude={e}"));

        let query = [branch, exclude]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join("&");

        if query.is_empty() {
            String::new()
        } else {
            format!("?{query}")
        }
    }
}
