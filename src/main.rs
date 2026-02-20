mod cli;
mod error;
mod platform;
mod trash;

use crate::cli::Cli;
use crate::platform::CurrentPlatformManager;
use crate::trash::TrashManager;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let mut exit_code = 0;

    for path in cli.paths {
        if path.symlink_metadata().is_err() {
            eprintln!("Error: File or directory not found: {:?}", path);
            exit_code = 1;
            continue;
        }

        if let Err(e) = CurrentPlatformManager::move_to_trash(&path) {
            eprintln!("Error moving {:?} to trash: {}", path, e);
            exit_code = 1;
        } else {
            println!("Moved to trash: {:?}", path);
        }
    }

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
