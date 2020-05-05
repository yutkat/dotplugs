mod display;
mod git;
mod github;
mod repository;
mod subcommand;

use anyhow::Result;
use env_logger::Target;

pub fn check() -> Result<()> {
    init_logger();
    subcommand::checker::check()?;
    Ok(())
}

pub fn check_output_json() -> Result<()> {
    init_logger();
    let j = subcommand::checker::output_json()?;
    println!("{}", j);
    Ok(())
}

pub fn update() -> Result<()> {
    init_logger();
    subcommand::updater::update()?;
    Ok(())
}

pub fn view() -> Result<()> {
    init_logger();
    subcommand::viewer::view()?;
    Ok(())
}

pub fn update_with_confirm() -> Result<()> {
    init_logger();
    subcommand::updater::update_after_checking()?;
    Ok(())
}

fn init_logger() {
    let mut builder = pretty_env_logger::formatted_builder();
    if let Ok(s) = ::std::env::var("RUST_LOG") {
        builder.parse_filters(&s);
    } else {
        builder.parse_filters("info");
    }
    builder.target(Target::Stderr).init();
}
