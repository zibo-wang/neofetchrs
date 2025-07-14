//! Output formatting and display module
//!
//! This module handles the formatting and display of system information alongside ASCII art.

use crate::ascii_art::AsciiArt;
use crate::config::Config;
use crate::system_info::SystemInfo;
use crate::utils;
use anyhow::Result;
use colored::*;

/// Information item structure
#[derive(Debug, Clone)]
pub struct InfoItem {
    pub label: String,
    pub value: String,
    pub show: bool,
}

/// Generate the complete output combining ASCII art and system information
pub fn generate_output(system_info: &SystemInfo, config: &Config) -> Result<String> {
    let ascii_art = AsciiArt::new();
    let info_items = get_info_items(system_info, config);

    // Get the OS name for ASCII art selection
    let os_name = system_info.get_field("os").unwrap_or("linux");

    // Get ASCII logo
    let default_logo = vec!["".to_string()];
    let logo = ascii_art.get_logo(os_name).unwrap_or(&default_logo);
    let colored_logo = ascii_art.colorize_logo(os_name, logo);

    // Calculate dimensions
    let logo_width = ascii_art.get_logo_width(os_name);
    let logo_height = colored_logo.len();

    // Generate output
    let mut output = String::new();

    if config.behavior.json {
        return generate_json_output(system_info);
    }

    if config.display.stdout {
        return generate_stdout_output(&info_items, system_info, config);
    }

    // Filter out items that shouldn't be shown
    let visible_items: Vec<&InfoItem> = info_items.iter().filter(|item| item.show).collect();

    // Calculate available width for info text
    let terminal_width = utils::get_terminal_width();
    let ascii_and_gap_width = logo_width + config.display.gap as usize;
    let available_info_width = if terminal_width > ascii_and_gap_width + 10 {
        terminal_width - ascii_and_gap_width - 5 // Small margin for safety
    } else {
        40 // Fallback minimum
    };

    // Combine ASCII art with system information
    let max_lines = std::cmp::max(logo_height, visible_items.len());
    let mut info_index = 0;

    for i in 0..max_lines {
        let mut line = String::new();

        // Add ASCII art line
        if i < logo_height {
            line.push_str(&colored_logo[i]);
            // Pad to consistent width
            let actual_width = ascii_art.strip_ansi_codes(&colored_logo[i]).chars().count();
            if actual_width < logo_width {
                line.push_str(&" ".repeat(logo_width - actual_width));
            }
        } else {
            // Add padding to maintain alignment
            line.push_str(&" ".repeat(logo_width));
        }

        // Add gap between ASCII art and info
        line.push_str(&" ".repeat(config.display.gap as usize));

        // Add system information line
        if info_index < visible_items.len() {
            let info_item = visible_items[info_index];
            let formatted_info = if info_item.label.is_empty() {
                // Special cases like title, underline, colors
                format_special_item_with_width(info_item, config, available_info_width)
            } else {
                format_info_item_with_width(info_item, config, available_info_width)
            };
            line.push_str(&formatted_info);
            info_index += 1;
        }

        output.push_str(&line);
        output.push('\n');
    }

    // Add color blocks at the bottom if enabled
    if config.format.color_blocks {
        let colors = system_info.get_field("colors").unwrap_or("");
        if !colors.is_empty() {
            let color_lines: Vec<&str> = colors.split('\n').collect();
            for color_line in color_lines {
                if !color_line.is_empty() {
                    // Add padding to align with the info section
                    output.push_str(&" ".repeat(logo_width + config.display.gap as usize));
                    output.push_str(color_line);
                    output.push('\n');
                }
            }
        }
    }

    Ok(output)
}

/// Get the list of information items to display
fn get_info_items(system_info: &SystemInfo, config: &Config) -> Vec<InfoItem> {
    let mut items = Vec::new();

    // Default info items (matching the original neofetch config)
    items.push(InfoItem {
        label: "".to_string(),
        value: system_info.get_field("title").unwrap_or("").to_string(),
        show: true,
    });

    items.push(InfoItem {
        label: "".to_string(),
        value: generate_underline(system_info.get_field("title").unwrap_or(""), config),
        show: config.info.underline_enabled,
    });

    items.push(InfoItem {
        label: "OS".to_string(),
        value: system_info.get_field("os").unwrap_or("").to_string(),
        show: true,
    });

    items.push(InfoItem {
        label: "Host".to_string(),
        value: system_info.get_field("host").unwrap_or("").to_string(),
        show: true,
    });

    items.push(InfoItem {
        label: "Kernel".to_string(),
        value: system_info.get_field("kernel").unwrap_or("").to_string(),
        show: true,
    });

    items.push(InfoItem {
        label: "Uptime".to_string(),
        value: system_info.get_field("uptime").unwrap_or("").to_string(),
        show: true,
    });

    items.push(InfoItem {
        label: "Packages".to_string(),
        value: system_info.get_field("packages").unwrap_or("").to_string(),
        show: true,
    });

    items.push(InfoItem {
        label: "Shell".to_string(),
        value: system_info.get_field("shell").unwrap_or("").to_string(),
        show: true,
    });

    items.push(InfoItem {
        label: "Resolution".to_string(),
        value: system_info
            .get_field("resolution")
            .unwrap_or("")
            .to_string(),
        show: !system_info.get_field("resolution").unwrap_or("").is_empty()
            && system_info.get_field("resolution").unwrap_or("") != "Unknown",
    });

    items.push(InfoItem {
        label: "DE".to_string(),
        value: system_info.get_field("de").unwrap_or("").to_string(),
        show: !system_info.get_field("de").unwrap_or("").is_empty()
            && system_info.get_field("de").unwrap_or("") != "Unknown",
    });

    items.push(InfoItem {
        label: "WM".to_string(),
        value: system_info.get_field("wm").unwrap_or("").to_string(),
        show: !system_info.get_field("wm").unwrap_or("").is_empty()
            && system_info.get_field("wm").unwrap_or("") != "Unknown",
    });

    items.push(InfoItem {
        label: "WM Theme".to_string(),
        value: system_info.get_field("wm_theme").unwrap_or("").to_string(),
        show: !system_info.get_field("wm_theme").unwrap_or("").is_empty()
            && system_info.get_field("wm_theme").unwrap_or("") != "Unknown",
    });

    items.push(InfoItem {
        label: "Theme".to_string(),
        value: system_info.get_field("theme").unwrap_or("").to_string(),
        show: !system_info.get_field("theme").unwrap_or("").is_empty()
            && system_info.get_field("theme").unwrap_or("") != "Unknown",
    });

    items.push(InfoItem {
        label: "Icons".to_string(),
        value: system_info.get_field("icons").unwrap_or("").to_string(),
        show: !system_info.get_field("icons").unwrap_or("").is_empty()
            && system_info.get_field("icons").unwrap_or("") != "Unknown",
    });

    items.push(InfoItem {
        label: "Terminal".to_string(),
        value: system_info.get_field("terminal").unwrap_or("").to_string(),
        show: true,
    });

    items.push(InfoItem {
        label: "Terminal Font".to_string(),
        value: system_info
            .get_field("terminal_font")
            .unwrap_or("")
            .to_string(),
        show: !system_info
            .get_field("terminal_font")
            .unwrap_or("")
            .is_empty()
            && system_info.get_field("terminal_font").unwrap_or("") != "Unknown",
    });

    items.push(InfoItem {
        label: "CPU".to_string(),
        value: system_info.get_field("cpu").unwrap_or("").to_string(),
        show: true,
    });

    items.push(InfoItem {
        label: "GPU".to_string(),
        value: system_info.get_field("gpu").unwrap_or("").to_string(),
        show: !system_info.get_field("gpu").unwrap_or("").is_empty()
            && system_info.get_field("gpu").unwrap_or("") != "Unknown",
    });

    items.push(InfoItem {
        label: "Memory".to_string(),
        value: system_info.get_field("memory").unwrap_or("").to_string(),
        show: true,
    });

    items
}

/// Format a regular information item with specific width
fn format_info_item_with_width(item: &InfoItem, config: &Config, max_width: usize) -> String {
    if item.value.is_empty() || item.value == "Unknown" {
        return String::new();
    }

    // Apply colors like original neofetch
    let colored_label = if config.info.bold {
        item.label.bold().cyan().to_string()
    } else {
        item.label.cyan().to_string()
    };

    let colored_separator = config.info.separator.white().to_string();
    let colored_value = item.value.white().to_string();

    let formatted = format!("{}{} {}", colored_label, colored_separator, colored_value);

    // Truncate if too long to prevent wrapping
    truncate_text(&formatted, max_width)
}

/// Truncate text to fit within specified width (accounting for ANSI escape codes)
fn truncate_text(text: &str, max_width: usize) -> String {
    // Calculate visible length (excluding ANSI escape codes)
    let visible_len = strip_ansi_for_length(text);

    if visible_len <= max_width {
        text.to_string()
    } else {
        // Truncate while preserving ANSI codes
        truncate_with_ansi(text, max_width)
    }
}

/// Truncate text while preserving ANSI escape codes
fn truncate_with_ansi(text: &str, max_width: usize) -> String {
    let mut result = String::new();
    let mut visible_count = 0;
    let mut in_escape = false;
    let mut chars = text.chars();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            in_escape = true;
            result.push(ch);
        } else if in_escape {
            result.push(ch);
            if ch == 'm' {
                in_escape = false;
            }
        } else {
            if visible_count >= max_width.saturating_sub(3) {
                result.push_str("...");
                break;
            }
            result.push(ch);
            visible_count += 1;
        }
    }

    result
}

/// Calculate the visible length of text (excluding ANSI escape codes)
fn strip_ansi_for_length(text: &str) -> usize {
    let mut length = 0;
    let mut in_escape = false;

    for ch in text.chars() {
        if ch == '\x1b' {
            in_escape = true;
        } else if in_escape && ch == 'm' {
            in_escape = false;
        } else if !in_escape {
            length += 1;
        }
    }

    length
}

/// Format special items like title, underline, colors with specific width
fn format_special_item_with_width(item: &InfoItem, config: &Config, max_width: usize) -> String {
    if item.label.is_empty() {
        // This could be title, underline, or colors
        if item.value.contains('\x1b') {
            // Already contains ANSI escape codes (like colors)
            item.value.clone()
        } else if item.value.chars().all(|c| c == '-' || c == '=' || c == '_') {
            // This is an underline
            let colored_underline = item.value.cyan().to_string();
            truncate_text(&colored_underline, max_width)
        } else {
            // This is likely the title
            let colored_title = if config.info.bold {
                item.value.bold().green().to_string()
            } else {
                item.value.green().to_string()
            };
            truncate_text(&colored_title, max_width)
        }
    } else {
        format_info_item_with_width(item, config, max_width)
    }
}

/// Generate underline for the title
fn generate_underline(title: &str, config: &Config) -> String {
    if !config.info.underline_enabled {
        return String::new();
    }

    let length = title.chars().count();
    config.info.underline_char.repeat(length)
}

/// Generate JSON output
fn generate_json_output(system_info: &SystemInfo) -> Result<String> {
    let mut json_obj = serde_json::Map::new();

    json_obj.insert(
        "title".to_string(),
        serde_json::Value::String(system_info.get_field("title").unwrap_or("").to_string()),
    );
    json_obj.insert(
        "os".to_string(),
        serde_json::Value::String(system_info.get_field("os").unwrap_or("").to_string()),
    );
    json_obj.insert(
        "host".to_string(),
        serde_json::Value::String(system_info.get_field("host").unwrap_or("").to_string()),
    );
    json_obj.insert(
        "kernel".to_string(),
        serde_json::Value::String(system_info.get_field("kernel").unwrap_or("").to_string()),
    );
    json_obj.insert(
        "uptime".to_string(),
        serde_json::Value::String(system_info.get_field("uptime").unwrap_or("").to_string()),
    );
    json_obj.insert(
        "packages".to_string(),
        serde_json::Value::String(system_info.get_field("packages").unwrap_or("").to_string()),
    );
    json_obj.insert(
        "shell".to_string(),
        serde_json::Value::String(system_info.get_field("shell").unwrap_or("").to_string()),
    );
    json_obj.insert(
        "cpu".to_string(),
        serde_json::Value::String(system_info.get_field("cpu").unwrap_or("").to_string()),
    );
    json_obj.insert(
        "gpu".to_string(),
        serde_json::Value::String(system_info.get_field("gpu").unwrap_or("").to_string()),
    );
    json_obj.insert(
        "memory".to_string(),
        serde_json::Value::String(system_info.get_field("memory").unwrap_or("").to_string()),
    );

    let json_value = serde_json::Value::Object(json_obj);
    Ok(serde_json::to_string_pretty(&json_value)?)
}

/// Generate stdout-only output (no ASCII art)
fn generate_stdout_output(
    info_items: &[InfoItem],
    system_info: &SystemInfo,
    config: &Config,
) -> Result<String> {
    let mut output = String::new();

    for item in info_items {
        if item.show && !item.value.is_empty() && item.value != "Unknown" {
            if item.label.is_empty() {
                output.push_str(&item.value);
            } else {
                output.push_str(&format!("{}: {}", item.label, item.value));
            }
            output.push('\n');
        }
    }

    // Add color blocks if enabled
    if config.format.color_blocks {
        let colors = system_info.get_field("colors").unwrap_or("");
        if !colors.is_empty() {
            output.push('\n');
            output.push_str(colors);
            output.push('\n');
        }
    }

    Ok(output)
}
