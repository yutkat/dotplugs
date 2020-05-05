use crate::repository::git_directory::GitDirectory;
use crate::repository::CanReposit;
use crate::repository::Repositories;
use anyhow::{anyhow, Result};
use std::process::Command;

pub struct Zinit;

impl CanReposit for Zinit {
    fn get_repositories() -> Result<Repositories> {
        let path = Zinit::get_plugin_root_dir()?;
        GitDirectory::get_repositories(path)
    }
}

impl Zinit {
    fn get_plugin_root_dir() -> Result<String> {
        let cmd = format!(
            r##"source ~/.zshrc && zinit zstatus | grep 'Plugin directory' | cut -d' ' -f3 | tr -d '\n' | sed 's/\x1b\[[0-9;]*m//g'"##
        );
        log::debug!("zinit cmd: {}", cmd);
        let output = Command::new("zsh").arg("-c").arg(cmd).output()?;
        log::debug!("process exited with: {}", output.status);
        let stdout = output.stdout;
        let dir = String::from_utf8(stdout)?;
        log::debug!("zinit dir: {}", &dir);
        if !std::path::Path::new(&dir).exists() {
            return Err(anyhow!("zinit dir not found {}", &dir));
        }
        Ok(dir)
    }
}
