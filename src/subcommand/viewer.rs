mod table_tui;

use anyhow::Result;
use prettytable::{cell, row, Table};

pub fn view() -> Result<()> {
    let repos = crate::repository::new()?;
    let github_info = crate::github::repo_info::create_info(&repos)?;

    let mut table = Table::new();
    let header = vec!["uri", "star", "updated"];
    for g in &github_info {
        table.add_row(row![
            g.name_with_owner,
            g.stargazers.to_string(),
            g.updated_at.format("%Y-%m-%dT%H:%M:%S").to_string()
        ]);
    }

    table_tui::display(&header, &table)?;
    Ok(())
}
