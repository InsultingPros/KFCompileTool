use kf_compile_tool::constants::exit_codes;
use kf_compile_tool::utility::create_runtime_vars;
use kf_compile_tool::{config_helper, ucc_wrapper};
use std::path::PathBuf;
use std::{
    process::ExitCode,
    time::{Duration, Instant},
};

fn main() -> ExitCode {
    let now: Instant = Instant::now();

    // 1. check internal config file
    config_helper::internal_config_exists().unwrap_or_else(|e| {
        eprintln!("Terminated with error: {}", e);
        std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
    });
    let elapsed: Duration = now.elapsed();
    println!("> #1 Elapsed: {:.2?}", elapsed);

    // 2. read internal config file
    let (global_section, local_section) = match config_helper::parse_internal_config() {
        Ok(result) => (result.0, result.1),
        Err(e) => {
            eprintln!("Terminated with error: {}", e);
            std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
        }
    };
    let elapsed: Duration = now.elapsed();
    println!("> #2 Elapsed: {:.2?}", elapsed);

    // 3. create `kfcompile.ini`
    let edit_packages: Vec<String> = local_section
        .edit_packages
        .split(',')
        .map(|s| s.to_string())
        .collect();
    config_helper::create_kf_config(&edit_packages).unwrap_or_else(|e| {
        eprintln!("Terminated with error: {}", e);
        std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
    });
    let elapsed: Duration = now.elapsed();
    println!("> #3 Elapsed: {:.2?}", elapsed);

    // 4. check compile dir
    ucc_wrapper::correct_compile_directory(&global_section.dir_compile).unwrap_or_else(|e| {
        eprintln!("Terminated with error: {}", e);
        std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
    });
    let elapsed: Duration = now.elapsed();
    println!("> #4 Elapsed: {:.2?}", elapsed);

    // test
    let x = create_runtime_vars(global_section, local_section);
    println!("{:?}", x);

    // 5. start compilation
    ucc_wrapper::start_compilation(x.compiled_paths.path_ucc).unwrap_or_else(|e| {
        eprintln!("Terminated with error: {}", e);
        std::process::exit(i32::from(exit_codes::ERROR_CANNOT_MAKE));
    });
    let elapsed: Duration = now.elapsed();
    println!("> #5 Elapsed: {:.2?}", elapsed);

    ExitCode::from(exit_codes::ERROR_SUCCESS)
}
