//! Neofetch-rs: A fast, highly customizable system info script written in Rust
//! 
//! This is a Rust rewrite of the popular neofetch system information tool.
//! It provides detailed system information in a visually appealing format
//! with ASCII art logos for various operating systems and distributions.

pub mod config;
pub mod system_info;
pub mod ascii_art;
pub mod output;
pub mod cli;
pub mod utils;

use anyhow::Result;
use config::Config;
use system_info::SystemInfo;

/// Main application structure
pub struct Neofetch {
    config: Config,
    system_info: SystemInfo,
}

impl Neofetch {
    /// Create a new Neofetch instance with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        let system_info = SystemInfo::new()?;
        Ok(Self {
            config,
            system_info,
        })
    }

    /// Run the neofetch application
    pub fn run(&mut self) -> Result<()> {
        // Gather system information
        self.system_info.gather_all(&self.config)?;
        
        // Generate and display output
        let output = output::generate_output(&self.system_info, &self.config)?;
        println!("{}", output);
        
        Ok(())
    }
}
