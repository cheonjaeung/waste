use crate::cli::Cli;
use crate::error::WasteError;
use crate::platform::CurrentPlatformManager;
use crate::trash::TrashManager;

/// Runs the main application, taking CLI arguments and moving specified paths to the trash.
/// Returns the exit code.
pub fn run(cli: Cli) -> i32 {
    let mut exit_code = 0;

    for path in cli.paths {
        if !path.exists() && !path.is_symlink() {
            eprintln!("[ERROR] {}", WasteError::NotFound(path));
            exit_code = 1;
            continue;
        }

        if let Err(e) = CurrentPlatformManager::move_to_trash(&path) {
            eprintln!("[ERROR] {}", e);
            exit_code = 1;
        } else if cli.verbose {
            println!("Successfully moved '{}' to the trash.", path.display());
        }
    }

    exit_code
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;

    #[test]
    fn test_run_with_file() {
        let temp_dir = std::env::temp_dir().join("waste_test_run_with_file");
        fs::create_dir_all(&temp_dir).unwrap();
        let file_path = temp_dir.join("test.txt");
        {
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "test data").unwrap();
        }

        let cli = Cli {
            paths: vec![file_path.clone()],
            verbose: false,
        };

        let exit_code = run(cli);
        assert_eq!(exit_code, 0);
        assert!(!file_path.exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_run_with_nonexistent_file() {
        let nonexistent = PathBuf::from("/tmp/waste_test_run_with_nonexistent_file");
        let cli = Cli {
            paths: vec![nonexistent],
            verbose: false,
        };

        let exit_code = run(cli);
        assert_eq!(exit_code, 1);
    }

    #[test]
    fn test_run_with_multiple_files() {
        let temp_dir = std::env::temp_dir().join("waste_test_run_with_multiple_files");
        fs::create_dir_all(&temp_dir).unwrap();
        let file1_path = temp_dir.join("test1.txt");
        let file2_path = temp_dir.join("test2.txt");
        let file3_path = temp_dir.join("test3.txt");
        {
            let mut file1 = File::create(&file1_path).unwrap();
            let mut file2 = File::create(&file2_path).unwrap();
            let mut file3 = File::create(&file3_path).unwrap();
            writeln!(file1, "test data").unwrap();
            writeln!(file2, "test data").unwrap();
            writeln!(file3, "test data").unwrap();
        }

        let cli = Cli {
            paths: vec![file1_path.clone(), file2_path.clone(), file3_path.clone()],
            verbose: false,
        };

        let exit_code = run(cli);
        assert_eq!(exit_code, 0);
        assert!(!file1_path.exists());
        assert!(!file2_path.exists());
        assert!(!file3_path.exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_run_with_multiple_files_one_missing() {
        let temp_dir = std::env::temp_dir().join("waste_test_run_with_multiple_files_one_missing");
        fs::create_dir_all(&temp_dir).unwrap();
        let file1_path = temp_dir.join("test1.txt");
        let file2_path = temp_dir.join("test2.txt");
        let nonexistent_path = temp_dir.join("nonexistent_test.txt");
        {
            let mut file1 = File::create(&file1_path).unwrap();
            let mut file2 = File::create(&file2_path).unwrap();
            writeln!(file1, "test data").unwrap();
            writeln!(file2, "test data").unwrap();
        }

        let cli = Cli {
            paths: vec![
                file1_path.clone(),
                file2_path.clone(),
                nonexistent_path.clone(),
            ],
            verbose: false,
        };

        let exit_code = run(cli);
        assert_eq!(exit_code, 1);
        assert!(!file1_path.exists());
        assert!(!file2_path.exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_run_with_directory() {
        let temp_dir = std::env::temp_dir().join("waste_test_run_with_directory");
        fs::create_dir_all(&temp_dir).unwrap();
        let dir_path = temp_dir.join("testdir");
        fs::create_dir_all(&dir_path).unwrap();
        let file_path = dir_path.join("test.txt");
        {
            let mut file = File::create(&file_path).unwrap();
            writeln!(file, "inner data").unwrap();
        }

        let cli = Cli {
            paths: vec![dir_path.clone()],
            verbose: true,
        };

        let exit_code = run(cli);
        assert_eq!(exit_code, 0);
        assert!(!dir_path.exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_run_with_nonexistent_directory() {
        let nonexistent_dir = PathBuf::from("/tmp/test_run_with_nonexistent_directory");
        let cli = Cli {
            paths: vec![nonexistent_dir],
            verbose: false,
        };

        let exit_code = run(cli);
        assert_eq!(exit_code, 1);
    }

    #[test]
    #[cfg(unix)]
    fn test_run_with_symlink() {
        use std::os::unix::fs::symlink;
        let temp_dir = std::env::temp_dir().join("waste_test_run_with_symlink");
        fs::create_dir_all(&temp_dir).unwrap();
        let target_path = temp_dir.join("test_symlink_target.txt");
        let link_path = temp_dir.join("test_symlink.txt");

        {
            let mut file = File::create(&target_path).unwrap();
            writeln!(file, "target data").unwrap();
        }

        let _ = fs::remove_file(&link_path);
        symlink(&target_path, &link_path).unwrap();

        let cli = Cli {
            paths: vec![link_path.clone()],
            verbose: false,
        };

        let exit_code = run(cli);
        assert_eq!(exit_code, 0);

        assert!(!link_path.exists());
        assert!(target_path.exists());

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
