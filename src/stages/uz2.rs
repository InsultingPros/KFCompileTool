use crate::RuntimeVariables;
use crate::errors::CompileToolErrors;
use crate::utils::print_fancy_block;
use kfuz2_lib::helper::{PathChecks, try_to_compress};
use kfuz2_lib::types::InputArguments;
use std::fs;

/// _
/// # Errors
/// _
pub fn make_uz2(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if !runtime_vars.mod_settings.make_redirect {
        return Ok(());
    }

    print_fancy_block("Create redirect file");

    if let Some(mut redirect) = runtime_vars.paths.path_redirect.as_ref() {
        if redirect.get_file_name().is_none() {
            eprint!(
                "`dir_Redirect` is not specified in your config file!\n\
                Creating a `Redirect` folder in your compilation directory: {}\n",
                runtime_vars.paths.compile_dir_redirect.display()
            );

            redirect.clone_from(&&runtime_vars.paths.compile_dir_redirect);
        }

        if !redirect.try_exists()? {
            println!(
                "Specified `dir_Redirect` `{}` is not found!\n\
                Trying to create it for you.\n",
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
