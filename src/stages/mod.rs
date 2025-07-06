pub mod post_pass;
pub mod pre_pass;
pub mod ucc;
pub mod uz2;

use crate::{RuntimeVariables, errors::CompileToolErrors, release_manager};
use std::time::Instant;

/// _
/// # Errors
/// _
pub fn run(runtime_vars: &mut RuntimeVariables) -> Result<(), CompileToolErrors> {
    let now: Instant = Instant::now();

    // All checks and processing BEFORE compilations step
    pre_pass::run(runtime_vars)?;

    // 1. Start the unrealscript compilation
    // 2. int file
    ucc::run(runtime_vars)?;

    // uz2 file
    if let Err(e) = uz2::make_uz2(runtime_vars) {
        eprintln!(
            "Error while creating redirect file: {e}.\n\
            Skipping this step."
        );
    }

    // All checks and processing AFTER compilations step
    post_pass::run(runtime_vars)?;

    // make a release folder / zip
    // if failed do not discard everything
    if let Err(e) = release_manager::make_release(runtime_vars) {
        eprintln!(
            "Error while creating release: {e}.\n\
            Skipping this step."
        );
    }

    println!("> Elapsed: {:.2?}", now.elapsed());
    Ok(())
}
