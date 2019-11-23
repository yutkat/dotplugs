mod display;
mod git;
mod repository;
mod update_checker;

use crate::repository::vim_plug;
use crate::repository::CanReposit;
use env_logger::Target;
use failure::Error;

pub fn run() -> Result<(), Error> {
    init_logger();
    let repos = vim_plug::VimPlug::get_repositories()?;
    let statues = git::get_status(&repos)?;
    println!("{:#?}", statues);
    Ok(())
}

pub fn check() -> Result<(), Error> {
    init_logger();
    update_checker::check()?;
    Ok(())
}

pub fn check_output_json() -> Result<(), Error> {
    init_logger();
    let repos = vim_plug::VimPlug::get_repositories()?;
    let statues = git::get_status(&repos)?;
    let j = serde_json::to_string(&statues)?;
    println!("{}", j);
    Ok(())
}

fn init_logger() {
    let mut builder = pretty_env_logger::formatted_builder();
    if let Ok(s) = ::std::env::var("RUST_LOG") {
        builder.parse_filters(&s);
    } else {
        builder.parse_filters("info");
    }
    builder.target(Target::Stderr).init()
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
    fn run_normal() {
        init();
        match run() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
