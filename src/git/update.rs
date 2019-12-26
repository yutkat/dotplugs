use crate::git::GitStatus;
use crate::git::UpdateStatus;
use crate::repository::Repositories;
use crate::repository::Repository;
use failure::Error;
use std::process::Child;

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
    let mut children = vec![];
    for status in statuses {
        if status.status == UpdateStatus::Required {
            children.push(update_repository_by_command(&status.dir)?);
            eprintln!("Update: {}", &status.uri);
        }
    }

    for mut c in children {
        c.wait()?;
    }
    Ok(())
}

fn update_repository_by_command<S: Into<String>>(dir: S) -> Result<Child, Error> {
    let child = std::process::Command::new("git")
        .args(&["pull", "--no-stat", "--recurse-submodules"])
        .current_dir(dir.into())
        .stdout(std::process::Stdio::null())
        .spawn()?;
    Ok(child)
}
