use crate::{CompileToolErrors, RuntimeVariables, utility::copy_file_if_exists};
use std::{
    fs,
    io::{Error, ErrorKind},
};
use zip::{CompressionMethod, write::SimpleFileOptions};
use zip_extensions::zip_create_from_directory_with_options;

/// _
/// # Errors
/// _
/// # Panics
/// _
pub fn compress_release_folder(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    let release_folder = &runtime_vars.release_options.as_ref().unwrap().path_mod;
    let zip_file_name = &runtime_vars.release_options.as_ref().unwrap().zip_name;

    let release_zip = &runtime_vars
        .release_options
        .as_ref()
        .unwrap()
        .path_root
        .join(zip_file_name);

    // dbg!(release_folder, release_zip, &zip_file_name);
    if release_folder.read_dir()?.next().is_none() {
        return Err(CompileToolErrors::IOError(Error::new(
            ErrorKind::NotFound,
            format!(
                "Release folder `{}` is empty! Aborting zip creation.",
                release_folder.display()
            ),
        )));
    }

    let zip_options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    zip_create_from_directory_with_options(release_zip, release_folder, |_| zip_options)?;

    std::fs::copy(release_zip, release_folder.join(zip_file_name))?;
    std::fs::remove_file(release_zip)?;
    Ok(())
}

/// _
/// # Errors
/// _
/// # Panics
/// _
pub fn prepare_release_folder(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    let output_dir = &runtime_vars.release_options.as_ref().unwrap().path_root;
    if !output_dir.exists() {
        fs::create_dir(output_dir)?;
    }
    // dbg!(output_dir);

    let release_folder = &runtime_vars.release_options.as_ref().unwrap().path_mod;
    if release_folder.exists() {
        fs::remove_dir_all(release_folder)?;
    }
    // dbg!(&release_folder);

    fs::create_dir(release_folder)?;
    Ok(())
}

/// _
/// # Errors
/// _
/// # Panics
/// _
pub fn compose_release_folder(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    let system = &runtime_vars.release_options.as_ref().unwrap().path_system;
    if !system.try_exists()? {
        fs::create_dir(system)?;
    }

    // move files
    copy_file_if_exists(
        &runtime_vars.compiled_paths.path_package_u,
        &system.join(&runtime_vars.compiled_paths.name_package_u),
    )?;
    copy_file_if_exists(
        &runtime_vars.compiled_paths.path_package_ucl,
        &system.join(&runtime_vars.compiled_paths.name_package_ucl),
    )?;
    copy_file_if_exists(
        &runtime_vars.compiled_paths.path_package_int,
        &system.join(&runtime_vars.compiled_paths.name_package_int),
    )?;

    if runtime_vars.compile_options.make_redirect {
        let redirect = &runtime_vars.release_options.as_ref().unwrap().path_redirect;
        if !redirect.try_exists()? {
            fs::create_dir(redirect)?;
        }

        copy_file_if_exists(
            &runtime_vars.compiled_paths.path_package_uz2,
            &redirect.join(&runtime_vars.compiled_paths.name_package_uz2),
        )?;
    }

    Ok(())
}

/// _
/// # Errors
/// _
/// # Panics
/// _
pub fn make_release(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    // make release only if requested
    if runtime_vars.release_options.is_none() {
        return Ok(());
    }

    // create / cleanup remnants from older compilations
    prepare_release_folder(runtime_vars)?;
    compose_release_folder(runtime_vars)?;

    // compress folder if required
    if let Some(options) = &runtime_vars.release_options {
        if options.make_zip {
            // println!("compressing!");
            compress_release_folder(runtime_vars)?;
        }
    }

    Ok(())
}
