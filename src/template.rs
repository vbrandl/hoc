pub struct RepoInfo<'a> {
    pub commit_url: &'a str,
    pub commits: u64,
    pub domain: &'a str,
    pub head: &'a str,
    pub hoc: u64,
    pub hoc_pretty: &'a str,
    pub path: &'a str,
    pub url: &'a str,
}
