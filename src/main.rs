mod args;

use anyhow::{anyhow, Result};
use dotplugs;

fn main() -> Result<()> {
    let matches = args::load()?;
    match matches.subcommand() {
        ("check", Some(sub_m)) => {
            if sub_m.is_present("json") {
                return dotplugs::check_output_json();
            }
            dotplugs::check()?
        }
        ("update", Some(sub_m)) => {
            if sub_m.is_present("yes") {
                dotplugs::update()?
            } else {
                dotplugs::update_with_confirm()?
            }
        }
        ("viewer", _) => dotplugs::view()?,
        _ => {
            return Err(anyhow!("subcommand not found"));
        }
    }
    Ok(())
}
