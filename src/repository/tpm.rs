use crate::repository::CanReposit;
use crate::repository::Repositories;
use crate::repository::Repository;
use failure::format_err;
use failure::Error;
use std::path::Path;
use std::process::Command;

pub struct Tpm;

impl CanReposit for Tpm {
    fn get_repositories() -> Result<Repositories, Error> {
        if !Tpm::is_running_tmux()? {
            return Ok(vec![]);
        }
        if !Tpm::exists_plugin_manager()? {
            return Ok(vec![]);
        }
        Tpm::create_repositories()
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

    fn exists_plugin_manager() -> Result<bool, Error> {
        let path = match std::env::var("TMUX_PLUGIN_MANAGER_PATH") {
            Ok(n) => n,
            Err(_) => return Ok(false),
        };
        if path.is_empty() {
            return Ok(false);
        }
        if !std::path::Path::new(&path).exists() {
            return Ok(false);
        }
        log::debug!("tpm exists");
        Ok(true)
    }

    fn get_url<P: AsRef<Path>>(path: P) -> Result<String, Error> {
        let cmd = format!(
            r##"cd {} && git config --get remote.origin.url"##,
            path.as_ref().to_str().ok_or(format_err!("convert error"))?
        );
        log::debug!("command to get tpm remote origin: {}", cmd);
        let output = Command::new("sh").arg("-c").arg(cmd).output()?;
        log::debug!("process exited with: {}", output.status);
        let stdout = output.stdout;
        let mut url = String::from_utf8(stdout)?;
        let len = url.trim_end_matches(&['\r', '\n'][..]).len();
        url.truncate(len);
        Ok(url)
    }

    fn create_repositories() -> Result<Repositories, Error> {
        let mut r = vec![];
        let dir = std::env::var("TMUX_PLUGIN_MANAGER_PATH")?;
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let url = Tpm::get_url(&path)?;
                let repo = Repository {
                    uri: url,
                    dir: path
                        .to_str()
                        .ok_or(format_err!("convert error"))?
                        .to_string(),
                };
                r.push(repo);
            }
        }
        Ok(r)
    }
}
