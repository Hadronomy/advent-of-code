use color_eyre::eyre::Result;

mod cli;
mod algorithm;

fn main() -> Result<()> {
    color_eyre::install()?;
    cli::run()?;
    Ok(())
}
