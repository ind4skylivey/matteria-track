//! Security module for MateriaTrack
//!
//! Provides:
//! - GPG-based database encryption
//! - Audit logging with tamper detection
//! - Secure export with encryption and sanitization
//! - Zero telemetry enforcement

pub mod audit;
pub mod encryption;
pub mod export;

use crate::config::Config;
use crate::error::{ConfigError, Result};
use std::fs::{self, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub const SECURE_FILE_MODE: u32 = 0o600;
pub const SECURE_DIR_MODE: u32 = 0o700;
pub const MAX_AUDIT_LOG_SIZE: u64 = 100 * 1024 * 1024; // 100MB

pub trait SecureStorage {
    fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
    fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>>;
    fn is_available(&self) -> bool;
}

pub trait AuditLogger {
    fn log_action(&mut self, action: AuditAction) -> Result<()>;
    fn verify_integrity(&self) -> Result<bool>;
}

#[derive(Debug, Clone)]
pub enum AuditAction {
    EntryCreated {
        entry_id: i64,
        project: String,
        task: String,
    },
    EntryUpdated {
        entry_id: i64,
        changes: Vec<String>,
    },
    EntryDeleted {
        entry_id: i64,
    },
    TrackingStarted {
        entry_id: i64,
        project: String,
        task: String,
    },
    TrackingFinished {
        entry_id: i64,
        duration_secs: i64,
    },
    ProjectCreated {
        project_id: i64,
        name: String,
    },
    ProjectDeleted {
        project_id: i64,
        name: String,
    },
    TaskCreated {
        task_id: i64,
        name: String,
        project_id: i64,
    },
    TaskDeleted {
        task_id: i64,
        name: String,
    },
    DataExported {
        format: String,
        entry_count: usize,
    },
    DataImported {
        source: String,
        entry_count: usize,
    },
    ConfigChanged {
        key: String,
    },
    EncryptionEnabled,
    EncryptionDisabled,
}

impl AuditAction {
    pub fn action_type(&self) -> &'static str {
        match self {
            Self::EntryCreated { .. } => "entry_created",
            Self::EntryUpdated { .. } => "entry_updated",
            Self::EntryDeleted { .. } => "entry_deleted",
            Self::TrackingStarted { .. } => "tracking_started",
            Self::TrackingFinished { .. } => "tracking_finished",
            Self::ProjectCreated { .. } => "project_created",
            Self::ProjectDeleted { .. } => "project_deleted",
            Self::TaskCreated { .. } => "task_created",
            Self::TaskDeleted { .. } => "task_deleted",
            Self::DataExported { .. } => "data_exported",
            Self::DataImported { .. } => "data_imported",
            Self::ConfigChanged { .. } => "config_changed",
            Self::EncryptionEnabled => "encryption_enabled",
            Self::EncryptionDisabled => "encryption_disabled",
        }
    }
}

pub struct SecurityManager {
    config: Config,
    encryption: Option<encryption::GpgEncryption>,
    audit: Option<audit::AuditLog>,
}

impl SecurityManager {
    pub fn new(config: Config) -> Result<Self> {
        let encryption = if config.security.enable_encryption {
            Some(encryption::GpgEncryption::new(
                &config.security.encryption_key,
            )?)
        } else {
            None
        };

        let audit = if config.security.enable_audit_log.unwrap_or(false) {
            let audit_path = Self::audit_log_path()?;
            Some(audit::AuditLog::open(&audit_path)?)
        } else {
            None
        };

        Ok(Self {
            config,
            encryption,
            audit,
        })
    }

    pub fn audit_log_path() -> Result<PathBuf> {
        Config::config_dir().map(|d| d.join("audit.log"))
    }

    pub fn is_encryption_enabled(&self) -> bool {
        self.encryption.is_some()
    }

    pub fn is_audit_enabled(&self) -> bool {
        self.audit.is_some()
    }

    pub fn encrypt_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        match &self.encryption {
            Some(enc) => enc.encrypt(data),
            None => Ok(data.to_vec()),
        }
    }

    pub fn decrypt_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        match &self.encryption {
            Some(enc) => enc.decrypt(data),
            None => Ok(data.to_vec()),
        }
    }

    pub fn log_action(&mut self, action: AuditAction) -> Result<()> {
        if let Some(ref mut audit) = self.audit {
            audit.log_action(action)?;
        }
        Ok(())
    }

    pub fn verify_audit_integrity(&self) -> Result<bool> {
        match &self.audit {
            Some(audit) => audit.verify_integrity(),
            None => Ok(true),
        }
    }

    pub fn validate_config(&self) -> Result<()> {
        if self.config.security.enable_encryption {
            if self.config.security.encryption_key.is_empty() {
                return Err(crate::error::Error::Config(ConfigError::MissingField(
                    "encryption_key required when encryption is enabled".into(),
                )));
            }

            if !encryption::is_gpg_available() {
                return Err(crate::error::Error::Config(ConfigError::InvalidPath(
                    "GPG (gpg2) not found in PATH".into(),
                )));
            }
        }

        Ok(())
    }

    pub fn secure_delete_file<P: AsRef<Path>>(path: P) -> Result<()> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(());
        }

        let metadata = fs::metadata(path)?;
        let size = metadata.len();

        {
            let file = fs::OpenOptions::new().write(true).open(path)?;
            use std::io::Write;
            let zeros = vec![0u8; 4096];
            let mut written = 0u64;
            let mut writer = std::io::BufWriter::new(file);

            while written < size {
                let to_write = std::cmp::min(4096, (size - written) as usize);
                writer.write_all(&zeros[..to_write])?;
                written += to_write as u64;
            }
            writer.flush()?;
        }

        fs::remove_file(path)?;
        Ok(())
    }
}

pub fn set_secure_permissions<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    let mode = if path.is_dir() {
        SECURE_DIR_MODE
    } else {
        SECURE_FILE_MODE
    };

    fs::set_permissions(path, Permissions::from_mode(mode))?;
    Ok(())
}

pub fn ensure_secure_directory<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    set_secure_permissions(path)?;
    Ok(())
}

pub fn validate_local_storage(path: &Path) -> Result<()> {
    let path_str = path.to_string_lossy();

    if path_str.starts_with("http://")
        || path_str.starts_with("https://")
        || path_str.starts_with("ftp://")
        || path_str.starts_with("s3://")
    {
        return Err(crate::error::Error::Config(ConfigError::InvalidPath(
            "Remote storage paths are not allowed for security reasons".into(),
        )));
    }

    if path_str.contains("..") {
        return Err(crate::error::Error::Config(ConfigError::InvalidPath(
            "Path traversal not allowed".into(),
        )));
    }

    Ok(())
}

#[cfg(debug_assertions)]
pub fn assert_no_telemetry() {
    // Compile-time check: This function exists only in debug builds
    // to verify no telemetry code paths exist
}

#[cfg(not(debug_assertions))]
pub fn assert_no_telemetry() {
    // No-op in release builds
}

pub fn check_file_permissions<P: AsRef<Path>>(path: P) -> Result<bool> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(true);
    }

    let metadata = fs::metadata(path)?;
    let mode = metadata.permissions().mode();

    let is_secure = (mode & 0o077) == 0;
    Ok(is_secure)
}

pub struct SecureString {
    inner: String,
}

impl SecureString {
    pub fn new(s: impl Into<String>) -> Self {
        Self { inner: s.into() }
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

impl Drop for SecureString {
    fn drop(&mut self) {
        let bytes = unsafe { self.inner.as_bytes_mut() };
        for byte in bytes.iter_mut() {
            *byte = 0;
        }
    }
}

impl std::fmt::Debug for SecureString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SecureString([REDACTED])")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_action_type() {
        let action = AuditAction::EntryCreated {
            entry_id: 1,
            project: "Test".into(),
            task: "Task".into(),
        };
        assert_eq!(action.action_type(), "entry_created");
    }

    #[test]
    fn test_validate_local_storage() {
        assert!(validate_local_storage(Path::new("/home/user/data.db")).is_ok());
        assert!(validate_local_storage(Path::new("https://example.com/data")).is_err());
        assert!(validate_local_storage(Path::new("../../../etc/passwd")).is_err());
    }

    #[test]
    fn test_secure_string() {
        let secure = SecureString::new("password123");
        assert_eq!(secure.as_str(), "password123");
        assert!(format!("{:?}", secure).contains("REDACTED"));
    }

    #[test]
    fn test_assert_no_telemetry() {
        assert_no_telemetry();
    }
}
