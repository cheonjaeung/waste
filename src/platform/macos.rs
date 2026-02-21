use crate::error::WasteError;
use crate::trash::TrashManager;
use std::path::Path;

/// macOS implementation of the TrashManager.
///
/// This manager uses Apple's Foundation framework to move files to the system trash.
pub struct MacosTrashManager;

impl TrashManager for MacosTrashManager {
    /// Moves a file or directory to the macOS trash.
    fn move_to_trash(_path: &Path) -> Result<(), WasteError> {
        unimplemented!("macOS trash implementation is coming soon")
    }
}
