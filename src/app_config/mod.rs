pub mod parser;

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
