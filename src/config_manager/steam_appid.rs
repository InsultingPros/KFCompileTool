use crate::{RuntimeVariables, errors::CompileToolErrors};
use std::{
    fs::{self, OpenOptions},
    io::Write as _,
    path::PathBuf,
};

#[derive(Debug)]
enum FileEditPermission {
    ReadOnly,
    Write,
}

/// Included default config file.
pub const STEAM_APPID_TXT: &str = "steam_appid.txt";

/// _
/// # Errors
/// _
pub fn create_hacky_steamappid(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    let steam_appid_file: &PathBuf = runtime_vars.paths.temp_steam_appid.as_ref();
    // remove the file if it exists
    if steam_appid_file.try_exists()? {
        // Set file attributes to normal (removing read-only)
        set_steamappid_readonly(steam_appid_file, &FileEditPermission::Write)?;
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
    set_steamappid_readonly(steam_appid_file, &FileEditPermission::ReadOnly)?;

    Ok(())
}

/// Set file readonly attribute
fn set_steamappid_readonly(
    file: &PathBuf,
    to: &FileEditPermission,
) -> Result<(), CompileToolErrors> {
    let metadata = match fs::metadata(file) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to retrieve metadata from file {}", file.display());
            return Err(CompileToolErrors::IOError(e));
        }
    };
    let mut permsissions = metadata.permissions();

    let b: bool = match to {
        FileEditPermission::ReadOnly => true,
        FileEditPermission::Write => false,
    };
    permsissions.set_readonly(b);

    if let Err(e) = fs::set_permissions(file, permsissions) {
        eprintln!(
            "Failed to set {to:?} permission for file {}",
            file.display()
        );
        return Err(CompileToolErrors::IOError(e));
    }

    Ok(())
}
