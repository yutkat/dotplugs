use crate::repository::CanReposit;
use crate::repository::Repositories;
use crate::repository::Repository;
use failure::format_err;
use failure::Error;
use std::process::Command;

pub struct Zplugin;

impl CanReposit for Zplugin {
    fn get_repositories() -> Result<Repositories, Error> {
        if !Zplugin::exists_plugin_manager()? {
            return Ok(vec![]);
        }
        let dir = Zplugin::get_plugin_dir()?;
        let short_urls = Zplugin::get_url()?;
        Zplugin::create_repositories(&short_urls, &dir)
    }
}

impl Zplugin {
    fn exists_plugin_manager() -> Result<bool, Error> {
        let cmd = format!(r##"source ~/.zshrc && command -v zplugin > /dev/null 2>&1"##);
        log::debug!("exists check: {}", cmd);
        let status = Command::new("zsh").arg("-c").arg(cmd).status()?;
        log::debug!("process exited with: {}", status);
        Ok(status.success())
    }

    fn get_url() -> Result<Vec<String>, Error> {
        let cmd = format!(
            r##"source ~/.zshrc && zplugin list | cut -d' ' -f1 | sed 's/\x1b\[[0-9;]*m//g'"##
        );
        log::debug!("output zplugin list: {}", cmd);
        let output = Command::new("zsh").arg("-c").arg(cmd).output()?;
        log::debug!("process exited with: {}", output.status);
        let stdout = output.stdout;
        let urls = String::from_utf8(stdout)?;
        let urls_v = urls
            .split_ascii_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        println!("{:#?}", urls_v);
        Ok(urls_v)
    }

    fn get_plugin_dir() -> Result<String, Error> {
        let cmd = format!(r##"source ~/.zshrc && zplugin zstatus | grep 'Plugin directory' | cut -d' ' -f3 | tr -d '\n' | sed 's/\x1b\[[0-9;]*m//g'"##);
        log::debug!("zplugin cmd: {}", cmd);
        let output = Command::new("zsh").arg("-c").arg(cmd).output()?;
        log::debug!("process exited with: {}", output.status);
        let stdout = output.stdout;
        let dir = String::from_utf8(stdout)?;
        log::debug!("zplugin dir: {}", &dir);
        if !std::path::Path::new(&dir).exists() {
            return Err(format_err!("zplugin dir not found {}", &dir));
        }
        Ok(dir)
    }

    fn create_repositories<S: AsRef<str>, T: AsRef<str>>(
        urls: &Vec<S>,
        dir: &T,
    ) -> Result<Repositories, Error> {
        let mut r = vec![];
        for u in urls {
            let repo = Repository {
                uri: format!("https://git::@github.com/{}", u.as_ref()),
                dir: format!("{}/{}", dir.as_ref(), u.as_ref().replace("/", "---")),
            };
            r.push(repo);
        }
        Ok(r)
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
    fn get_plugin_dir_normal() -> Result<(), Error> {
        init();
        Zplugin::get_plugin_dir()?;
        Ok(())
    }
}
