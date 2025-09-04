use crate::{RuntimeVariables, errors::CompileToolErrors, utility::copy_file};

/// _
/// # Errors
/// _
pub fn copy_files_to_another_kf(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if !runtime_vars.mod_settings.move_files {
        return Ok(());
    }

    if let Some(kf_dir) = &runtime_vars.paths.path_where_to_copy {
        if !kf_dir.exists() {
            eprintln!(
                "path_where_to_copy `{}` doesn't exist. Can't copy any files.",
                kf_dir.display()
            );
            return Ok(());
        }
        let system = &kf_dir.join("System");
        if !system.exists() {
            eprintln!(
                "path_where_to_copy `{}` exists, but there is no `System` folder there. Wrong kf dir? Can't copy any files.",
                kf_dir.display()
            );
            return Ok(());
        }

        copy_file(
            &runtime_vars.paths.path_package_u,
            &system.join(&runtime_vars.paths.name_package_u),
        );
        copy_file(
            &runtime_vars.paths.path_package_ucl,
            &system.join(&runtime_vars.paths.name_package_ucl),
        );
        copy_file(
            &runtime_vars.paths.path_package_int,
            &system.join(&runtime_vars.paths.name_package_int),
        );
    }

    Ok(())
}

/// _
/// # Errors
/// _
pub fn run(runtime_vars: &mut RuntimeVariables) -> Result<(), CompileToolErrors> {
    // 1. Move files to another kf dir
    copy_files_to_another_kf(runtime_vars)?;

    Ok(())
}
