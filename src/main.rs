mod app;
mod cli;
mod error;
mod platform;
mod trash;

use crate::app::run;
use crate::cli::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let exit_code = run(cli);

    if exit_code != 0 {
        std::process::exit(exit_code);
    }
}
