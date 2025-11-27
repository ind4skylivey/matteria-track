//! Integration modules for MatteriaTrack
//!
//! Provides external integrations:
//! - Git: Auto-import commits during tracking sessions
//! - Obsidian: Bidirectional sync with daily notes
//! - DWM: Statusbar output with Nerd Font icons
//! - Zeit: Import from Zeit time tracker database

pub mod dwm;
pub mod git;
pub mod obsidian;
pub mod zeit;

use crate::config::Config;
use crate::error::Result;
use crate::models::Entry;
use std::path::PathBuf;

pub trait Integration {
    fn name(&self) -> &'static str;
    fn is_enabled(&self, config: &Config) -> bool;
    fn validate_config(&self, config: &Config) -> Result<()>;
}

pub trait TimeExporter: Integration {
    fn export_entry(&self, entry: &Entry, config: &Config) -> Result<()>;
    fn export_entries(&self, entries: &[Entry], config: &Config) -> Result<()> {
        for entry in entries {
            self.export_entry(entry, config)?;
        }
        Ok(())
    }
}

pub trait TimeImporter: Integration {
    fn import_entries(&self, config: &Config) -> Result<Vec<Entry>>;
}

#[derive(Debug, Clone)]
pub struct IntegrationStatus {
    pub name: String,
    pub enabled: bool,
    pub configured: bool,
    pub last_sync: Option<chrono::DateTime<chrono::Utc>>,
    pub error: Option<String>,
}

impl IntegrationStatus {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            enabled: false,
            configured: false,
            last_sync: None,
            error: None,
        }
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    pub fn with_configured(mut self, configured: bool) -> Self {
        self.configured = configured;
        self
    }

    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
}

pub struct IntegrationManager {
    config: Config,
}

impl IntegrationManager {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn git(&self) -> git::GitIntegration {
        git::GitIntegration::new()
    }

    pub fn obsidian(&self) -> obsidian::ObsidianIntegration {
        obsidian::ObsidianIntegration::new()
    }

    pub fn dwm(&self) -> dwm::DwmIntegration {
        dwm::DwmIntegration::new()
    }

    pub fn zeit(&self) -> zeit::ZeitImporter {
        zeit::ZeitImporter::new()
    }

    pub fn status_all(&self) -> Vec<IntegrationStatus> {
        vec![
            self.check_git_status(),
            self.check_obsidian_status(),
            self.check_dwm_status(),
        ]
    }

    fn check_git_status(&self) -> IntegrationStatus {
        let git = self.git();
        let enabled = git.is_enabled(&self.config);
        let configured = self.config.tracking.auto_import_git;

        IntegrationStatus::new("Git")
            .with_enabled(enabled)
            .with_configured(configured)
    }

    fn check_obsidian_status(&self) -> IntegrationStatus {
        let obsidian = self.obsidian();
        let enabled = obsidian.is_enabled(&self.config);
        let configured = !self.config.integrations.obsidian_path.is_empty();

        let mut status = IntegrationStatus::new("Obsidian")
            .with_enabled(enabled)
            .with_configured(configured);

        if configured {
            if let Err(e) = obsidian.validate_config(&self.config) {
                status = status.with_error(e.to_string());
            }
        }

        status
    }

    fn check_dwm_status(&self) -> IntegrationStatus {
        IntegrationStatus::new("DWM/Statusbar")
            .with_enabled(true)
            .with_configured(true)
    }
}

pub fn detect_git_repo(path: &std::path::Path) -> Option<PathBuf> {
    let mut current = path.to_path_buf();

    loop {
        let git_dir = current.join(".git");
        if git_dir.exists() {
            return Some(current);
        }

        if !current.pop() {
            break;
        }
    }

    None
}

pub fn expand_path(path: &str) -> Result<PathBuf> {
    if path.starts_with("~/") {
        dirs::home_dir()
            .map(|home| home.join(&path[2..]))
            .ok_or_else(|| {
                crate::error::Error::Config(crate::error::ConfigError::InvalidPath(
                    "Cannot expand home directory".into(),
                ))
            })
    } else if let Some(stripped) = path.strip_prefix('$') {
        let var_end = stripped
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .unwrap_or(stripped.len());
        let var_name = &stripped[..var_end];
        let rest = &stripped[var_end..];

        std::env::var(var_name)
            .map(|val| PathBuf::from(val).join(rest.trim_start_matches('/')))
            .map_err(|_| {
                crate::error::Error::Config(crate::error::ConfigError::InvalidPath(format!(
                    "Environment variable ${} not set",
                    var_name
                )))
            })
    } else {
        Ok(PathBuf::from(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_path_absolute() {
        let path = expand_path("/absolute/path").unwrap();
        assert_eq!(path, PathBuf::from("/absolute/path"));
    }

    #[test]
    fn test_expand_path_home() {
        let path = expand_path("~/test").unwrap();
        assert!(path.to_string_lossy().contains("test"));
        assert!(!path.to_string_lossy().starts_with("~"));
    }

    #[test]
    fn test_integration_status() {
        let status = IntegrationStatus::new("Test")
            .with_enabled(true)
            .with_configured(true);

        assert_eq!(status.name, "Test");
        assert!(status.enabled);
        assert!(status.configured);
    }
}
