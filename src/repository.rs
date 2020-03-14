mod tpm;
mod vim_plug;
mod zinit;

use failure::format_err;
use failure::Error;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    pub uri: String,
    pub dir: String,
}

pub type Repositories = Vec<Repository>;

trait CanReposit {
    fn get_repositories() -> Result<Repositories, Error>;
}

pub fn new() -> Result<Repositories, Error> {
    let mut repos = vec![];
    repos.extend(vim_plug::VimPlug::get_repositories()?);
    repos.extend(zinit::Zinit::get_repositories()?);
    repos.extend(tpm::Tpm::get_repositories()?);
    Ok(repos)
}

impl Repository {
    pub fn get_name_with_owner(&self) -> Result<String, Error> {
        let mut parts = self.uri.trim_end_matches(".git").rsplit('/');
        match (parts.next(), parts.next()) {
            (Some(name), Some(owner)) => Ok(format!("{}/{}", owner, name)),
            _ => Err(format_err!("wrong format for the repository name param (we expect something like facebook/graphql)"))
        }
    }
}

mod git_directory {
    use crate::repository::Repositories;
    use crate::repository::Repository;
    use failure::format_err;
    use failure::Error;
    use std::path::Path;
    use std::process::Command;

    pub struct GitDirectory;

    impl GitDirectory {
        pub fn get_repositories<S: Into<String>>(path: S) -> Result<Repositories, Error> {
            let p = path.into();
            log::debug!("git repository root {}", p);
            if !GitDirectory::exists_plugin_manager(&p)? {
                return Ok(vec![]);
            }
            GitDirectory::create_repositories_struct(&p)
        }

        fn exists_plugin_manager<S: Into<String>>(path: S) -> Result<bool, Error> {
            let path = path.into();
            if path.is_empty() {
                return Ok(false);
            }
            if !std::path::Path::new(&path).exists() {
                return Ok(false);
            }
            log::debug!("repositories exists");
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

        fn create_repositories_struct<S: Into<String>>(path: S) -> Result<Repositories, Error> {
            let mut r = vec![];
            for entry in std::fs::read_dir(path.into())? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let url = GitDirectory::get_url(&path)?;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = pretty_env_logger::formatted_builder()
            .is_test(true)
            .parse_filters("DEBUG")
            .try_init();
    }

    #[test]
    fn serde_test() -> Result<(), Error> {
        let data = r#"
            [
                {
                    "uri": "https://git::@github.com/thecodesmith/vim-groovy.git",
                    "dir": "/home/test/.vim/plugged/vim-groovy/"
                },
                {
                    "uri": "https://git::@github.com/emonkak/vim-operator-sort.git",
                    "dir": "/home/test/.vim/plugged/vim-operator-sort/"
                },
                {
                    "uri": "https://git::@github.com/kana/vim-operator-user.git",
                    "dir": "/home/test/.vim/plugged/vim-operator-user/"
                },
                {
                    "uri": "https://git::@github.com/moll/vim-bbye.git",
                    "dir": "/home/test/.vim/plugged/vim-bbye/"
                }
            ]
        "#;
        serde_json::from_str::<Vec<Repository>>(data)?;
        Ok(())
    }

    #[test]
    fn convert_query_ok() -> Result<(), Error> {
        init();
        let r = Repository {
            uri: "https://git::@github.com/kana/vim-operator-user.git".to_string(),
            dir: "/home/test/.vim/plugged/vim-operator-user/".to_string(),
        };
        let s = r.get_name_with_owner()?;
        assert_eq!(s, "kana/vim-operator-user");
        Ok(())
    }
}
