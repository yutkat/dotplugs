mod table_tui;

use failure::format_err;
use failure::Error;
use prettytable::{cell, row, Table};

pub fn view() -> Result<(), Error> {
    let repos = crate::repository::new()?;

    let mut table = Table::new();
    let header = vec!["uri"];
    for repo in &repos {
        let s: Vec<&str> = repo.uri.rsplit('/').collect();
        if s.len() < 2 {
            return Err(format_err!(
                "Wrong git remote repository name: {}",
                &repo.uri
            ));
        }
        let uri_short = format!("{}/{}", s[1], s[0]);
        table.add_row(row![uri_short]);
    }

    table_tui::display(&header, &table)?;
    Ok(())
}
