use super::{
    APP_CONFIG_NAME, APP_CONFIG_TEMPLATE, ConfigStruct, GLOBAL_SECTION_NAME, GlobalSection,
    ModSection,
};
use crate::cli::MyOptions;
use crate::errors::CompileToolErrors;
use configparser::ini::Ini;
use std::{fs, path::Path};

/// _
/// # Errors
/// _
pub fn parse_config(env_arguments: &MyOptions) -> Result<ConfigStruct, CompileToolErrors> {
    let my_config: Ini = load_config()?;
    let global_section: GlobalSection = parse_global_section(&my_config, env_arguments)?;
    let mod_section: ModSection = parse_mod_section(&my_config, &global_section.package_name)?;

    Ok(ConfigStruct {
        global_section,
        mod_section,
    })
}

fn load_config() -> Result<Ini, CompileToolErrors> {
    // Check if our settings file exists in the same directory.
    // If not - create a default one and warn the user.
    if !Path::new(APP_CONFIG_NAME).exists() {
        fs::write(APP_CONFIG_NAME, APP_CONFIG_TEMPLATE)?;
        return Err(CompileToolErrors::StringErrors(format!(
            "{APP_CONFIG_NAME} doesn't exist! Created a default config for you, edit and come back again."
        )));
    }

    let mut my_config: Ini = Ini::new();
    if let Err(e) = my_config.load(APP_CONFIG_NAME) {
        return Err(CompileToolErrors::StringErrors(format!(
            "Couldn't load {APP_CONFIG_NAME}, Error: {e}"
        )));
    }

    Ok(my_config)
}

fn parse_global_section(
    config: &Ini,
    env_arguments: &MyOptions,
) -> Result<GlobalSection, CompileToolErrors> {
    // check if we even have the section
    let sections: Vec<String> = config.sections();

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
    let package_name: String = if env_arguments.mod_name.is_empty() {
        get_cfg_string("mutatorName", config)?
    } else {
        env_arguments.mod_name[0].to_string()
    };

    let dir_compiler = get_cfg_string("dir_Compile", config)?;
    let dir_source_files = get_cfg_string("dir_Classes", config)?;
    let dir_redirect = config.get(GLOBAL_SECTION_NAME, "dir_Redirect");
    let dir_copy_to = config.get(GLOBAL_SECTION_NAME, "dir_MoveTo");
    let dir_release_output = config.get(GLOBAL_SECTION_NAME, "dir_ReleaseOutput");

    Ok(GlobalSection {
        package_name,
        dir_compiler,
        dir_source_files,
        dir_redirect,
        dir_copy_to,
        dir_release_output,
    })
}

fn parse_mod_section(config: &Ini, mod_section: &str) -> Result<ModSection, CompileToolErrors> {
    if !config.sections().contains(&mod_section.to_lowercase()) {
        return Err(CompileToolErrors::StringErrors(format!(
            "Section named `{mod_section}` is not found in {APP_CONFIG_NAME}. Aborting!"
        )));
    }
    // this one is important, handle it
    let Some(edit_packages) = config.get(mod_section, "EditPackages") else {
        return Err(CompileToolErrors::StringErrors(format!(
            "Key `EditPackages` is not found (or empty) in {APP_CONFIG_NAME}. Aborting!"
        )));
    };
    // everything else is optional, we can set `false` on fail
    let compile_outsideof_kf =
        get_cfg_bool_unwrap_or_default("bICompileOutsideofKF", mod_section, config);
    let alt_directories = get_cfg_bool_unwrap_or_default("bAltDirectories", mod_section, config);
    let move_files = get_cfg_bool_unwrap_or_default("bMoveFiles", mod_section, config);
    let create_int = get_cfg_bool_unwrap_or_default("bCreateINT", mod_section, config);
    let make_redirect = get_cfg_bool_unwrap_or_default("bMakeRedirect", mod_section, config);
    let make_release = get_cfg_bool_unwrap_or_default("bMakeRelease", mod_section, config);

    Ok(ModSection {
        edit_packages,
        compile_outsideof_kf,
        alt_directories,
        move_files,
        create_int,
        make_redirect,
        make_release,
    })
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
