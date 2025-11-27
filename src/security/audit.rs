//! Audit logging system for MatteriaTrack
//!
//! Provides append-only, tamper-evident audit logging with SHA256 checksums.

use crate::error::Result;
use crate::security::{set_secure_permissions, AuditAction, AuditLogger, MAX_AUDIT_LOG_SIZE};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub details: serde_json::Value,
    pub user: String,
    pub checksum: String,
    pub prev_checksum: String,
}

impl AuditEntry {
    pub fn new(action: &AuditAction, prev_checksum: &str) -> Self {
        let timestamp = Utc::now();
        let user = get_current_user();
        let action_type = action.action_type().to_string();
        let details = action_to_details(action);

        let mut entry = Self {
            timestamp,
            action: action_type,
            details,
            user,
            checksum: String::new(),
            prev_checksum: prev_checksum.to_string(),
        };

        entry.checksum = entry.calculate_checksum();
        entry
    }

    pub fn calculate_checksum(&self) -> String {
        let data = format!(
            "{}|{}|{}|{}|{}",
            self.timestamp.to_rfc3339(),
            self.action,
            self.details,
            self.user,
            self.prev_checksum
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn verify(&self) -> bool {
        let expected = self.calculate_checksum();
        self.checksum == expected
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).map_err(|e| crate::error::Error::Parse(e.to_string()))
    }

    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| crate::error::Error::Parse(e.to_string()))
    }
}

fn action_to_details(action: &AuditAction) -> serde_json::Value {
    match action {
        AuditAction::EntryCreated {
            entry_id,
            project,
            task,
        } => serde_json::json!({
            "entry_id": entry_id,
            "project": project,
            "task": task
        }),
        AuditAction::EntryUpdated { entry_id, changes } => serde_json::json!({
            "entry_id": entry_id,
            "changes": changes
        }),
        AuditAction::EntryDeleted { entry_id } => serde_json::json!({
            "entry_id": entry_id
        }),
        AuditAction::TrackingStarted {
            entry_id,
            project,
            task,
        } => serde_json::json!({
            "entry_id": entry_id,
            "project": project,
            "task": task
        }),
        AuditAction::TrackingFinished {
            entry_id,
            duration_secs,
        } => serde_json::json!({
            "entry_id": entry_id,
            "duration_secs": duration_secs
        }),
        AuditAction::ProjectCreated { project_id, name } => serde_json::json!({
            "project_id": project_id,
            "name": name
        }),
        AuditAction::ProjectDeleted { project_id, name } => serde_json::json!({
            "project_id": project_id,
            "name": name
        }),
        AuditAction::TaskCreated {
            task_id,
            name,
            project_id,
        } => serde_json::json!({
            "task_id": task_id,
            "name": name,
            "project_id": project_id
        }),
        AuditAction::TaskDeleted { task_id, name } => serde_json::json!({
            "task_id": task_id,
            "name": name
        }),
        AuditAction::DataExported {
            format,
            entry_count,
        } => serde_json::json!({
            "format": format,
            "entry_count": entry_count
        }),
        AuditAction::DataImported {
            source,
            entry_count,
        } => serde_json::json!({
            "source": source,
            "entry_count": entry_count
        }),
        AuditAction::ConfigChanged { key } => serde_json::json!({
            "key": key
        }),
        AuditAction::EncryptionEnabled => serde_json::json!({}),
        AuditAction::EncryptionDisabled => serde_json::json!({}),
    }
}

fn get_current_user() -> String {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
}

pub struct AuditLog {
    path: PathBuf,
    last_checksum: String,
}

impl AuditLog {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref().to_path_buf();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let last_checksum = if path.exists() {
            Self::get_last_checksum(&path)?
        } else {
            "genesis".to_string()
        };

        if path.exists() {
            set_secure_permissions(&path)?;
        }

        Ok(Self {
            path,
            last_checksum,
        })
    }

    fn get_last_checksum<P: AsRef<Path>>(path: P) -> Result<String> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut last_line = None;
        #[allow(clippy::lines_filter_map_ok)]
        for l in reader.lines().flatten() {
            if !l.trim().is_empty() {
                last_line = Some(l);
            }
        }

        match last_line {
            Some(line) => {
                let entry = AuditEntry::from_json(&line)?;
                Ok(entry.checksum)
            }
            None => Ok("genesis".to_string()),
        }
    }

    fn should_rotate(&self) -> Result<bool> {
        if !self.path.exists() {
            return Ok(false);
        }

        let metadata = fs::metadata(&self.path)?;
        Ok(metadata.len() >= MAX_AUDIT_LOG_SIZE)
    }

    fn rotate(&self) -> Result<()> {
        if !self.path.exists() {
            return Ok(());
        }

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let rotated_name = format!(
            "{}.{}.log",
            self.path.file_stem().unwrap_or_default().to_string_lossy(),
            timestamp
        );

        let rotated_path = self.path.with_file_name(rotated_name);
        fs::rename(&self.path, &rotated_path)?;

        set_secure_permissions(&rotated_path)?;

        Ok(())
    }

    pub fn entry_count(&self) -> Result<usize> {
        if !self.path.exists() {
            return Ok(0);
        }

        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        #[allow(clippy::lines_filter_map_ok)]
        Ok(reader.lines().filter_map(|l| l.ok()).count())
    }

    pub fn read_entries(&self, limit: Option<usize>) -> Result<Vec<AuditEntry>> {
        if !self.path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);

        let mut entries = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            let entry = AuditEntry::from_json(&line)?;
            entries.push(entry);

            if let Some(lim) = limit {
                if entries.len() >= lim {
                    break;
                }
            }
        }

        Ok(entries)
    }

    pub fn read_last_entries(&self, count: usize) -> Result<Vec<AuditEntry>> {
        let all = self.read_entries(None)?;
        let start = all.len().saturating_sub(count);
        Ok(all[start..].to_vec())
    }

    pub fn search(&self, filter: &AuditFilter) -> Result<Vec<AuditEntry>> {
        let entries = self.read_entries(None)?;

        let filtered: Vec<AuditEntry> = entries
            .into_iter()
            .filter(|e| {
                if let Some(ref action) = filter.action {
                    if e.action != *action {
                        return false;
                    }
                }
                if let Some(ref since) = filter.since {
                    if e.timestamp < *since {
                        return false;
                    }
                }
                if let Some(ref until) = filter.until {
                    if e.timestamp > *until {
                        return false;
                    }
                }
                if let Some(ref user) = filter.user {
                    if e.user != *user {
                        return false;
                    }
                }
                true
            })
            .collect();

        Ok(filtered)
    }
}

impl AuditLogger for AuditLog {
    fn log_action(&mut self, action: AuditAction) -> Result<()> {
        if self.should_rotate()? {
            self.rotate()?;
            self.last_checksum = "genesis".to_string();
        }

        let entry = AuditEntry::new(&action, &self.last_checksum);
        let json = entry.to_json()?;

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;

        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", json)?;
        writer.flush()?;

        set_secure_permissions(&self.path)?;

        self.last_checksum = entry.checksum;

        Ok(())
    }

    fn verify_integrity(&self) -> Result<bool> {
        if !self.path.exists() {
            return Ok(true);
        }

        let entries = self.read_entries(None)?;
        let mut prev_checksum = "genesis".to_string();

        for entry in entries {
            if entry.prev_checksum != prev_checksum {
                return Ok(false);
            }

            if !entry.verify() {
                return Ok(false);
            }

            prev_checksum = entry.checksum;
        }

        Ok(true)
    }
}

#[derive(Debug, Default)]
pub struct AuditFilter {
    pub action: Option<String>,
    pub since: Option<DateTime<Utc>>,
    pub until: Option<DateTime<Utc>>,
    pub user: Option<String>,
}

impl AuditFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    pub fn with_since(mut self, since: DateTime<Utc>) -> Self {
        self.since = Some(since);
        self
    }

    pub fn with_until(mut self, until: DateTime<Utc>) -> Self {
        self.until = Some(until);
        self
    }

    pub fn with_user(mut self, user: impl Into<String>) -> Self {
        self.user = Some(user.into());
        self
    }
}

pub fn format_audit_report(entries: &[AuditEntry]) -> String {
    let mut output = String::new();

    output.push_str("=== MatteriaTrack Audit Report ===\n\n");
    output.push_str(&format!("Total entries: {}\n", entries.len()));

    if let (Some(first), Some(last)) = (entries.first(), entries.last()) {
        output.push_str(&format!(
            "Period: {} to {}\n",
            first.timestamp.format("%Y-%m-%d %H:%M:%S"),
            last.timestamp.format("%Y-%m-%d %H:%M:%S")
        ));
    }

    output.push_str("\n--- Entries ---\n\n");

    for entry in entries {
        output.push_str(&format!(
            "[{}] {} by {}\n  Details: {}\n  Checksum: {}...{}\n\n",
            entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
            entry.action,
            entry.user,
            entry.details,
            &entry.checksum[..8],
            &entry.checksum[entry.checksum.len() - 8..]
        ));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_audit_entry_checksum() {
        let action = AuditAction::EntryCreated {
            entry_id: 1,
            project: "Test".into(),
            task: "Task".into(),
        };

        let entry = AuditEntry::new(&action, "genesis");
        assert!(!entry.checksum.is_empty());
        assert!(entry.verify());
    }

    #[test]
    fn test_audit_entry_tamper_detection() {
        let action = AuditAction::EntryCreated {
            entry_id: 1,
            project: "Test".into(),
            task: "Task".into(),
        };

        let mut entry = AuditEntry::new(&action, "genesis");
        entry.action = "tampered".to_string();
        assert!(!entry.verify());
    }

    #[test]
    fn test_audit_log_operations() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("audit.log");

        let mut log = AuditLog::open(&path).unwrap();

        let action = AuditAction::TrackingStarted {
            entry_id: 1,
            project: "Test".into(),
            task: "Task".into(),
        };

        log.log_action(action).unwrap();

        assert_eq!(log.entry_count().unwrap(), 1);
        assert!(log.verify_integrity().unwrap());
    }

    #[test]
    fn test_audit_filter() {
        let filter = AuditFilter::new()
            .with_action("entry_created")
            .with_user("testuser");

        assert_eq!(filter.action, Some("entry_created".to_string()));
        assert_eq!(filter.user, Some("testuser".to_string()));
    }

    #[test]
    fn test_get_current_user() {
        let user = get_current_user();
        assert!(!user.is_empty());
    }
}
