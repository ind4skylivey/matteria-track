//! Theme system for MatteriaTrack
//!
//! Provides customizable Materia-themed color palettes and icons.

pub mod materia;

use colored::{Color, Colorize};

pub use materia::{MateriaTheme, THEMES};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorPalette {
    pub primary: (u8, u8, u8),
    pub secondary: (u8, u8, u8),
    pub accent: (u8, u8, u8),
    pub text: (u8, u8, u8),
    pub muted: (u8, u8, u8),
    pub success: (u8, u8, u8),
    pub warning: (u8, u8, u8),
    pub error: (u8, u8, u8),
}

impl ColorPalette {
    pub const fn new(primary: (u8, u8, u8), secondary: (u8, u8, u8), accent: (u8, u8, u8)) -> Self {
        Self {
            primary,
            secondary,
            accent,
            text: (255, 255, 255),
            muted: (128, 128, 128),
            success: (100, 255, 100),
            warning: (255, 200, 50),
            error: (255, 80, 80),
        }
    }

    pub fn primary_color(&self) -> Color {
        Color::TrueColor {
            r: self.primary.0,
            g: self.primary.1,
            b: self.primary.2,
        }
    }

    pub fn secondary_color(&self) -> Color {
        Color::TrueColor {
            r: self.secondary.0,
            g: self.secondary.1,
            b: self.secondary.2,
        }
    }

    pub fn accent_color(&self) -> Color {
        Color::TrueColor {
            r: self.accent.0,
            g: self.accent.1,
            b: self.accent.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IconSet {
    pub materia: &'static str,
    pub project: &'static str,
    pub task: &'static str,
    pub time: &'static str,
    pub check: &'static str,
    pub cross: &'static str,
    pub arrow: &'static str,
    pub star: &'static str,
    pub fire: &'static str,
    pub trophy: &'static str,
    pub git: &'static str,
    pub calendar: &'static str,
}

impl Default for IconSet {
    fn default() -> Self {
        Self {
            materia: "💎",
            project: "",
            task: "",
            time: "",
            check: "✓",
            cross: "✗",
            arrow: "→",
            star: "⭐",
            fire: "🔥",
            trophy: "🏆",
            git: "",
            calendar: "",
        }
    }
}

pub trait Theme: Send + Sync {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn palette(&self) -> &ColorPalette;
    fn icons(&self) -> &IconSet;
    fn element_icon(&self) -> &'static str;

    fn format_primary(&self, text: &str) -> String {
        let (r, g, b) = self.palette().primary;
        text.truecolor(r, g, b).to_string()
    }

    fn format_secondary(&self, text: &str) -> String {
        let (r, g, b) = self.palette().secondary;
        text.truecolor(r, g, b).to_string()
    }

    fn format_accent(&self, text: &str) -> String {
        let (r, g, b) = self.palette().accent;
        text.truecolor(r, g, b).to_string()
    }

    fn format_success(&self, text: &str) -> String {
        let (r, g, b) = self.palette().success;
        text.truecolor(r, g, b).to_string()
    }

    fn format_warning(&self, text: &str) -> String {
        let (r, g, b) = self.palette().warning;
        text.truecolor(r, g, b).to_string()
    }

    fn format_error(&self, text: &str) -> String {
        let (r, g, b) = self.palette().error;
        text.truecolor(r, g, b).to_string()
    }

    fn format_muted(&self, text: &str) -> String {
        let (r, g, b) = self.palette().muted;
        text.truecolor(r, g, b).to_string()
    }

    fn preview(&self) -> String {
        let _p = self.palette();
        let i = self.icons();

        let mut output = String::new();

        output.push_str(&format!(
            "\n{} {} Theme: {} {}\n",
            i.materia,
            self.element_icon(),
            self.name(),
            i.materia
        ));
        output.push_str(&"━".repeat(50));
        output.push('\n');

        output.push_str(&format!(
            "\n{}\n",
            self.format_primary(&format!("Primary: {} {}", i.star, self.description()))
        ));
        output.push_str(&format!(
            "{}\n",
            self.format_secondary(&format!("Secondary: {} Supporting color", i.project))
        ));
        output.push_str(&format!(
            "{}\n",
            self.format_accent(&format!("Accent: {} Highlight color", i.fire))
        ));

        output.push_str(&format!("\n{}\n", "Status Colors:".bold()));
        output.push_str(&format!("{} ", self.format_success("Success")));
        output.push_str(&format!("{} ", self.format_warning("Warning")));
        output.push_str(&format!("{}\n", self.format_error("Error")));

        output.push_str(&format!("\n{}\n", "Icons:".bold()));
        output.push_str(&format!(
            "{} {} {} {} {} {} {} {}\n",
            i.materia, i.project, i.task, i.time, i.check, i.star, i.trophy, i.git
        ));

        output.push('\n');
        output.push_str(&"━".repeat(50));
        output.push('\n');

        output
    }
}

pub struct ThemeManager {
    current: MateriaTheme,
}

impl ThemeManager {
    pub fn new(theme_name: &str) -> Self {
        Self {
            current: MateriaTheme::from_name(theme_name),
        }
    }

    pub fn current(&self) -> &MateriaTheme {
        &self.current
    }

    pub fn set_theme(&mut self, name: &str) {
        self.current = MateriaTheme::from_name(name);
    }

    pub fn list_themes() -> Vec<&'static str> {
        THEMES.iter().map(|t| t.name()).collect()
    }

    pub fn preview_all() -> String {
        let mut output = String::new();

        output.push_str("\n💎 Available Materia Themes 💎\n");
        output.push_str(&"═".repeat(50));
        output.push('\n');

        for theme in THEMES.iter() {
            output.push_str(&format!(
                "\n{} {} - {}\n",
                theme.element_icon(),
                theme.format_primary(theme.name()),
                theme.description()
            ));

            let p = theme.palette();
            output.push_str(&format!(
                "   Colors: {} {} {}\n",
                "██".truecolor(p.primary.0, p.primary.1, p.primary.2),
                "██".truecolor(p.secondary.0, p.secondary.1, p.secondary.2),
                "██".truecolor(p.accent.0, p.accent.1, p.accent.2),
            ));
        }

        output.push('\n');
        output.push_str(&"═".repeat(50));
        output.push('\n');

        output
    }

    pub fn get_theme(name: &str) -> Option<&'static MateriaTheme> {
        THEMES.iter().find(|t| t.name().eq_ignore_ascii_case(name))
    }
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new("fire")
    }
}

pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some((r, g, b))
}

pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#FF4500"), Some((255, 69, 0)));
        assert_eq!(hex_to_rgb("00CED1"), Some((0, 206, 209)));
        assert_eq!(hex_to_rgb("invalid"), None);
    }

    #[test]
    fn test_rgb_to_hex() {
        assert_eq!(rgb_to_hex(255, 69, 0), "#FF4500");
        assert_eq!(rgb_to_hex(0, 206, 209), "#00CED1");
    }

    #[test]
    fn test_theme_manager() {
        let mut manager = ThemeManager::new("fire");
        assert_eq!(manager.current().name(), "Fire");

        manager.set_theme("ice");
        assert_eq!(manager.current().name(), "Ice");
    }

    #[test]
    fn test_list_themes() {
        let themes = ThemeManager::list_themes();
        assert!(themes.contains(&"Fire"));
        assert!(themes.contains(&"Ice"));
        assert!(themes.contains(&"Bahamut"));
    }

    #[test]
    fn test_color_palette() {
        let palette = ColorPalette::new((255, 0, 0), (0, 255, 0), (0, 0, 255));
        assert_eq!(palette.primary, (255, 0, 0));
        assert_eq!(palette.secondary, (0, 255, 0));
        assert_eq!(palette.accent, (0, 0, 255));
    }
}
