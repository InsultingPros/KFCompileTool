use std::fs;
use std::io::{Error, ErrorKind};

use crate::RuntimeVariables;
use crate::errors::CompileToolErrors;
use crate::utility::print_fancy_block;
use kfuz2_lib::helper::{PathChecks, try_to_compress};
use kfuz2_lib::types::InputArguments;

/// _
/// # Errors
/// _
pub fn make_uz2(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if !runtime_vars.mod_settings.make_redirect {
        return Ok(());
    }

    print_fancy_block("Create redirect file");
    if let Some(redirect) = &runtime_vars.paths.path_redirect {
        if redirect.get_file_name().is_none() {
            return Err(CompileToolErrors::IOError(Error::new(
                ErrorKind::NotFound,
                "`dir_Redirect` is not specified in your config file!",
            )));
        }
        if !redirect.try_exists()? {
            println!(
                "Specified `dir_Redirect` `{}` is not found!\n\
                Trying to create it for you.",
                redirect.display()
            );
            fs::create_dir(redirect)?;
        }

        let mut input_arguments = InputArguments {
            input_path: runtime_vars.paths.path_package_u.clone(),
            output_path: redirect.clone(),
            log_level: kfuz2_lib::types::LogLevel::Default,
            ignore_kf_files: true,
        };
        try_to_compress(&mut input_arguments)?;
    }

    Ok(())
}
