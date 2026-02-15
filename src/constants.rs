pub const LINE_SEPARATOR: &str = "\n######################################################\n";

/// KF1 file extensions.
pub const UNREAL_PACKAGES: [&str; 4] = [".u", ".ucl", ".u.uz2", ".int"];
/// Filter for files-directories, so we copy-paste only source files.
pub const IGNORE_LIST: [&str; 4] = [".git", "*.md", "Docs", "LICENSE"];
/// Alternative folder name, instead of `Classes`. Used to organize source files in a better way.
pub const ALT_SOURCE_DIR_NAME: &str = "sources";

/// Included default config file.
pub const STEAM_APPID_TXT: &str = "steam_appid.txt";

pub mod config_files {
    /// Default app config content.
    pub const APP_CONFIG_TEMPLATE: &str = include_str!("static\\DEFAULT_CONFIG.ini");
    /// App config's global section name.
    pub const GLOBAL_SECTION_NAME: &str = "global";
    /// Config name.
    pub const APP_CONFIG_NAME: &str = "kf_compile_tool.ini";

    /// included minimal kf.ini for ucc.exe
    pub const COMPILATION_CONFIG_TEMPLATE: &str = include_str!("static\\DEFAULT_KF_CONFIG.ini");
    /// config name
    pub const COMPILATION_CONFIG_NAME: &str = "kfcompile.ini";
}
