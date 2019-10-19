mod args;

use dotplugs;
use failure::Error;
use failure::format_err;

fn main() -> Result<(), Error> {
    let matches = args::load()?;
    match matches.subcommand() {
        ("check", Some(sub_m)) => {
            if sub_m.is_present("json") {
                return dotplugs::check_output_json();
            }
            dotplugs::check()?
        },
        _ => { return Err(format_err!("subcommand not found")); },
    }
    Ok(())
}
