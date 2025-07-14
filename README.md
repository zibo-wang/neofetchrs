# Neofetch-rs

A fast, highly customizable system information tool written in Rust, inspired by the original [neofetch](https://github.com/dylanaraps/neofetch).

## Table of Contents

- [Features](#features)
- [Installation](#installation)
  - [Quick Start](#quick-start)
  - [Installation Methods](#installation-methods)
  - [Build Options](#build-options)
- [Usage](#usage)
  - [Basic Usage](#basic-usage)
  - [Output Formats](#output-formats)
  - [Customization Options](#customization-options)
- [Configuration](#configuration)
- [Comprehensive Guides](#comprehensive-guides)
  - [🚀 Getting Started Guide](#-getting-started-guide)
  - [🔧 Build Guide](#-build-guide)
  - [📝 Configuration Guide](#-configuration-guide)
  - [🎨 Customization Guide](#-customization-guide)
  - [🔍 Troubleshooting Guide](#-troubleshooting-guide)
  - [📊 Performance Guide](#-performance-guide)
  - [🔧 Development Guide](#-development-guide)
- [Supported Systems](#supported-systems)
- [Comparison with Original Neofetch](#comparison-with-original-neofetch)
- [FAQ](#faq)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)
- [Roadmap](#roadmap)

## Features

- **Cross-platform**: Works on Linux, macOS, and Windows
- **Fast**: Written in Rust for optimal performance
- **Customizable**: Extensive configuration options
- **Multiple output formats**: Standard display, JSON, and stdout-only modes
- **ASCII art**: Displays OS-specific logos with colored output
- **Package manager detection**: Supports multiple package managers (apt, pacman, brew, etc.)
- **Comprehensive system info**: CPU, memory, kernel, uptime, packages, and more

## Installation

### Quick Start

The fastest way to get started is to build from source:

```bash
# Clone the repository
git clone https://github.com/zibo-wang/neofetchrs.git
cd neofetchrs

# Build optimized release version
make release

# Run the binary
./target/release/neofetch
```

### Installation Methods

#### 1. From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/zibo-wang/neofetchrs.git
cd neofetchrs

# Build and install system-wide
make install

# Or install to custom location
PREFIX=/usr/local make install
```

#### 2. Using Cargo

```bash
cargo install neofetch-rs
```

#### 3. Manual Build and Install

```bash
# Build optimized release version
cargo build --release

# Copy binary to your PATH
sudo cp target/release/neofetch /usr/local/bin/

# Make executable
sudo chmod +x /usr/local/bin/neofetch
```

### Build Options

| Command              | Description              | Binary Size | Performance |
| -------------------- | ------------------------ | ----------- | ----------- |
| `make build`         | Debug build with symbols | ~8MB        | Good        |
| `make release`       | Optimized release build  | ~1.3MB      | Excellent   |
| `make release-debug` | Release with debug info  | ~2MB        | Excellent   |

### Uninstallation

```bash
# If installed via make install
make uninstall

# If installed manually
sudo rm /usr/local/bin/neofetch
```

## Usage

### Basic Usage

```bash
# Display system information with ASCII art
neofetch

# Display help
neofetch --help

# Display version
neofetch --version
```

### Output Formats

```bash
# JSON output
neofetch --json

# Plain text output (no ASCII art)
neofetch --stdout

# Verbose output
neofetch --verbose
```

### Customization Options

```bash
# Hide/show specific information
neofetch --title-fqdn on
neofetch --package-managers tiny
neofetch --memory-unit gib

# ASCII art options
neofetch --ascii ubuntu
neofetch --ascii-bold off
neofetch --backend ascii

# Color options
neofetch --color-blocks off
neofetch --block-width 4
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

## Comprehensive Guides

### 🚀 Getting Started Guide

#### Step 1: Installation
```bash
# Clone and build
git clone https://github.com/zibo-wang/neofetchrs.git
cd neofetchrs
make release

# Test the build
./target/release/neofetch --version
```

#### Step 2: First Run
```bash
# Basic system info display
./target/release/neofetch

# Try different output formats
./target/release/neofetch --json
./target/release/neofetch --stdout
```

#### Step 3: System Installation
```bash
# Install system-wide
sudo make install

# Verify installation
which neofetch
neofetch --version
```

### 🔧 Build Guide

#### Development Build
```bash
# Fast compilation for development
make build
./target/debug/neofetch

# With verbose output for debugging
cargo build
RUST_LOG=debug ./target/debug/neofetch --verbose
```

#### Production Build
```bash
# Optimized release build (recommended)
make release

# Release with debug symbols (for profiling)
make release-debug

# Clean previous builds
make clean
```

#### Build Optimization Details
The release build includes:
- **Maximum optimization** (`-O3` equivalent)
- **Link-time optimization** (LTO) for better performance
- **Symbol stripping** for smaller binary size
- **Single codegen unit** for maximum optimization

### 📝 Configuration Guide

#### Creating Your First Config
```bash
# Create config directory
mkdir -p ~/.config/neofetch

# Create basic config file
cat > ~/.config/neofetch/config.toml << 'EOF'
[info]
title_fqdn = false
package_managers = "on"
memory_unit = "gib"

[display]
image_backend = "ascii"
ascii_bold = true

[format]
color_blocks = true
block_width = 3
EOF

# Test with custom config
neofetch --config ~/.config/neofetch/config.toml
```

#### Advanced Configuration Options
```toml
[info]
# System information options
title_fqdn = true              # Show full domain name
package_managers = "tiny"      # Show package manager names (on/off/tiny)
os_arch = true                 # Show OS architecture
cpu_cores = "logical"          # CPU core display (logical/physical)
cpu_speed = true               # Show CPU speed
speed_type = "max"             # Speed type (current/min/max)
kernel_shorthand = true        # Shorten kernel output
uptime_shorthand = "on"        # Uptime format (on/off/tiny)
shell_path = false             # Show shell path
shell_version = true           # Show shell version
memory_unit = "gib"            # Memory unit (kib/mib/gib/tib)
memory_percent = true          # Show memory percentage

[display]
# Visual display options
backend = "ascii"              # Image backend (ascii/off)
source = "auto"                # Image source (auto/distro/path)
ascii_bold = true              # Bold ASCII art
gap = 3                        # Gap between logo and info

[format]
# Output formatting
color_blocks = true            # Show color blocks
block_range = "0-15"           # Color range for blocks
block_width = 3                # Width of color blocks
block_height = 1               # Height of color blocks
```

### 🎨 Customization Guide

#### ASCII Art Customization
```bash
# Use specific distro ASCII art
neofetch --ascii ubuntu
neofetch --ascii arch
neofetch --ascii macos

# Disable ASCII art
neofetch --backend off

# Custom ASCII colors
neofetch --ascii-colors "4 6 1 8 8 6"

# Bold/non-bold ASCII
neofetch --ascii-bold on
neofetch --ascii-bold off
```

#### Output Customization
```bash
# Different output formats
neofetch --json > system_info.json
neofetch --stdout | grep "OS:"
neofetch --verbose 2>&1 | tee debug.log

# Memory unit options
neofetch --memory-unit kib    # Kibibytes
neofetch --memory-unit mib    # Mebibytes
neofetch --memory-unit gib    # Gibibytes
neofetch --memory-unit tib    # Tebibytes

# Color block customization
neofetch --color-blocks off
neofetch --block-range "0-7"
neofetch --block-width 4
neofetch --block-height 2
```

#### CPU Information Options
```bash
# Show different CPU core counts
neofetch --cpu-cores logical    # Logical cores (with hyperthreading)
neofetch --cpu-cores physical   # Physical cores only

# CPU speed options
neofetch --cpu-speed on
neofetch --speed-type current   # Current frequency
neofetch --speed-type max       # Maximum frequency
neofetch --speed-type min       # Minimum frequency
```

### 🔍 Troubleshooting Guide

#### Common Issues and Solutions

**Issue: Binary not found after installation**
```bash
# Check if binary exists
ls -la /usr/local/bin/neofetch

# Check PATH
echo $PATH

# Add to PATH if needed
export PATH="/usr/local/bin:$PATH"
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
```

**Issue: Permission denied**
```bash
# Make binary executable
chmod +x /usr/local/bin/neofetch

# Or reinstall with proper permissions
sudo make install
```

**Issue: Build fails**
```bash
# Update Rust toolchain
rustup update

# Clean and rebuild
make clean
make release

# Check Rust version (requires 1.70+)
rustc --version
```

**Issue: Missing system information**
```bash
# Run with verbose output for debugging
neofetch --verbose

# Check if specific tools are installed
which lscpu    # Linux CPU info
which sw_vers  # macOS system info
which wmic     # Windows system info
```

#### Debug Mode
```bash
# Build debug version
make build

# Run with debug logging
RUST_LOG=debug ./target/debug/neofetch --verbose

# Trace all system calls
RUST_LOG=trace ./target/debug/neofetch 2> debug.log
```

### 📊 Performance Guide

#### Benchmarking
```bash
# Time the execution
time neofetch

# Compare with original neofetch
time neofetch-original
time neofetch

# Memory usage comparison
/usr/bin/time -v neofetch
```

#### Optimization Tips
```bash
# Use release build for best performance
make release

# Disable unnecessary features for speed
neofetch --backend off --color-blocks off

# Use JSON output for scripting (faster parsing)
neofetch --json | jq '.cpu'
```

### 🔧 Development Guide

#### Setting Up Development Environment
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and setup
git clone https://github.com/zibo-wang/neofetchrs.git
cd neofetchrs

# Install development dependencies
rustup component add clippy rustfmt

# Run development checks
cargo clippy
cargo fmt --check
cargo test
```

#### Testing Your Changes
```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test integration

# Test specific functionality
cargo test system_info

# Run with different configurations
cargo run -- --json
cargo run -- --verbose
cargo run -- --config test_configs/minimal.toml
```

#### Code Quality
```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check for security issues
cargo audit

# Generate documentation
cargo doc --open
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

## FAQ

### General Questions

**Q: How is this different from the original neofetch?**
A: Neofetch-rs is written in Rust for better performance, smaller memory footprint, and cross-platform compatibility. It's ~4x faster and uses ~75% less memory than the original.

**Q: Can I use my existing neofetch config?**
A: Not directly. Neofetch-rs uses TOML configuration instead of bash scripts. However, most options have equivalent settings.

**Q: Does this replace the original neofetch?**
A: It can! The binary is named `neofetch` and supports most of the same command-line options.

**Q: Why is the binary so much smaller?**
A: The release build uses aggressive optimizations including LTO, symbol stripping, and maximum optimization levels.

### Technical Questions

**Q: What Rust version is required?**
A: Rust 1.70 or later is required for compilation.

**Q: Can I cross-compile for different platforms?**
A: Yes! Use `cargo build --target <target-triple>` for cross-compilation.

**Q: How do I contribute new features?**
A: See the Development Guide above and check the CONTRIBUTING.md file.

**Q: Is Windows support complete?**
A: Basic Windows support is implemented, but some features may be limited compared to Unix systems.

## Development

### Quick Development Setup

```bash
# Clone and setup development environment
git clone https://github.com/zibo-wang/neofetchrs.git
cd neofetchrs

# Build debug version for development
make build

# Run tests
cargo test

# Run with specific options
cargo run -- --json
```

### Available Make Targets

| Target               | Description             |
| -------------------- | ----------------------- |
| `make build`         | Build debug version     |
| `make release`       | Build optimized release |
| `make release-debug` | Release with debug info |
| `make install`       | Install release version |
| `make install-debug` | Install debug version   |
| `make uninstall`     | Remove installed binary |
| `make clean`         | Clean build artifacts   |

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
