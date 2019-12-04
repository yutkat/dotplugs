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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Repositories;
    use crate::repository::Repository;
    use failure::Error;
    extern crate pretty_env_logger;

    fn init() {
        let _ = pretty_env_logger::formatted_builder()
            .is_test(true)
            .parse_filters("DEBUG")
            .try_init();
    }

    #[test]
    fn get_status_not_found() -> Result<(), Error> {
        init();
        let repo = Repository {
            uri: "aaa".to_string(),
            dir: "/tmp/aaa".to_string(),
        };
        let repos: Repositories = vec![repo];
        println!("{:?}", repos);
        let n = get_status(&repos)?;
        assert_eq!(n[0].status, UpdateStatus::NotGitRepository);
        Ok(())
    }

    #[test]
    fn get_status_normal() -> Result<(), Error> {
        init();
        let repo = Repository {
            uri: "https://github.com/octocat/Spoon-Knife".to_string(),
            dir: "tests/data/Spoon-Knife".to_string(),
        };
        let repos: Repositories = vec![repo];
        let n = get_status(&repos)?;
        assert_ne!(n.len(), repos.len());
        Ok(())
    }
}
