use anyhow::{anyhow, Result};

pub fn get_current_branch(repo: &git2::Repository) -> Result<String> {
    let mut branches = repo.branches(None)?;
    let branch = branches
        .find(|b| b.as_ref().unwrap().0.is_head())
        .ok_or(anyhow!("There is no branch corresponding to HEAD"))??
        .0;
    Ok(branch.name()?.unwrap_or("master").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Repository;

    fn init() {
        let _ = pretty_env_logger::formatted_builder()
            .is_test(true)
            .parse_filters("DEBUG")
            .try_init();
    }

    #[test]
    fn get_current_branch_ok() -> Result<()> {
        use boolinator::Boolinator;
        use rand::Rng;
        init();
        let suffix = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(7)
            .collect::<String>();
        let target_git_dir = format!("/tmp/hellogitworld_{}", suffix);

        let repo_url = "https://github.com/githubtraining/hellogitworld";
        std::process::Command::new("git")
            .args(&["clone", repo_url, &target_git_dir])
            .current_dir("/tmp")
            .output()?
            .status
            .success()
            .as_result(true, anyhow!("git command error"))?;
        std::process::Command::new("git")
            .args(&["checkout", "bisect"])
            .current_dir(&target_git_dir)
            .output()?
            .status
            .success()
            .as_result(true, anyhow!("git command error"))?;
        let repo = Repository {
            uri: repo_url.to_string(),
            dir: target_git_dir,
        };
        let git_repo = git2::Repository::open(&repo.dir)?;
        let branch = get_current_branch(&git_repo)?;
        assert_eq!(branch, "bisect");
        Ok(())
    }
}
