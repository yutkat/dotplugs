mod branch;
mod fetch;
mod status;
mod update;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum UpdateStatus {
    Required,
    Already,
    NotGitRepository,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitStatus {
    pub uri: String,
    pub dir: String,
    pub branch: String,
    pub status: UpdateStatus,
}

pub use status::get_status_async as get_status;
pub use update::update_repositorie_using_cached_statuss as update_using_cached_status;
pub use update::update_repositories as update;
