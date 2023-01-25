#![allow(dead_code)]
/// config name
pub const COMPILATION_CONFIG_NAME: &str = "kfcompile.ini";
/// included minimal kf.ini for ucc.exe
pub const COMPILATION_CONFIG_TEMPLATE: &str = include_str!("./static/kfcompile.ini");

/// Config name.
pub const INTERNAL_CONFIG_NAME: &str = "CompileSettings.ini";
/// Included default config file.
pub const INTERNAL_CONFIG_TEMPLATE: &str = include_str!("./static/CompileSettings.ini");

/// KF1 file extensions.
pub const UNREAL_PACKAGES: [&str; 4] = [".u", ".ucl", ".u.uz2", ".int"];
/// Filter for files-directories, so we copy-paste only source files.
pub const IGNORE_LIST: [&str; 4] = [".git", "*.md", "Docs", "LICENSE"];

/// Define application exit codes, specific to each platforms
///
/// Reference: <https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499->
#[cfg(target_family = "windows")]
pub mod exit_codes {
    pub const ERROR_SUCCESS: u8 = 0;
    pub const ERROR_CANNOT_MAKE: u8 = 82;
    // pub const ARGUMENT_PARSING_ERROR: u8 = 2;
    // pub const ERROR_BAD_ARGUMENTS: u8 = 160;
}

/// Define application exit codes, specific to each platform
///
/// Reference: <https://unix.stackexchange.com/a/254747>
#[cfg(target_family = "unix")]
pub mod exit_codes {
    pub const ERROR_SUCCESS: u8 = 0;
    pub const ERROR_CANNOT_MAKE: u8 = 1;
    pub const ARGUMENT_PARSING_ERROR: u8 = 2;
    pub const ERROR_BAD_ARGUMENTS: u8 = 128;
}
