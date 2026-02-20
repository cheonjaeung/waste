use crate::error::WasteError;
use crate::trash::TrashManager;
use std::path::Path;

pub struct LinuxTrashManager;

impl TrashManager for LinuxTrashManager {
    fn move_to_trash(_path: &Path) -> Result<(), WasteError> {
        unimplemented!("Linux trash implementation is coming soon")
    }
}
