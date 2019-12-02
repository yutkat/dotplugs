use crate::git::GitStatus;
use crate::git::UpdateStatus;
use crate::repository::Repositories;
use crate::repository::Repository;
use failure::Error;

pub fn update_repository(repo: &Repository) -> Result<(), Error> {
    // libgit2 does not implement shallow fetch
    update_repository_by_command(&repo.dir)?;

    Ok(())
}

pub fn update_repositories(repos: &Repositories) -> Result<(), Error> {
    for repo in repos {
        update_repository(repo)?;
    }
    Ok(())
}

pub fn update_repositorie_using_cached_statuss(statuses: &Vec<GitStatus>) -> Result<(), Error> {
    for status in statuses {
        if status.status == UpdateStatus::Required {
            update_repository_by_command(&status.dir)?;
        }
    }
    Ok(())
}

fn update_repository_by_command<S: Into<String>>(dir: S) -> Result<(), Error> {
    std::process::Command::new("git")
        .args(&["pull", "--no-stat"])
        .current_dir(dir.into())
        .spawn()?;
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
    fn update_repository_ok() -> Result<(), Error> {
        init();
        let repo = Repository {
            uri: "https://github.com/fatih/vim-go.git".to_string(),
            dir: "/home/osft/dotfiles/.vim/plugged/vim-go".to_string(),
        };
        update_repository(&repo)?;
        Ok(())
    }
}
