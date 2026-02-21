use crate::error::WasteError;
use std::path::Path;

/// A trait that defines the interface for moving files and directories to the system trash.
///
/// This trait is implemented by each platform-specific manager (macOS, Linux, Windows)
/// to provide a consistent way to handle trash operations across different operating systems.
pub trait TrashManager {
    /// Moves the specified file or directory to the system's trash or recycle bin.
    ///
    /// # Arguments
    ///
    /// * `path` - A reference to the path of the file or directory to be moved.
    ///
    /// # Errors
    ///
    /// Returns a `WasteError` when the operation failed.
    fn move_to_trash(path: &Path) -> Result<(), WasteError>;
}
