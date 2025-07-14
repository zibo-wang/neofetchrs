use anyhow::Result;
use neofetch_rs::{cli, Neofetch};

fn main() -> Result<()> {
    // Parse command-line arguments
    let config = cli::parse_args()?;

    // Handle special cases
    if config.behavior.verbose {
        println!("Neofetch-rs v7.1.0");
        println!("Configuration loaded successfully");
    }

    // Create and run neofetch
    let mut neofetch = Neofetch::new(config)?;
    neofetch.run()?;

    Ok(())
}
