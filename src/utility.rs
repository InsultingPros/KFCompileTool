use crate::{RuntimeVariables, SourcesCopied, errors::CompileToolErrors};
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

const LINE_SEPARATOR: &str = "\n######################################################\n";

pub fn print_fancy_block(input: &str) {
    println!("{LINE_SEPARATOR}{input}{LINE_SEPARATOR}");
}

/// —
/// # Errors
/// —
pub fn get_walkdir_iterator(
    input_path: &PathBuf,
) -> Result<impl IntoIterator<Item = DirEntry>, CompileToolErrors> {
    fn is_hidden(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .is_some_and(|s| s.starts_with('.'))
    }

    let walker = WalkDir::new(input_path).into_iter();
    let result_iter = walker.filter_entry(|e| !is_hidden(e)).flatten();

    Ok(result_iter)
}

/// _
/// # Errors
/// _
pub fn copy_directory(source: &PathBuf, destination: &Path) -> Result<(), CompileToolErrors> {
    let source_content = get_walkdir_iterator(source)?;

    for entry in source_content {
        let from = entry.path();
        let to = destination.join(from.strip_prefix(source)?);
        // println!("\tcopy {} => {}", from.display(), to.display());

        // create directories
        if entry.file_type().is_dir() {
            if let Err(e) = fs::create_dir(to) {
                match e.kind() {
                    ErrorKind::AlreadyExists => {}
                    _ => return Err(e.into()),
                }
            }
        }
        // copy files
        else if entry.file_type().is_file() {
            fs::copy(from, to)?;
        }
        // ignore the rest
        else {
            eprintln!("copy: ignored symlink {}", from.display());
        }
    }
    Ok(())
}

/// _
/// # Errors
/// _
pub fn delete_file(file: &PathBuf) {
    if !file.exists() {
        return;
    }

    if let Err(e) = fs::remove_file(file) {
        eprintln!(
            "Error while removing previous compilation file {}: {}",
            file.display(),
            e
        );
    }
}

/// _
/// # Errors
/// _
pub fn copy_file(from: &PathBuf, to: &PathBuf) {
    if !from.exists() {
        return;
    }

    if let Err(e) = fs::copy(from, to) {
        eprintln!(
            "Error while trying to copy {} to {}: {}",
            from.display(),
            to.display(),
            e
        );
    }
}

/// _
/// # Errors
/// _
pub fn copy_file_if_exists(from: &PathBuf, to: &Path) -> Result<(), CompileToolErrors> {
    if from.exists() {
        fs::copy(from, to)?;
        println!("Copied {} ==> {}!", from.display(), to.display());
    } else {
        println!("{} doesn't exist!", from.display());
    }

    Ok(())
}

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

#[derive(Debug)]
pub enum FileEditPermission {
    ReadOnly,
    Write,
}

impl FileEditPermission {
    /// Returns true if the permission is readonly
    const fn is_readonly(&self) -> bool {
        matches!(self, Self::ReadOnly)
    }
}

/// _
/// # Errors
/// _
pub fn set_file_readonly<P: AsRef<Path>>(
    file: P,
    permission: &FileEditPermission,
) -> Result<(), CompileToolErrors> {
    let file_path = file.as_ref();

    let metadata = fs::metadata(file_path).map_err(|e| {
        eprintln!(
            "Failed to retrieve metadata from file {}",
            file_path.display()
        );
        CompileToolErrors::IOError(e)
    })?;

    let mut permissions = metadata.permissions();
    permissions.set_readonly(permission.is_readonly());

    fs::set_permissions(file_path, permissions).map_err(|e| {
        eprintln!(
            "Failed to set {permission:?} permission for file {}",
            file_path.display()
        );
        CompileToolErrors::IOError(e)
    })?;

    Ok(())
}
