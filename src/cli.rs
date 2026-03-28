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
    #[arg(value_name = "PATH", required_unless_present = "list")]
    pub paths: Vec<PathBuf>,

    /// Print items in the trash
    #[arg(short = 'l', long = "list", conflicts_with = "paths")]
    pub list: bool,

    /// Show more information about the operation
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,
}
