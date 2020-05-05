use crate::repository::git_directory::GitDirectory;
use crate::repository::CanReposit;
use crate::repository::Repositories;
use anyhow::{anyhow, Result};

pub struct VimPlug;

impl CanReposit for VimPlug {
    fn get_repositories() -> Result<Repositories> {
        let path = VimPlug::get_plugin_root_dir()?;
        GitDirectory::get_repositories(path)
    }
}

impl VimPlug {
    fn get_plugin_root_dir() -> Result<String> {
        let mut dir = dirs::home_dir().ok_or(anyhow!("Home env not found"))?;
        dir.push(".vim/plugged");
        log::debug!("vim-plug dir: {:?}", &dir);
        if !&dir.exists() {
            return Err(anyhow!("vim-plug dir not found {:?}", &dir));
        }
        Ok(dir
            .into_os_string()
            .into_string()
            .map_err(|x| anyhow!("Home env not found {:?}", x))?)
    }
}
