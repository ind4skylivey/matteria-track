//! DWM/i3/Polybar statusbar integration for MateriaTrack
//!
//! Provides compact statusbar output with Nerd Font icons.

use crate::config::Config;
use crate::error::Result;
use crate::models::Entry;
use crate::theme::MateriaTheme;

use super::Integration;

pub struct DwmIntegration {
    theme: MateriaTheme,
    show_task: bool,
    show_duration: bool,
    separator: String,
}

impl DwmIntegration {
    pub fn new() -> Self {
        Self {
            theme: MateriaTheme::Fire,
            show_task: true,
            show_duration: true,
            separator: " | ".to_string(),
        }
    }

    pub fn with_theme(mut self, theme: MateriaTheme) -> Self {
        self.theme = theme;
        self
    }

    pub fn with_show_task(mut self, show: bool) -> Self {
        self.show_task = show;
        self
    }

    pub fn with_separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = sep.into();
        self
    }

    pub fn format_status(&self, project: &str, task: &str, duration_secs: i64) -> String {
        let materia = "💎";
        let duration = format_duration(duration_secs);

        if self.show_task {
            format!(
                "{} [{}] {}{}⏱ {}",
                materia, project, task, self.separator, duration
            )
        } else {
            format!("{} {}{}⏱ {}", materia, project, self.separator, duration)
        }
    }

    pub fn format_idle(&self) -> String {
        format!("{} idle", self.theme.icon())
    }

    pub fn format_entry(&self, entry: &Entry, project: &str, task: &str) -> String {
        let duration_secs = entry.duration().num_seconds();
        self.format_status(project, task, duration_secs)
    }

    pub fn format_polybar(&self, project: &str, task: &str, duration_secs: i64) -> String {
        let (r, g, b) = self.theme.primary_color();
        let color = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let duration = format_duration(duration_secs);

        format!(
            "%{{F{}}}💎%{{F-}} [{}] {} | ⏱ {}",
            color, project, task, duration
        )
    }

    pub fn format_i3blocks(&self, project: &str, task: &str, duration_secs: i64) -> String {
        let duration = format_duration(duration_secs);
        format!("💎 {} > {} | {}", project, task, duration)
    }

    pub fn format_waybar(&self, project: &str, task: &str, duration_secs: i64) -> WaybarOutput {
        let duration = format_duration(duration_secs);

        WaybarOutput {
            text: format!("💎 {} | ⏱ {}", project, duration),
            tooltip: format!(
                "Project: {}\nTask: {}\nDuration: {}",
                project, task, duration
            ),
            class: format!("materia-{}", theme_class(self.theme)),
            percentage: calculate_day_percentage(duration_secs),
        }
    }

    pub fn format_lemonbar(&self, project: &str, task: &str, duration_secs: i64) -> String {
        let (r, g, b) = self.theme.primary_color();
        let color = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let duration = format_duration(duration_secs);

        format!(
            "%{{F{}}}💎%{{F-}} {} > {}  ⏱ {}",
            color, project, task, duration
        )
    }

    pub fn format_tmux(&self, project: &str, _task: &str, duration_secs: i64) -> String {
        let (r, g, b) = self.theme.primary_color();
        let duration = format_duration(duration_secs);

        format!(
            "#[fg=colour{}]💎#[default] {} | ⏱ {}",
            rgb_to_256(r, g, b),
            project,
            duration
        )
    }

    pub fn format_custom(
        &self,
        format_string: &str,
        project: &str,
        task: &str,
        duration_secs: i64,
    ) -> String {
        let duration = format_duration(duration_secs);
        let hours = duration_secs / 3600;
        let minutes = (duration_secs % 3600) / 60;
        let seconds = duration_secs % 60;

        format_string
            .replace("{icon}", self.theme.icon())
            .replace("{materia}", "💎")
            .replace("{project}", project)
            .replace("{task}", task)
            .replace("{duration}", &duration)
            .replace("{hours}", &hours.to_string())
            .replace("{minutes}", &format!("{:02}", minutes))
            .replace("{seconds}", &format!("{:02}", seconds))
            .replace(
                "{hh:mm:ss}",
                &format!("{:02}:{:02}:{:02}", hours, minutes, seconds),
            )
            .replace("{hh:mm}", &format!("{:02}:{:02}", hours, minutes))
    }
}

impl Default for DwmIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl Integration for DwmIntegration {
    fn name(&self) -> &'static str {
        "DWM/Statusbar"
    }

    fn is_enabled(&self, _config: &Config) -> bool {
        true
    }

    fn validate_config(&self, _config: &Config) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct WaybarOutput {
    pub text: String,
    pub tooltip: String,
    pub class: String,
    pub percentage: u8,
}

fn format_duration(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

fn theme_class(theme: MateriaTheme) -> &'static str {
    match theme {
        MateriaTheme::Fire => "fire",
        MateriaTheme::Ice => "ice",
        MateriaTheme::Lightning => "lightning",
        MateriaTheme::Earth => "earth",
        MateriaTheme::Wind => "wind",
    }
}

fn calculate_day_percentage(duration_secs: i64) -> u8 {
    let eight_hours = 8 * 3600;
    let percentage = (duration_secs as f64 / eight_hours as f64 * 100.0).min(100.0);
    percentage as u8
}

fn rgb_to_256(r: u8, g: u8, b: u8) -> u8 {
    if r == g && g == b {
        if r < 8 {
            return 16;
        }
        if r > 248 {
            return 231;
        }
        return ((r as u16 - 8) / 10) as u8 + 232;
    }

    16 + 36 * (r / 51) + 6 * (g / 51) + (b / 51)
}

pub fn generate_dwm_script(config_path: &str) -> String {
    format!(
        r#"#!/bin/bash
# MateriaTrack DWM Statusbar Script
# Auto-generated - do not edit manually

MTRACK_BIN="materiatrack"
CONFIG="{}"
INTERVAL=1

while true; do
    status=$($MTRACK_BIN statusbar --short 2>/dev/null)
    if [ -n "$status" ]; then
        xsetroot -name "$status"
    else
        xsetroot -name "💎 idle"
    fi
    sleep $INTERVAL
done
"#,
        config_path
    )
}

pub fn generate_polybar_module() -> String {
    r#"[module/materiatrack]
type = custom/script
exec = materiatrack statusbar 2>/dev/null || echo "💎 idle"
interval = 1
format = <label>
format-prefix = " "
format-prefix-foreground = ${colors.primary}
label = %output%
click-left = materiatrack dashboard &
click-right = materiatrack finish
"#
    .to_string()
}

pub fn generate_i3blocks_config() -> String {
    r#"[materiatrack]
command=materiatrack statusbar 2>/dev/null || echo "💎 idle"
interval=1
color=#FF6432
"#
    .to_string()
}

pub fn generate_waybar_module() -> String {
    r#"{
  "custom/materiatrack": {
    "exec": "materiatrack statusbar --format json 2>/dev/null",
    "return-type": "json",
    "interval": 1,
    "on-click": "materiatrack dashboard",
    "on-click-right": "materiatrack finish"
  }
}
"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0), "00:00");
        assert_eq!(format_duration(65), "01:05");
        assert_eq!(format_duration(3661), "01:01:01");
    }

    #[test]
    fn test_format_status() {
        let dwm = DwmIntegration::new();
        let status = dwm.format_status("Project", "Task", 3600);

        assert!(status.contains("Project"));
        assert!(status.contains("Task"));
        assert!(status.contains("01:00:00"));
    }

    #[test]
    fn test_format_idle() {
        let dwm = DwmIntegration::new().with_theme(MateriaTheme::Ice);
        let idle = dwm.format_idle();

        assert!(idle.contains("idle"));
    }

    #[test]
    fn test_format_custom() {
        let dwm = DwmIntegration::new();
        let custom =
            dwm.format_custom("{materia} {project} ({hh:mm})", "MyProject", "MyTask", 5400);

        assert_eq!(custom, "💎 MyProject (01:30)");
    }

    #[test]
    fn test_rgb_to_256() {
        assert_eq!(rgb_to_256(255, 0, 0), 196);
        assert_eq!(rgb_to_256(0, 255, 0), 46);
        assert_eq!(rgb_to_256(0, 0, 255), 21);
    }
}
