use std::{fs::OpenOptions, io::Write as _, path::PathBuf};

use crate::{
    RuntimeVariables,
    errors::CompileToolErrors,
    utility::{FileEditPermission, set_file_readonly},
};

/// Included default config file.
pub const STEAM_APPID_TXT: &str = "steam_appid.txt";

pub trait SteamAppID {
    /// _
    /// # Errors
    /// _
    fn create_hacky_steamappid(&self) -> Result<(), CompileToolErrors>;
    /// _
    /// # Errors
    /// _
    fn remove_steam_appid(&self) -> Result<(), CompileToolErrors>;
}

impl SteamAppID for RuntimeVariables {
    fn create_hacky_steamappid(&self) -> Result<(), CompileToolErrors> {
        let steam_appid_file: &PathBuf = self.paths.temp_steam_appid.as_ref();
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

    fn remove_steam_appid(&self) -> Result<(), CompileToolErrors> {
        // steamappid.txt
        if self.paths.temp_steam_appid.try_exists()? {
            set_file_readonly(
                self.paths.temp_steam_appid.as_ref(),
                &FileEditPermission::Write,
            )?;

            std::fs::remove_file(self.paths.temp_steam_appid.as_ref())?;
        }

        Ok(())
    }
}
