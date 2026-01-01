use crate::{RuntimeVariables, SourcesCopied};
use std::fs;

/// Which folder to delete
/// # Panics
/// _
pub fn source_folder_conflict_resolver(runtime_variables: &mut RuntimeVariables) {
    println!(
        "Enter `d` to delete the internal sources folder - `{}`\n\
        Or `I` to ignore this message and compile as is.\n\
        If you input anything else or an empty string - process will be automatically closed.",
        runtime_variables.paths.compile_dir_sources.display()
    );
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    if input.ends_with('\n') {
        input.pop();
    }
    if input.ends_with('\r') {
        input.pop();
    }
    // dbg!(&input);

    if input.to_lowercase() == "d" {
        if let Err(e) = fs::remove_dir_all(&runtime_variables.paths.compile_dir_sources) {
            eprintln!("Error while trying to resolve source folder conflict: {e}");
        }
        return;
    }
    if input.to_lowercase() == "i" {
        runtime_variables.sources_copied_state = SourcesCopied::Ignored;
        return;
    }

    // for all other cases - close the process
    eprintln!(
        "Terminated with error: Input was empty / incorrect.\n\
        Decide which source files folder do you want to keep and remove them manually."
    );
    std::process::exit(0);
}
