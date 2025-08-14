use crate::{
    COMPILATION_CONFIG_NAME, RuntimeVariables,
    traits::{kf_config::KFConfig as _, steam_appid::SteamAppID as _},
};

impl Drop for RuntimeVariables {
    /// Clean up the garbage that we've created.
    fn drop(&mut self) {
        // 1. remove temp kfcompile.ini
        if let Err(e) = self.remove_kf_config() {
            eprintln!("Failed to remove temporary kf config `{COMPILATION_CONFIG_NAME}`: {e}");
        }

        // 2. remove temp steam_appid.txt
        if let Err(e) = self.remove_steam_appid() {
            eprintln!("Failed to remove temporary `steam_appid.txt`: {e}");
        }

        // 3. handle alt style source code organization
        if let Err(e) = self.handle_alt_style_directories() {
            eprintln!("Failed to remove `Classes` folder in sources directory: {e}");
        }

        // 4. handle external sources dir case
        if let Err(e) = self.remove_copied_sources() {
            eprintln!("Failed to remove temporary sources folder in compilation directory: {e}");
        }
    }
}
