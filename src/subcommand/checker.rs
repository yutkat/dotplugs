use anyhow::Result;

pub fn check() -> Result<()> {
    let repos = crate::repository::new()?;
    let statuses = crate::git::get_status(&repos)?;
    crate::display::display(&statuses);
    Ok(())
}

pub fn output_json() -> Result<String> {
    let repos = crate::repository::new()?;
    let statuses = crate::git::get_status(&repos)?;
    let j = serde_json::to_string(&statuses)?;
    Ok(j)
}
