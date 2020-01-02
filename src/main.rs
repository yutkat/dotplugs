mod args;

use dotplugs;
use failure::format_err;
use failure::Error;

fn main() -> Result<(), Error> {
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
                // Todo add a force function
                dotplugs::update()?
            }
            dotplugs::update_with_confirm()?
        }
        ("viewer", _) => dotplugs::view()?,
        _ => {
            return Err(format_err!("subcommand not found"));
        }
    }
    Ok(())
}
