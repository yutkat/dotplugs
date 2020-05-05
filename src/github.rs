pub mod repo_info;

use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct GitHubInfo {
    pub name_with_owner: String,
    pub stargazers: i64,
    pub updated_at: NaiveDateTime,
}
