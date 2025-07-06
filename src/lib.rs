use crate::config_manager::kf_config::COMPILATION_CONFIG_NAME;
use crate::config_manager::steam_appid::STEAM_APPID_TXT;
use config_manager::app_config::{GlobalSection, ModSection};
use std::{path::PathBuf, rc::Rc};

pub mod cli;
pub mod config_manager;
pub mod errors;
pub mod release_manager;
pub mod stages;
pub mod utility;

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
    pub edit_packages: Rc<Vec<String>>,
    /// Create localization file.
    pub create_int: bool,
    /// Source files are somewhere else.
    pub sources_are_somewhere_else: bool,
    /// Where do our source files actually lay.
    pub path_source_files: PathBuf,
    /// Use alternative source file organization.
    pub alt_directories: bool,
    /// Copy default config or no?
    pub copy_default_ini: bool,
    /// Make an uz2 or no?
    pub make_redirect: bool,
    /// Make a release or no?
    pub make_release: bool,
}

impl ModSettings {
    fn new(global_section: &GlobalSection, local_section: &ModSection) -> Self {
        Self {
            package_name: global_section.package_name.clone(),
            edit_packages: Rc::new(
                local_section
                    .edit_packages
                    .split(',')
                    .map(std::string::ToString::to_string)
                    .collect(),
            ),
            create_int: local_section.create_int,
            sources_are_somewhere_else: local_section.compile_outsideof_kf,
            path_source_files: PathBuf::from(&global_section.dir_source_files)
                .join(global_section.package_name.clone()),
            alt_directories: local_section.alt_directories,
            copy_default_ini: true,
            make_redirect: local_section.make_redirect,
            make_release: local_section.make_release,
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
    pub ucc_exe: Rc<PathBuf>,
    /// Path to temporary kf.ini. For example "D:\\Dedicated Server\\System\\kfcompile.ini".
    pub temp_kf_ini: PathBuf,
    /// Path to hacked `steam_appid.txt`. For example "D:\\Dedicated Server\\System\\`steam_appid.txt`".
    /// # Should be deleted after compilation attempt!
    pub temp_steam_appid: Rc<PathBuf>,

    /// Path to compiled binary file. For example "D:\\Dedicated Server\\System\\`PACKAGE_NAME`.u".
    pub path_package_u: PathBuf,
    /// Path to compiled `ucl` file. For example "D:\\Dedicated Server\\System\\`PACKAGE_NAME`.ucl".
    pub path_package_ucl: PathBuf,
    /// Path to compiled localization file. For example "D:\\Dedicated Server\\System\\`PACKAGE_NAME`.int".
    pub path_package_int: PathBuf,
    /// Path to compiled mod's redirect file. For example "D:\\Dedicated Server\\Redirect\\`PACKAGE_NAME`.uz2".
    pub path_package_uz2: PathBuf,
    /// Path to initially created redirect file ("D:\\Dedicated Server\\System\\`PACKAGE_NAME`.uz2"). Should be deleted or moved!
    pub path_package_uz2_init: PathBuf,
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
    fn new(global_section: &GlobalSection) -> Self {
        let compile_dir: PathBuf = PathBuf::from(global_section.dir_compiler.clone());
        let package_name = &global_section.package_name;

        Self {
            compile_dir: compile_dir.clone(),
            compile_dir_system: compile_dir.join("System"),
            compile_dir_redirect: compile_dir.join("Redirect"),
            compile_dir_sources: compile_dir.join(package_name),

            ucc_exe: Rc::new(compile_dir.join("System").join("UCC.exe")),
            temp_kf_ini: compile_dir.join("System").join(COMPILATION_CONFIG_NAME),
            temp_steam_appid: Rc::new(compile_dir.join(format!("System\\{STEAM_APPID_TXT}"))),

            path_package_u: compile_dir.join(format!("System\\{package_name}.u")),
            path_package_ucl: compile_dir.join(format!("System\\{package_name}.ucl")),
            path_package_uz2: compile_dir.join(format!("Redirect\\{package_name}.u.uz2")),
            path_package_uz2_init: compile_dir.join(format!("System\\{package_name}.u.uz2")),
            path_package_int: compile_dir.join(format!("System\\{package_name}.int")),
            path_package_ini: compile_dir.join(format!("System\\{package_name}.ini")),

            name_package_u: format!("{package_name}.u"),
            name_package_ucl: format!("{package_name}.ucl"),
            name_package_uz2: format!("{package_name}.u.uz2"),
            name_package_ini: format!("{package_name}.ini"),
            name_package_int: format!("{package_name}.int"),

            output_location: global_section
                .dir_release_output
                .as_ref()
                .map(PathBuf::from),
            path_where_to_copy: global_section.dir_copy_to.as_ref().map(PathBuf::from),
            path_redirect: global_section.dir_redirect.as_ref().map(PathBuf::from),
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
    fn new(global_section: &GlobalSection, local_section: &ModSection) -> Self {
        Self {
            mod_settings: ModSettings::new(global_section, local_section),
            paths: CompilationPaths::new(global_section),
            sources_copied_state: SourcesCopied::default(),
        }
    }
}
