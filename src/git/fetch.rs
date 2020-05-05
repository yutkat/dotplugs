use crate::git::branch;
use crate::repository::Repository;
use anyhow::Result;
use log::warn;

pub fn fetch_repository(repo: &Repository) -> Result<()> {
    // libgit2 does not implement shallow fetch
    // https://github.com/libgit2/libgit2/issues/3058
    fetch_repository_by_command(repo)?;
    Ok(())
}

#[allow(dead_code)]
fn fetch_repository_by_git2rs(repo: &Repository) -> Result<()> {
    let git_repo = git2::Repository::open(&repo.dir)?;
    let branch = branch::get_current_branch(&git_repo)?;
    git_repo.find_remote("origin")?.fetch(
        &[&branch],
        Some(&mut git2::FetchOptions::new()),
        None,
    )?;
    Ok(())
}

fn fetch_repository_by_command(repo: &Repository) -> Result<()> {
    let output = std::process::Command::new("git")
        .args(&["fetch", "-a"])
        .current_dir(&repo.dir)
        .output()?;
    if !output.status.success() {
        warn!("git fetch is failure [Exit code: {}]", output.status);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Repository;
    use anyhow::{anyhow, Result};

    fn init() {
        let _ = pretty_env_logger::formatted_builder()
            .is_test(true)
            .parse_filters("DEBUG")
            .try_init();
    }

    #[test]
    fn fetch_repository_status_for_shallow_ok() -> Result<()> {
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
        std::process::Command::new("git")
            .args(&[
                "update-ref",
                "refs/remotes/origin/master",
                "refs/remotes/origin/master~2",
            ])
            .current_dir(&target_git_dir)
            .output()?
            .status
            .success()
            .as_result(true, anyhow!("git command error"))?;
        std::process::Command::new("git")
            .args(&["gc", "--prune=now"])
            .current_dir(&target_git_dir)
            .output()?
            .status
            .success()
            .as_result(true, anyhow!("git command error"))?;
        let sha1_before = String::from_utf8(
            std::process::Command::new("git")
                .args(&["rev-parse", "origin/HEAD"])
                .current_dir(&target_git_dir)
                .output()?
                .stdout,
        )?;
        let repo = Repository {
            uri: "https://github.com/octocat/Spoon-Knife".to_string(),
            dir: target_git_dir.to_string(),
        };
        fetch_repository(&repo)?;
        let sha1_after = String::from_utf8(
            std::process::Command::new("git")
                .args(&["rev-parse", "origin/HEAD"])
                .current_dir(&target_git_dir)
                .output()?
                .stdout,
        )?;
        assert_ne!(sha1_before, sha1_after);
        std::fs::remove_dir_all(&target_git_dir).unwrap_or(());
        Ok(())
    }
}
