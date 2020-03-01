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
        let mut dir = dirs::home_dir().ok_or(format_err!("Home env not found"))?;
        dir.push(".vim/plugged");
        log::debug!("vim-plug dir: {:?}", &dir);
        if !&dir.exists() {
            return Err(format_err!("vim-plug dir not found {:?}", &dir));
        }
        Ok(dir
            .into_os_string()
            .into_string()
            .map_err(|x| format_err!("Home env not found {:?}", x))?)
    }
}
