use clap::{load_yaml, App};
use yaml_rust::Yaml;
use failure::Error;
use lazy_static::lazy_static;

lazy_static! {
    static ref APP_YAML: Yaml = {
        load_yaml!("cli.yml").clone()
    };
    static ref ARGS_MATCHES: clap::ArgMatches<'static> = {
        let app = App::from_yaml(&APP_YAML);
        app.get_matches()
    };
}

pub fn load() -> Result<clap::ArgMatches<'static>, Error> {
    match ARGS_MATCHES.occurrences_of("verbose") {
        0 => {}
        _ => std::env::set_var("RUST_LOG", "debug"),
    }

    Ok(ARGS_MATCHES.clone())
}
