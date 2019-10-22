pub mod vim_plug;
pub mod zplugin;

use failure::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub uri: String,
    pub dir: String,
}

pub type Repositories = Vec<Repository>;

pub trait CanReposit {
    fn get_repositories() -> Result<Repositories, Error>;
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
