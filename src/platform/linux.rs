use crate::error::WasteError;
use crate::trash::TrashManager;
use std::path::Path;

/// Linux implementation of the TrashManager.
///
/// This manager follows the FreeDesktop.org Trash specification to move files to the system trash.
pub struct LinuxTrashManager;

impl TrashManager for LinuxTrashManager {
    /// Moves a file or directory to the Linux trash.
    fn move_to_trash(_path: &Path) -> Result<(), WasteError> {
        unimplemented!("Linux trash implementation is coming soon")
    }
}
