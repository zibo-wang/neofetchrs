//! Utility functions for neofetch-rs
//!
//! This module contains various utility functions used throughout the application.

use anyhow::Result;
use std::process::Command;

/// Execute a shell command and return its output
pub fn execute_command(command: &str, args: &[&str]) -> Result<String> {
    let output = Command::new(command).args(args).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Ok(String::new())
    }
}

/// Check if a command exists in the system PATH
pub fn command_exists(command: &str) -> bool {
    Command::new("which")
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Trim quotes from a string
pub fn trim_quotes(s: &str) -> String {
    s.trim_matches('"').trim_matches('\'').to_string()
}

/// Clean up CPU name by removing common suffixes and prefixes
pub fn clean_cpu_name(name: &str) -> String {
    name.replace("(R)", "")
        .replace("(TM)", "")
        .replace("(tm)", "")
        .replace("CPU", "")
        .replace("Processor", "")
        .replace("  ", " ")
        .trim()
        .to_string()
}

/// Convert bytes to human-readable format
pub fn bytes_to_human_readable(bytes: u64, unit: &str) -> String {
    match unit.to_lowercase().as_str() {
        "kib" => format!("{:.1}KiB", bytes as f64 / 1024.0),
        "mib" => format!("{:.1}MiB", bytes as f64 / 1024.0 / 1024.0),
        "gib" => format!("{:.1}GiB", bytes as f64 / 1024.0 / 1024.0 / 1024.0),
        _ => format!("{:.1}MiB", bytes as f64 / 1024.0 / 1024.0),
    }
}

/// Format uptime in a human-readable way
pub fn format_uptime(seconds: u64, shorthand: bool) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;

    if shorthand {
        if days > 0 {
            format!("{}d {}h {}m", days, hours, minutes)
        } else if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    } else {
        if days > 0 {
            format!("{} days, {} hours, {} mins", days, hours, minutes)
        } else if hours > 0 {
            format!("{} hours, {} mins", hours, minutes)
        } else {
            format!("{} mins", minutes)
        }
    }
}

/// Get the terminal width
pub fn get_terminal_width() -> usize {
    if let Ok(output) = Command::new("tput").arg("cols").output() {
        if output.status.success() {
            if let Ok(width_str) = String::from_utf8(output.stdout) {
                if let Ok(width) = width_str.trim().parse::<usize>() {
                    return width;
                }
            }
        }
    }

    // Fallback to environment variable
    if let Ok(cols) = std::env::var("COLUMNS") {
        if let Ok(width) = cols.parse::<usize>() {
            return width;
        }
    }

    // Default fallback
    80
}

/// Get the terminal height
pub fn get_terminal_height() -> usize {
    if let Ok(output) = Command::new("tput").arg("lines").output() {
        if output.status.success() {
            if let Ok(height_str) = String::from_utf8(output.stdout) {
                if let Ok(height) = height_str.trim().parse::<usize>() {
                    return height;
                }
            }
        }
    }

    // Fallback to environment variable
    if let Ok(lines) = std::env::var("LINES") {
        if let Ok(height) = lines.parse::<usize>() {
            return height;
        }
    }

    // Default fallback
    24
}

/// Detect the current operating system
pub fn detect_os() -> String {
    #[cfg(target_os = "linux")]
    {
        // Try to detect specific Linux distribution
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    let name = line.split('=').nth(1).unwrap_or("");
                    return trim_quotes(name);
                }
            }
        }

        if let Ok(content) = std::fs::read_to_string("/etc/lsb-release") {
            for line in content.lines() {
                if line.starts_with("DISTRIB_DESCRIPTION=") {
                    let name = line.split('=').nth(1).unwrap_or("");
                    return trim_quotes(name);
                }
            }
        }

        "Linux".to_string()
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = execute_command("sw_vers", &["-productName"]) {
            if let Ok(version) = execute_command("sw_vers", &["-productVersion"]) {
                return format!("{} {}", output, version);
            }
            return output;
        }
        "macOS".to_string()
    }

    #[cfg(target_os = "windows")]
    {
        "Windows".to_string()
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        "Unknown".to_string()
    }
}

/// Get the hostname
pub fn get_hostname() -> String {
    whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string())
}

/// Get the username
pub fn get_username() -> String {
    whoami::username()
}

/// Check if running in a container
pub fn is_container() -> bool {
    // Check for common container indicators
    std::path::Path::new("/.dockerenv").exists()
        || std::env::var("container").is_ok()
        || std::fs::read_to_string("/proc/1/cgroup")
            .map(|content| content.contains("docker") || content.contains("lxc"))
            .unwrap_or(false)
}

/// Get the current shell
pub fn get_current_shell() -> String {
    if let Ok(shell) = std::env::var("SHELL") {
        std::path::Path::new(&shell)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string()
    } else {
        "Unknown".to_string()
    }
}

/// Parse version string from command output
pub fn parse_version_from_output(output: &str) -> Option<String> {
    // Common version patterns
    let patterns = [
        r"(\d+\.\d+\.\d+)",
        r"(\d+\.\d+)",
        r"version (\d+\.\d+\.\d+)",
        r"v(\d+\.\d+\.\d+)",
    ];

    for pattern in &patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(captures) = re.captures(output) {
                if let Some(version) = captures.get(1) {
                    return Some(version.as_str().to_string());
                }
            }
        }
    }

    None
}
