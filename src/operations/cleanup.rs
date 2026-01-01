use crate::{
    COMPILATION_CONFIG_NAME, RuntimeVariables,
    operations::{
        directory_handler::{handle_alt_style_directories, remove_copied_sources},
        kf_config::remove_kf_config,
        steam_appid::remove_steam_appid,
    },
};

impl Drop for RuntimeVariables {
    /// Clean up the garbage that we've created.
    fn drop(&mut self) {
        // 1. remove temp kfcompile.ini
        if let Err(e) = remove_kf_config(self) {
            eprintln!("Failed to remove temporary kf config `{COMPILATION_CONFIG_NAME}`: {e}");
        }

        // 2. remove temp steam_appid.txt
        if let Err(e) = remove_steam_appid(self) {
            eprintln!("Failed to remove temporary `steam_appid.txt`: {e}");
        }

        // 3. handle alt style source code organization
        if let Err(e) = handle_alt_style_directories(self) {
            eprintln!("Failed to remove `Classes` folder in sources directory: {e}");
        }

        // 4. handle external sources dir case
        if let Err(e) = remove_copied_sources(self) {
            eprintln!("Failed to remove temporary sources folder in compilation directory: {e}");
        }
    }
}
