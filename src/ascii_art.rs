//! ASCII art and logo management for neofetch-rs
//!
//! This module handles the ASCII art logos for different operating systems and distributions.

use colored::*;
use std::collections::HashMap;

/// ASCII art manager
pub struct AsciiArt {
    logos: HashMap<String, Vec<String>>,
    colors: HashMap<String, Vec<Color>>,
}

impl AsciiArt {
    /// Create a new ASCII art manager
    pub fn new() -> Self {
        let mut ascii_art = Self {
            logos: HashMap::new(),
            colors: HashMap::new(),
        };

        ascii_art.load_default_logos();
        ascii_art
    }

    /// Load default ASCII logos for various operating systems
    fn load_default_logos(&mut self) {
        // macOS logo
        self.logos.insert(
            "macos".to_string(),
            vec![
                "                    'c.".to_string(),
                "                 ,xNMM.".to_string(),
                "               .OMMMMo".to_string(),
                "               OMMM0,".to_string(),
                "     .;loddo:' loolloddol;.".to_string(),
                "   cKMMMMMMMMMMNWMMMMMMMMMM0:".to_string(),
                " .KMMMMMMMMMMMMMMMMMMMMMMMWd.".to_string(),
                " XMMMMMMMMMMMMMMMMMMMMMMMX.".to_string(),
                ";MMMMMMMMMMMMMMMMMMMMMMMM:".to_string(),
                ":MMMMMMMMMMMMMMMMMMMMMMMM:".to_string(),
                ".MMMMMMMMMMMMMMMMMMMMMMMMX.".to_string(),
                " kMMMMMMMMMMMMMMMMMMMMMMMMWd.".to_string(),
                " .XMMMMMMMMMMMMMMMMMMMMMMMMMMk".to_string(),
                "  .XMMMMMMMMMMMMMMMMMMMMMMMMK.".to_string(),
                "    kMMMMMMMMMMMMMMMMMMMMMMd".to_string(),
                "     ;KMMMMMMMWXXWMMMMMMMk.".to_string(),
                "       .cooc,.    .,coo:.".to_string(),
            ],
        );

        self.colors.insert(
            "macos".to_string(),
            vec![
                Color::Green,
                Color::Yellow,
                Color::Red,
                Color::Magenta,
                Color::Blue,
                Color::Cyan,
            ],
        );

        // Ubuntu logo
        self.logos.insert(
            "ubuntu".to_string(),
            vec![
                "         _".to_string(),
                "     ---(_)".to_string(),
                " _/  ---  \\".to_string(),
                "(_) |   |".to_string(),
                "  \\  --- _/".to_string(),
                "     ---(_)".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
        );

        self.colors
            .insert("ubuntu".to_string(), vec![Color::Red, Color::White]);

        // Arch Linux logo
        self.logos.insert(
            "arch".to_string(),
            vec![
                "                   -`".to_string(),
                "                  .o+`".to_string(),
                "                 `ooo/".to_string(),
                "                `+oooo:".to_string(),
                "               `+oooooo:".to_string(),
                "               -+oooooo+:".to_string(),
                "             `/:-:++oooo+:".to_string(),
                "            `/++++/+++++++:".to_string(),
                "           `/++++++++++++++:".to_string(),
                "          `/+++ooooooooo+++/".to_string(),
                "         ./ooosssso++osssssso+`".to_string(),
                "        .oossssso-````/ossssss+`".to_string(),
                "       -osssssso.      :ssssssso.".to_string(),
                "      :osssssss/        osssso+++.".to_string(),
                "     /ossssssss/        +ssssooo/-".to_string(),
                "   `/ossssso+/:-        -:/+osssso+-".to_string(),
                "  `+sso+:-`                 `.-/+oso:".to_string(),
                " `++:.                           `-/+/".to_string(),
                " .`                                 `/".to_string(),
            ],
        );

        self.colors
            .insert("arch".to_string(), vec![Color::Cyan, Color::Blue]);

        // Debian logo
        self.logos.insert(
            "debian".to_string(),
            vec![
                "       _,met$$$$$gg.".to_string(),
                "    ,g$$$$$$$$$$$$$$$P.".to_string(),
                r#"  ,g$$P"     """Y$$."."#.to_string(),
                r#" ,$$P'              `$$$."#.to_string(),
                r#"',$$P       ,ggs.     `$$b:"#.to_string(),
                r#"`d$$'     ,$P"'   .    $$$"#.to_string(),
                r#" $$P      d$'     ,    $$P"#.to_string(),
                r#" $$:      $$.   -    ,d$$'"#.to_string(),
                r#" $$;      Y$b._   _,d$P'"#.to_string(),
                r#" Y$$.    `.`"Y$$$$P"'"#.to_string(),
                r#" `$$b      "-.__"#.to_string(),
                "  `Y$$".to_string(),
                "   `Y$$.".to_string(),
                "     `$$b.".to_string(),
                "       `Y$$b.".to_string(),
                r#"          `"Y$b._"#.to_string(),
                r#"              `"""#.to_string(),
            ],
        );

        self.colors
            .insert("debian".to_string(), vec![Color::Red, Color::White]);

        // Fedora logo
        self.logos.insert(
            "fedora".to_string(),
            vec![
                "             .',;::::;,'.".to_string(),
                "         .';;;;;;;;;;;;;,'.".to_string(),
                "      .,;;;;;;;;;;;;;;;;;;;,.".to_string(),
                "    .,;;;;;;;;;;;;;;;;;;;;;;;;,".to_string(),
                "   .;;;;;;;;;;;;;;;;;;;;;;;;;;;;;,".to_string(),
                "  .;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;,".to_string(),
                " .;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;,".to_string(),
                ".;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;,".to_string(),
                ";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;".to_string(),
                ";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;".to_string(),
                ";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;".to_string(),
                ";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;".to_string(),
                ".;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;".to_string(),
                " ';;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;'".to_string(),
                "  ';;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;'".to_string(),
                "    ';;;;;;;;;;;;;;;;;;;;;;;;;;;;;;'".to_string(),
                "        '''''''''''''''''''''".to_string(),
            ],
        );

        self.colors
            .insert("fedora".to_string(), vec![Color::Blue, Color::White]);

        // Generic Linux logo
        self.logos.insert(
            "linux".to_string(),
            vec![
                "        #####".to_string(),
                "       #######".to_string(),
                "       ##O#O##".to_string(),
                "       #VVVVV#".to_string(),
                "       ##>|<##".to_string(),
                "      #########".to_string(),
                "     ###########".to_string(),
                "    #############".to_string(),
                "   ###############".to_string(),
                "  #################".to_string(),
                " ###################".to_string(),
                "#####################".to_string(),
                "#####################".to_string(),
                "#####################".to_string(),
                " ###################".to_string(),
                "  #################".to_string(),
                "   ###############".to_string(),
            ],
        );

        self.colors
            .insert("linux".to_string(), vec![Color::Yellow, Color::White]);

        // Windows logo
        self.logos.insert(
            "windows".to_string(),
            vec![
                "        ,.=:!!t3Z3z.,".to_string(),
                "       :tt:::tt333EE3".to_string(),
                "       Et:::ztt33EEEL".to_string(),
                "      @Ee., ..,]3333".to_string(),
                "     ;[=t]::;]3333333".to_string(),
                "    :t]::;]]]3333333".to_string(),
                "   :]]]]]]]]]]333333".to_string(),
                "  .]]]]]]]]]]]33333".to_string(),
                " .]]]]]]]]]]]]3333".to_string(),
                ".]]]]]]]]]]]]333".to_string(),
                "]]]]]]]]]]]]]33".to_string(),
                "3]]]]]]]]]]]]3".to_string(),
                "33]]]]]]]]]]3".to_string(),
                "333]]]]]]]]3".to_string(),
                "3333]]]]]]3".to_string(),
                "33333]]]]3".to_string(),
                "333333]]3".to_string(),
            ],
        );

        self.colors.insert(
            "windows".to_string(),
            vec![Color::Blue, Color::Red, Color::Green, Color::Yellow],
        );
    }

    /// Get ASCII logo for a specific OS/distribution
    pub fn get_logo(&self, os_name: &str) -> Option<&Vec<String>> {
        let normalized_name = os_name.to_lowercase();

        // Try exact match first
        if let Some(logo) = self.logos.get(&normalized_name) {
            return Some(logo);
        }

        // Try partial matches
        if normalized_name.contains("ubuntu") {
            return self.logos.get("ubuntu");
        } else if normalized_name.contains("arch") {
            return self.logos.get("arch");
        } else if normalized_name.contains("debian") {
            return self.logos.get("debian");
        } else if normalized_name.contains("fedora") {
            return self.logos.get("fedora");
        } else if normalized_name.contains("mac") || normalized_name.contains("darwin") {
            return self.logos.get("macos");
        } else if normalized_name.contains("windows") {
            return self.logos.get("windows");
        } else if normalized_name.contains("linux") {
            return self.logos.get("linux");
        }

        // Default to generic Linux logo
        self.logos.get("linux")
    }

    /// Get colors for a specific OS/distribution
    pub fn get_colors(&self, os_name: &str) -> Option<&Vec<Color>> {
        let normalized_name = os_name.to_lowercase();

        // Try exact match first
        if let Some(colors) = self.colors.get(&normalized_name) {
            return Some(colors);
        }

        // Try partial matches
        if normalized_name.contains("ubuntu") {
            return self.colors.get("ubuntu");
        } else if normalized_name.contains("arch") {
            return self.colors.get("arch");
        } else if normalized_name.contains("debian") {
            return self.colors.get("debian");
        } else if normalized_name.contains("fedora") {
            return self.colors.get("fedora");
        } else if normalized_name.contains("mac") || normalized_name.contains("darwin") {
            return self.colors.get("macos");
        } else if normalized_name.contains("windows") {
            return self.colors.get("windows");
        } else if normalized_name.contains("linux") {
            return self.colors.get("linux");
        }

        // Default to generic Linux colors
        self.colors.get("linux")
    }

    /// Apply colors to ASCII art lines
    pub fn colorize_logo(&self, os_name: &str, logo: &[String]) -> Vec<String> {
        let default_colors = vec![Color::White];
        let colors = self.get_colors(os_name).unwrap_or(&default_colors);
        let mut colored_lines = Vec::new();

        for (i, line) in logo.iter().enumerate() {
            let color_index = i % colors.len();
            let color = &colors[color_index];

            let colored_line = match color {
                Color::Red => line.red().to_string(),
                Color::Green => line.green().to_string(),
                Color::Yellow => line.yellow().to_string(),
                Color::Blue => line.blue().to_string(),
                Color::Magenta => line.magenta().to_string(),
                Color::Cyan => line.cyan().to_string(),
                Color::White => line.white().to_string(),
                _ => line.to_string(),
            };

            colored_lines.push(colored_line);
        }

        colored_lines
    }

    /// Get the width of the ASCII logo (excluding ANSI escape codes)
    pub fn get_logo_width(&self, os_name: &str) -> usize {
        if let Some(logo) = self.get_logo(os_name) {
            logo.iter()
                .map(|line| self.strip_ansi_codes(line).chars().count())
                .max()
                .unwrap_or(0)
        } else {
            0
        }
    }

    /// Strip ANSI escape codes from a string to get the actual display width
    pub fn strip_ansi_codes(&self, text: &str) -> String {
        // Simple ANSI escape code removal
        let mut result = String::new();
        let mut in_escape = false;
        let mut chars = text.chars();

        while let Some(ch) = chars.next() {
            if ch == '\x1b' {
                in_escape = true;
            } else if in_escape && ch == 'm' {
                in_escape = false;
            } else if !in_escape {
                result.push(ch);
            }
        }

        result
    }

    /// Get the height of the ASCII logo
    pub fn get_logo_height(&self, os_name: &str) -> usize {
        if let Some(logo) = self.get_logo(os_name) {
            logo.len()
        } else {
            0
        }
    }
}
