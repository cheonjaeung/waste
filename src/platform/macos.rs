use crate::error::WasteError;
use crate::trash::TrashManager;
use std::path::Path;

pub struct MacosTrashManager;

impl TrashManager for MacosTrashManager {
    fn move_to_trash(_path: &Path) -> Result<(), WasteError> {
        unimplemented!("macOS trash implementation is coming soon")
    }
}
