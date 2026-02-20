use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WasteError {
    #[error("File or directory not found: {0}")]
    NotFound(PathBuf),

    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("Platform specific error: {0}")]
    PlatformSpecific(String),

    #[error("Metadata error: {0}")]
    MetadataError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
