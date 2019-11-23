use crate::git::branch;
use crate::repository::Repository;
use failure::Error;
use log::warn;

pub fn fetch_repository(repo: &Repository) -> Result<(), Error> {
    // libgit2 does not implement shallow fetch
    // https://github.com/libgit2/libgit2/issues/3058
    fetch_repository_by_command(repo)?;

    Ok(())
}

#[allow(dead_code)]
fn fetch_repository_by_git2rs(repo: &Repository) -> Result<(), Error> {
    let git_repo = git2::Repository::open(&repo.dir)?;
    let branch = branch::get_current_branch(&git_repo)?;
    git_repo.find_remote("origin")?.fetch(
        &[&branch],
        Some(&mut git2::FetchOptions::new()),
        None,
    )?;
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Repository;
    extern crate pretty_env_logger;

    fn init() {
        let _ = pretty_env_logger::formatted_builder()
            .is_test(true)
            .parse_filters("DEBUG")
            .try_init();
    }

    #[test]
    fn get_repository_status_ok() -> Result<(), Error> {
        init();
        let repo = Repository {
            uri: "https://github.com/fatih/vim-go.git".to_string(),
            dir: "/home/osft/dotfiles/.vim/plugged/vim-go".to_string(),
        };
        fetch_repository(&repo)?;
        Ok(())
    }
}
