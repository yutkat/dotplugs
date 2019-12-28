mod tpm;
mod util;
mod vim_plug;
mod zplugin;

use failure::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
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
    repos.extend(zplugin::Zplugin::get_repositories()?);
    repos.extend(tpm::Tpm::get_repositories()?);
    return Ok(repos);
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
