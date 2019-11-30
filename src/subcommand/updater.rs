use crate::git;
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
    git::update_using_cached_status(&statuses)?;
    Ok(())
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
