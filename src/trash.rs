use crate::error::WasteError;
use std::path::Path;

pub trait TrashManager {
    fn move_to_trash(path: &Path) -> Result<(), WasteError>;
}
