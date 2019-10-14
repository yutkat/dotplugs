mod args;

use dotplugs;
use failure::Error;
use failure::format_err;

fn main() -> Result<(), Error> {
    let matches = args::load()?;
    match matches.subcommand() {
        ("check", _) => { dotplugs::check()? },
        _ => { return Err(format_err!("subcommand not found")); },
    }
    Ok(())
}
