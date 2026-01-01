use crate::{
    RuntimeVariables,
    constants::config_files::{COMPILATION_CONFIG_NAME, COMPILATION_CONFIG_TEMPLATE},
    errors::CompileToolErrors,
};
use std::fmt::Write as _;

/// Creates temporary kf.ini for compilation and adds required `Editpackages`.
/// # Errors
/// _
pub fn create_kf_config(input: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    let mut new_content: String = COMPILATION_CONFIG_TEMPLATE.to_string();

    for package in input.mod_settings.edit_packages.as_ref() {
        // dbg!(package);
        writeln!(&mut new_content, "EditPackages={package}")?;
    }

    std::fs::write(
        input.paths.compile_dir_system.join(COMPILATION_CONFIG_NAME),
        &new_content,
    )?;

    Ok(())
}

/// Remove our temporary compilation `kf.ini`.
/// # Errors
/// _
pub fn remove_kf_config(input: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if input.paths.temp_kf_ini.try_exists()? {
        std::fs::remove_file(&input.paths.temp_kf_ini)?;
    }

    Ok(())
}
