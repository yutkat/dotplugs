mod display;
mod git;
mod repository;
mod subcommand;

use env_logger::Target;
use failure::Error;

pub fn check() -> Result<(), Error> {
    init_logger();
    subcommand::checker::check()?;
    Ok(())
}

pub fn update() -> Result<(), Error> {
    init_logger();
    subcommand::updater::update()?;
    Ok(())
}

pub fn update_with_confirm() -> Result<(), Error> {
    init_logger();
    subcommand::updater::update_after_checking()?;
    Ok(())
}

pub fn check_output_json() -> Result<(), Error> {
    init_logger();
    let j = subcommand::checker::output_json()?;
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
        match check() {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }
}
