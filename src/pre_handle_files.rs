use crate::{
    ALT_SOURCE_DIR_NAME, CompileToolErrors, RuntimeVariables,
    utility::{copy_directory, copy_file, delete_file, get_walkdir_iterator},
};
use std::{
    fs,
    io::{Error, ErrorKind},
};

/// _
/// # Errors
/// _
pub fn move_sources_to_compile_dir(
    runtime_vars: &RuntimeVariables,
) -> Result<(), CompileToolErrors> {
    // check if we even need this to execute
    if !runtime_vars.compile_options.sources_are_somewhere_else {
        return Ok(());
    }
    // Does source folder even exist?
    let out_sources = &runtime_vars.compile_options.path_source_files;
    if !out_sources.try_exists()? {
        return Err(CompileToolErrors::IOError(Error::new(
            ErrorKind::NotFound,
            format!(
                "Source folder `{}` is empty. Aborting compilation!",
                out_sources.display()
            ),
        )));
    }
    // warn if people want to use source files from another directory, but the same source folder exists in compile_dir
    // when we were automatically deleting the later one, some people who didn't read the documentation were a bit angry :v
    let in_sources = &runtime_vars.compiled_paths.compile_dir_sources;
    if in_sources.exists() {
        return Err(CompileToolErrors::IOError(Error::new(
            ErrorKind::AlreadyExists,
            format!(
                "You've set `bICompileOutsideofKF` key in config file, but there is a duplicate {} mod folder in compilation directory {}! Decide what folder do you want to use, and edit that config entry.",
                runtime_vars.compile_options.package_name,
                runtime_vars.compiled_paths.compile_dir.display()
            ),
        )));
    }

    // else delete remnants of mod folder, if exist
    copy_directory(out_sources, in_sources)?;
    // and copy-paste new files from source dir

    Ok(())
}

/// _
/// # Errors
/// _
pub fn delete_old_binaries(runtime_vars: &RuntimeVariables) {
    delete_file(&runtime_vars.compiled_paths.path_package_u);
    delete_file(&runtime_vars.compiled_paths.path_package_ucl);
    delete_file(&runtime_vars.compiled_paths.path_package_int);
}

/// _
/// # Errors
/// _
pub fn check_alt_style_feature(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if !runtime_vars.compile_options.alt_directories {
        return Ok(());
    }
    let classes_folder = &runtime_vars
        .compile_options
        .path_source_files
        .join("Classes");
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
        .compile_options
        .path_source_files
        .join(ALT_SOURCE_DIR_NAME);
    if !sources_folder.exists() {
        return Err(CompileToolErrors::IOError(Error::new(
            ErrorKind::NotFound,
            format!(
                "Source folder `{}` doesn't exist. Aborting compilation!",
                sources_folder.display()
            ),
        )));
    }
    Ok(())
}

/// _
/// # Errors
/// _
pub fn handle_alt_style_directories(
    runtime_vars: &RuntimeVariables,
) -> Result<(), CompileToolErrors> {
    if !runtime_vars.compile_options.alt_directories {
        return Ok(());
    }

    // skip checks, we did them earlier
    let classes_folder = &runtime_vars
        .compiled_paths
        .compile_dir_sources
        .join("Classes");
    let sources_folder = &runtime_vars
        .compiled_paths
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
pub fn run(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    // . Check the alternative dir style even before copying
    check_alt_style_feature(runtime_vars)?;
    // 1. Check where are the source files, and where to copy them
    move_sources_to_compile_dir(runtime_vars)?;
    // 2. Cover altrnative source code organization
    handle_alt_style_directories(runtime_vars)?;
    // 3. Delete older binary files before
    delete_old_binaries(runtime_vars);
    Ok(())
}
