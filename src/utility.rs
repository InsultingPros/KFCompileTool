use crate::CompileToolErrors;
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};

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
