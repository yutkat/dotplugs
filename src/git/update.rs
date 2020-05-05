use crate::git::GitStatus;
use crate::git::UpdateStatus;
use anyhow::Result;
use std::process::Child;

pub fn update_repositories(statuses: &Vec<GitStatus>) -> Result<()> {
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

fn update_repository_by_command<S: Into<String>>(dir: S) -> Result<Child> {
    let child = std::process::Command::new("git")
        .args(&["pull", "--no-stat", "--recurse-submodules"])
        .current_dir(dir.into())
        .stdout(std::process::Stdio::null())
        .spawn()?;
    Ok(child)
}
