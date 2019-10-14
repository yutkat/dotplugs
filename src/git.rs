use crate::repository::Repositories;
use failure::Error;
use log::debug;
use log::warn;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum UpdateStatus {
    Required,
    Already,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitStatus {
    pub uri: String,
    pub dir: String,
    pub status: UpdateStatus,
}

pub fn get_status(repos: &Repositories) -> Result<Vec<GitStatus>, Error> {
    let mut git_statuses = Vec::<GitStatus>::new();
    for repo in repos {
        let git_repo = match git2::Repository::open(&repo.dir) {
            Ok(n) => n,
            Err(e) => {
                warn!("Not found repository: {}. {}", &repo.dir, e);
                continue;
            }
        };
        git_statuses.push(GitStatus {
            uri: repo.uri.to_string(),
            dir: repo.dir.to_string(),
            status: get_update_status(&git_repo)?,
        })
    }
    Ok(git_statuses)
}

fn get_update_status(repo: &git2::Repository) -> Result<UpdateStatus, Error> {
    debug!(
        "local_hash: {:?} remote_hash: {:?}",
        repo.revparse_single("HEAD")?.id(),
        repo.revparse_single("origin/HEAD")?.id()
    );
    let local_hash = repo.revparse_single("HEAD")?.id();
    let remote_hash = repo.revparse_single("origin/HEAD")?.id();
    if local_hash == remote_hash {
        return Ok(UpdateStatus::Already);
    }
    Ok(UpdateStatus::Required)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Repositories;
    use crate::repository::Repository;
    extern crate env_logger;

    fn init() {
        std::env::set_var("RUST_LOG", "debug");
        let _ = env_logger::builder().is_test(true).try_init();
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
        assert_eq!(n.len(), repos.len());
        Ok(())
    }

    #[test]
    fn get_update_status_of_needed() -> Result<(), Error> {
        init();
        let repo = Repository {
            uri: "https://github.com/octocat/Spoon-Knife".to_string(),
            dir: "tests/data/Spoon-Knife".to_string(),
        };
        let git_repo = git2::Repository::open(&repo.dir).unwrap();
        let n = get_update_status(&git_repo)?;
        assert_eq!(n, UpdateStatus::Required);
        Ok(())
    }

    #[test]
    fn get_update_status_of_updated() -> Result<(), Error> {
        init();
        std::fs::remove_dir_all("/tmp/Spoon-Knife").unwrap_or(());
        let repo = Repository {
            uri: "https://github.com/octocat/Spoon-Knife".to_string(),
            dir: "/tmp/Spoon-Knife".to_string(),
        };
        let git_repo = git2::Repository::clone(&repo.uri, &repo.dir).unwrap();
        git_repo
            .reference_symbolic(
                "refs/remotes/origin/HEAD",
                "refs/remotes/origin/master",
                true,
                "",
            )
            .unwrap();
        let n = get_update_status(&git_repo)?;
        assert_eq!(n, UpdateStatus::Already);
        Ok(())
    }
}
