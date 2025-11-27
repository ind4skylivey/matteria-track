//! Configuration management for MatteriaTrack

use crate::error::{ConfigError, Result};
use crate::theme::MateriaTheme;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

const CONFIG_FILE: &str = "config.toml";
const APP_DIR: &str = "materiatrack";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub ui: UiConfig,
    pub tracking: TrackingConfig,
    pub notifications: NotificationConfig,
    pub integrations: IntegrationConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingConfig {
    pub auto_import_git: bool,
    pub git_repo_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enable: bool,
    #[serde(default)]
    pub reminder_interval: Option<u64>,
    #[serde(default)]
    pub daily_summary_hour: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub obsidian_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enable_encryption: bool,
    pub encryption_key: String,
    #[serde(default)]
    pub enable_audit_log: Option<bool>,
    #[serde(default)]
    pub audit_log_path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                path: default_db_path(),
            },
            ui: UiConfig {
                theme: "fire".to_string(),
            },
            tracking: TrackingConfig {
                auto_import_git: false,
                git_repo_path: String::new(),
            },
            notifications: NotificationConfig {
                enable: false,
                reminder_interval: None,
                daily_summary_hour: None,
            },
            integrations: IntegrationConfig {
                obsidian_path: String::new(),
            },
            security: SecurityConfig {
                enable_encryption: false,
                encryption_key: String::new(),
                enable_audit_log: Some(false),
                audit_log_path: None,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| ConfigError::ParseError(format!("Failed to read config: {}", e)))?;

        let config: Config =
            toml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;

        config.validate()?;
        Ok(config)
    }

    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| ConfigError::NotFound(format!("{}: {}", path.as_ref().display(), e)))?;

        let config: Config =
            toml::from_str(&content).map_err(|e| ConfigError::ParseError(e.to_string()))?;

        config.validate()?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content =
            toml::to_string_pretty(self).map_err(|e| ConfigError::ParseError(e.to_string()))?;

        fs::write(&config_path, content)?;
        Ok(())
    }

    pub fn config_dir() -> Result<PathBuf> {
        dirs::config_dir().map(|p| p.join(APP_DIR)).ok_or_else(|| {
            ConfigError::InvalidPath("Cannot determine config directory".into()).into()
        })
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join(CONFIG_FILE))
    }

    pub fn data_dir() -> Result<PathBuf> {
        dirs::data_local_dir()
            .map(|p| p.join(APP_DIR))
            .ok_or_else(|| {
                ConfigError::InvalidPath("Cannot determine data directory".into()).into()
            })
    }

    fn validate(&self) -> Result<()> {
        if self.security.enable_encryption && self.security.encryption_key.is_empty() {
            return Err(ConfigError::MissingField(
                "encryption_key required when encryption is enabled".into(),
            )
            .into());
        }

        Ok(())
    }

    pub fn theme(&self) -> MateriaTheme {
        self.ui.theme.parse().unwrap_or(MateriaTheme::Fire)
    }

    pub fn db_path(&self) -> Result<PathBuf> {
        expand_path(&self.database.path)
    }

    pub fn git_repo_path(&self) -> Result<Option<PathBuf>> {
        if self.tracking.git_repo_path.is_empty() {
            return Ok(None);
        }
        expand_path(&self.tracking.git_repo_path).map(Some)
    }

    pub fn obsidian_path(&self) -> Result<Option<PathBuf>> {
        if self.integrations.obsidian_path.is_empty() {
            return Ok(None);
        }
        expand_path(&self.integrations.obsidian_path).map(Some)
    }
}

pub fn expand_path(path: &str) -> Result<PathBuf> {
    if path.starts_with("~/") {
        dirs::home_dir()
            .map(|home| home.join(&path[2..]))
            .ok_or_else(|| ConfigError::InvalidPath("Cannot expand home directory".into()).into())
    } else {
        Ok(PathBuf::from(path))
    }
}

fn default_db_path() -> String {
    dirs::data_local_dir()
        .map(|p| {
            p.join(APP_DIR)
                .join("materiatrack.db")
                .to_string_lossy()
                .to_string()
        })
        .unwrap_or_else(|| "~/.local/share/materiatrack/materiatrack.db".to_string())
}

pub struct EncryptionManager {
    enabled: bool,
    key_id: String,
}

impl EncryptionManager {
    pub fn new(config: &SecurityConfig) -> Self {
        Self {
            enabled: config.enable_encryption,
            key_id: config.encryption_key.clone(),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if !self.enabled {
            return Ok(data.to_vec());
        }

        use std::process::Command;
        let output = Command::new("gpg")
            .args(["--encrypt", "--armor", "-r", &self.key_id])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(ref mut stdin) = child.stdin {
                    stdin.write_all(data)?;
                }
                child.wait_with_output()
            })
            .map_err(|e| ConfigError::EncryptionError(e.to_string()))?;

        if !output.status.success() {
            return Err(ConfigError::EncryptionError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            )
            .into());
        }

        Ok(output.stdout)
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>> {
        if !self.enabled {
            return Ok(data.to_vec());
        }

        use std::process::Command;
        let output = Command::new("gpg")
            .args(["--decrypt"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(ref mut stdin) = child.stdin {
                    stdin.write_all(data)?;
                }
                child.wait_with_output()
            })
            .map_err(|e| ConfigError::EncryptionError(e.to_string()))?;

        if !output.status.success() {
            return Err(ConfigError::EncryptionError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            )
            .into());
        }

        Ok(output.stdout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(!config.security.enable_encryption);
        assert_eq!(config.ui.theme, "fire");
    }

    #[test]
    fn test_expand_path() {
        let path = expand_path("/absolute/path").unwrap();
        assert_eq!(path, PathBuf::from("/absolute/path"));
    }

    #[test]
    fn test_theme_parsing() {
        let config = Config::default();
        assert_eq!(config.theme(), MateriaTheme::Fire);
    }
}
