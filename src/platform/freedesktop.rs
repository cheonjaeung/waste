use crate::error::WasteError;
use crate::trash::TrashItem;
use crate::trash::TrashManager;
use std::path::Path;

/// Linux implementation of the TrashManager.
///
/// This manager follows the FreeDesktop.org Trash specification to move files to the system trash.
pub struct FreeDesktopTrashManager;

impl TrashManager for FreeDesktopTrashManager {
    /// Moves a file or directory to the Linux trash.
    fn move_to_trash(_path: &Path) -> Result<(), WasteError> {
        unimplemented!("Linux trash implementation is coming soon")
    }

    /// Lists the items currently in the system's trash or recycle bin.
    fn list_trash() -> Result<Vec<TrashItem>, WasteError> {
        unimplemented!("Linux trash list implementation is coming soon")
    }
}
