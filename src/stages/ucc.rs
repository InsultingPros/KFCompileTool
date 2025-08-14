use crate::traits::kf_config::COMPILATION_CONFIG_NAME;
use crate::utility::print_fancy_block;
use crate::{RuntimeVariables, errors::CompileToolErrors};
use std::io::{BufRead, BufReader, Error};
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
fn compile(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    print_fancy_block("Start compilation");

    let mut child = Command::new(runtime_vars.paths.ucc_exe.as_ref())
        .stdout(Stdio::piped())
        .arg("make")
        .arg(format!("ini={COMPILATION_CONFIG_NAME}"))
        .arg("-EXPORTCACHE")
        .spawn()?;

    // Get stdout before waiting
    let ucc_exe = child
        .stdout
        .take()
        .ok_or_else(|| Error::other("Could not capture standard output."))?;

    print_stdout(ucc_exe);

    // Wait for the process to complete and get the status
    let status = child.wait()?;

    if !status.success() {
        let exit_code = status
            .code()
            .map_or_else(|| "terminated by signal".to_string(), |c| c.to_string());
        return Err(CompileToolErrors::from(Error::other(format!(
            "UCC compilation failed with exit code: {exit_code}"
        ))));
    }

    Ok(())
}

fn dumpint(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    if !runtime_vars.mod_settings.create_int {
        return Ok(());
    }

    print_fancy_block("Create localization file");

    let mut child = Command::new("cmd")
        .current_dir(runtime_vars.paths.compile_dir_system.as_path())
        .stdout(Stdio::piped())
        .arg("/C")
        .arg("UCC.exe")
        .arg("dumpint")
        .arg(&runtime_vars.paths.name_package_u)
        .spawn()?;

    // Get stdout before waiting
    let ucc_exe = child
        .stdout
        .take()
        .ok_or_else(|| Error::other("Could not capture standard output."))?;

    print_stdout(ucc_exe);

    // Wait for the process to complete and get the status
    let status = child.wait()?;

    if !status.success() {
        let exit_code = status
            .code()
            .map_or_else(|| "terminated by signal".to_string(), |c| c.to_string());
        return Err(CompileToolErrors::from(Error::other(format!(
            "UCC dumpint failed with exit code: {exit_code}"
        ))));
    }

    Ok(())
}

fn print_stdout(child: ChildStdout) {
    // create a BufReader to show the stdout in real time
    let reader = BufReader::new(child);
    // show the output in real time
    reader
        .lines()
        .map_while(Result::ok)
        .for_each(|line| println!("{line}"));
}

/// _
/// # Errors
/// _
pub fn run(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    compile(runtime_vars)?;
    dumpint(runtime_vars)?;

    Ok(())
}
