use crate::cli::MyOptions;
use crate::{RuntimeVariables, errors::CompileToolErrors};
use configparser::ini::Ini;
use std::fs;
use std::path::Path;

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
            if env_arguments.mod_name.is_empty() {
                result_global = get_global_section(&my_config, None)?;
                result_local = get_local_section(&my_config, &result_global.package_name)?;
            } else {
                result_global = get_global_section(&my_config, Some(&env_arguments.mod_name[0]))?;
                result_local = get_local_section(&my_config, &env_arguments.mod_name[0])?;
            }
        }
        Err(e) => {
            return Err(CompileToolErrors::StringErrors(format!(
                "Couldn't load {APP_CONFIG_NAME}, Error: {e}"
            )));
        }
    }

    let result: RuntimeVariables = RuntimeVariables::new(&result_global, &result_local);
    // dbg!(&result);
    Ok(result)
}

/// _
/// # Errors
/// _
pub fn get_global_section(
    config: &Ini,
    mod_name: Option<&str>,
) -> Result<GlobalSection, CompileToolErrors> {
    // check if we even have the section
    let sections: Vec<String> = config.sections();
    // dbg!(&sections);
    // no sections at all?
    if sections.is_empty() {
        return Err(CompileToolErrors::StringErrors(format!(
            "There are no sections at all in {APP_CONFIG_NAME}! Check your config file"
        )));
    }
    // no [Global]?
    if !sections.contains(&"global".to_string()) {
        return Err(CompileToolErrors::StringErrors(
            "There is no `[Global]` section in the config! Fix your config file.".to_string(),
        ));
    }
    // these ones are important, handle them
    // let package_name: String;
    let package_name: String = if let Some(name) = mod_name {
        name.to_string()
    } else {
        get_cfg_string("mutatorName", config)?
    };

    let dir_compiler = get_cfg_string("dir_Compile", config)?;
    let dir_source_files = get_cfg_string("dir_Classes", config)?;
    let dir_copy_to = config.get(GLOBAL_SECTION_NAME, "dir_MoveTo");
    let dir_release_output = config.get(GLOBAL_SECTION_NAME, "dir_ReleaseOutput");
    let dir_redirect = config.get(GLOBAL_SECTION_NAME, "dir_Redirect");

    let result: GlobalSection = GlobalSection {
        package_name,
        dir_compiler,
        dir_source_files,
        dir_redirect,
        dir_copy_to,
        dir_release_output,
    };
    // dbg!(&result);
    Ok(result)
}

#[inline]
fn get_cfg_string(key: &str, config: &Ini) -> Result<String, CompileToolErrors> {
    config.get(GLOBAL_SECTION_NAME, key).map_or_else(
        || {
            Err(CompileToolErrors::StringErrors(format!(
                "'{key}' path isn't specified in {APP_CONFIG_NAME}'s `{GLOBAL_SECTION_NAME}` section. Aborting!"
            )))
        },
        Ok,
    )
}

/// _
/// # Errors
/// _
pub fn get_local_section(config: &Ini, section: &str) -> Result<ModSection, CompileToolErrors> {
    if !config.sections().contains(&section.to_lowercase()) {
        return Err(CompileToolErrors::StringErrors(format!(
            "Section named `{section}` is not found in {APP_CONFIG_NAME}. Aborting!"
        )));
    }
    // this one is important, handle it
    let Some(edit_packages) = config.get(section, "EditPackages") else {
        return Err(CompileToolErrors::StringErrors(format!(
            "Key `EditPackages` is not found (or empty) in {APP_CONFIG_NAME}. Aborting!"
        )));
    };
    // everything else is optional, we can set `false` on fail
    let compile_outsideof_kf =
        get_cfg_bool_unwrap_or_default("bICompileOutsideofKF", section, config);
    let alt_directories = get_cfg_bool_unwrap_or_default("bAltDirectories", section, config);
    let move_files = get_cfg_bool_unwrap_or_default("bMoveFiles", section, config);
    let create_int = get_cfg_bool_unwrap_or_default("bCreateINT", section, config);
    let make_redirect = get_cfg_bool_unwrap_or_default("bMakeRedirect", section, config);
    let make_release = get_cfg_bool_unwrap_or_default("bMakeRelease", section, config);

    let result: ModSection = ModSection {
        edit_packages,
        compile_outsideof_kf,
        alt_directories,
        move_files,
        create_int,
        make_redirect,
        make_release,
    };
    // dbg!(&result);
    Ok(result)
}

#[inline]
fn get_cfg_bool_unwrap_or_default(key: &str, section: &str, config: &Ini) -> bool {
    if let Ok(Some(result)) = config.getbool(section, key) {
        return result;
    }
    // else there is a missing key in config file, warn the user and return the default
    println!(
        "#WARNING: Key `{key}` is not found in {APP_CONFIG_NAME}'s `{section}` section.\n\
        We set it to false for this run, but it is desirable to fill it explicitely in config file.\n"
    );
    false
}
