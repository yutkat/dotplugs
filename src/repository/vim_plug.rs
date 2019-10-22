use crate::repository::CanReposit;
use crate::repository::Repositories;
use failure::format_err;
use failure::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub struct VimPlug;

impl CanReposit for VimPlug {
    fn get_repositories() -> Result<Repositories, Error> {
        let vimrc = "~/.vimrc";
        if !VimPlug::exists_plugin_manager(&vimrc)? {
            return Ok(vec![]);
        }
        let path = VimPlug::output_plugins_file(&vimrc)?;
        VimPlug::get_repositories_from_path(path)
    }
}

impl VimPlug {
    fn exists_plugin_manager<P: AsRef<Path>>(vimrc: P) -> Result<bool, Error> {
        let cmd = format!(
            r##"nvim -es -u {} +"if exists(':PlugInstall') | qall | else | cq | endif""##,
            vimrc
                .as_ref()
                .to_str()
                .ok_or(format_err!("convert error"))?
        );
        log::debug!("output vim-plug list: {}", cmd);
        let status = Command::new("sh").arg("-c").arg(cmd).status()?;
        log::debug!("process exited with: {}", status);
        Ok(status.success())
    }

    fn output_plugins_file<P: AsRef<Path>>(vimrc: P) -> Result<PathBuf, Error> {
        let cmd = format!(r##"nvim -es -u {} +"redir! > /tmp/vim_plug.json | echo substitute(string(values(map(copy(g:plugs), {{index, val -> {{'uri': val['uri'], 'dir': val['dir']}}}}))), \"'\", '\"', 'g') | redir END""##, vimrc.as_ref().to_str().ok_or(format_err!("convert error"))?);
        log::debug!("output vim-plug list: {}", cmd);
        let status = Command::new("sh").arg("-c").arg(cmd).status()?;
        log::debug!("process exited with: {}", status);
        Ok(PathBuf::from("/tmp/vim_plug.json"))
    }

    fn get_repositories_from_path<P: AsRef<Path>>(path: P) -> Result<Repositories, Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let repos: Repositories = serde_json::from_reader(reader)?;
        Ok(repos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        std::env::set_var("RUST_LOG", "debug");
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn get_repositories_normal() -> Result<(), Error> {
        init();
        VimPlug::get_repositories()?;
        Ok(())
    }

    #[test]
    fn get_repositories_from_path_normal() -> Result<(), Error> {
        init();
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let plug_list = format!(
            "{}/tests/data/vim_plug.json",
            project_root.to_str().unwrap()
        );

        VimPlug::get_repositories_from_path(plug_list)?;
        Ok(())
    }

    #[test]
    fn output_file_normal() -> Result<(), Error> {
        init();
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let vimrc = format!("{}/tests/data/vimrc", project_root.to_str().unwrap());
        let path = VimPlug::output_plugins_file(vimrc)?;
        assert!(path.exists());
        Ok(())
    }

    #[test]
    fn exists_vim_plug_normal() -> Result<(), Error> {
        init();
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let vimrc = format!("{}/tests/data/vimrc", project_root.to_str().unwrap());
        assert!(VimPlug::exists_plugin_manager(vimrc)?);
        Ok(())
    }
}
