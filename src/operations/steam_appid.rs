use crate::utils::io::{FileEditPermission, set_file_readonly};
use crate::{RuntimeVariables, errors::CompileToolErrors};
use std::{fs::OpenOptions, io::Write as _, path::PathBuf};

/// _
/// # Errors
/// _
pub fn create_hacky_steamappid(input: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    let steam_appid_file: &PathBuf = input.paths.temp_steam_appid.as_ref();
    // remove the file if it exists
    if steam_appid_file.try_exists()? {
        // Set file attributes to normal (removing read-only)
        set_file_readonly(steam_appid_file, &FileEditPermission::Write)?;
        // Now delete the file
        std::fs::remove_file(steam_appid_file)?;
    }

    let mut hack_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(steam_appid_file)?;
    hack_file.write_all(b"3")?;
    set_file_readonly(steam_appid_file, &FileEditPermission::ReadOnly)?;

    Ok(())
}

/// _
/// # Errors
/// _
pub fn remove_steam_appid(input: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    // steamappid.txt
    if input.paths.temp_steam_appid.try_exists()? {
        set_file_readonly(
            input.paths.temp_steam_appid.as_ref(),
            &FileEditPermission::Write,
        )?;

        std::fs::remove_file(input.paths.temp_steam_appid.as_ref())?;
    }

    Ok(())
}
