use color_eyre::eyre::Result;

mod cli;
pub mod algorithm;

fn main() -> Result<()> {
    color_eyre::install()?;
    cli::run()?;
    Ok(())
}
