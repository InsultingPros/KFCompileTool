use kf_compile_tool::{
    RuntimeVariables, cli::MyOptions, config_manager::app_config::parse_app_config,
    errors::CompileToolErrors, post_pass::cleanup_leftover_files,
};
use std::{process::ExitCode, time::Instant};

fn run(runtime_vars: &mut RuntimeVariables) -> Result<(), CompileToolErrors> {
    let now: Instant = Instant::now();

    // All checks and processing BEFORE compilations step
    kf_compile_tool::pre_pass::run(runtime_vars)?;

    // Start the unrealscript compilation
    kf_compile_tool::ucc_wrapper::ucc_compile(runtime_vars)?;

    // int file
    kf_compile_tool::ucc_wrapper::ucc_dumpint(runtime_vars)?;

    // uz2 file
    if let Err(e) = kf_compile_tool::compressor::make_uz2(runtime_vars) {
        eprintln!(
            "Error while creating redirect file: {e}.\n\
            Skipping this step."
        );
    }

    // All checks and processing AFTER compilations step
    kf_compile_tool::post_pass::run(runtime_vars)?;

    // make a release folder / zip
    // if failed do not discard everything
    if let Err(e) = kf_compile_tool::release_manager::make_release(runtime_vars) {
        eprintln!(
            "Error while creating release: {e}.\n\
            Skipping this step."
        );
    }

    println!("> Elapsed: {:.2?}", now.elapsed());
    Ok(())
}

#[cfg(target_os = "windows")]
fn main() -> ExitCode {
    let env_arguments: MyOptions = gumdrop::Options::parse_args_default_or_exit();
    let mut runtime_vars: RuntimeVariables = match parse_app_config(&env_arguments) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Terminated with error: {e}");
            press_enter(&env_arguments);
            return ExitCode::FAILURE;
        }
    };

    match run(&mut runtime_vars) {
        Ok(()) => {
            press_enter(&env_arguments);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Terminated with error: {e}");
            if let Err(e) = cleanup_leftover_files(&mut runtime_vars) {
                eprintln!("failed to cleanup leftover files: {e}");
            }
            press_enter(&env_arguments);
            ExitCode::FAILURE
        }
    }
}

fn press_enter(env_arguments: &MyOptions) {
    if !env_arguments.hold {
        return;
    }

    println!("Press ENTER to continue...");
    std::io::stdin()
        .read_line(&mut String::new())
        .expect("error: unable to read user input");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    compile_error!("This code only compiles on Windows.");
}
