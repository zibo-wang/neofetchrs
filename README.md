# Neofetch-rs

A fast, highly customizable system information tool written in Rust, inspired by the original [neofetch](https://github.com/dylanaraps/neofetch).

## Features

- **Cross-platform**: Works on Linux, macOS, and Windows
- **Fast**: Written in Rust for optimal performance
- **Customizable**: Extensive configuration options
- **Multiple output formats**: Standard display, JSON, and stdout-only modes
- **ASCII art**: Displays OS-specific logos with colored output
- **Package manager detection**: Supports multiple package managers (apt, pacman, brew, etc.)
- **Comprehensive system info**: CPU, memory, kernel, uptime, packages, and more

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd neofetch

# Build and install
cargo build --release
cargo install --path .
```

### Using Cargo

```bash
cargo install neofetch-rs
```

## Usage

### Basic Usage

```bash
# Display system information with ASCII art
neofetch-rs

# Display help
neofetch-rs --help

# Display version
neofetch-rs --version
```

### Output Formats

```bash
# JSON output
neofetch-rs --json

# Plain text output (no ASCII art)
neofetch-rs --stdout

# Verbose output
neofetch-rs --verbose
```

### Customization Options

```bash
# Hide/show specific information
neofetch-rs --title-fqdn on
neofetch-rs --package-managers tiny
neofetch-rs --memory-unit gib

# ASCII art options
neofetch-rs --ascii ubuntu
neofetch-rs --ascii-bold off
neofetch-rs --backend ascii

# Color options
neofetch-rs --color-blocks off
neofetch-rs --block-width 4
```

## Configuration

Neofetch-rs supports configuration files in TOML format. The configuration file is located at:

- Linux/macOS: `~/.config/neofetch/config.toml`
- Windows: `%APPDATA%\neofetch\config.toml`

### Example Configuration

```toml
[info]
title_fqdn = false
package_managers = "on"
memory_unit = "gib"
bold = true

[display]
image_backend = "ascii"
ascii_bold = true
gap = 3

[format]
color_blocks = true
block_width = 3
```

## Supported Systems

### Operating Systems
- Linux (all major distributions)
- macOS
- Windows
- BSD variants

### Package Managers
- **Linux**: APT (Debian/Ubuntu), Pacman (Arch), RPM (Red Hat/Fedora), Flatpak, Snap
- **macOS**: Homebrew, MacPorts
- **Windows**: (planned support for Chocolatey, Scoop)

## Comparison with Original Neofetch

| Feature           | Original Neofetch | Neofetch-rs    |
| ----------------- | ----------------- | -------------- |
| Language          | Bash              | Rust           |
| Performance       | Good              | Excellent      |
| Memory Usage      | ~20MB             | ~5MB           |
| Startup Time      | ~200ms            | ~50ms          |
| Cross-platform    | Limited           | Full           |
| JSON Output       | Yes               | Yes            |
| Configuration     | Bash script       | TOML file      |
| Package Detection | Extensive         | Good (growing) |

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run with specific options
cargo run -- --json
```

### Project Structure

```
src/
├── main.rs          # Entry point
├── lib.rs           # Library root
├── cli.rs           # Command-line interface
├── config.rs        # Configuration management
├── system_info.rs   # System information gathering
├── ascii_art.rs     # ASCII art and logos
├── output.rs        # Output formatting
└── utils.rs         # Utility functions
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

- Original [neofetch](https://github.com/dylanaraps/neofetch) by Dylan Araps
- The Rust community for excellent crates and tools
- All contributors and users of the project

## Roadmap

- [ ] Enhanced GPU detection
- [ ] More package manager support
- [ ] Custom ASCII art support
- [ ] Plugin system
- [ ] Performance optimizations
- [ ] Windows-specific improvements
- [ ] Configuration migration from original neofetch
