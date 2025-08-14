use kf_compile_tool::stages::run;
use kf_compile_tool::{RuntimeVariables, app_config::parse_app_config, cli::MyOptions};
use std::process::ExitCode;

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
