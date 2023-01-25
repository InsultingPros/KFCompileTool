#![allow(dead_code)]
use std::path::PathBuf;

/// `Global` section of config file.
#[derive(Debug, PartialEq)]
pub struct SectionGlobal {
    /// Name of an existing local section.
    pub package_name: String,
    /// Directory of our sources
    pub dir_classes: String,
    /// Where we are compiling on.
    pub dir_compile: String,
    /// Move files to here after successful compilation.
    pub dir_move_to: Option<String>,
    /// Release folder.
    pub dir_release_output: Option<String>,
}

/// Per mod section of config file.
#[derive(Debug, PartialEq)]
pub struct SectionLocal {
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

/// All variables that we need during runtime
pub struct RuntimeVariables {
    /// Mod package name.
    pub mutator_name: String,
    /// Where are we compiling, home directory.
    pub dir_compile: PathBuf,
    /// Where to move files on succesfull compilation.
    pub dir_move_to: Option<PathBuf>,
    /// Where to move compiled files.
    pub dir_release_output: Option<PathBuf>,
    /// Where are our source files.
    pub dir_classes: PathBuf,
    /// `EditPackages` aka parents of this mod.
    pub edit_packages: Vec<String>,
    /// Source files are somewhere else.
    pub i_compile_outsideof_kf: bool,
    /// Use alternative source file organization.
    pub alt_directories: bool,
    /// Move files to `dir_move_to`.
    pub move_files: bool,
    /// Create localization file.
    pub create_int: bool,
    /// Create redirect file.
    pub make_redirect: bool,
    /// Create release folder.
    pub make_release: bool,
    // todo
    pub path_source_files: Option<PathBuf>,
    pub path_compile_dir: Option<PathBuf>,
    pub path_ucc: Option<PathBuf>,
    pub path_compile_dir_sys: Option<PathBuf>,
    pub path_compiled_file_u: Option<PathBuf>,
    pub path_compiled_file_ucl: Option<PathBuf>,
    pub path_compiled_file_uz2: Option<PathBuf>,
    pub path_compiled_file_int: Option<PathBuf>,
    pub path_compilation_ini: Option<PathBuf>,
    pub path_garbage_file: Option<PathBuf>,
    pub path_release: Option<PathBuf>,
    pub path_move_to: Option<PathBuf>,
}
