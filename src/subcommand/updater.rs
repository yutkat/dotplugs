use crate::git::GitStatus;
use crate::repository::vim_plug;
use crate::repository::zplugin;
use crate::repository::CanReposit;
use failure::Error;

pub fn update() -> Result<(), Error> {
    let mut statues = vec![];
    statues.push(get_vim_plug()?);
    statues.push(get_zplugin()?);
    Ok(())
}

fn get_vim_plug() -> Result<Vec<GitStatus>, Error> {
    let repos = vim_plug::VimPlug::get_repositories()?;
    let statues = crate::git::get_status(&repos)?;
    Ok(statues)
}

fn get_zplugin() -> Result<Vec<GitStatus>, Error> {
    let repos = zplugin::Zplugin::get_repositories()?;
    let statues = crate::git::get_status(&repos)?;
    Ok(statues)
}
