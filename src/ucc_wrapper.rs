use crate::config_manager::kf_config::COMPILATION_CONFIG_NAME;
use crate::{CompileToolErrors, RuntimeVariables};
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{ChildStdout, Command, Stdio};

// Reference: https://rust-lang-nursery.github.io/rust-cookbook/os/external.html?highlight=stdout#continuously-process-child-process-outputs
/// Actual compilation process
/// - Consumes UCC.exe path.
/// - Start a `Command` with given arguments
///     - Call ucc's `make` commandlet.
///     - Pass our custom, compilation config (kfcompile.ini).
///     - Pass `-EXPORTCACHE` to reliably create `ucl` files.
/// # Errors
///
/// Will return `Err` if `filename` does not exist or the user does not have
/// permission to read it.
pub fn ucc_compile(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    let ucc_exe = Command::new(runtime_vars.compiled_paths.ucc_exe.as_ref())
        .stdout(Stdio::piped())
        .arg("make")
        .arg(format!("ini={COMPILATION_CONFIG_NAME}"))
        .arg("-EXPORTCACHE")
        .spawn()?
        .stdout
        .ok_or_else(|| Error::other("Could not capture standard output."))?;

    print_stdout(ucc_exe);
    Ok(())
}

/// _
/// # Errors
/// _
pub fn ucc_dumpint(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if !runtime_vars.compile_options.create_int {
        return Ok(());
    }

    let ucc_exe = Command::new("cmd")
        .current_dir(runtime_vars.compiled_paths.compile_dir_system.as_path())
        .stdout(Stdio::piped())
        .arg("/C")
        .arg("UCC.exe")
        .arg("dumpint")
        .arg(&runtime_vars.compiled_paths.name_package_u)
        .spawn()?
        .stdout
        .ok_or_else(|| Error::other("Could not capture standard output."))?;

    print_stdout(ucc_exe);
    Ok(())
}

pub fn print_stdout(child: ChildStdout) {
    // create a BufReader to show the stdout in real time
    let reader = BufReader::new(child);
    // show the output in real time
    reader
        .lines()
        .map_while(Result::ok)
        .for_each(|line| println!("{line}"));
}

/// # Errors
///
/// Will return `Err` if `filename` does not exist or the user does not have
/// permission to read it.s
pub fn validate_compile_directory(
    runtime_vars: &RuntimeVariables,
) -> Result<bool, CompileToolErrors> {
    if !runtime_vars.compiled_paths.compile_dir.try_exists()? {
        return Err(CompileToolErrors::IOError(Error::new(
            ErrorKind::NotFound,
            format!(
                "path `{}` doesn't exist!",
                runtime_vars.compiled_paths.compile_dir.display()
            ),
        )));
    }

    // dbg!(
    //     &runtime_vars.compiled_paths.ucc_exe,
    //     runtime_vars.compiled_paths.ucc_exe.try_exists()?
    // );

    Ok(true)
}
