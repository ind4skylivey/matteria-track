//! Secure export functionality for MateriaTrack
//!
//! Provides encrypted exports and data sanitization.

use crate::error::{ConfigError, Result};
use crate::models::EntryWithDetails;
use crate::security::encryption::{find_gpg_binary, GpgEncryption};
use crate::security::SecureStorage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub encrypt: bool,
    pub recipient: Option<String>,
    pub sanitize: bool,
    pub password_protect: bool,
    pub format: ExportFormat,
    pub since: Option<DateTime<Utc>>,
    pub projects: Option<Vec<String>>,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            encrypt: false,
            recipient: None,
            sanitize: false,
            password_protect: false,
            format: ExportFormat::Json,
            since: None,
            projects: None,
        }
    }
}

impl ExportOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_encryption(mut self, recipient: impl Into<String>) -> Self {
        self.encrypt = true;
        self.recipient = Some(recipient.into());
        self
    }

    pub fn with_sanitization(mut self) -> Self {
        self.sanitize = true;
        self
    }

    pub fn with_password(mut self) -> Self {
        self.password_protect = true;
        self
    }

    pub fn with_format(mut self, format: ExportFormat) -> Self {
        self.format = format;
        self
    }

    pub fn with_since(mut self, since: DateTime<Utc>) -> Self {
        self.since = Some(since);
        self
    }

    pub fn with_projects(mut self, projects: Vec<String>) -> Self {
        self.projects = Some(projects);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Json,
    Csv,
    Markdown,
}

impl ExportFormat {
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Csv => "csv",
            Self::Markdown => "md",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "csv" => Self::Csv,
            "md" | "markdown" => Self::Markdown,
            _ => Self::Json,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SanitizedEntry {
    pub id: i64,
    pub project_name: String,
    pub task_name: String,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub duration_seconds: i64,
}

impl From<&EntryWithDetails> for SanitizedEntry {
    fn from(entry: &EntryWithDetails) -> Self {
        Self {
            id: entry.entry.id,
            project_name: entry.project_name.clone(),
            task_name: entry.task_name.clone(),
            start: entry.entry.start,
            end: entry.entry.end,
            duration_seconds: entry.entry.duration().num_seconds(),
        }
    }
}

pub struct SecureExporter {
    options: ExportOptions,
}

impl SecureExporter {
    pub fn new(options: ExportOptions) -> Self {
        Self { options }
    }

    pub fn export_entries<P: AsRef<Path>>(
        &self,
        entries: &[EntryWithDetails],
        output_path: P,
    ) -> Result<ExportResult> {
        let output_path = output_path.as_ref();

        let filtered = self.filter_entries(entries);

        let data = if self.options.sanitize {
            self.format_sanitized(&filtered)?
        } else {
            self.format_full(&filtered)?
        };

        let final_path = if self.options.encrypt {
            self.encrypt_and_write(data.as_bytes(), output_path)?
        } else if self.options.password_protect {
            self.create_protected_archive(data.as_bytes(), output_path)?
        } else {
            fs::write(output_path, &data)?;
            output_path.to_path_buf()
        };

        Ok(ExportResult {
            path: final_path,
            entry_count: filtered.len(),
            encrypted: self.options.encrypt,
            sanitized: self.options.sanitize,
            format: self.options.format,
        })
    }

    fn filter_entries<'a>(&self, entries: &'a [EntryWithDetails]) -> Vec<&'a EntryWithDetails> {
        entries
            .iter()
            .filter(|e| {
                if let Some(ref since) = self.options.since {
                    if e.entry.start < *since {
                        return false;
                    }
                }

                if let Some(ref projects) = self.options.projects {
                    if !projects.contains(&e.project_name) {
                        return false;
                    }
                }

                true
            })
            .collect()
    }

    fn format_full(&self, entries: &[&EntryWithDetails]) -> Result<String> {
        match self.options.format {
            ExportFormat::Json => serde_json::to_string_pretty(entries)
                .map_err(|e| crate::error::Error::Parse(e.to_string())),
            ExportFormat::Csv => self.format_csv(entries, false),
            ExportFormat::Markdown => self.format_markdown(entries, false),
        }
    }

    fn format_sanitized(&self, entries: &[&EntryWithDetails]) -> Result<String> {
        let sanitized: Vec<SanitizedEntry> =
            entries.iter().map(|e| SanitizedEntry::from(*e)).collect();

        match self.options.format {
            ExportFormat::Json => serde_json::to_string_pretty(&sanitized)
                .map_err(|e| crate::error::Error::Parse(e.to_string())),
            ExportFormat::Csv => self.format_csv(entries, true),
            ExportFormat::Markdown => self.format_markdown(entries, true),
        }
    }

    fn format_csv(&self, entries: &[&EntryWithDetails], sanitize: bool) -> Result<String> {
        let mut csv = if sanitize {
            String::from("id,project,task,start,end,duration_seconds\n")
        } else {
            String::from("id,project,task,start,end,duration_seconds,notes,git_commits\n")
        };

        for e in entries {
            let end_str = e.entry.end.map_or(String::new(), |t| t.to_rfc3339());
            let duration = e.entry.duration().num_seconds();

            if sanitize {
                csv.push_str(&format!(
                    "{},{},{},{},{},{}\n",
                    e.entry.id,
                    escape_csv(&e.project_name),
                    escape_csv(&e.task_name),
                    e.entry.start.to_rfc3339(),
                    end_str,
                    duration
                ));
            } else {
                let notes = e.entry.notes.as_deref().unwrap_or("");
                let commits = e.entry.git_commits.join("; ");
                csv.push_str(&format!(
                    "{},{},{},{},{},{},{},{}\n",
                    e.entry.id,
                    escape_csv(&e.project_name),
                    escape_csv(&e.task_name),
                    e.entry.start.to_rfc3339(),
                    end_str,
                    duration,
                    escape_csv(notes),
                    escape_csv(&commits)
                ));
            }
        }

        Ok(csv)
    }

    fn format_markdown(&self, entries: &[&EntryWithDetails], sanitize: bool) -> Result<String> {
        let mut md = String::from("# MateriaTrack Export\n\n");
        md.push_str(&format!(
            "Generated: {}\n\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        if sanitize {
            md.push_str("*Note: This export has been sanitized (notes and commits removed)*\n\n");
        }

        md.push_str("## Entries\n\n");
        md.push_str("| Date | Project | Task | Duration |\n");
        md.push_str("|------|---------|------|----------|\n");

        for e in entries {
            let date = e.entry.start.format("%Y-%m-%d");
            let duration = format_duration(e.entry.duration().num_seconds());
            md.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                date, e.project_name, e.task_name, duration
            ));
        }

        if !sanitize {
            md.push_str("\n## Details\n\n");
            for e in entries {
                md.push_str(&format!("### Entry #{}\n\n", e.entry.id));
                md.push_str(&format!("- **Project**: {}\n", e.project_name));
                md.push_str(&format!("- **Task**: {}\n", e.task_name));
                md.push_str(&format!("- **Start**: {}\n", e.entry.start.to_rfc3339()));
                if let Some(end) = e.entry.end {
                    md.push_str(&format!("- **End**: {}\n", end.to_rfc3339()));
                }
                if let Some(ref notes) = e.entry.notes {
                    md.push_str(&format!("- **Notes**: {}\n", notes));
                }
                if !e.entry.git_commits.is_empty() {
                    md.push_str("- **Git Commits**:\n");
                    for commit in &e.entry.git_commits {
                        md.push_str(&format!("  - {}\n", commit));
                    }
                }
                md.push('\n');
            }
        }

        Ok(md)
    }

    fn encrypt_and_write(&self, data: &[u8], output_path: &Path) -> Result<PathBuf> {
        let recipient = self.options.recipient.as_ref().ok_or_else(|| {
            crate::error::Error::Config(ConfigError::MissingField(
                "GPG recipient required for encryption".into(),
            ))
        })?;

        let gpg = GpgEncryption::new(recipient)?;
        let encrypted = gpg.encrypt(data)?;

        let encrypted_path =
            output_path.with_extension(format!("{}.gpg", self.options.format.extension()));

        fs::write(&encrypted_path, encrypted)?;

        Ok(encrypted_path)
    }

    fn create_protected_archive(&self, data: &[u8], output_path: &Path) -> Result<PathBuf> {
        let archive_path = output_path.with_extension("zip");
        let file = File::create(&archive_path)?;
        let mut zip = ZipWriter::new(file);

        let filename = format!("export.{}", self.options.format.extension());

        let options =
            SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        zip.start_file(&filename, options).map_err(|e| {
            crate::error::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e))
        })?;

        zip.write_all(data)?;
        zip.finish().map_err(|e| {
            crate::error::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e))
        })?;

        Ok(archive_path)
    }
}

#[derive(Debug)]
pub struct ExportResult {
    pub path: PathBuf,
    pub entry_count: usize,
    pub encrypted: bool,
    pub sanitized: bool,
    pub format: ExportFormat,
}

impl ExportResult {
    pub fn summary(&self) -> String {
        let mut parts = vec![format!("Exported {} entries", self.entry_count)];

        if self.encrypted {
            parts.push("encrypted with GPG".into());
        }
        if self.sanitized {
            parts.push("sanitized".into());
        }

        format!("{} to {}", parts.join(", "), self.path.display())
    }
}

pub fn verify_gpg_recipient(recipient: &str) -> Result<bool> {
    let gpg = find_gpg_binary().ok_or_else(|| {
        crate::error::Error::Config(ConfigError::InvalidPath("GPG not found".into()))
    })?;

    let output = std::process::Command::new(&gpg)
        .args(["--list-keys", recipient])
        .output()
        .map_err(|e| crate::error::Error::Config(ConfigError::EncryptionError(e.to_string())))?;

    Ok(output.status.success())
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn format_duration(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;

    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

pub fn decrypt_import<P: AsRef<Path>>(encrypted_path: P) -> Result<Vec<u8>> {
    let gpg = find_gpg_binary().ok_or_else(|| {
        crate::error::Error::Config(ConfigError::InvalidPath("GPG not found".into()))
    })?;

    let mut file = File::open(encrypted_path)?;
    let mut encrypted_data = Vec::new();
    file.read_to_end(&mut encrypted_data)?;

    let output = std::process::Command::new(&gpg)
        .args(["--decrypt"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(&encrypted_data)?;
            }
            child.wait_with_output()
        })
        .map_err(|e| crate::error::Error::Config(ConfigError::EncryptionError(e.to_string())))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(crate::error::Error::Config(ConfigError::EncryptionError(
            format!("Decryption failed: {}", stderr),
        )));
    }

    Ok(output.stdout)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_options_default() {
        let opts = ExportOptions::default();
        assert!(!opts.encrypt);
        assert!(!opts.sanitize);
        assert_eq!(opts.format, ExportFormat::Json);
    }

    #[test]
    fn test_export_options_builder() {
        let opts = ExportOptions::new()
            .with_encryption("user@example.com")
            .with_sanitization()
            .with_format(ExportFormat::Csv);

        assert!(opts.encrypt);
        assert!(opts.sanitize);
        assert_eq!(opts.recipient, Some("user@example.com".into()));
        assert_eq!(opts.format, ExportFormat::Csv);
    }

    #[test]
    fn test_export_format_extension() {
        assert_eq!(ExportFormat::Json.extension(), "json");
        assert_eq!(ExportFormat::Csv.extension(), "csv");
        assert_eq!(ExportFormat::Markdown.extension(), "md");
    }

    #[test]
    fn test_escape_csv() {
        assert_eq!(escape_csv("simple"), "simple");
        assert_eq!(escape_csv("with,comma"), "\"with,comma\"");
        assert_eq!(escape_csv("with\"quote"), "\"with\"\"quote\"");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0), "0m");
        assert_eq!(format_duration(3600), "1h 0m");
        assert_eq!(format_duration(5400), "1h 30m");
    }

    #[test]
    fn test_sanitized_entry() {
        use crate::models::{Entry, EntryWithDetails};

        let entry = EntryWithDetails {
            entry: Entry {
                id: 1,
                project_id: 1,
                task_id: 1,
                start: Utc::now(),
                end: Some(Utc::now()),
                notes: Some("secret notes".into()),
                git_commits: vec!["abc123".into()],
            },
            project_name: "Project".into(),
            task_name: "Task".into(),
            project_color: None,
        };

        let sanitized = SanitizedEntry::from(&entry);
        assert_eq!(sanitized.project_name, "Project");
        // Notes and commits should not be in sanitized version
    }
}
