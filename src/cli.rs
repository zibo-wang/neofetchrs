//! Command-line interface for neofetch-rs
//!
//! This module handles command-line argument parsing and configuration.

use crate::config::*;
use anyhow::Result;
use clap::{Arg, ArgAction, Command};

/// Parse command-line arguments and return a configuration
pub fn parse_args() -> Result<Config> {
    let matches = Command::new("neofetch-rs")
        .version("7.1.0")
        .author("Zibo Wang <zibo.w@outlook.com>")
        .about("A fast, highly customizable system info script written in Rust")
        .arg(
            Arg::new("config")
                .long("config")
                .value_name("FILE")
                .help("Specify a custom config file")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("no_config")
                .long("no-config")
                .help("Don't load any config file")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("stdout")
                .long("stdout")
                .help("Turn off all colors and disables image backend")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("json")
                .long("json")
                .help("Output system information in JSON format")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Display verbose output")
                .action(ArgAction::SetTrue),
        )
        // Info options
        .arg(
            Arg::new("title_fqdn")
                .long("title-fqdn")
                .value_name("BOOL")
                .help("Hide/Show Fully qualified domain name")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("package_managers")
                .long("package-managers")
                .value_name("MODE")
                .help("Show/Hide Package Manager names")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("os_arch")
                .long("os-arch")
                .value_name("BOOL")
                .help("Hide/Show OS Architecture")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("cpu_cores")
                .long("cpu-cores")
                .value_name("TYPE")
                .help("Display CPU cores")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("cpu_speed")
                .long("cpu-speed")
                .value_name("BOOL")
                .help("Hide/Show cpu speed")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("speed_type")
                .long("speed-type")
                .value_name("TYPE")
                .help("Change the type of cpu speed to display")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("distro_shorthand")
                .long("distro-shorthand")
                .value_name("MODE")
                .help("Shorten the output of distro")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("kernel_shorthand")
                .long("kernel-shorthand")
                .value_name("BOOL")
                .help("Shorten the output of kernel")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("uptime_shorthand")
                .long("uptime-shorthand")
                .value_name("MODE")
                .help("Shorten the output of uptime")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("shell_path")
                .long("shell-path")
                .value_name("BOOL")
                .help("Show the path to $SHELL")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("shell_version")
                .long("shell-version")
                .value_name("BOOL")
                .help("Show $SHELL version")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("memory_unit")
                .long("memory-unit")
                .value_name("UNIT")
                .help("Change memory output unit")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("memory_percent")
                .long("memory-percent")
                .value_name("BOOL")
                .help("Display memory percentage")
                .action(ArgAction::Set),
        )
        // Display options
        .arg(
            Arg::new("backend")
                .long("backend")
                .value_name("BACKEND")
                .help("Which image backend to use")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("source")
                .long("source")
                .value_name("SOURCE")
                .help("Which image or ascii file to use")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("ascii")
                .long("ascii")
                .value_name("DISTRO")
                .help("Shortcut to --backend ascii --source distro")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("ascii_colors")
                .long("ascii-colors")
                .value_name("COLORS")
                .help("Colors to print the ascii art")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("ascii_bold")
                .long("ascii-bold")
                .value_name("BOOL")
                .help("Whether or not to bold the ascii logo")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("logo")
                .short('L')
                .long("logo")
                .help("Hide the info text and only show the ascii logo")
                .action(ArgAction::SetTrue),
        )
        // Color options
        .arg(
            Arg::new("color_blocks")
                .long("color-blocks")
                .value_name("BOOL")
                .help("Enable/Disable the color blocks")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("block_range")
                .long("block-range")
                .value_name("RANGE")
                .help("Range of colors to print as blocks")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("block_width")
                .long("block-width")
                .value_name("NUM")
                .help("Width of the color blocks")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("block_height")
                .long("block-height")
                .value_name("NUM")
                .help("Height of the color blocks")
                .action(ArgAction::Set),
        )
        .get_matches();

    // Start with default config or load from file
    let mut config = if matches.get_flag("no_config") {
        Config::default()
    } else {
        Config::load().unwrap_or_default()
    };

    // Override with command-line arguments
    if let Some(config_file) = matches.get_one::<String>("config") {
        let config_str = std::fs::read_to_string(config_file)?;
        config = toml::from_str(&config_str)?;
    }

    if matches.get_flag("stdout") {
        config.display.stdout = true;
        config.behavior.stdout = true;
    }

    if matches.get_flag("json") {
        config.behavior.json = true;
    }

    if matches.get_flag("verbose") {
        config.behavior.verbose = true;
    }

    if matches.get_flag("logo") {
        config.display.image_backend = ImageBackend::Ascii;
        // Hide info text, only show logo
    }

    // Info options
    if let Some(value) = matches.get_one::<String>("title_fqdn") {
        config.info.title_fqdn = value.parse().unwrap_or(false);
    }

    if let Some(value) = matches.get_one::<String>("package_managers") {
        config.info.package_managers = match value.as_str() {
            "on" => PackageManagerDisplay::On,
            "tiny" => PackageManagerDisplay::Tiny,
            "off" => PackageManagerDisplay::Off,
            _ => PackageManagerDisplay::On,
        };
    }

    if let Some(value) = matches.get_one::<String>("os_arch") {
        config.info.os_arch = value.parse().unwrap_or(true);
    }

    if let Some(value) = matches.get_one::<String>("cpu_cores") {
        config.info.cpu_cores = match value.as_str() {
            "logical" => CpuCoreDisplay::Logical,
            "physical" => CpuCoreDisplay::Physical,
            "off" => CpuCoreDisplay::Off,
            _ => CpuCoreDisplay::Logical,
        };
    }

    if let Some(value) = matches.get_one::<String>("distro_shorthand") {
        config.info.distro_shorthand = match value.as_str() {
            "on" => DistroShorthand::On,
            "tiny" => DistroShorthand::Tiny,
            "off" => DistroShorthand::Off,
            _ => DistroShorthand::Off,
        };
    }

    if let Some(value) = matches.get_one::<String>("uptime_shorthand") {
        config.info.uptime_shorthand = match value.as_str() {
            "on" => UptimeShorthand::On,
            "tiny" => UptimeShorthand::Tiny,
            "off" => UptimeShorthand::Off,
            _ => UptimeShorthand::On,
        };
    }

    if let Some(value) = matches.get_one::<String>("memory_unit") {
        config.info.memory_unit = match value.as_str() {
            "kib" => MemoryUnit::Kib,
            "mib" => MemoryUnit::Mib,
            "gib" => MemoryUnit::Gib,
            _ => MemoryUnit::Mib,
        };
    }

    // Display options
    if let Some(value) = matches.get_one::<String>("backend") {
        config.display.image_backend = match value.as_str() {
            "ascii" => ImageBackend::Ascii,
            "caca" => ImageBackend::Caca,
            "catimg" => ImageBackend::Catimg,
            "chafa" => ImageBackend::Chafa,
            "jp2a" => ImageBackend::Jp2a,
            "iterm2" => ImageBackend::Iterm2,
            "off" => ImageBackend::Off,
            "pixterm" => ImageBackend::Pixterm,
            "sixel" => ImageBackend::Sixel,
            "termpix" => ImageBackend::Termpix,
            "tycat" => ImageBackend::Tycat,
            "w3m" => ImageBackend::W3m,
            "kitty" => ImageBackend::Kitty,
            "ueberzug" => ImageBackend::Ueberzug,
            "viu" => ImageBackend::Viu,
            _ => ImageBackend::Ascii,
        };
    }

    if let Some(value) = matches.get_one::<String>("ascii") {
        config.display.image_backend = ImageBackend::Ascii;
        config.display.ascii_distro = Some(value.clone());
    }

    if let Some(value) = matches.get_one::<String>("ascii_bold") {
        config.display.ascii_bold = value.parse().unwrap_or(true);
    }

    // Color options
    if let Some(value) = matches.get_one::<String>("color_blocks") {
        config.format.color_blocks = value.parse().unwrap_or(true);
    }

    if let Some(value) = matches.get_one::<String>("block_width") {
        config.format.block_width = value.parse().unwrap_or(3);
    }

    if let Some(value) = matches.get_one::<String>("block_height") {
        config.format.block_height = value.parse().unwrap_or(1);
    }

    Ok(config)
}
