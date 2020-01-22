use crate::repository::git_directory::GitDirectory;
use crate::repository::CanReposit;
use crate::repository::Repositories;
use failure::format_err;
use failure::Error;

pub struct VimPlug;


impl CanReposit for VimPlug {
    fn get_repositories() -> Result<Repositories, Error> {
        let path = VimPlug::get_plugin_root_dir()?;
        GitDirectory::get_repositories(path)
    }
}

impl VimPlug {
    fn get_plugin_root_dir() -> Result<String, Error> {
        let dir = "~/.vim/plugged".to_string();
        log::debug!("vim-plug dir: {}", &dir);
        if !std::path::Path::new(&dir).exists() {
            return Err(format_err!("vim-plug dir not found {}", &dir));
        }
        Ok(dir)
    }
}
