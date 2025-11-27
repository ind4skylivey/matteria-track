//! Obsidian integration for MateriaTrack
//!
//! Bidirectional sync with Obsidian daily notes.
//! Exports time entries to markdown and imports time blocks from notes.

use crate::config::Config;
use crate::error::{ConfigError, Result};
use crate::models::{Entry, EntryWithDetails};
use chrono::{DateTime, Local, NaiveDate, NaiveTime, TimeZone, Utc};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::Integration;

const MATERIATRACK_HEADER: &str = "## MateriaTrack";
const TIME_BLOCK_PATTERN: &str = r"- \[(\d{2}:\d{2})-(\d{2}:\d{2})\] (.+?) > (.+)";

pub struct ObsidianIntegration {
    vault_path: Option<PathBuf>,
    daily_notes_folder: String,
    date_format: String,
}

impl ObsidianIntegration {
    pub fn new() -> Self {
        Self {
            vault_path: None,
            daily_notes_folder: "daily".to_string(),
            date_format: "%Y-%m-%d".to_string(),
        }
    }

    pub fn with_vault(mut self, path: PathBuf) -> Self {
        self.vault_path = Some(path);
        self
    }

    pub fn with_daily_folder(mut self, folder: impl Into<String>) -> Self {
        self.daily_notes_folder = folder.into();
        self
    }

    pub fn with_date_format(mut self, format: impl Into<String>) -> Self {
        self.date_format = format.into();
        self
    }

    pub fn from_config(config: &Config) -> Result<Self> {
        if config.integrations.obsidian_path.is_empty() {
            return Err(crate::error::Error::Config(ConfigError::MissingField(
                "obsidian_path".into(),
            )));
        }

        let vault_path = super::expand_path(&config.integrations.obsidian_path)?;

        Ok(Self {
            vault_path: Some(vault_path),
            daily_notes_folder: "daily".to_string(),
            date_format: "%Y-%m-%d".to_string(),
        })
    }

    fn get_vault_path(&self) -> Result<&PathBuf> {
        self.vault_path.as_ref().ok_or_else(|| {
            crate::error::Error::Config(ConfigError::MissingField("obsidian_path".into()))
        })
    }

    fn daily_note_path(&self, date: NaiveDate) -> Result<PathBuf> {
        let vault = self.get_vault_path()?;
        let filename = format!("{}.md", date.format(&self.date_format));
        Ok(vault.join(&self.daily_notes_folder).join(filename))
    }

    pub fn export_entry(&self, entry: &EntryWithDetails) -> Result<()> {
        let date = entry.entry.start.with_timezone(&Local).date_naive();
        let note_path = self.daily_note_path(date)?;

        if let Some(parent) = note_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let time_block = self.format_time_block(entry);
        let content = if note_path.exists() {
            self.update_existing_note(&note_path, &time_block)?
        } else {
            self.create_new_note(date, &time_block)
        };

        fs::write(&note_path, content)?;
        Ok(())
    }

    pub fn export_entries(&self, entries: &[EntryWithDetails]) -> Result<usize> {
        let mut by_date: HashMap<NaiveDate, Vec<&EntryWithDetails>> = HashMap::new();

        for entry in entries {
            if entry.entry.end.is_some() {
                let date = entry.entry.start.with_timezone(&Local).date_naive();
                by_date.entry(date).or_default().push(entry);
            }
        }

        let mut count = 0;
        for (date, day_entries) in by_date {
            self.export_day_entries(date, &day_entries)?;
            count += day_entries.len();
        }

        Ok(count)
    }

    fn export_day_entries(&self, date: NaiveDate, entries: &[&EntryWithDetails]) -> Result<()> {
        let note_path = self.daily_note_path(date)?;

        if let Some(parent) = note_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let time_blocks: Vec<String> = entries
            .iter()
            .map(|e| self.format_time_block(e))
            .collect();

        let section_content = time_blocks.join("\n");

        let content = if note_path.exists() {
            self.update_existing_note(&note_path, &section_content)?
        } else {
            self.create_new_note(date, &section_content)
        };

        fs::write(&note_path, content)?;
        Ok(())
    }

    fn format_time_block(&self, entry: &EntryWithDetails) -> String {
        let start = entry.entry.start.with_timezone(&Local);
        let end = entry
            .entry
            .end
            .map(|e| e.with_timezone(&Local))
            .unwrap_or_else(|| Local::now());

        let duration = entry.entry.duration();
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;

        let mut line = format!(
            "- [{}-{}] {} > {} ({}h{}m)",
            start.format("%H:%M"),
            end.format("%H:%M"),
            entry.project_name,
            entry.task_name,
            hours,
            minutes
        );

        if let Some(ref notes) = entry.entry.notes {
            if !notes.is_empty() {
                line.push_str(&format!(" - {}", notes));
            }
        }

        line
    }

    fn create_new_note(&self, date: NaiveDate, time_blocks: &str) -> String {
        format!(
            "# {}\n\n{}\n{}\n\n---\n",
            date.format("%A, %B %d, %Y"),
            MATERIATRACK_HEADER,
            time_blocks
        )
    }

    fn update_existing_note(&self, path: &Path, time_blocks: &str) -> Result<String> {
        let content = fs::read_to_string(path)?;

        if let Some(section_start) = content.find(MATERIATRACK_HEADER) {
            let before = &content[..section_start];

            let section_end = content[section_start..]
                .find("\n## ")
                .or_else(|| content[section_start..].find("\n---"))
                .map(|i| section_start + i)
                .unwrap_or(content.len());

            let after = &content[section_end..];

            Ok(format!(
                "{}{}\n{}\n{}",
                before, MATERIATRACK_HEADER, time_blocks, after
            ))
        } else {
            Ok(format!("{}\n{}\n{}\n", content, MATERIATRACK_HEADER, time_blocks))
        }
    }

    pub fn import_entries(&self, date: NaiveDate) -> Result<Vec<ImportedTimeBlock>> {
        let note_path = self.daily_note_path(date)?;

        if !note_path.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&note_path)?;
        self.parse_time_blocks(&content, date)
    }

    pub fn import_range(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<ImportedTimeBlock>> {
        let mut all_blocks = Vec::new();
        let mut current = start;

        while current <= end {
            if let Ok(blocks) = self.import_entries(current) {
                all_blocks.extend(blocks);
            }
            current = current.succ_opt().unwrap_or(current);
        }

        Ok(all_blocks)
    }

    fn parse_time_blocks(&self, content: &str, date: NaiveDate) -> Result<Vec<ImportedTimeBlock>> {
        let re = regex::Regex::new(TIME_BLOCK_PATTERN).map_err(|e| {
            crate::error::Error::Parse(format!("Invalid regex: {}", e))
        })?;

        let mut blocks = Vec::new();

        let section_start = content.find(MATERIATRACK_HEADER);
        let search_content = if let Some(start) = section_start {
            let end = content[start..]
                .find("\n## ")
                .or_else(|| content[start..].find("\n---"))
                .map(|i| start + i)
                .unwrap_or(content.len());
            &content[start..end]
        } else {
            content
        };

        for cap in re.captures_iter(search_content) {
            let start_time = NaiveTime::parse_from_str(&cap[1], "%H:%M").ok();
            let end_time = NaiveTime::parse_from_str(&cap[2], "%H:%M").ok();
            let project = cap[3].trim().to_string();
            let task = cap[4].trim().to_string();

            if let (Some(st), Some(et)) = (start_time, end_time) {
                let start_dt = Local
                    .from_local_datetime(&date.and_time(st))
                    .latest()
                    .map(|dt| dt.with_timezone(&Utc));

                let end_dt = Local
                    .from_local_datetime(&date.and_time(et))
                    .latest()
                    .map(|dt| dt.with_timezone(&Utc));

                if let (Some(start), Some(end)) = (start_dt, end_dt) {
                    blocks.push(ImportedTimeBlock {
                        project,
                        task,
                        start,
                        end,
                        notes: None,
                    });
                }
            }
        }

        Ok(blocks)
    }

    pub fn sync_status(&self) -> Result<SyncStatus> {
        let vault = self.get_vault_path()?;
        let daily_path = vault.join(&self.daily_notes_folder);

        if !vault.exists() {
            return Ok(SyncStatus {
                vault_exists: false,
                daily_folder_exists: false,
                note_count: 0,
                last_modified: None,
            });
        }

        let note_count = if daily_path.exists() {
            fs::read_dir(&daily_path)
                .map(|entries| {
                    entries
                        .filter_map(|e| e.ok())
                        .filter(|e| {
                            e.path()
                                .extension()
                                .map(|ext| ext == "md")
                                .unwrap_or(false)
                        })
                        .count()
                })
                .unwrap_or(0)
        } else {
            0
        };

        let last_modified = fs::metadata(&daily_path)
            .ok()
            .and_then(|m| m.modified().ok())
            .map(DateTime::<Utc>::from);

        Ok(SyncStatus {
            vault_exists: vault.exists(),
            daily_folder_exists: daily_path.exists(),
            note_count,
            last_modified,
        })
    }
}

impl Default for ObsidianIntegration {
    fn default() -> Self {
        Self::new()
    }
}

impl Integration for ObsidianIntegration {
    fn name(&self) -> &'static str {
        "Obsidian"
    }

    fn is_enabled(&self, config: &Config) -> bool {
        !config.integrations.obsidian_path.is_empty()
    }

    fn validate_config(&self, config: &Config) -> Result<()> {
        if config.integrations.obsidian_path.is_empty() {
            return Ok(());
        }

        let path = super::expand_path(&config.integrations.obsidian_path)?;

        if !path.exists() {
            return Err(crate::error::Error::Config(ConfigError::InvalidPath(
                format!("Obsidian vault not found: {}", path.display()),
            )));
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ImportedTimeBlock {
    pub project: String,
    pub task: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub notes: Option<String>,
}

impl ImportedTimeBlock {
    pub fn duration_seconds(&self) -> i64 {
        (self.end - self.start).num_seconds()
    }

    pub fn to_entry(&self, project_id: i64, task_id: i64) -> Entry {
        Entry {
            id: 0,
            project_id,
            task_id,
            start: self.start,
            end: Some(self.end),
            notes: self.notes.clone(),
            git_commits: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyncStatus {
    pub vault_exists: bool,
    pub daily_folder_exists: bool,
    pub note_count: usize,
    pub last_modified: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_obsidian_new() {
        let obs = ObsidianIntegration::new();
        assert!(obs.vault_path.is_none());
        assert_eq!(obs.daily_notes_folder, "daily");
    }

    #[test]
    fn test_format_time_block() {
        let obs = ObsidianIntegration::new();

        let entry = EntryWithDetails {
            entry: Entry {
                id: 1,
                project_id: 1,
                task_id: 1,
                start: Utc::now() - chrono::Duration::hours(1),
                end: Some(Utc::now()),
                notes: None,
                git_commits: Vec::new(),
            },
            project_name: "TestProject".to_string(),
            task_name: "TestTask".to_string(),
            project_color: None,
        };

        let block = obs.format_time_block(&entry);
        assert!(block.contains("TestProject"));
        assert!(block.contains("TestTask"));
        assert!(block.contains("1h0m"));
    }
}
