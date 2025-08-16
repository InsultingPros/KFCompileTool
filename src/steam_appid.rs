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
        std::fs::write(self.paths.temp_steam_appid.as_ref(), b"3")?;
        set_file_readonly(
            self.paths.temp_steam_appid.as_ref(),
            &FileEditPermission::ReadOnly,
        )?;

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
