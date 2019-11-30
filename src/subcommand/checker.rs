use crate::repository::vim_plug;
use crate::repository::zplugin;
use crate::repository::CanReposit;
use failure::Error;

pub fn check() -> Result<(), Error> {
    display_vim_plug()?;
    display_zplugin()?;
    Ok(())
}

fn display_vim_plug() -> Result<(), Error> {
    let repos = vim_plug::VimPlug::get_repositories()?;
    let statuses = crate::git::get_status(&repos)?;
    crate::display::display(&statuses);
    Ok(())
}

fn display_zplugin() -> Result<(), Error> {
    let repos = zplugin::Zplugin::get_repositories()?;
    let statuses = crate::git::get_status(&repos)?;
    crate::display::display(&statuses);
    Ok(())
}

pub fn output_json() -> Result<String, Error> {
    let repos = vim_plug::VimPlug::get_repositories()?;
    let statuses = crate::git::get_status(&repos)?;
    let j = serde_json::to_string(&statuses)?;
    Ok(j)
}
