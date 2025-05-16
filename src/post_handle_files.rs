use crate::{CompileToolErrors, RuntimeVariables, utility::copy_file};
use std::fs;

/// _
/// # Errors
/// _
pub fn handle_alt_style_directories(
    runtime_vars: &RuntimeVariables,
) -> Result<(), CompileToolErrors> {
    // if we don't use alternate style or copy-paste files from somewhere else - do nothing.
    if !runtime_vars.compile_options.alt_directories
        || runtime_vars.compile_options.sources_are_somewhere_else
    {
        return Ok(());
    }

    let classes_folder = &runtime_vars
        .compiled_paths
        .compile_dir_sources
        .join("Classes");

    if classes_folder.exists() {
        fs::remove_dir_all(classes_folder)?;
    }

    Ok(())
}

/// _
/// # Errors
/// _
pub fn remove_copied_sources(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if runtime_vars.compile_options.sources_are_somewhere_else {
        fs::remove_dir_all(&runtime_vars.compiled_paths.compile_dir_sources)?;
    }
    Ok(())
}

/// _
/// # Errors
/// _
pub fn copy_files_to_another_kf(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if let Some(kf_dir) = &runtime_vars.path_where_to_copy {
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
            &runtime_vars.compiled_paths.path_package_u,
            &system.join(&runtime_vars.compiled_paths.name_package_u),
        );
        copy_file(
            &runtime_vars.compiled_paths.path_package_ucl,
            &system.join(&runtime_vars.compiled_paths.name_package_ucl),
        );
        copy_file(
            &runtime_vars.compiled_paths.path_package_int,
            &system.join(&runtime_vars.compiled_paths.name_package_int),
        );
    }

    Ok(())
}

/// _
/// # Errors
/// _
pub fn move_redirect_file(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if !runtime_vars.compile_options.make_redirect {
        return Ok(());
    }

    // move redirect file to the proper folder
    // create the folder if doesn't exist
    let redirect_dir = &runtime_vars.compiled_paths.compile_dir_redirect;
    if !redirect_dir.exists() {
        fs::create_dir(redirect_dir)?;
    }
    fs::copy(
        &runtime_vars.compiled_paths.path_package_uz2_init,
        &runtime_vars.compiled_paths.path_package_uz2,
    )?;
    fs::remove_file(&runtime_vars.compiled_paths.path_package_uz2_init)?;

    Ok(())
}

/// _
/// # Errors
/// _
pub fn remove_kf_ini(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if runtime_vars.compiled_paths.temp_kf_ini.exists() {
        fs::remove_file(&runtime_vars.compiled_paths.temp_kf_ini)?;
    }
    Ok(())
}

/// _
/// # Errors
/// _
#[allow(clippy::permissions_set_readonly_false)]
pub fn remove_steam_appid(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    // steamappid.txt
    if runtime_vars.compiled_paths.temp_steam_appid.exists() {
        let steamapp_id = runtime_vars.compiled_paths.temp_steam_appid.as_ref();

        let metadata = fs::metadata(steamapp_id)?;
        let mut permissions = metadata.permissions();
        permissions.set_readonly(false);
        fs::set_permissions(steamapp_id, permissions)?;
        fs::remove_file(steamapp_id)?;
    }

    Ok(())
}

/// _
/// # Errors
/// _
pub fn cleanup_leftover_files(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    remove_kf_ini(runtime_vars)?;
    remove_steam_appid(runtime_vars)?;
    remove_copied_sources(runtime_vars)?;
    handle_alt_style_directories(runtime_vars)?;
    Ok(())
}

/// _
/// # Errors
/// _
pub fn run(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    // 1. Clean garbage that we've created
    cleanup_leftover_files(runtime_vars)?;
    // 2. Move redirect file to redirect folder
    // move_redirect_file(runtime_vars)?;
    // 3. Move files to another kf dir
    copy_files_to_another_kf(runtime_vars)?;
    Ok(())
}
