use crate::git;
use crate::git::UpdateStatus;
use colored::Colorize;
use failure::Error;

pub fn update() -> Result<(), Error> {
    let repos = crate::repository::new()?;
    git::update(&repos)?;
    Ok(())
}

pub fn update_after_checking() -> Result<(), Error> {
    let repos = crate::repository::new()?;
    let statuses = crate::git::get_status(&repos)?;
    crate::display::display(&statuses);

    if statuses.iter().all(|x| x.status != UpdateStatus::Required) {
        return Ok(());
    }

    if is_continued_by_user()? {
        git::update_using_cached_status(&statuses)?;
        eprintln!("{}", "Update successful".bold());
    }
    Ok(())
}

fn is_continued_by_user() -> Result<bool, Error> {
    eprint!("{}", "Do you want to continue? [Y/n] ".bold());
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer)?;
    let len = answer.trim_end_matches(&['\r', '\n'][..]).len();
    answer.truncate(len);
    if answer == "Y" || answer == "y" || answer == "" {
        return Ok(true);
    }

    println!("Canceled");
    return Ok(false);
}
