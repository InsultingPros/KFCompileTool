use crate::{
    RuntimeVariables, SourcesCopied,
    constants::ALT_SOURCE_DIR_NAME,
    errors::CompileToolErrors,
    operations::{kf_config::create_kf_config, steam_appid::create_hacky_steamappid},
    utils::{
        conflict_resolver::source_folder_conflict_resolver,
        io::{copy_directory, copy_file, delete_file, get_walkdir_iterator},
    },
};
use std::{
    fs,
    io::{Error, ErrorKind},
};

/// # Errors
///
/// Will return `Err` if `filename` does not exist or the user does not have
/// permission to read it.s
pub fn validate_compile_directory(
    runtime_vars: &RuntimeVariables,
) -> Result<(), CompileToolErrors> {
    if !runtime_vars.paths.compile_dir.try_exists()? {
        return Err(CompileToolErrors::IOError(Error::new(
            ErrorKind::NotFound,
            format!(
                "path `{}` doesn't exist!",
                runtime_vars.paths.compile_dir.display()
            ),
        )));
    }
    if !runtime_vars.paths.ucc_exe.try_exists()? {
        return Err(CompileToolErrors::IOError(Error::new(
            ErrorKind::NotFound,
            format!("{} not found!", runtime_vars.paths.ucc_exe.display()),
        )));
    }

    Ok(())
}

/// _
/// # Errors
/// _
pub fn check_alt_style_feature(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if !runtime_vars.mod_settings.alt_directories {
        return Ok(());
    }
    let classes_folder = &runtime_vars.mod_settings.path_source_files.join("Classes");
    if classes_folder.exists() {
        return Err(CompileToolErrors::IOError(Error::new(
            ErrorKind::NotFound,
            format!(
                "You have `Classes` folder `{}` in your mod, but you want to compile with `alt_directories` key.\n\
                Decide which one to leave and edit the app config. Aborting compilation!",
                classes_folder.display()
            ),
        )));
    }

    let sources_folder = &runtime_vars
        .mod_settings
        .path_source_files
        .join(ALT_SOURCE_DIR_NAME);
    if !sources_folder.exists() {
        return Err(CompileToolErrors::IOError(Error::new(
            ErrorKind::NotFound,
            format!(
                "You've set `alt_directories` key, but `Source` folder `{}` doesn't exist.\n\
                Aborting compilation!",
                sources_folder.display()
            ),
        )));
    }
    Ok(())
}

/// _
/// # Errors
/// _
pub fn move_sources_to_compile_dir(
    runtime_vars: &mut RuntimeVariables,
) -> Result<(), CompileToolErrors> {
    // check if we even need this to execute
    if !runtime_vars.mod_settings.sources_are_somewhere_else {
        return Ok(());
    }
    // Does source folder even exist?
    let alternative_location = runtime_vars.mod_settings.path_source_files.clone();
    if !alternative_location.try_exists()? {
        return Err(CompileToolErrors::IOError(Error::new(
            ErrorKind::NotFound,
            format!(
                "Source folder `{}` is empty. Aborting compilation!",
                alternative_location.display()
            ),
        )));
    }
    // warn if people want to use source files from another directory, but the same source folder exists in compile_dir
    // when we were automatically deleting the later one, some people who didn't read the documentation were a bit angry :v
    let default_location = runtime_vars.paths.compile_dir_sources.clone();
    if default_location.exists() {
        eprintln!(
            "You've set `bICompileOutsideofKF` key in config file, but there is a duplicate {} mod folder in compilation directory {}!\n\
            Decide what folder do you want to use, and edit that config entry.",
            runtime_vars.mod_settings.package_name,
            runtime_vars.paths.compile_dir.display()
        );
        source_folder_conflict_resolver(runtime_vars);
    }

    if runtime_vars.sources_copied_state != SourcesCopied::Ignored {
        // set the flag, for later safe delete
        runtime_vars.sources_copied_state = SourcesCopied::FromExternalDir;
        // now copy-paste the new files from alternative dir
        copy_directory(&alternative_location, &default_location)?;
    }

    Ok(())
}

/// _
/// # Errors
/// _
pub fn handle_alt_style_directories(
    runtime_vars: &RuntimeVariables,
) -> Result<(), CompileToolErrors> {
    if !runtime_vars.mod_settings.alt_directories {
        return Ok(());
    }

    // skip checks, we did them earlier
    let classes_folder = &runtime_vars.paths.compile_dir_sources.join("Classes");
    let sources_folder = &runtime_vars
        .paths
        .compile_dir_sources
        .join(ALT_SOURCE_DIR_NAME);

    fs::create_dir(classes_folder)?;
    let source_contents = get_walkdir_iterator(sources_folder)?
        .into_iter()
        .filter(|p| p.path().is_file());
    for entry in source_contents {
        let path = &entry.into_path();
        copy_file(
            path,
            &classes_folder.join(path.file_name().unwrap_or_default()),
        );
    }

    Ok(())
}

/// _
/// # Errors
/// _
pub fn delete_old_binaries(runtime_vars: &RuntimeVariables) {
    delete_file(&runtime_vars.paths.path_package_u);
    delete_file(&runtime_vars.paths.path_package_ucl);
    delete_file(&runtime_vars.paths.path_package_int);
}

/// _
/// # Errors
/// _
pub fn run(runtime_vars: &mut RuntimeVariables) -> Result<(), CompileToolErrors> {
    // 1. Check if compile dir even exists
    validate_compile_directory(runtime_vars)?;
    // 2. Check the alternative dir style even before copying
    check_alt_style_feature(runtime_vars)?;
    // 3. Check where are the source files, and where to copy them
    move_sources_to_compile_dir(runtime_vars)?;
    // 4. Cover altrnative source code organization
    handle_alt_style_directories(runtime_vars)?;
    // 5. Delete older binary files before
    delete_old_binaries(runtime_vars);
    // 6. Create temporary kfcompile.ini
    create_kf_config(runtime_vars)?;
    // 7. Create temporary steam_appid.txt
    create_hacky_steamappid(runtime_vars)?;

    Ok(())
}
