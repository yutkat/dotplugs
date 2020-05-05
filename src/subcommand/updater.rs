use crate::git;
use crate::git::UpdateStatus;
use anyhow::Result;
use colored::Colorize;

pub fn update() -> Result<()> {
    let repos = crate::repository::new()?;
    let statuses = crate::git::get_status(&repos)?;
    crate::display::display(&statuses);

    if statuses.iter().all(|x| x.status != UpdateStatus::Required) {
        return Ok(());
    }

    git::update(&statuses)?;
    eprintln!("{}", "Update successful".bold());
    Ok(())
}

pub fn update_after_checking() -> Result<()> {
    let repos = crate::repository::new()?;
    let statuses = crate::git::get_status(&repos)?;
    crate::display::display(&statuses);

    if statuses.iter().all(|x| x.status != UpdateStatus::Required) {
        return Ok(());
    }

    if is_continued_by_user()? {
        git::update(&statuses)?;
        eprintln!("{}", "Update successful".bold());
    }
    Ok(())
}

fn is_continued_by_user() -> Result<bool> {
    eprint!("{}", "Do you want to continue? [Y/n] ".bold());
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;
    let len = answer.trim_end_matches(&['\r', '\n'][..]).len();
    answer.truncate(len);
    if answer == "Y" || answer == "y" || answer == "" {
        return Ok(true);
    }

    println!("Canceled");
    Ok(false)
}
