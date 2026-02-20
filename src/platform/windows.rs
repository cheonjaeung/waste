use crate::error::WasteError;
use crate::trash::TrashManager;
use std::path::Path;

pub struct WindowsTrashManager;

impl TrashManager for WindowsTrashManager {
    fn move_to_trash(_path: &Path) -> Result<(), WasteError> {
        unimplemented!("Windows trash implementation is coming soon")
    }
}
