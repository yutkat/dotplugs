mod git;
mod repository;

use crate::repository::vim_plug;
use crate::repository::CanReposit;
use failure::Error;

pub fn run() -> Result<(), Error> {
    let repos = vim_plug::VimPlug::get_repositories()?;
    let statues = git::get_status(&repos)?;
    println!("{:?}", statues);
    Ok(())
}

pub fn check() -> Result<(), Error> {
    let repos = vim_plug::VimPlug::get_repositories()?;
    let statues = git::get_status(&repos)?;
    println!("{:?}", statues);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_normal() {
        match run() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
