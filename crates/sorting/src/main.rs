use color_eyre::eyre::Result;

pub mod algorithm;
mod cli;
pub mod utils;

fn main() -> Result<()> {
    color_eyre::install()?;
    cli::run()?;
    Ok(())
}
