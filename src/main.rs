mod cli;
mod error;
mod platform;
mod trash;

use crate::cli::Cli;
use crate::error::WasteError;
use crate::platform::CurrentPlatformManager;
use crate::trash::TrashManager;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
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
        }
    }

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
