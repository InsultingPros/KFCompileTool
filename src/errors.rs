use kfuz2_lib::errors::UZ2LibErrors;
use std::path::StripPrefixError;

#[derive(thiserror::Error, Debug)]
pub enum CompileToolErrors {
    // #[error("Some IO error?")]
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    WriteError(#[from] std::fmt::Error),
    #[error("{0}")]
    StringErrors(String),
    #[error(transparent)]
    ZipErrors(#[from] zip::result::ZipError),
    #[error(transparent)]
    WalkDirErrors(#[from] walkdir::Error),
    #[error(transparent)]
    PathErrors(#[from] StripPrefixError),
    #[error(transparent)]
    Uz2LibErrors(#[from] UZ2LibErrors),
}
