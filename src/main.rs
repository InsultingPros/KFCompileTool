use kf_compile_tool::cli::MyOptions;
use kf_compile_tool::{
    CompileToolErrors, RuntimeVariables,
    config_manager::{
        app_config::parse_app_config, kf_config::create_kf_config,
        steam_appid::create_hacky_steamappid,
    },
    post_pass::cleanup_leftover_files,
    release_manager::make_release,
    ucc_wrapper,
};
use kfuz2_lib::helper::try_to_compress;
use kfuz2_lib::types::InputArguments;
use std::{process::ExitCode, time::Instant};

fn run(runtime_vars: &RuntimeVariables) -> Result<(), CompileToolErrors> {
    let now: Instant = Instant::now();
    let mut counter: usize = 0usize;

    // _
    ucc_wrapper::validate_compile_directory(runtime_vars)?;
    counter += 1;
    println!(
        "> #{} Check compile dir. Elapsed: {:.2?}",
        counter,
        now.elapsed()
    );

    // _
    match kf_compile_tool::pre_pass::run(runtime_vars) {
        Ok(()) => {}
        // if an error happens, than we can't even cleanup at this stage
        // better exit early so we won't remove source files by accident
        Err(e) => {
            eprintln!("Terminated with error: {e}");
            std::process::exit(0);
        }
    }
    counter += 1;
    println!(
        "> #{} Handle all files BEFORE compilation. Elapsed: {:.2?}",
        counter,
        now.elapsed()
    );

    // _
    create_kf_config(runtime_vars)?;
    counter += 1;
    println!(
        "> #{} Create `kfcompile.ini`. Elapsed: {:.2?}",
        counter,
        now.elapsed()
    );

    // _
    create_hacky_steamappid(runtime_vars)?;
    counter += 1;
    println!(
        "> #{} Create hacky steam_appid.txt. Elapsed: {:.2?}",
        counter,
        now.elapsed()
    );

    // _
    ucc_wrapper::ucc_compile(runtime_vars)?;
    counter += 1;
    println!(
        "> #{} Start compilation. Elapsed: {:.2?}",
        counter,
        now.elapsed()
    );

    // _
    ucc_wrapper::ucc_dumpint(runtime_vars)?;
    counter += 1;
    println!(
        "> #{} Create localization file. Elapsed: {:.2?}",
        counter,
        now.elapsed()
    );

    // _
    let mut input_arguments = InputArguments {
        input_path: runtime_vars.compiled_paths.path_package_u.clone(),
        output_path: runtime_vars.compiled_paths.compile_dir_redirect.clone(),
        log_level: kfuz2_lib::types::LogLevel::Default,
        ignore_kf_files: true,
    };
    try_to_compress(&mut input_arguments).unwrap_or_else(|e| {
        eprintln!("Terminated with error: {e}. Skipping redirect file creation.");
    });

    counter += 1;
    println!(
        "> #{} Create redirect file. Elapsed: {:.2?}",
        counter,
        now.elapsed()
    );

    // _
    kf_compile_tool::post_pass::run(runtime_vars)?;
    counter += 1;
    println!(
        "> #{} Handle all files AFTER compilation. Elapsed: {:.2?}",
        counter,
        now.elapsed()
    );

    // _
    make_release(runtime_vars)?;
    counter += 1;
    println!(
        "> #{} Create Release. Elapsed: {:.2?}",
        counter,
        now.elapsed()
    );

    Ok(())
}

#[cfg(target_os = "windows")]
fn main() -> ExitCode {
    let env_arguments: MyOptions = gumdrop::Options::parse_args_default_or_exit();
    let runtime_vars: RuntimeVariables = match parse_app_config(&env_arguments) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Terminated with error: {e}");
            return ExitCode::FAILURE;
        }
    };

    match run(&runtime_vars) {
        Ok(()) => {
            if env_arguments.hold {
                println!("Press ENTER to continue...");
                std::io::stdin()
                    .read_line(&mut String::new())
                    .expect("error: unable to read user input");
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Terminated with error: {e}");
            if let Err(e) = cleanup_leftover_files(&runtime_vars) {
                eprintln!("failed to cleanup leftover files: {e}");
            }
            ExitCode::FAILURE
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    compile_error!("This code only compiles on Windows.");
}
