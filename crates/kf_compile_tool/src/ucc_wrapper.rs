use crate::constants::COMPILATION_CONFIG_NAME;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn correct_compile_directory(location: &str) -> Result<bool, String> {
    println!("Input: {}", location);
    let input_path: PathBuf = PathBuf::from(location);

    if !input_path.exists() {
        return Ok(false);
    }

    let input_path = input_path.join("System\\ucc.exe");
    println!("local path ucc: {:?}", input_path);
    println!("local ucc.exe exists: {:?}", input_path.try_exists());

    Ok(true)
}

/// Actual compilation process
/// - Consumes UCC.exe path.
/// - Start a `Command` with given arguments
///     - Call ucc's `make` commandlet.
///     - Pass our custom, compilation config (kfcompile.ini).
///     - Pass `-EXPORTCACHE` to reliably create `ucl` files.
// Source: https://rust-lang-nursery.github.io/rust-cookbook/os/external.html?highlight=stdout#continuously-process-child-process-outputs
pub fn start_compilation(ucc_path: PathBuf) -> Result<(), Error> {
    let ucc_exe = Command::new(ucc_path)
        .stdout(Stdio::piped())
        .arg("make")
        .arg(format!("ini={}", COMPILATION_CONFIG_NAME))
        .arg("-EXPORTCACHE")
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;

    // create a BufReader to show the stdout in real time
    let reader = BufReader::new(ucc_exe);
    // show the output in real time
    reader
        .lines()
        .map_while(Result::ok)
        // .filter(|line| line.find("usb").is_some())
        .for_each(|line| println!("{}", line));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_dir() {
        // using this path for tests
        let server = "D:\\Games\\KF Dedicated Server";

        println!("Test result {:?}", correct_compile_directory(server));
    }

    #[test]
    fn test_compilation() {
        let ucc_path: PathBuf = PathBuf::from("D:\\Games\\KF Dedicated Server\\System\\ucc.exe");
        println!("Test result {:?}", start_compilation(ucc_path));
    }
}
