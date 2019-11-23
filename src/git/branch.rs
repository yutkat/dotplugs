use failure::format_err;
use failure::Error;

pub fn get_current_branch(repo: &git2::Repository) -> Result<String, Error> {
    let mut branches = repo.branches(None)?;
    let branch = branches
        .find(|b| b.as_ref().unwrap().0.is_head())
        .ok_or(format_err!("There is no branch corresponding to HEAD"))??
        .0;
    Ok(branch.name()?.unwrap_or("master").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::Repository;
    extern crate pretty_env_logger;

    fn init() {
        let _ = pretty_env_logger::formatted_builder()
            .is_test(true)
            .parse_filters("DEBUG")
            .try_init();
    }
    #[test]
    fn fetch_test() -> Result<(), Error> {
        init();
        let repo = Repository {
            uri: "https://github.com/Shougo/defx.nvim.git".to_string(),
            dir: "/home/osft/dotfiles/.vim/plugged/defx.nvim".to_string(),
        };
        let git_repo = git2::Repository::open(&repo.dir)?;
        let branch = get_current_branch(&git_repo)?;
        git_repo
            .find_remote("origin")?
            .fetch(&[&branch], None, None)?;
        Ok(())
    }
}
