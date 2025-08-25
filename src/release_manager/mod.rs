use crate::{
    RuntimeVariables,
    errors::CompileToolErrors,
    release_manager::zip_extension::zip_create_from_directory_with_options,
    utility::{copy_file_if_exists, print_fancy_block},
};
use std::{
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};
use tempfile::NamedTempFile;
use zip::{CompressionMethod, write::SimpleFileOptions};

pub mod zip_extension;

#[derive(Debug)]
pub struct ReleaseOptions {
    /// zip's file name
    pub zip_name: String,
    /// the output location
    pub path_output: PathBuf,
    /// output folder for this exact mod: "C:\\Users\\Pepe User\\Desktop\\Mutators\\`MY_MOD`"
    pub path_mod: PathBuf,
    /// "C:\\Users\\Pepe User\\Desktop\\Mutators\\`MY_MOD`\\System"
    pub path_system: PathBuf,
    /// "C:\\Users\\Pepe User\\Desktop\\Mutators\\`MY_MOD`\\Redirect"
    pub path_redirect: PathBuf,
}

impl ReleaseOptions {
    fn new(runtime_vars: &RuntimeVariables) -> Self {
        let zip_name: String = format!("{}.zip", runtime_vars.mod_settings.package_name);
        let path_output: PathBuf =
            PathBuf::from(runtime_vars.paths.output_location.as_ref().unwrap());
        let path_mod: PathBuf = path_output.join(&runtime_vars.mod_settings.package_name);
        let path_system: PathBuf = path_mod.join("System");
        let path_redirect: PathBuf = path_mod.join("Redirect");

        Self {
            zip_name,
            path_output,
            path_mod,
            path_system,
            path_redirect,
        }
    }
}

/// _
/// # Errors
/// _
/// # Panics
/// _
pub fn prepare_release_folders(
    runtime_vars: &RuntimeVariables,
    release_options: &ReleaseOptions,
) -> Result<(), CompileToolErrors> {
    // Create output directory if none
    if !release_options.path_output.exists() {
        fs::create_dir(&release_options.path_output)?;
    }
    // if there is a mod folder - clean it up
    if release_options.path_mod.exists() {
        fs::remove_dir_all(&release_options.path_mod)?;
    }
    // and create it again
    fs::create_dir(&release_options.path_mod)?;
    // create the system folder
    fs::create_dir(&release_options.path_system)?;
    // and redirect folder, if required
    if runtime_vars.mod_settings.make_redirect {
        fs::create_dir(&release_options.path_redirect)?;
    }

    Ok(())
}

/// _
/// # Errors
/// _
/// # Panics
/// _
pub fn compose_release(
    runtime_vars: &RuntimeVariables,
    release_options: &ReleaseOptions,
) -> Result<(), CompileToolErrors> {
    // move files
    copy_file_if_exists(
        &runtime_vars.paths.path_package_u,
        &release_options
            .path_system
            .join(&runtime_vars.paths.name_package_u),
    )?;
    copy_file_if_exists(
        &runtime_vars.paths.path_package_ucl,
        &release_options
            .path_system
            .join(&runtime_vars.paths.name_package_ucl),
    )?;
    copy_file_if_exists(
        &runtime_vars.paths.path_package_int,
        &release_options
            .path_system
            .join(&runtime_vars.paths.name_package_int),
    )?;
    if runtime_vars.mod_settings.make_redirect {
        copy_file_if_exists(
            &runtime_vars.paths.path_package_uz2,
            &release_options
                .path_redirect
                .join(&runtime_vars.paths.name_package_uz2),
        )?;
    }

    Ok(())
}

/// _
/// # Errors
/// _
/// # Panics
/// _
pub fn compress_release_folder(release_options: &ReleaseOptions) -> Result<(), CompileToolErrors> {
    if release_options.path_mod.read_dir()?.next().is_none() {
        return Err(CompileToolErrors::IOError(Error::new(
            ErrorKind::NotFound,
            format!(
                "Release folder `{}` is empty! Aborting zip creation.",
                release_options.path_mod.display()
            ),
        )));
    }

    let zip_options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    let tmp_release_zip = NamedTempFile::new()?;

    zip_create_from_directory_with_options(
        &tmp_release_zip.path().to_path_buf(),
        &release_options.path_mod,
        |_| zip_options,
    )?;

    std::fs::copy(
        tmp_release_zip,
        release_options.path_mod.join(&release_options.zip_name),
    )?;
    Ok(())
}

/// _
/// # Errors
/// _
/// # Panics
/// _
pub fn make_release(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    // make release only if requested
    if !runtime_vars.mod_settings.make_release {
        return Ok(());
    }

    if runtime_vars.paths.output_location.is_none() {
        return Err(CompileToolErrors::StringErrors(format!(
            "You try to make a release for {}, but you didn't specify `dir_ReleaseOutput` variable in config file!",
            runtime_vars.mod_settings.package_name
        )));
    }

    print_fancy_block("Create Release");
    let release_options = &ReleaseOptions::new(runtime_vars);
    // dbg!(release_options);

    // cleanup remnants from older compilations or make one from sratch
    prepare_release_folders(runtime_vars, release_options)?;
    // create the release
    compose_release(runtime_vars, release_options)?;
    // compress folder if required
    compress_release_folder(release_options)?;

    Ok(())
}
