use crate::repository::util::git_directory::GitDirectory;
use crate::repository::CanReposit;
use crate::repository::Repositories;
use failure::Error;
use std::process::Command;

pub struct Tpm;

impl CanReposit for Tpm {
    fn get_repositories() -> Result<Repositories, Error> {
        if !Tpm::is_running_tmux()? {
            return Ok(vec![]);
        }
        let path = match std::env::var("TMUX_PLUGIN_MANAGER_PATH") {
            Ok(n) => n,
            Err(_) => return Ok(vec![]),
        };
        GitDirectory::get_repositories(path)
    }
}

impl Tpm {
    fn is_running_tmux() -> Result<bool, Error> {
        let cmd = format!(r##"tmux info"##);
        log::debug!("exists check: {}", cmd);
        let status = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .stdout(std::process::Stdio::null())
            .status()?;
        log::debug!("process exited with: {}", status);
        Ok(status.success())
    }
}
