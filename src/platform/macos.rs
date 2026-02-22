use crate::error::WasteError;
use crate::trash::TrashManager;
use objc2_foundation::{NSFileManager, NSString, NSURL};
use std::path::Path;

/// macOS implementation of the TrashManager.
///
/// This manager uses Apple's Foundation framework to move files to the system trash.
pub struct MacosTrashManager;

impl TrashManager for MacosTrashManager {
    /// Moves a file or directory to the macOS trash.
    fn move_to_trash(path: &Path) -> Result<(), WasteError> {
        let path_str = path
            .to_str()
            .ok_or_else(|| WasteError::MetadataError("Invalid UTF-8 in path".into()))?;

        let ns_string = NSString::from_str(path_str);
        let ns_url = NSURL::fileURLWithPath(&ns_string);
        let ns_file_manager = NSFileManager::defaultManager();

        let result = ns_file_manager.trashItemAtURL_resultingItemURL_error(&ns_url, None);

        match result {
            Ok(()) => Ok(()),
            Err(error) => {
                let err_desc = error.localizedDescription().to_string();
                Err(WasteError::PlatformError(format!(
                    "macOS Trash error: {}",
                    err_desc
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_move_to_trash_file() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("waste_test_file.txt");
        {
            let mut file = File::create(&file_path).expect("Failed to create test file");
            writeln!(file, "Test content for waste CLI").expect("Failed to write to test file");
        }
        assert!(file_path.exists());

        let result = MacosTrashManager::move_to_trash(&file_path);
        assert!(
            result.is_ok(),
            "Failed to move file to trash: {:?}",
            result.err()
        );

        assert!(
            !file_path.exists(),
            "File still exists at original path after moving to trash"
        );
    }

    #[test]
    fn test_move_to_trash_nonexistent_file() {
        let nonexistent_path = Path::new("/tmp/this_file_definitely_does_not_exist_waste_cli");
        let result = MacosTrashManager::move_to_trash(nonexistent_path);
        assert!(result.is_err());
    }
}
