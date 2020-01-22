mod repo_info;

#[derive(Debug)]
struct GitHubInfo {
    pub uri: String,
    pub dir: String,
    pub stargazers: i64,
}
