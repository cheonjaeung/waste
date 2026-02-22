use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur when using the waste.
#[derive(Error, Debug)]
pub enum WasteError {
    /// The file or directory was not found.
    #[error("File or directory not found: '{0}'")]
    NotFound(PathBuf),

    /// Permission was denied when accessing the file or directory.
    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    /// Error occurred within the platform-specific trash implementation.
    #[error("Platform specific error: {0}")]
    PlatformError(String),

    /// File metadata processing was failed.
    #[error("Metadata error: {0}")]
    MetadataError(String),

    /// General I/O error occurred.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
