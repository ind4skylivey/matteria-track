//! Time tracking engine for MatteriaTrack

use crate::config::Config;
use crate::database::Database;
use crate::error::{Result, TrackingError};
use crate::models::{apply_time_offset, Entry, Project, Task};
use chrono::{DateTime, Utc};
use git2::Repository;
use std::path::Path;

pub struct TrackingEngine {
    db: Database,
    config: Config,
}

impl TrackingEngine {
    pub fn new(db: Database, config: Config) -> Self {
        Self { db, config }
    }

    pub fn start_tracking(
        &self,
        project_name: &str,
        task_name: &str,
        begin_offset: Option<&str>,
        notes: Option<&str>,
    ) -> Result<(Entry, Project, Task)> {
        if let Some(active) = self.db.get_active_tracking()? {
            let project = self.db.get_project(active.project_id)?.unwrap();
            let task = self.db.get_task(active.task_id)?.unwrap();
            return Err(TrackingError::AlreadyTracking(format!(
                "{} -> {}",
                project.name, task.name
            ))
            .into());
        }

        let project = self.db.get_or_create_project(project_name)?;
        let task = self.db.get_or_create_task(project.id, task_name)?;

        let start = if let Some(offset) = begin_offset {
            apply_time_offset(Utc::now(), offset).unwrap_or_else(Utc::now)
        } else {
            Utc::now()
        };

        let mut entry = Entry::new(project.id, task.id).with_start(start);

        if let Some(n) = notes {
            entry = entry.with_notes(n);
        }

        self.db.create_entry(&mut entry)?;
        self.db.set_active_tracking(entry.id)?;

        Ok((entry, project, task))
    }

    pub fn finish_tracking(
        &self,
        new_task: Option<&str>,
        begin_offset: Option<&str>,
        end_offset: Option<&str>,
        notes: Option<&str>,
    ) -> Result<(Entry, Project, Task)> {
        let mut entry = self
            .db
            .get_active_tracking()?
            .ok_or(TrackingError::NotTracking)?;

        let project = self
            .db
            .get_project(entry.project_id)?
            .ok_or_else(|| TrackingError::ProjectNotFound(entry.project_id.to_string()))?;

        let mut task = self
            .db
            .get_task(entry.task_id)?
            .ok_or_else(|| TrackingError::TaskNotFound(entry.task_id.to_string()))?;

        if let Some(task_name) = new_task {
            task = self.db.get_or_create_task(project.id, task_name)?;
            entry.task_id = task.id;
        }

        if let Some(offset) = begin_offset {
            if let Some(new_start) = apply_time_offset(entry.start, offset) {
                entry.start = new_start;
            }
        }

        let end_time = if let Some(offset) = end_offset {
            apply_time_offset(Utc::now(), offset).unwrap_or_else(Utc::now)
        } else {
            Utc::now()
        };

        entry.finish_at(end_time);

        if let Some(n) = notes {
            entry.notes = Some(n.to_string());
        }

        if self.config.tracking.auto_import_git {
            if let Ok(Some(repo_path)) = self.config.git_repo_path() {
                if let Ok(commits) = get_recent_commits(&repo_path, entry.start, end_time) {
                    entry.git_commits = commits;
                }
            } else if let Some(ref repo) = task.git_repo {
                if let Ok(commits) = get_recent_commits(Path::new(repo), entry.start, end_time) {
                    entry.git_commits = commits;
                }
            }
        }

        self.db.update_entry(&entry)?;
        self.db.clear_active_tracking()?;

        Ok((entry, project, task))
    }

    pub fn get_status(&self) -> Result<Option<(Entry, Project, Task)>> {
        if let Some(entry) = self.db.get_active_tracking()? {
            let project = self.db.get_project(entry.project_id)?;
            let task = self.db.get_task(entry.task_id)?;

            if let (Some(p), Some(t)) = (project, task) {
                return Ok(Some((entry, p, t)));
            }
        }
        Ok(None)
    }

    pub fn cancel_tracking(&self) -> Result<Option<Entry>> {
        if let Some(entry) = self.db.get_active_tracking()? {
            self.db.clear_active_tracking()?;
            self.db.delete_entry(entry.id)?;
            return Ok(Some(entry));
        }
        Ok(None)
    }

    pub fn amend_entry(
        &self,
        entry_id: i64,
        start: Option<DateTime<Utc>>,
        end: Option<DateTime<Utc>>,
        notes: Option<&str>,
        project: Option<&str>,
        task: Option<&str>,
    ) -> Result<Entry> {
        let mut entry = self
            .db
            .get_entry(entry_id)?
            .ok_or_else(|| crate::error::Error::NotFound(format!("Entry {}", entry_id)))?;

        if let Some(s) = start {
            entry.start = s;
        }

        if let Some(e) = end {
            entry.end = Some(e);
        }

        if let Some(n) = notes {
            entry.notes = Some(n.to_string());
        }

        if let Some(project_name) = project {
            let p = self.db.get_or_create_project(project_name)?;
            entry.project_id = p.id;
        }

        if let Some(task_name) = task {
            let t = self.db.get_or_create_task(entry.project_id, task_name)?;
            entry.task_id = t.id;
        }

        self.db.update_entry(&entry)?;
        Ok(entry)
    }

    pub fn db(&self) -> &Database {
        &self.db
    }
}

fn get_recent_commits<P: AsRef<Path>>(
    repo_path: P,
    since: DateTime<Utc>,
    until: DateTime<Utc>,
) -> Result<Vec<String>> {
    let repo = Repository::open(repo_path)?;
    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;

    let mut commits = Vec::new();
    let since_ts = since.timestamp();
    let until_ts = until.timestamp();

    for oid in revwalk.flatten() {
        if let Ok(commit) = repo.find_commit(oid) {
            let commit_time = commit.time().seconds();

            if commit_time < since_ts {
                break;
            }

            if commit_time <= until_ts {
                let short_id = commit.id().to_string()[..7].to_string();
                let message = commit
                    .summary()
                    .unwrap_or("")
                    .chars()
                    .take(50)
                    .collect::<String>();

                commits.push(format!("{}: {}", short_id, message));
            }
        }
    }

    Ok(commits)
}

pub fn format_duration_short(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;

    if hours > 0 {
        format!("{}h{}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

pub fn format_duration_long(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}

pub fn statusbar_output(
    project: &str,
    task: &str,
    duration_secs: i64,
    short: bool,
    icon: Option<&str>,
) -> String {
    let icon = icon.unwrap_or("💎");
    let duration = format_duration_short(duration_secs);

    if short {
        format!("{} {} {}", icon, project, duration)
    } else {
        format!("{} {}:{} {}", icon, project, task, duration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration_short() {
        assert_eq!(format_duration_short(3661), "1h1m");
        assert_eq!(format_duration_short(1800), "30m");
        assert_eq!(format_duration_short(60), "1m");
    }

    #[test]
    fn test_format_duration_long() {
        assert_eq!(format_duration_long(3661), "1h 1m 1s");
        assert_eq!(format_duration_long(125), "2m 5s");
        assert_eq!(format_duration_long(45), "45s");
    }

    #[test]
    fn test_statusbar_output() {
        let output = statusbar_output("Project", "Task", 3600, false, None);
        assert!(output.contains("Project"));
        assert!(output.contains("Task"));
        assert!(output.contains("1h0m"));
    }
}
