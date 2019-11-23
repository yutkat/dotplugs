use crate::repository::Repositories;
use crate::repository::Repository;
use failure::format_err;
use failure::Error;
use futures::executor;
use futures::task::SpawnExt;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::sync::{Arc, Mutex};
use termion::clear;

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
    pub status: UpdateStatus,
}

pub fn get_status(repos: &Repositories) -> Result<Vec<GitStatus>, Error> {
    get_status_async(repos)
}

#[allow(dead_code)]
fn get_status_sync(repos: &Repositories) -> Result<Vec<GitStatus>, Error> {
    let mut git_statuses = Vec::<GitStatus>::new();

    for repo in repos.clone() {
        eprint!("\r{}Checking: {}", clear::CurrentLine, repo.uri);
        std::io::stdout().flush().unwrap();
        match fetch_repository(&repo) {
            Ok(sts) => git_statuses.push(sts),
            Err(e) => warn!("\n{:?}: {:?}", &repo, e),
        }
    }
    eprint!("\r{}", clear::CurrentLine);
    Ok(git_statuses)
}

fn get_status_async(repos: &Repositories) -> Result<Vec<GitStatus>, Error> {
    let git_statuses = Arc::new(Mutex::new(Vec::<GitStatus>::new()));
    let pool = executor::ThreadPool::new()?;
    let mut futures = vec![];
    for repo in repos.clone() {
        let git_statuses = Arc::clone(&git_statuses);
        let future = async move {
            eprint!("\r{}Checking: {}", clear::CurrentLine, repo.uri);
            std::io::stdout().flush().unwrap();
            match fetch_repository(&repo) {
                Ok(sts) => git_statuses.lock().unwrap().push(sts),
                Err(e) => {
                    warn!("\r{}", clear::CurrentLine);
                    warn!("{:?}: {:?}", &repo, e);
                }
            }
        };
        futures.push(pool.spawn_with_handle(future)?);
    }
    executor::block_on(futures::future::join_all(futures));
    let g = Arc::try_unwrap(git_statuses).map_err(|e| format_err!("Async Error {:?}", e))?;
    eprint!("\r{}", clear::CurrentLine);
    Ok(g.into_inner()?)
}

fn fetch_repository(repo: &Repository) -> Result<GitStatus, Error> {
    let git_repo = match git2::Repository::open(&repo.dir) {
        Ok(x) => x,
        Err(_) => {
            return Ok(GitStatus {
                uri: repo.uri.to_string(),
                dir: repo.dir.to_string(),
                status: UpdateStatus::NotGitRepository,
            })
        }
    };
    // libgit2 does not implement shallow fetch
    // https://github.com/libgit2/libgit2/issues/3058
    fetch_repository_by_command(repo)?;

    // let branch = get_current_branch(&git_repo)?;
    // git_repo.find_remote("origin")?.fetch(
    //     &[&branch],
    //     Some(&mut git2::FetchOptions::new()),
    //     None,
    // )?;

    Ok(GitStatus {
        uri: repo.uri.to_string(),
        dir: repo.dir.to_string(),
        status: get_update_status(&git_repo)?,
    })
}

fn fetch_repository_by_command(repo: &Repository) -> Result<(), Error> {
    let output = std::process::Command::new("git")
        .args(&["fetch", "-a"])
        .current_dir(&repo.dir)
        .output()?;
    if !output.status.success() {
        warn!("git fetch is failure [Exit code: {}]", output.status);
    }
    Ok(())
}

fn get_update_status(repo: &git2::Repository) -> Result<UpdateStatus, Error> {
    let branch_name = get_current_branch(&repo)?;
    let remote_branch_name = format!("origin/{}", &branch_name);

    let local_hash = repo.revparse_single("HEAD")?.id();
    let remote_hash = repo.revparse_single(&remote_branch_name)?.id();
    // let remote_hash = repo.revparse_single("FETCH_HEAD")?.id();
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

    #[test]
    fn get_repository_status_ok() -> Result<(), Error> {
        let repo = Repository {
            uri: "https://github.com/fatih/vim-go.git".to_string(),
            dir: "/home/osft/dotfiles/.vim/plugged/vim-go".to_string(),
        };
        fetch_repository(&repo)?;
        Ok(())
    }

    #[test]
    fn fetch_test() -> Result<(), Error> {
        init();
        let repo = Repository {
            uri: "https://github.com/Shougo/defx.nvim.git".to_string(),
            dir: "/home/osft/dotfiles/.vim/plugged/defx.nvim".to_string(),
        };
        let git_repo = git2::Repository::open(&repo.dir)?;
        let branch = get_current_branch(&git_repo)?;
        git_repo
            .find_remote("origin")?
            .fetch(&[&branch], None, None)?;
        Ok(())
    }
}
