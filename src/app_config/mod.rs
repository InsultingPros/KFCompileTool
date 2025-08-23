pub mod parser;

/// Default app config content.
const APP_CONFIG_TEMPLATE: &str = r";= Home repo: https://github.com/InsultingPros/KFCompileTool
[Global]
mutatorName=BitCore
dir_Compile=D:\Games\KF Dedicated Server
dir_Classes=D:\Documents\Killing Floor Archive\03. Projects\Mods
dir_Redirect=D:\Games\KF Dedicated Server\Redirect
dir_MoveTo=D:\Games\SteamLibrary\steamapps\common\KillingFloor
dir_ReleaseOutput=C:\Users\Pepe User\Desktop\Mutators

[BitCore]
EditPackages=BitCore
bICompileOutsideofKF=True
bAltDirectories=False
bMoveFiles=False
bCreateINT=True
bMakeRedirect=True
bMakeRelease=True
";
/// App config's global section name.
pub const GLOBAL_SECTION_NAME: &str = "global";
/// Config name.
pub const APP_CONFIG_NAME: &str = "kf_compile_tool.ini";

#[derive(Debug)]
pub struct ConfigStruct {
    pub global_section: GlobalSection,
    pub mod_section: ModSection,
}

/// `Global` section of config file.
#[derive(Debug)]
pub struct GlobalSection {
    /// Name of an existing local section.
    pub package_name: String,
    /// Where we are compiling on.
    pub dir_compiler: String,
    /// Directory of our sources
    pub dir_source_files: String,
    /// Redirect location
    pub dir_redirect: Option<String>,
    /// Move files to here after successful compilation.
    pub dir_copy_to: Option<String>,
    /// Release folder.
    pub dir_release_output: Option<String>,
}

/// Per mod section of config file.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct ModSection {
    /// `EditPackages` of this mod.
    ///
    /// **This is not a list!** Separate dependencies by comma.
    pub edit_packages: String,
    /// Are our sources in the same directory as `dir_compile`.
    pub compile_outsideof_kf: bool,
    /// Are we using alternative source file organization style.
    pub alt_directories: bool,
    /// Move files to `dir_move_to`.
    pub move_files: bool,
    /// Create localization files.
    pub create_int: bool,
    /// Create redirect file.
    pub make_redirect: bool,
    /// Move compiled files to `dir_release_output`.
    pub make_release: bool,
}
