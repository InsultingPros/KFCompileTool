use crate::{
    RuntimeVariables, SourcesCopied,
    errors::CompileToolErrors,
    utility::{copy_file, source_folder_conflict_resolver},
};
use std::fs;

/// _
/// # Errors
/// _
pub fn remove_kf_ini(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if runtime_vars.paths.temp_kf_ini.try_exists()? {
        fs::remove_file(&runtime_vars.paths.temp_kf_ini)?;
    }
    Ok(())
}

/// _
/// # Errors
/// _
#[allow(clippy::permissions_set_readonly_false)]
pub fn remove_steam_appid(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    // steamappid.txt
    if runtime_vars.paths.temp_steam_appid.try_exists()? {
        let steamapp_id = runtime_vars.paths.temp_steam_appid.as_ref();

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
pub fn remove_copied_sources(runtime_vars: &mut RuntimeVariables) -> Result<(), CompileToolErrors> {
    if runtime_vars.sources_copied_state == SourcesCopied::FromExternalDir {
        fs::remove_dir_all(&runtime_vars.paths.compile_dir_sources)?;
    }

    if runtime_vars.sources_copied_state == SourcesCopied::Ignored {
        return Ok(());
    }

    if runtime_vars.mod_settings.sources_are_somewhere_else
        && runtime_vars.paths.compile_dir_sources.try_exists()?
    {
        source_folder_conflict_resolver(runtime_vars);
    }

    Ok(())
}

/// _
/// # Errors
/// _
pub fn handle_alt_style_directories(
    runtime_vars: &RuntimeVariables,
) -> Result<(), CompileToolErrors> {
    // if we don't use alternate style or copy-paste files from somewhere else - do nothing.
    if !runtime_vars.mod_settings.alt_directories
        || runtime_vars.mod_settings.sources_are_somewhere_else
    {
        return Ok(());
    }

    let classes_folder = &runtime_vars.paths.compile_dir_sources.join("Classes");

    if classes_folder.exists() {
        fs::remove_dir_all(classes_folder)?;
    }

    Ok(())
}

/// _
/// # Errors
/// _
pub fn cleanup_leftover_files(
    runtime_vars: &mut RuntimeVariables,
) -> Result<(), CompileToolErrors> {
    // 1. remove temp kfcompile.ini
    remove_kf_ini(runtime_vars)?;
    // 2. remove temp steam_appid.txt
    remove_steam_appid(runtime_vars)?;
    // 3. handle alt style source code organization
    handle_alt_style_directories(runtime_vars)?;
    // 4. handle external sources dir case
    remove_copied_sources(runtime_vars)?;

    Ok(())
}

/// _
/// # Errors
/// _
pub fn copy_files_to_another_kf(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
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
    // 1. Clean garbage that we've created
    cleanup_leftover_files(runtime_vars)?;
    // 2. Move files to another kf dir
    copy_files_to_another_kf(runtime_vars)?;

    Ok(())
}
