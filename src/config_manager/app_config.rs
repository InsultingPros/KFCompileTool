use crate::cli::MyOptions;
use crate::{CompileToolErrors, RuntimeVariables};
use configparser::ini::Ini;
use std::fs;
use std::path::Path;

/// Default app config content.
const APP_CONFIG_TEMPLATE: &str = r";= Home repo: https://github.com/InsultingPros/KFCompileTool
[Global]
mutatorName=BitCore
dir_Compile=D:\Games\KF Dedicated Server
dir_MoveTo=D:\Games\SteamLibrary\steamapps\common\KillingFloor
dir_ReleaseOutput=C:\Users\Pepe User\Desktop\Mutators
dir_Classes=D:\Documents\Killing Floor Archive\03. Projects\Mods

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

/// `Global` section of config file.
#[derive(Debug)]
pub struct GlobalSection {
    /// Name of an existing local section.
    pub package_name: String,
    /// Where we are compiling on.
    pub dir_compiler: String,
    /// Move files to here after successful compilation.
    pub dir_copy_to: Option<String>,
    /// Release folder.
    pub dir_release_output: Option<String>,
    /// Directory of our sources
    pub dir_source_files: String,
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

/// _
/// # Errors
/// _
pub fn parse_app_config(env_arguments: &MyOptions) -> Result<RuntimeVariables, CompileToolErrors> {
    // Check if our settings file exists in the same directory.
    // If not - create a default one and warn the user.
    if !Path::new(APP_CONFIG_NAME).exists() {
        fs::write(APP_CONFIG_NAME, APP_CONFIG_TEMPLATE)?;
        return Err(CompileToolErrors::StringErrors(format!(
            "{APP_CONFIG_NAME} didn't exist! Created a default config for you, edit and come back again."
        )));
    }

    let mut my_config: Ini = Ini::new();
    let result_global: GlobalSection;
    let result_local: ModSection;

    match my_config.load(APP_CONFIG_NAME) {
        Ok(_) => {
            result_global = get_global_section(&my_config)?;
            if env_arguments.mod_name.is_empty() {
                result_local = get_local_section(&my_config, &result_global.package_name)?;
            } else {
                result_local = get_local_section(&my_config, &env_arguments.mod_name[0])?;
            }
        }
        Err(e) => {
            return Err(CompileToolErrors::StringErrors(format!(
                "Still couldn't load {APP_CONFIG_NAME}, error: {e}"
            )));
        }
    }

    Ok(RuntimeVariables::new(&result_global, &result_local))
}

#[inline]
/// _
/// # Errors
/// _
pub fn get_global_section(app_config: &Ini) -> Result<GlobalSection, CompileToolErrors> {
    // check if we even have the section
    let sections = app_config.sections();
    // dbg!(&sections);
    if !sections.contains(&"global".to_string()) {
        return Err(CompileToolErrors::StringErrors(
            "There is no `[Global]` section in the config!".to_string(),
        ));
    }
    // these ones are important, handle them
    let Some(package_name) = app_config.get(GLOBAL_SECTION_NAME, "mutatorName") else {
        return Err(CompileToolErrors::StringErrors(
            "'mutatorName' isn't specified in the config, aborting!".to_string(),
        ));
    };
    let Some(dir_compile) = app_config.get(GLOBAL_SECTION_NAME, "dir_Compile") else {
        return Err(CompileToolErrors::StringErrors(
            "'dir_Compile' path isn't specified in the config, aborting!".to_string(),
        ));
    };
    let Some(dir_classes) = app_config.get(GLOBAL_SECTION_NAME, "dir_Classes") else {
        return Err(CompileToolErrors::StringErrors(
            "'dir_Classes' path isn't specified in the config, aborting!".to_string(),
        ));
    };

    let result: GlobalSection = GlobalSection {
        package_name,
        dir_compiler: dir_compile,
        dir_source_files: dir_classes,
        dir_copy_to: app_config.get(GLOBAL_SECTION_NAME, "dir_MoveTo"),
        dir_release_output: app_config.get(GLOBAL_SECTION_NAME, "dir_ReleaseOutput"),
    };

    Ok(result)
}

#[inline]
/// _
/// # Errors
/// _
pub fn get_local_section(
    app_config: &Ini,
    package_name: &str,
) -> Result<ModSection, CompileToolErrors> {
    if !app_config.sections().contains(&package_name.to_lowercase()) {
        return Err(CompileToolErrors::StringErrors(format!(
            "Section named `{package_name}` not found in {APP_CONFIG_NAME}, aborting!"
        )));
    }

    // this one is important, handle it
    let Some(edit_packages) = app_config.get(package_name, "EditPackages") else {
        return Err(CompileToolErrors::StringErrors(format!(
            "`EditPackages` variable is not found in {APP_CONFIG_NAME} / is empty, aborting!"
        )));
    };

    let result: ModSection = ModSection {
        edit_packages,
        compile_outsideof_kf: app_config
            .getbool(package_name, "bICompileOutsideofKF")
            .map_err(CompileToolErrors::StringErrors)?
            .unwrap_or_default(),
        alt_directories: app_config
            .getbool(package_name, "bAltDirectories")
            .map_err(CompileToolErrors::StringErrors)?
            .unwrap_or_default(),
        move_files: app_config
            .getbool(package_name, "bMoveFiles")
            .map_err(CompileToolErrors::StringErrors)?
            .unwrap_or_default(),
        create_int: app_config
            .getbool(package_name, "bCreateINT")
            .map_err(CompileToolErrors::StringErrors)?
            .unwrap_or_default(),
        make_redirect: app_config
            .getbool(package_name, "bMakeRedirect")
            .map_err(CompileToolErrors::StringErrors)?
            .unwrap_or_default(),
        make_release: app_config
            .getbool(package_name, "bMakeRelease")
            .map_err(CompileToolErrors::StringErrors)?
            .unwrap_or_default(),
    };

    Ok(result)
}
