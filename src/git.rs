use crate::repository::Repositories;
use crate::repository::Repository;
use failure::format_err;
use failure::Error;
use log::debug;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use futures::executor;
use futures::task::SpawnExt;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum UpdateStatus {
    Required,
    Already,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitStatus {
    pub uri: String,
    pub dir: String,
    pub status: UpdateStatus,
}

pub fn get_status(repos: &Repositories) -> Result<Vec<GitStatus>, Error> {
    let git_statuses = Arc::new(Mutex::new(Vec::<GitStatus>::new()));
    let pool = executor::ThreadPool::new()?;
    let mut handles = vec![];

    for repo in repos.clone() {
        let git_statuses = Arc::clone(&git_statuses);
        // println!("{:?}", &repo);
        let future = async move {
            let sts = get_repository_status(&repo).unwrap();
            // println!("{:?}", &sts);
            git_statuses.lock().unwrap().push(sts);
        };
        handles.push(pool.spawn_with_handle(future).unwrap());
    }
    executor::block_on(futures::future::join_all(handles));

    Ok(Arc::try_unwrap(git_statuses).unwrap().into_inner()?)
}

fn get_repository_status(repo: &Repository) -> Result<GitStatus, Error> {
    let git_repo = git2::Repository::open(&repo.dir)?;
    git_repo.find_remote("origin")?.fetch(&[""], None, None)?;
    Ok(GitStatus {
        uri: repo.uri.to_string(),
        dir: repo.dir.to_string(),
        status: get_update_status(&git_repo)?,
    })
}

fn get_update_status(repo: &git2::Repository) -> Result<UpdateStatus, Error> {
    let branch_name = get_current_branch(&repo)?;
    let remote_branch_name = format!("origin/{}", &branch_name);

    let local_hash = repo.revparse_single("HEAD")?.id();
    let remote_hash = repo.revparse_single(&remote_branch_name)?.id();
    debug!(
        "local_hash: {:?} remote_hash: {:?}",
        &local_hash, &remote_hash
    );
    if local_hash == remote_hash {
        return Ok(UpdateStatus::Already);
    }
    Ok(UpdateStatus::Required)
}

fn get_current_branch(repo: &git2::Repository) -> Result<String, Error> {
    let mut branches = repo.branches(None)?;
    let branch = branches
        .find(|b| b.as_ref().unwrap().0.is_head())
        .ok_or(format_err!("There is no branch corresponding to HEAD"))??
        .0;
    Ok(branch.name()?.unwrap_or("master").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Repositories;
    use crate::repository::Repository;
    extern crate pretty_env_logger;

    fn init() {
        std::env::set_var("RUST_LOG", "debug");
        let _ = pretty_env_logger::formatted_builder()
            .is_test(true)
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
