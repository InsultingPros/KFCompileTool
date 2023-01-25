#[derive(thiserror::Error, Debug)]
pub enum CompileToolErrors {
    // #[error("Some IO error?")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    WriteError(#[from] std::fmt::Error),
}
