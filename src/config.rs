//! Configuration management for neofetch-rs
//! 
//! This module handles loading and managing configuration from files and command-line arguments.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Information display settings
    pub info: InfoConfig,
    
    /// ASCII art and image settings
    pub display: DisplayConfig,
    
    /// Output formatting settings
    pub format: FormatConfig,
    
    /// Performance and behavior settings
    pub behavior: BehaviorConfig,
}

/// Information gathering configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoConfig {
    pub title_fqdn: bool,
    pub package_managers: PackageManagerDisplay,
    pub os_arch: bool,
    pub cpu_cores: CpuCoreDisplay,
    pub cpu_speed: bool,
    pub speed_type: SpeedType,
    pub speed_shorthand: bool,
    pub distro_shorthand: DistroShorthand,
    pub kernel_shorthand: bool,
    pub uptime_shorthand: UptimeShorthand,
    pub cpu_brand: bool,
    pub gpu_brand: bool,
    pub gpu_type: GpuType,
    pub refresh_rate: bool,
    pub shell_path: bool,
    pub shell_version: bool,
    pub memory_unit: MemoryUnit,
    pub memory_percent: bool,
    pub disk_show: Vec<String>,
    pub disk_subtitle: DiskSubtitle,
    pub disk_percent: bool,
    pub music_player: MusicPlayer,
    pub song_format: String,
    pub song_shorthand: bool,
    pub mpc_args: Vec<String>,
    pub colors: Vec<u8>,
    pub bold: bool,
    pub underline_enabled: bool,
    pub underline_char: String,
    pub separator: String,
}

/// Display and ASCII art configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub image_backend: ImageBackend,
    pub image_source: ImageSource,
    pub ascii_distro: Option<String>,
    pub ascii_colors: Vec<String>,
    pub ascii_bold: bool,
    pub image_loop: bool,
    pub thumbnail_dir: PathBuf,
    pub crop_mode: CropMode,
    pub crop_offset: CropOffset,
    pub image_size: ImageSize,
    pub gap: i32,
    pub yoffset: i32,
    pub xoffset: i32,
    pub background_color: Option<String>,
    pub stdout: bool,
}

/// Output formatting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatConfig {
    pub color_blocks: bool,
    pub block_range: (u8, u8),
    pub block_width: u8,
    pub block_height: u8,
    pub col_offset: String,
    pub bar_char_elapsed: String,
    pub bar_char_total: String,
    pub bar_border: bool,
    pub bar_length: u8,
    pub bar_color_elapsed: String,
    pub bar_color_total: String,
    pub cpu_display: DisplayMode,
    pub memory_display: DisplayMode,
    pub battery_display: DisplayMode,
    pub disk_display: DisplayMode,
}

/// Behavior and performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    pub cache_dir: PathBuf,
    pub config_file: Option<PathBuf>,
    pub stdout: bool,
    pub verbose: bool,
    pub json: bool,
}

// Enums for configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageManagerDisplay {
    On,
    Tiny,
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CpuCoreDisplay {
    Logical,
    Physical,
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpeedType {
    Scaling,
    Base,
    Max,
    Bios,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistroShorthand {
    On,
    Tiny,
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UptimeShorthand {
    On,
    Tiny,
    Off,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GpuType {
    All,
    Dedicated,
    Integrated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryUnit {
    Kib,
    Mib,
    Gib,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiskSubtitle {
    Mount,
    Name,
    Dir,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MusicPlayer {
    Auto,
    Player(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageBackend {
    Ascii,
    Caca,
    Catimg,
    Chafa,
    Jp2a,
    Iterm2,
    Off,
    Pixterm,
    Sixel,
    Termpix,
    Tycat,
    W3m,
    Kitty,
    Ueberzug,
    Viu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageSource {
    Auto,
    Ascii,
    Wallpaper,
    Path(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CropMode {
    Normal,
    Fit,
    Fill,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CropOffset {
    Northwest,
    North,
    Northeast,
    West,
    Center,
    East,
    Southwest,
    South,
    Southeast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageSize {
    Auto,
    Size(u32, u32),
    Percent(u8),
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisplayMode {
    Bar,
    Infobar,
    Barinfo,
    Off,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            info: InfoConfig::default(),
            display: DisplayConfig::default(),
            format: FormatConfig::default(),
            behavior: BehaviorConfig::default(),
        }
    }
}

impl Default for InfoConfig {
    fn default() -> Self {
        Self {
            title_fqdn: false,
            package_managers: PackageManagerDisplay::On,
            os_arch: true,
            cpu_cores: CpuCoreDisplay::Logical,
            cpu_speed: true,
            speed_type: SpeedType::Bios,
            speed_shorthand: false,
            distro_shorthand: DistroShorthand::Off,
            kernel_shorthand: true,
            uptime_shorthand: UptimeShorthand::On,
            cpu_brand: true,
            gpu_brand: true,
            gpu_type: GpuType::All,
            refresh_rate: false,
            shell_path: false,
            shell_version: true,
            memory_unit: MemoryUnit::Mib,
            memory_percent: false,
            disk_show: vec!["/".to_string()],
            disk_subtitle: DiskSubtitle::Mount,
            disk_percent: true,
            music_player: MusicPlayer::Auto,
            song_format: "%artist% - %album% - %title%".to_string(),
            song_shorthand: false,
            mpc_args: vec![],
            colors: (1..=6).collect(),
            bold: true,
            underline_enabled: true,
            underline_char: "-".to_string(),
            separator: ":".to_string(),
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            image_backend: ImageBackend::Ascii,
            image_source: ImageSource::Auto,
            ascii_distro: None,
            ascii_colors: vec!["distro".to_string()],
            ascii_bold: true,
            image_loop: false,
            thumbnail_dir: dirs::cache_dir().unwrap_or_default().join("neofetch"),
            crop_mode: CropMode::Normal,
            crop_offset: CropOffset::Center,
            image_size: ImageSize::Auto,
            gap: 3,
            yoffset: 0,
            xoffset: 0,
            background_color: None,
            stdout: false,
        }
    }
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            color_blocks: true,
            block_range: (0, 15),
            block_width: 3,
            block_height: 1,
            col_offset: "auto".to_string(),
            bar_char_elapsed: "━".to_string(),
            bar_char_total: "━".to_string(),
            bar_border: true,
            bar_length: 15,
            bar_color_elapsed: "distro".to_string(),
            bar_color_total: "distro".to_string(),
            cpu_display: DisplayMode::Off,
            memory_display: DisplayMode::Off,
            battery_display: DisplayMode::Off,
            disk_display: DisplayMode::Off,
        }
    }
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            cache_dir: dirs::cache_dir().unwrap_or_default().join("neofetch"),
            config_file: None,
            stdout: false,
            verbose: false,
            json: false,
        }
    }
}

impl Config {
    /// Load configuration from file and merge with defaults
    pub fn load() -> Result<Self> {
        let mut config = Self::default();
        
        // Try to load user config file
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("neofetch").join("config.toml");
            if config_path.exists() {
                let config_str = std::fs::read_to_string(&config_path)?;
                let user_config: Config = toml::from_str(&config_str)?;
                config = user_config;
            }
        }
        
        Ok(config)
    }
    
    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        if let Some(config_dir) = dirs::config_dir() {
            let neofetch_dir = config_dir.join("neofetch");
            std::fs::create_dir_all(&neofetch_dir)?;
            
            let config_path = neofetch_dir.join("config.toml");
            let config_str = toml::to_string_pretty(self)?;
            std::fs::write(config_path, config_str)?;
        }
        
        Ok(())
    }
}
