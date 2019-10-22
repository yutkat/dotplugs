mod git;
mod repository;
mod display;
mod update_checker;

use crate::repository::vim_plug;
use crate::repository::CanReposit;
use failure::Error;

pub fn run() -> Result<(), Error> {
    env_logger::init();

    let repos = vim_plug::VimPlug::get_repositories()?;
    let statues = git::get_status(&repos)?;
    println!("{:#?}", statues);
    Ok(())
}

pub fn check() -> Result<(), Error> {
    env_logger::init();

    update_checker::check()?;
    Ok(())
}

pub fn check_output_json() -> Result<(), Error> {
    env_logger::init();

    let repos = vim_plug::VimPlug::get_repositories()?;
    let statues = git::get_status(&repos)?;
    let j = serde_json::to_string(&statues)?;
    println!("{}", j);
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
