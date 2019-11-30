mod branch;
mod fetch;
mod status;
mod update;

use crate::repository::Repositories;
use failure::Error;
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

pub fn get_status(repos: &Repositories) -> Result<Vec<GitStatus>, Error> {
    status::get_status_async(repos)
}

pub fn update(repos: &Repositories) -> Result<(), Error> {
    update::update_repositories(repos)
}

pub fn update_using_cached_status(statuses: &Vec<GitStatus>) -> Result<(), Error> {
    update::update_repositorie_using_cached_statuss(statuses)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Repositories;
    use crate::repository::Repository;
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
        let n = get_status(&repos)?;
        assert_ne!(n.len(), repos.len());
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
