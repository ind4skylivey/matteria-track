//! Zeit database import for MateriaTrack
//!
//! Imports time entries from Zeit time tracker SQLite database.

use crate::config::Config;
use crate::database::Database;
use crate::error::{ConfigError, Result};
use crate::models::{Entry, Project, Task};
use chrono::{DateTime, TimeZone, Utc};
use rusqlite::Connection;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use super::{Integration, TimeImporter};

pub struct ZeitImporter {
    db_path: Option<PathBuf>,
}

impl ZeitImporter {
    pub fn new() -> Self {
        Self { db_path: None }
    }

    pub fn with_db_path(mut self, path: PathBuf) -> Self {
        self.db_path = Some(path);
        self
    }

    pub fn default_zeit_path() -> Option<PathBuf> {
        dirs::data_local_dir().map(|d| d.join("zeit").join("zeit.db"))
    }

    fn get_db_path(&self) -> Result<PathBuf> {
        self.db_path
            .clone()
            .or_else(Self::default_zeit_path)
            .ok_or_else(|| {
                crate::error::Error::Config(ConfigError::NotFound(
                    "Zeit database path not found".into(),
                ))
            })
    }

    pub fn import_to_database(&self, target_db: &Database) -> Result<ImportResult> {
        let zeit_path = self.get_db_path()?;

        if !zeit_path.exists() {
            return Err(crate::error::Error::Config(ConfigError::NotFound(
                format!("Zeit database not found: {}", zeit_path.display()),
            )));
        }

        let zeit_conn = Connection::open(&zeit_path)?;

        let mut project_map: HashMap<String, Project> = HashMap::new();
        let mut task_map: HashMap<(i64, String), Task> = HashMap::new();

        let mut result = ImportResult::default();

        let zeit_projects = self.read_zeit_projects(&zeit_conn)?;
        for zp in zeit_projects {
            let mut project = Project::new(&zp.name);
            target_db.create_project(&mut project)?;
            project_map.insert(zp.name.clone(), project);
            result.projects_imported += 1;
        }

        let zeit_entries = self.read_zeit_entries(&zeit_conn)?;

        for ze in zeit_entries {
            let project = project_map
                .entry(ze.project.clone())
                .or_insert_with(|| {
                    let mut p = Project::new(&ze.project);
                    let _ = target_db.create_project(&mut p);
                    result.projects_imported += 1;
                    p
                });

            let task_key = (project.id, ze.task.clone());
            let task = task_map.entry(task_key.clone()).or_insert_with(|| {
                let mut t = Task::new(project.id, &ze.task);
                let _ = target_db.create_task(&mut t);
                result.tasks_imported += 1;
                t
            });

            let mut entry = Entry {
                id: 0,
                project_id: project.id,
                task_id: task.id,
                start: ze.start,
                end: ze.end,
                notes: ze.notes,
                git_commits: Vec::new(),
            };

            target_db.create_entry(&mut entry)?;
            result.entries_imported += 1;
        }

        Ok(result)
    }

    fn read_zeit_projects(&self, conn: &Connection) -> Result<Vec<ZeitProject>> {
        let mut stmt = conn.prepare(
            "SELECT DISTINCT project FROM entries WHERE project IS NOT NULL AND project != ''",
        )?;

        let projects = stmt
            .query_map([], |row| {
                Ok(ZeitProject {
                    name: row.get(0)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(projects)
    }

    fn read_zeit_entries(&self, conn: &Connection) -> Result<Vec<ZeitEntry>> {
        let has_task_column = conn
            .prepare("SELECT task FROM entries LIMIT 1")
            .is_ok();

        let query = if has_task_column {
            "SELECT project, task, start, finish, note FROM entries 
             WHERE project IS NOT NULL AND project != ''
             ORDER BY start ASC"
        } else {
            "SELECT project, '' as task, start, finish, note FROM entries 
             WHERE project IS NOT NULL AND project != ''
             ORDER BY start ASC"
        };

        let mut stmt = conn.prepare(query)?;

        let entries = stmt
            .query_map([], |row| {
                let project: String = row.get(0)?;
                let task: String = row.get::<_, Option<String>>(1)?.unwrap_or_else(|| "default".to_string());
                let start_str: String = row.get(2)?;
                let end_str: Option<String> = row.get(3)?;
                let notes: Option<String> = row.get(4)?;

                let start = parse_zeit_datetime(&start_str);
                let end = end_str.as_ref().map(|s| parse_zeit_datetime(s));

                Ok(ZeitEntry {
                    project,
                    task,
                    start,
                    end,
                    notes,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok(entries)
    }

    pub fn preview(&self) -> Result<ImportPreview> {
        let zeit_path = self.get_db_path()?;

        if !zeit_path.exists() {
            return Err(crate::error::Error::Config(ConfigError::NotFound(
                format!("Zeit database not found: {}", zeit_path.display()),
            )));
        }

        let conn = Connection::open(&zeit_path)?;

        let entry_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM entries WHERE project IS NOT NULL",
            [],
            |row| row.get(0),
        )?;

        let project_count: i64 = conn.query_row(
            "SELECT COUNT(DISTINCT project) FROM entries WHERE project IS NOT NULL AND project != ''",
            [],
            |row| row.get(0),
        )?;

        let date_range = conn.query_row(
            "SELECT MIN(start), MAX(start) FROM entries WHERE project IS NOT NULL",
            [],
            |row| {
                let min: Option<String> = row.get(0)?;
                let max: Option<String> = row.get(1)?;
                Ok((min, max))
            },
        )?;

        let oldest = date_range.0.map(|s| parse_zeit_datetime(&s));
        let newest = date_range.1.map(|s| parse_zeit_datetime(&s));

        Ok(ImportPreview {
            source_path: zeit_path,
            entry_count: entry_count as usize,
            project_count: project_count as usize,
            oldest_entry: oldest,
            newest_entry: newest,
        })
    }
}

impl Default for ZeitImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl Integration for ZeitImporter {
    fn name(&self) -> &'static str {
        "Zeit"
    }

    fn is_enabled(&self, _config: &Config) -> bool {
        self.db_path.is_some() || Self::default_zeit_path().map(|p| p.exists()).unwrap_or(false)
    }

    fn validate_config(&self, _config: &Config) -> Result<()> {
        let path = self.get_db_path()?;
        if !path.exists() {
            return Err(crate::error::Error::Config(ConfigError::NotFound(
                format!("Zeit database not found: {}", path.display()),
            )));
        }
        Ok(())
    }
}

impl TimeImporter for ZeitImporter {
    fn import_entries(&self, _config: &Config) -> Result<Vec<Entry>> {
        let zeit_path = self.get_db_path()?;
        let conn = Connection::open(&zeit_path)?;

        let zeit_entries = self.read_zeit_entries(&conn)?;

        let entries: Vec<Entry> = zeit_entries
            .into_iter()
            .map(|ze| Entry {
                id: 0,
                project_id: 0,
                task_id: 0,
                start: ze.start,
                end: ze.end,
                notes: ze.notes,
                git_commits: Vec::new(),
            })
            .collect();

        Ok(entries)
    }
}

#[derive(Debug, Clone)]
struct ZeitProject {
    name: String,
}

#[derive(Debug, Clone)]
struct ZeitEntry {
    project: String,
    task: String,
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
    notes: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ImportResult {
    pub projects_imported: usize,
    pub tasks_imported: usize,
    pub entries_imported: usize,
    pub errors: Vec<String>,
}

impl ImportResult {
    pub fn summary(&self) -> String {
        format!(
            "Imported {} projects, {} tasks, {} entries",
            self.projects_imported, self.tasks_imported, self.entries_imported
        )
    }
}

#[derive(Debug, Clone)]
pub struct ImportPreview {
    pub source_path: PathBuf,
    pub entry_count: usize,
    pub project_count: usize,
    pub oldest_entry: Option<DateTime<Utc>>,
    pub newest_entry: Option<DateTime<Utc>>,
}

impl ImportPreview {
    pub fn display(&self) -> String {
        let date_range = match (&self.oldest_entry, &self.newest_entry) {
            (Some(old), Some(new)) => format!(
                "{} to {}",
                old.format("%Y-%m-%d"),
                new.format("%Y-%m-%d")
            ),
            _ => "Unknown".to_string(),
        };

        format!(
            "Source: {}\nProjects: {}\nEntries: {}\nDate range: {}",
            self.source_path.display(),
            self.project_count,
            self.entry_count,
            date_range
        )
    }
}

fn parse_zeit_datetime(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                .map(|ndt| Utc.from_utc_datetime(&ndt))
        })
        .or_else(|_| {
            chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
                .map(|ndt| Utc.from_utc_datetime(&ndt))
        })
        .unwrap_or_else(|_| Utc::now())
}

pub fn import_from_json<P: AsRef<Path>>(path: P, target_db: &Database) -> Result<ImportResult> {
    let content = std::fs::read_to_string(path)?;
    let entries: Vec<JsonEntry> = serde_json::from_str(&content)?;

    let mut result = ImportResult::default();
    let mut project_map: HashMap<String, Project> = HashMap::new();
    let mut task_map: HashMap<(i64, String), Task> = HashMap::new();

    for je in entries {
        let project = project_map.entry(je.project.clone()).or_insert_with(|| {
            let mut p = Project::new(&je.project);
            let _ = target_db.create_project(&mut p);
            result.projects_imported += 1;
            p
        });

        let task_name = je.task.as_deref().unwrap_or("default");
        let task_key = (project.id, task_name.to_string());
        let task = task_map.entry(task_key).or_insert_with(|| {
            let mut t = Task::new(project.id, task_name);
            let _ = target_db.create_task(&mut t);
            result.tasks_imported += 1;
            t
        });

        let mut entry = Entry {
            id: 0,
            project_id: project.id,
            task_id: task.id,
            start: je.start,
            end: je.end,
            notes: je.notes,
            git_commits: Vec::new(),
        };

        target_db.create_entry(&mut entry)?;
        result.entries_imported += 1;
    }

    Ok(result)
}

#[derive(Debug, serde::Deserialize)]
struct JsonEntry {
    project: String,
    task: Option<String>,
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
    notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zeit_importer_new() {
        let importer = ZeitImporter::new();
        assert!(importer.db_path.is_none());
    }

    #[test]
    fn test_parse_zeit_datetime() {
        let dt = parse_zeit_datetime("2024-01-15T10:30:00+00:00");
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2024-01-15");

        let dt2 = parse_zeit_datetime("2024-01-15 10:30:00");
        assert_eq!(dt2.format("%Y-%m-%d").to_string(), "2024-01-15");
    }

    #[test]
    fn test_import_result_summary() {
        let result = ImportResult {
            projects_imported: 3,
            tasks_imported: 10,
            entries_imported: 50,
            errors: Vec::new(),
        };

        let summary = result.summary();
        assert!(summary.contains("3 projects"));
        assert!(summary.contains("10 tasks"));
        assert!(summary.contains("50 entries"));
    }
}
