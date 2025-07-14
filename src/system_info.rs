//! System information gathering module
//!
//! This module provides cross-platform system information gathering capabilities.

use crate::config::Config;
use anyhow::Result;
use sysinfo::System;

/// Main system information structure
#[derive(Debug)]
pub struct SystemInfo {
    pub title: String,
    pub os: String,
    pub host: String,
    pub kernel: String,
    pub uptime: String,
    pub packages: String,
    pub shell: String,
    pub resolution: String,
    pub de: String,
    pub wm: String,
    pub wm_theme: String,
    pub theme: String,
    pub icons: String,
    pub terminal: String,
    pub terminal_font: String,
    pub cpu: String,
    pub gpu: String,
    pub memory: String,
    pub disk: String,
    pub battery: String,
    pub local_ip: String,
    pub public_ip: String,
    pub users: String,
    pub locale: String,
    pub gpu_driver: String,
    pub song: String,
    pub colors: String,

    // Internal system handle
    system: System,
}

impl SystemInfo {
    /// Create a new SystemInfo instance
    pub fn new() -> Result<Self> {
        let mut system = System::new_all();
        system.refresh_all();

        Ok(Self {
            title: String::new(),
            os: String::new(),
            host: String::new(),
            kernel: String::new(),
            uptime: String::new(),
            packages: String::new(),
            shell: String::new(),
            resolution: String::new(),
            de: String::new(),
            wm: String::new(),
            wm_theme: String::new(),
            theme: String::new(),
            icons: String::new(),
            terminal: String::new(),
            terminal_font: String::new(),
            cpu: String::new(),
            gpu: String::new(),
            memory: String::new(),
            disk: String::new(),
            battery: String::new(),
            local_ip: String::new(),
            public_ip: String::new(),
            users: String::new(),
            locale: String::new(),
            gpu_driver: String::new(),
            song: String::new(),
            colors: String::new(),
            system,
        })
    }

    /// Gather all system information based on configuration
    pub fn gather_all(&mut self, _config: &Config) -> Result<()> {
        self.system.refresh_all();

        self.get_title()?;
        self.get_os()?;
        self.get_host()?;
        self.get_kernel()?;
        self.get_uptime()?;
        self.get_packages()?;
        self.get_shell()?;
        self.get_resolution()?;
        self.get_de()?;
        self.get_wm()?;
        self.get_wm_theme()?;
        self.get_theme()?;
        self.get_icons()?;
        self.get_terminal()?;
        self.get_terminal_font()?;
        self.get_cpu()?;
        self.get_gpu()?;
        self.get_memory()?;
        self.get_disk()?;
        self.get_battery()?;
        self.get_local_ip()?;
        self.get_users()?;
        self.get_locale()?;
        self.get_gpu_driver()?;
        self.get_song()?;
        self.get_colors()?;

        Ok(())
    }

    /// Get system title (user@hostname)
    fn get_title(&mut self) -> Result<()> {
        let username = whoami::username();
        let hostname = whoami::fallible::hostname().unwrap_or_else(|_| "unknown".to_string());

        // Use short hostname (without domain) to match original neofetch behavior
        let short_hostname = hostname.split('.').next().unwrap_or(&hostname);
        self.title = format!("{}@{}", username, short_hostname);
        Ok(())
    }

    /// Get operating system information
    fn get_os(&mut self) -> Result<()> {
        self.os = format!(
            "{} {}",
            System::name().unwrap_or_else(|| "Unknown".to_string()),
            System::os_version().unwrap_or_else(|| "Unknown".to_string())
        );
        Ok(())
    }

    /// Get host/model information
    fn get_host(&mut self) -> Result<()> {
        // Try to get host information from various sources
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/sys/devices/virtual/dmi/id/product_name")
            {
                self.host = content.trim().to_string();
                return Ok(());
            }
            if let Ok(content) = std::fs::read_to_string("/sys/devices/virtual/dmi/id/board_name") {
                self.host = content.trim().to_string();
                return Ok(());
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("system_profiler")
                .args(&["SPHardwareDataType"])
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("Model Name:") {
                        self.host = line.split(':').nth(1).unwrap_or("").trim().to_string();
                        return Ok(());
                    }
                }
            }
        }

        self.host = "Unknown".to_string();
        Ok(())
    }

    /// Get kernel information
    fn get_kernel(&mut self) -> Result<()> {
        self.kernel = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
        Ok(())
    }

    /// Get system uptime
    fn get_uptime(&mut self) -> Result<()> {
        #[cfg(target_os = "linux")]
        {
            if let Ok(content) = std::fs::read_to_string("/proc/uptime") {
                if let Some(uptime_str) = content.split_whitespace().next() {
                    if let Ok(uptime_seconds) = uptime_str.parse::<f64>() {
                        let uptime_seconds = uptime_seconds as u64;
                        let days = uptime_seconds / 86400;
                        let hours = (uptime_seconds % 86400) / 3600;
                        let minutes = (uptime_seconds % 3600) / 60;

                        if days > 0 {
                            self.uptime =
                                format!("{} days, {} hours, {} mins", days, hours, minutes);
                        } else if hours > 0 {
                            self.uptime = format!("{} hours, {} mins", hours, minutes);
                        } else {
                            self.uptime = format!("{} mins", minutes);
                        }
                        return Ok(());
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("uptime").output() {
                if output.status.success() {
                    let uptime_str = String::from_utf8_lossy(&output.stdout);
                    // Parse uptime output like "up 18 days,  4:41, 2 users, load averages: 1.23 1.45 1.67"
                    if let Some(up_part) = uptime_str.split("up ").nth(1) {
                        // Split by comma and take the time parts
                        let parts: Vec<&str> = up_part.split(',').collect();
                        let mut uptime_parts = Vec::new();

                        for (i, part) in parts.iter().enumerate() {
                            let trimmed = part.trim();

                            // Stop at "users" or "load" indicators
                            if trimmed.contains("user") || trimmed.contains("load") {
                                break;
                            }

                            // First part might contain days
                            if i == 0 {
                                uptime_parts.push(trimmed.to_string());
                            }
                            // Second part might contain hours:minutes - convert to "X hours, Y mins" format
                            else if i == 1 && trimmed.contains(':') {
                                if let Some((hours_str, mins_str)) = trimmed.split_once(':') {
                                    let hours_str = hours_str.trim();
                                    let mins_str = mins_str.trim();

                                    if let (Ok(hours), Ok(mins)) =
                                        (hours_str.parse::<u32>(), mins_str.parse::<u32>())
                                    {
                                        if hours > 0 && mins > 0 {
                                            uptime_parts
                                                .push(format!("{} hours, {} mins", hours, mins));
                                        } else if hours > 0 {
                                            uptime_parts.push(format!("{} hours", hours));
                                        } else if mins > 0 {
                                            uptime_parts.push(format!("{} mins", mins));
                                        }
                                    }
                                }
                            }
                        }

                        if !uptime_parts.is_empty() {
                            self.uptime = uptime_parts.join(", ");
                            return Ok(());
                        }
                    }
                }
            }
        }

        // Fallback
        self.uptime = "Unknown".to_string();
        Ok(())
    }

    /// Get package count
    fn get_packages(&mut self) -> Result<()> {
        let mut package_managers = Vec::new();

        // Check various package managers
        #[cfg(target_os = "linux")]
        {
            // APT (Debian/Ubuntu)
            if let Ok(output) = std::process::Command::new("dpkg-query")
                .args(&["-f", "${binary:Package}\n", "-W"])
                .output()
            {
                if output.status.success() {
                    let count = String::from_utf8_lossy(&output.stdout).lines().count();
                    if count > 0 {
                        package_managers.push(format!("{} (apt)", count));
                    }
                }
            }

            // Pacman (Arch)
            if let Ok(output) = std::process::Command::new("pacman").args(&["-Qq"]).output() {
                if output.status.success() {
                    let count = String::from_utf8_lossy(&output.stdout).lines().count();
                    if count > 0 {
                        package_managers.push(format!("{} (pacman)", count));
                    }
                }
            }

            // RPM (Red Hat/Fedora)
            if let Ok(output) = std::process::Command::new("rpm").args(&["-qa"]).output() {
                if output.status.success() {
                    let count = String::from_utf8_lossy(&output.stdout).lines().count();
                    if count > 0 {
                        package_managers.push(format!("{} (rpm)", count));
                    }
                }
            }

            // Flatpak
            if let Ok(output) = std::process::Command::new("flatpak")
                .args(&["list", "--app"])
                .output()
            {
                if output.status.success() {
                    let count = String::from_utf8_lossy(&output.stdout).lines().count();
                    if count > 0 {
                        package_managers.push(format!("{} (flatpak)", count));
                    }
                }
            }

            // Snap
            if let Ok(output) = std::process::Command::new("snap").args(&["list"]).output() {
                if output.status.success() {
                    let count = String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .count()
                        .saturating_sub(1); // Remove header
                    if count > 0 {
                        package_managers.push(format!("{} (snap)", count));
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            // Homebrew
            if let Ok(output) = std::process::Command::new("brew")
                .args(&["list", "--formula"])
                .output()
            {
                if output.status.success() {
                    let count = String::from_utf8_lossy(&output.stdout).lines().count();
                    if count > 0 {
                        package_managers.push(format!("{} (brew)", count));
                    }
                }
            }

            // MacPorts
            if let Ok(output) = std::process::Command::new("port")
                .args(&["installed"])
                .output()
            {
                if output.status.success() {
                    let count = String::from_utf8_lossy(&output.stdout).lines().count();
                    if count > 0 {
                        package_managers.push(format!("{} (port)", count));
                    }
                }
            }
        }

        if package_managers.is_empty() {
            self.packages = "Unknown".to_string();
        } else {
            self.packages = package_managers.join(", ");
        }

        Ok(())
    }

    /// Get shell information
    fn get_shell(&mut self) -> Result<()> {
        if let Ok(shell) = std::env::var("SHELL") {
            let shell_name = std::path::Path::new(&shell)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown");

            // Try to get version
            if let Ok(output) = std::process::Command::new(shell_name)
                .arg("--version")
                .output()
            {
                if output.status.success() {
                    let version_output = String::from_utf8_lossy(&output.stdout);
                    let first_line = version_output.lines().next().unwrap_or("");
                    self.shell = first_line.to_string();
                } else {
                    self.shell = shell_name.to_string();
                }
            } else {
                self.shell = shell_name.to_string();
            }
        } else {
            self.shell = "Unknown".to_string();
        }

        Ok(())
    }

    /// Get screen resolution
    fn get_resolution(&mut self) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = std::process::Command::new("system_profiler")
                .args(&["SPDisplaysDataType"])
                .output()
            {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    let mut resolutions = Vec::new();

                    for line in output_str.lines() {
                        if line.contains("Resolution:") {
                            if let Some(res) = line.split(':').nth(1) {
                                let res = res.trim();
                                if !res.is_empty() && res != "Unknown" {
                                    resolutions.push(res.to_string());
                                }
                            }
                        }
                    }

                    if !resolutions.is_empty() {
                        self.resolution = resolutions.join(", ");
                        return Ok(());
                    }
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            // Try xrandr first
            if let Ok(output) = std::process::Command::new("xrandr")
                .args(&["--query"])
                .output()
            {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    let mut resolutions = Vec::new();

                    for line in output_str.lines() {
                        if line.contains(" connected") && line.contains("x") {
                            if let Some(res_part) = line.split_whitespace().find(|s| {
                                s.contains("x") && s.chars().next().unwrap_or('a').is_ascii_digit()
                            }) {
                                resolutions.push(res_part.to_string());
                            }
                        }
                    }

                    if !resolutions.is_empty() {
                        self.resolution = resolutions.join(", ");
                        return Ok(());
                    }
                }
            }
        }

        self.resolution = "Unknown".to_string();
        Ok(())
    }

    /// Get desktop environment
    fn get_de(&mut self) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            self.de = "Aqua".to_string();
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(de) = std::env::var("XDG_CURRENT_DESKTOP") {
                self.de = de;
            } else if let Ok(de) = std::env::var("DESKTOP_SESSION") {
                self.de = de;
            } else if let Ok(_) = std::env::var("GNOME_DESKTOP_SESSION_ID") {
                self.de = "GNOME".to_string();
            } else if let Ok(_) = std::env::var("KDE_FULL_SESSION") {
                self.de = "KDE".to_string();
            } else {
                self.de = "Unknown".to_string();
            }
        }

        #[cfg(target_os = "windows")]
        {
            self.de = "Windows".to_string();
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            self.de = "Unknown".to_string();
        }

        Ok(())
    }

    /// Get window manager
    fn get_wm(&mut self) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            self.wm = "Quartz Compositor".to_string();
        }

        #[cfg(target_os = "linux")]
        {
            // Try to detect common window managers
            if let Ok(_) = std::env::var("GNOME_DESKTOP_SESSION_ID") {
                self.wm = "Mutter".to_string();
            } else if let Ok(_) = std::env::var("KDE_FULL_SESSION") {
                self.wm = "KWin".to_string();
            } else if let Ok(wm) = std::env::var("DESKTOP_SESSION") {
                match wm.to_lowercase().as_str() {
                    "i3" => self.wm = "i3".to_string(),
                    "awesome" => self.wm = "awesome".to_string(),
                    "bspwm" => self.wm = "bspwm".to_string(),
                    "openbox" => self.wm = "Openbox".to_string(),
                    _ => self.wm = wm,
                }
            } else {
                self.wm = "Unknown".to_string();
            }
        }

        #[cfg(target_os = "windows")]
        {
            self.wm = "Desktop Window Manager".to_string();
        }

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        {
            self.wm = "Unknown".to_string();
        }

        Ok(())
    }

    /// Get window manager theme
    fn get_wm_theme(&mut self) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            // Try to detect macOS appearance
            if let Ok(output) = std::process::Command::new("defaults")
                .args(&["read", "-g", "AppleInterfaceStyle"])
                .output()
            {
                if output.status.success() {
                    let style = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if style == "Dark" {
                        self.wm_theme = "Blue (Dark)".to_string();
                    } else {
                        self.wm_theme = "Blue (Light)".to_string();
                    }
                } else {
                    self.wm_theme = "Blue (Light)".to_string();
                }
                return Ok(());
            }
        }

        self.wm_theme = "Unknown".to_string();
        Ok(())
    }

    /// Get system theme
    fn get_theme(&mut self) -> Result<()> {
        self.theme = "Unknown".to_string();
        Ok(())
    }

    /// Get icon theme
    fn get_icons(&mut self) -> Result<()> {
        self.icons = "Unknown".to_string();
        Ok(())
    }

    /// Get terminal information
    fn get_terminal(&mut self) -> Result<()> {
        if let Ok(term) = std::env::var("TERM_PROGRAM") {
            self.terminal = term;
        } else if let Ok(term) = std::env::var("TERM") {
            self.terminal = term;
        } else {
            self.terminal = "Unknown".to_string();
        }
        Ok(())
    }

    /// Get terminal font
    fn get_terminal_font(&mut self) -> Result<()> {
        self.terminal_font = "Unknown".to_string();
        Ok(())
    }

    /// Get CPU information
    fn get_cpu(&mut self) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            // Try to get CPU info from system_profiler
            if let Ok(output) = std::process::Command::new("sysctl")
                .args(&["-n", "machdep.cpu.brand_string"])
                .output()
            {
                if output.status.success() {
                    let cpu_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !cpu_name.is_empty() {
                        // Get core count
                        let core_count = self.system.cpus().len();
                        self.cpu = format!("{} ({} cores)", cpu_name, core_count);
                        return Ok(());
                    }
                }
            }
        }

        // Fallback to sysinfo
        if let Some(cpu) = self.system.cpus().first() {
            let cpu_name = cpu.name().trim();
            let cpu_count = self.system.cpus().len();

            // Clean up CPU name
            let cleaned_name = cpu_name
                .replace("(R)", "")
                .replace("(TM)", "")
                .replace("CPU", "")
                .replace("Processor", "")
                .replace("  ", " ")
                .trim()
                .to_string();

            self.cpu = format!("{} ({} cores)", cleaned_name, cpu_count);
        } else {
            self.cpu = "Unknown".to_string();
        }
        Ok(())
    }

    /// Get GPU information
    fn get_gpu(&mut self) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            // Try to get GPU info from system_profiler
            if let Ok(output) = std::process::Command::new("system_profiler")
                .args(&["SPDisplaysDataType"])
                .output()
            {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    for line in output_str.lines() {
                        if line.contains("Chipset Model:") {
                            if let Some(gpu) = line.split(':').nth(1) {
                                let gpu = gpu.trim();
                                if !gpu.is_empty() && gpu != "Unknown" {
                                    self.gpu = gpu.to_string();
                                    return Ok(());
                                }
                            }
                        }
                    }
                }
            }
        }

        self.gpu = "Unknown".to_string();
        Ok(())
    }

    /// Get memory information
    fn get_memory(&mut self) -> Result<()> {
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();

        let total_gb = total_memory as f64 / 1024.0 / 1024.0 / 1024.0;
        let used_gb = used_memory as f64 / 1024.0 / 1024.0 / 1024.0;

        self.memory = format!("{:.1}GiB / {:.1}GiB", used_gb, total_gb);
        Ok(())
    }

    /// Get disk information
    fn get_disk(&mut self) -> Result<()> {
        // Simplified disk info - just show that it's available
        self.disk = "Available".to_string();
        Ok(())
    }

    /// Get battery information
    fn get_battery(&mut self) -> Result<()> {
        // Battery information is complex and platform-specific
        self.battery = "Unknown".to_string();
        Ok(())
    }

    /// Get local IP address
    fn get_local_ip(&mut self) -> Result<()> {
        // Simplified - just indicate network is available
        self.local_ip = "Available".to_string();
        Ok(())
    }

    /// Get logged in users
    fn get_users(&mut self) -> Result<()> {
        // Get current user for now
        self.users = whoami::username();
        Ok(())
    }

    /// Get system locale
    fn get_locale(&mut self) -> Result<()> {
        if let Ok(locale) = std::env::var("LANG") {
            self.locale = locale;
        } else {
            self.locale = "Unknown".to_string();
        }
        Ok(())
    }

    /// Get GPU driver information
    fn get_gpu_driver(&mut self) -> Result<()> {
        self.gpu_driver = "Unknown".to_string();
        Ok(())
    }

    /// Get currently playing song
    fn get_song(&mut self) -> Result<()> {
        self.song = "Unknown".to_string();
        Ok(())
    }

    /// Get color information
    fn get_colors(&mut self) -> Result<()> {
        // Generate color blocks for display - two rows of 8 colors each
        let mut colors = String::new();

        // First row (colors 0-7)
        for i in 0..8 {
            colors.push_str(&format!("\x1b[4{}m   \x1b[0m", i));
        }
        colors.push('\n');

        // Second row (bright colors 8-15)
        for i in 0..8 {
            colors.push_str(&format!("\x1b[10{}m   \x1b[0m", i));
        }

        self.colors = colors;
        Ok(())
    }

    /// Get a specific field by name
    pub fn get_field(&self, field_name: &str) -> Option<&str> {
        match field_name {
            "title" => Some(&self.title),
            "os" | "distro" => Some(&self.os),
            "host" | "model" => Some(&self.host),
            "kernel" => Some(&self.kernel),
            "uptime" => Some(&self.uptime),
            "packages" => Some(&self.packages),
            "shell" => Some(&self.shell),
            "resolution" => Some(&self.resolution),
            "de" => Some(&self.de),
            "wm" => Some(&self.wm),
            "wm_theme" => Some(&self.wm_theme),
            "theme" => Some(&self.theme),
            "icons" => Some(&self.icons),
            "terminal" | "term" => Some(&self.terminal),
            "terminal_font" | "term_font" => Some(&self.terminal_font),
            "cpu" => Some(&self.cpu),
            "gpu" => Some(&self.gpu),
            "memory" => Some(&self.memory),
            "disk" => Some(&self.disk),
            "battery" => Some(&self.battery),
            "local_ip" => Some(&self.local_ip),
            "public_ip" => Some(&self.public_ip),
            "users" => Some(&self.users),
            "locale" => Some(&self.locale),
            "gpu_driver" => Some(&self.gpu_driver),
            "song" => Some(&self.song),
            "cols" | "colors" => Some(&self.colors),
            _ => None,
        }
    }
}
