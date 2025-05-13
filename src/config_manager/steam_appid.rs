use crate::{CompileToolErrors, RuntimeVariables};
use std::{
    fs::{self, OpenOptions, Permissions},
    io::Write as _,
    path::PathBuf,
};

/// Included default config file.
pub const STEAM_APPID_TXT: &str = "steam_appid.txt";

/// _
/// # Errors
/// _
pub fn create_hacky_steamappid(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    let steam_appid_file: &PathBuf = runtime_vars.compiled_paths.temp_steam_appid.as_ref();
    // remove the file if it exists
    if steam_appid_file.try_exists()? {
        // Set file attributes to normal (removing read-only)
        set_steamappid_readonly(steam_appid_file, false)?;
        // Now delete the file
        fs::remove_file(steam_appid_file)?;
    }

    let mut hack_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(steam_appid_file)?;
    hack_file.write_all(b"3")?;
    set_steamappid_readonly(steam_appid_file, true)?;

    Ok(())
}

/// Set file readonly attribute
/// # Errors
/// _
pub fn set_steamappid_readonly(file: &PathBuf, to: bool) -> Result<(), CompileToolErrors> {
    let mut perms: Permissions = fs::metadata(file)?.permissions();
    perms.set_readonly(to);
    fs::set_permissions(file, perms)?;

    Ok(())
}
