use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "waste",
    version,
    about = "Move files and directories to the trash"
)]
pub struct Cli {
    /// Files or directories to move to trash
    #[arg(required = true, value_name = "PATH")]
    pub paths: Vec<PathBuf>,
}
