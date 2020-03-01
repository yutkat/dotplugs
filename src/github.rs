pub mod repo_info;

#[derive(Debug)]
pub struct GitHubInfo {
    pub name_with_owner: String,
    pub stargazers: i64,
}
