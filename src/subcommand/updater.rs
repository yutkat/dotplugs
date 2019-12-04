use crate::git;
use crate::git::UpdateStatus;
use crate::repository::vim_plug;
use crate::repository::zplugin;
use crate::repository::CanReposit;
use failure::Error;

pub fn update() -> Result<(), Error> {
    update_vim_plug()?;
    update_zplugin()?;
    Ok(())
}

pub fn update_after_checking() -> Result<(), Error> {
    let mut statuses = vec![];
    let repos = vim_plug::VimPlug::get_repositories()?;
    statuses.extend(crate::git::get_status(&repos)?);
    let repos = zplugin::Zplugin::get_repositories()?;
    statuses.extend(crate::git::get_status(&repos)?);
    crate::display::display(&statuses);

    if statuses.iter().all(|x| x.status != UpdateStatus::Required) {
        return Ok(());
    }

    if is_continued_by_user()? {
        git::update_using_cached_status(&statuses)?;
    }
    Ok(())
}

fn is_continued_by_user() -> Result<(bool), Error> {
    println!("Do you want to continue? [Y/n]");
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

fn update_vim_plug() -> Result<(), Error> {
    let repos = vim_plug::VimPlug::get_repositories()?;
    git::update(&repos)?;
    Ok(())
}

fn update_zplugin() -> Result<(), Error> {
    let repos = zplugin::Zplugin::get_repositories()?;
    git::update(&repos)?;
    Ok(())
}
