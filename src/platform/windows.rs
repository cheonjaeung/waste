use crate::error::WasteError;
use crate::trash::TrashManager;
use std::path::Path;

/// Windows implementation of the TrashManager.
///
/// This manager uses the Windows Shell API to move files to the Recycle Bin.
pub struct WindowsTrashManager;

impl TrashManager for WindowsTrashManager {
    /// Moves a file or directory to the Windows Recycle Bin.
    fn move_to_trash(_path: &Path) -> Result<(), WasteError> {
        unimplemented!("Windows trash implementation is coming soon")
    }
}
