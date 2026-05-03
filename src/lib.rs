use crate::traits::steam_appid::STEAM_APPID_TXT;
use crate::{app_config::ConfigStruct, traits::kf_config::COMPILATION_CONFIG_NAME};
use std::path::PathBuf;

pub mod app_config;
pub mod cli;
pub mod errors;
pub mod release_manager;
pub mod stages;
pub mod traits;
pub mod utils;

/// KF1 file extensions.
pub const UNREAL_PACKAGES: [&str; 4] = [".u", ".ucl", ".u.uz2", ".int"];
/// Filter for files-directories, so we copy-paste only source files.
pub const IGNORE_LIST: [&str; 4] = [".git", "*.md", "Docs", "LICENSE"];
/// Alternative folder name, instead of `Classes`. Used to organize source files in a better way.
pub const ALT_SOURCE_DIR_NAME: &str = "sources";

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default)]
pub struct ModSettings {
    /// Mod package name.
    pub package_name: String,
    /// Mod's `EditPackages`.
    pub edit_packages: Vec<String>,
    /// Create localization file.
    pub create_int: bool,
    /// Source files are somewhere else.
    pub sources_are_somewhere_else: bool,
    /// Where do our source files actually lay.
    pub path_source_files: PathBuf,
    /// Use alternative source file organization.
    pub alt_directories: bool,
    /// Copy compiled files to defined kf directory
    pub move_files: bool,
    /// Make an uz2 or no?
    pub make_redirect: bool,
    /// Make a release or no?
    pub make_release: bool,
}

impl ModSettings {
    fn new(parsed_config: &ConfigStruct) -> Self {
        let global_section = &parsed_config.global_section;
        let mod_section = &parsed_config.mod_section;

        let package_name = global_section.package_name.clone();

        Self {
            edit_packages: mod_section
                .edit_packages
                .split(',')
                .map(std::string::ToString::to_string)
                .collect(),
            create_int: mod_section.create_int,
            sources_are_somewhere_else: mod_section.compile_outsideof_kf,
            path_source_files: PathBuf::from(&global_section.dir_source_files).join(&package_name),
            alt_directories: mod_section.alt_directories,
            move_files: mod_section.move_files,
            make_redirect: mod_section.make_redirect,
            make_release: mod_section.make_release,

            package_name,
        }
    }
}

/// compiled files
#[derive(Debug, Default)]
pub struct CompilationPaths {
    /// Compilation main directory. For example "D:\\Dedicated Server".
    pub compile_dir: PathBuf,
    /// Compilation `System` directory. For example "D:\\Dedicated Server\\System".
    pub compile_dir_system: PathBuf,
    /// Compilation `System` directory. For example "D:\\Dedicated Server\\Redirect".
    pub compile_dir_redirect: PathBuf,
    /// Source files directory in root.
    pub compile_dir_sources: PathBuf,

    /// Path to `UCC.exe`. For example "D:\\Dedicated Server\\System\\UCC.exe".
    pub ucc_exe: PathBuf,
    /// Path to temporary kf.ini. For example "D:\\Dedicated Server\\System\\kfcompile.ini".
    pub temp_kf_ini: PathBuf,
    /// Path to hacked `steam_appid.txt`. For example "D:\\Dedicated Server\\System\\`steam_appid.txt`".
    /// # Should be deleted after compilation attempt!
    pub temp_steam_appid: PathBuf,

    /// Path to compiled binary file. For example "D:\\Dedicated Server\\System\\`PACKAGE_NAME`.u".
    pub path_package_u: PathBuf,
    /// Path to compiled `ucl` file. For example "D:\\Dedicated Server\\System\\`PACKAGE_NAME`.ucl".
    pub path_package_ucl: PathBuf,
    /// Path to compiled localization file. For example "D:\\Dedicated Server\\System\\`PACKAGE_NAME`.int".
    pub path_package_int: PathBuf,
    /// Path to compiled mod's redirect file. For example "D:\\Dedicated Server\\Redirect\\`PACKAGE_NAME`.uz2".
    pub path_package_uz2: PathBuf,
    /// Path to compiled mod's redirect file. For example "D:\\Dedicated Server\\System\\`PACKAGE_NAME`.ini".
    pub path_package_ini: PathBuf,

    // package names, for easier access
    pub name_package_u: String,
    pub name_package_ucl: String,
    pub name_package_uz2: String,
    pub name_package_ini: String,
    pub name_package_int: String,

    pub output_location: Option<PathBuf>,
    /// Where to copy binary files after compilation
    pub path_where_to_copy: Option<PathBuf>,
    /// redirect location
    pub path_redirect: Option<PathBuf>,
}

impl CompilationPaths {
    fn new(parsed_config: &ConfigStruct) -> Self {
        let global_section = &parsed_config.global_section;
        let compile_dir: PathBuf = PathBuf::from(&global_section.dir_compiler);
        let package_name = &global_section.package_name;

        let u = format!("{package_name}.u");
        let ucl = format!("{package_name}.ucl");
        let uz2 = format!("{package_name}.u.uz2");
        let ini = format!("{package_name}.ini");
        let int = format!("{package_name}.int");
        let bind = compile_dir.join("System");
        let system = bind.as_path();

        Self {
            compile_dir_system: system.to_path_buf(),
            compile_dir_redirect: compile_dir.join("Redirect"),
            compile_dir_sources: compile_dir.join(package_name),

            ucc_exe: system.join("UCC.exe"),
            temp_kf_ini: system.join(COMPILATION_CONFIG_NAME),
            temp_steam_appid: system.join(STEAM_APPID_TXT),

            path_package_u: system.join(&u),
            path_package_ucl: system.join(&ucl),
            path_package_uz2: system.join(&uz2),
            path_package_int: system.join(&int),
            path_package_ini: system.join(&ini),

            name_package_u: u,
            name_package_ucl: ucl,
            name_package_uz2: uz2,
            name_package_ini: ini,
            name_package_int: int,

            output_location: global_section
                .dir_release_output
                .as_ref()
                .map(PathBuf::from),
            path_where_to_copy: global_section.dir_copy_to.as_ref().map(PathBuf::from),
            path_redirect: global_section.dir_redirect.as_ref().map(PathBuf::from),

            compile_dir,
        }
    }
}

#[derive(Default, PartialEq, Eq, Debug)]
pub enum SourcesCopied {
    #[default]
    Nope,
    FromExternalDir,
    Ignored,
}

#[derive(Debug, Default)]
/// some stupid superstruct for all important variables
pub struct RuntimeVariables {
    pub mod_settings: ModSettings,
    pub paths: CompilationPaths,
    pub sources_copied_state: SourcesCopied,
}

impl RuntimeVariables {
    #[must_use]
    pub fn new(parsed_config: &ConfigStruct) -> Self {
        Self {
            mod_settings: ModSettings::new(parsed_config),
            paths: CompilationPaths::new(parsed_config),
            sources_copied_state: SourcesCopied::default(),
        }
    }
}
