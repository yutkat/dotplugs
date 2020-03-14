mod table_tui;

use failure::Error;
use prettytable::{cell, row, Table};

pub fn view() -> Result<(), Error> {
    let repos = crate::repository::new()?;
    let github_info = crate::github::repo_info::create_info(&repos)?;

    let mut table = Table::new();
    let header = vec!["uri", "star"];
    for g in &github_info {
        table.add_row(row![g.name_with_owner]);
        table.add_row(row![g.stargazers.to_string()]);
    }

    table_tui::display(&header, &table)?;
    Ok(())
}
