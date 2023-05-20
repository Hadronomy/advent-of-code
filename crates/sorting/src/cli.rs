//! Contains the cli definition

use clap::Parser;
use eyre::Result;

// TODO: Override default help

/// Description
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Enables debugging
    #[arg(short, long)]
    debug: bool,
}

/// Parses the command line arguments,
/// and executes the matching subcommand
pub fn run() -> Result<Cli> {
    let _cli = Cli::parse();
    todo!();
}
