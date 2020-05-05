use crate::git::branch;
use crate::git::fetch;
use crate::git::GitStatus;
use crate::git::UpdateStatus;
use crate::repository::Repositories;
use crate::repository::Repository;
use anyhow::{anyhow, Result};
use futures::executor;
use futures::task::SpawnExt;
use log::{debug, warn};
use std::io::Write;
use std::sync::{Arc, Mutex};
use termion::clear;

#[allow(dead_code)]
pub fn get_status_sync(repos: &Repositories) -> Result<Vec<GitStatus>> {
    let mut git_statuses = Vec::<GitStatus>::new();

    for repo in repos.clone() {
        eprint!("\r{}Checking: {}", clear::CurrentLine, repo.uri);
        std::io::stdout().flush().unwrap();
        match get_status_after_fetch(&repo) {
            Ok(sts) => git_statuses.push(sts),
            Err(e) => warn!("\n{:?}: {:?}", &repo, e),
        }
    }
    eprint!("\r{}", clear::CurrentLine);
    Ok(git_statuses)
}

pub fn get_status_async(repos: &Repositories) -> Result<Vec<GitStatus>> {
    let git_statuses = Arc::new(Mutex::new(Vec::<GitStatus>::new()));
    let pool = executor::ThreadPool::new()?;
    let mut futures = vec![];
    for repo in repos.clone() {
        let git_statuses = Arc::clone(&git_statuses);
        let future = async move {
            eprint!("\r{}Checking: {}", clear::CurrentLine, repo.uri);
            std::io::stdout().flush().unwrap();
            match get_status_after_fetch(&repo) {
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
    let g = Arc::try_unwrap(git_statuses).map_err(|e| anyhow!("Async Error {:?}", e))?;
    eprint!("\r{}", clear::CurrentLine);
    Ok(g.into_inner()?)
}

fn get_status_after_fetch(repo: &Repository) -> Result<GitStatus> {
    let git_repo = match git2::Repository::open(&repo.dir) {
        Ok(x) => x,
        Err(_) => {
            return Ok(GitStatus {
                uri: repo.uri.to_string(),
                dir: repo.dir.to_string(),
                branch: "master".to_string(),
                status: UpdateStatus::NotGitRepository,
            })
        }
    };
    fetch::fetch_repository(repo)?;
    Ok(GitStatus {
        uri: repo.uri.to_string(),
        dir: repo.dir.to_string(),
        branch: branch::get_current_branch(&git_repo)?,
        status: get_update_status(&git_repo)?,
    })
}

fn get_update_status(repo: &git2::Repository) -> Result<UpdateStatus> {
    let branch_name = branch::get_current_branch(&repo)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Repository;
    extern crate pretty_env_logger;
    use anyhow::{anyhow, Result};

    fn init() {
        let _ = pretty_env_logger::formatted_builder()
            .is_test(true)
            .parse_filters("DEBUG")
            .try_init();
    }

    #[test]
    fn get_update_status_of_needed() -> Result<()> {
        use boolinator::Boolinator;
        use rand::Rng;
        init();
        let suffix = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(7)
            .collect::<String>();
        let target_git_dir = format!("/tmp/Spoon-Knife_{}", suffix);

        let repo_url = "https://github.com/octocat/Spoon-Knife";
        std::process::Command::new("git")
            .args(&["clone", "--depth=3", repo_url, &target_git_dir])
            .current_dir("/tmp")
            .output()?
            .status
            .success()
            .as_result(true, anyhow!("git command error"))?;
        std::process::Command::new("git")
            .args(&["reset", "--hard", "HEAD^^"])
            .current_dir(&target_git_dir)
            .output()?
            .status
            .success()
            .as_result(true, anyhow!("git command error"))?;
        let repo = Repository {
            uri: "https://github.com/octocat/Spoon-Knife".to_string(),
            dir: target_git_dir.to_string(),
        };
        let git_repo = git2::Repository::open(&repo.dir).unwrap();
        let n = get_update_status(&git_repo)?;
        assert_eq!(n, UpdateStatus::Required);
        std::fs::remove_dir_all(&target_git_dir).unwrap_or(());
        Ok(())
    }

    #[test]
    fn get_update_status_of_updated() -> Result<()> {
        use rand::Rng;
        init();
        let suffix = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(7)
            .collect::<String>();
        let target_git_dir = format!("/tmp/Spoon-Knife_{}", suffix);
        let repo = Repository {
            uri: "https://github.com/octocat/Spoon-Knife".to_string(),
            dir: target_git_dir.to_string(),
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
        std::fs::remove_dir_all(&target_git_dir).unwrap_or(());
        Ok(())
    }
}
