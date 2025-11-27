//! Data models for MatteriaTrack

use chrono::{DateTime, Duration, Local, Utc};
use serde::{Deserialize, Serialize};

pub type ProjectId = i64;
pub type TaskId = i64;
pub type EntryId = i64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Project {
    pub fn new(name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            name: name.into(),
            color: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn display_color(&self) -> &str {
        self.color.as_deref().unwrap_or("#FF6432")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub project_id: ProjectId,
    pub name: String,
    pub git_repo: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(project_id: ProjectId, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: 0,
            project_id,
            name: name.into(),
            git_repo: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_git_repo(mut self, repo: impl Into<String>) -> Self {
        self.git_repo = Some(repo.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: EntryId,
    pub project_id: ProjectId,
    pub task_id: TaskId,
    pub start: DateTime<Utc>,
    pub end: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub git_commits: Vec<String>,
}

impl Entry {
    pub fn new(project_id: ProjectId, task_id: TaskId) -> Self {
        Self {
            id: 0,
            project_id,
            task_id,
            start: Utc::now(),
            end: None,
            notes: None,
            git_commits: Vec::new(),
        }
    }

    pub fn with_start(mut self, start: DateTime<Utc>) -> Self {
        self.start = start;
        self
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn is_active(&self) -> bool {
        self.end.is_none()
    }

    pub fn finish(&mut self) {
        if self.end.is_none() {
            self.end = Some(Utc::now());
        }
    }

    pub fn finish_at(&mut self, time: DateTime<Utc>) {
        self.end = Some(time);
    }

    pub fn duration(&self) -> Duration {
        let end = self.end.unwrap_or_else(Utc::now);
        end.signed_duration_since(self.start)
    }

    pub fn duration_formatted(&self) -> String {
        let dur = self.duration();
        let hours = dur.num_hours();
        let minutes = dur.num_minutes() % 60;
        let seconds = dur.num_seconds() % 60;

        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }

    pub fn add_git_commit(&mut self, commit_hash: impl Into<String>) {
        self.git_commits.push(commit_hash.into());
    }

    pub fn start_local(&self) -> DateTime<Local> {
        self.start.with_timezone(&Local)
    }

    pub fn end_local(&self) -> Option<DateTime<Local>> {
        self.end.map(|e| e.with_timezone(&Local))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryWithDetails {
    pub entry: Entry,
    pub project_name: String,
    pub task_name: String,
    pub project_color: Option<String>,
}

impl EntryWithDetails {
    pub fn new(entry: Entry, project: &Project, task: &Task) -> Self {
        Self {
            entry,
            project_name: project.name.clone(),
            task_name: task.name.clone(),
            project_color: project.color.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingState {
    pub active_entry: Option<Entry>,
    pub project_name: Option<String>,
    pub task_name: Option<String>,
}

impl TrackingState {
    pub fn new() -> Self {
        Self {
            active_entry: None,
            project_name: None,
            task_name: None,
        }
    }

    pub fn is_tracking(&self) -> bool {
        self.active_entry.is_some()
    }

    pub fn start(&mut self, entry: Entry, project: &str, task: &str) {
        self.active_entry = Some(entry);
        self.project_name = Some(project.to_string());
        self.task_name = Some(task.to_string());
    }

    pub fn stop(&mut self) -> Option<Entry> {
        self.project_name = None;
        self.task_name = None;
        self.active_entry.take()
    }
}

impl Default for TrackingState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeStats {
    pub total_seconds: i64,
    pub entry_count: usize,
    pub projects: Vec<ProjectStats>,
}

impl TimeStats {
    pub fn total_duration(&self) -> Duration {
        Duration::seconds(self.total_seconds)
    }

    pub fn total_formatted(&self) -> String {
        let dur = self.total_duration();
        let hours = dur.num_hours();
        let minutes = dur.num_minutes() % 60;

        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStats {
    pub project_id: ProjectId,
    pub project_name: String,
    pub total_seconds: i64,
    pub entry_count: usize,
    pub tasks: Vec<TaskStats>,
}

impl ProjectStats {
    pub fn total_formatted(&self) -> String {
        let dur = Duration::seconds(self.total_seconds);
        let hours = dur.num_hours();
        let minutes = dur.num_minutes() % 60;

        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }

    pub fn percentage_of(&self, total: i64) -> f64 {
        if total == 0 {
            0.0
        } else {
            (self.total_seconds as f64 / total as f64) * 100.0
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStats {
    pub task_id: TaskId,
    pub task_name: String,
    pub total_seconds: i64,
    pub entry_count: usize,
}

impl TaskStats {
    pub fn total_formatted(&self) -> String {
        let dur = Duration::seconds(self.total_seconds);
        let hours = dur.num_hours();
        let minutes = dur.num_minutes() % 60;

        if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }
}

pub fn parse_time_offset(offset: &str) -> Option<Duration> {
    let offset = offset.trim();

    if offset.is_empty() {
        return None;
    }

    let negative = offset.starts_with('-');
    let offset = offset.trim_start_matches('-').trim_start_matches('+');

    if offset.contains(':') {
        let parts: Vec<&str> = offset.split(':').collect();
        match parts.len() {
            2 => {
                let hours: i64 = parts[0].parse().ok()?;
                let minutes: i64 = parts[1].parse().ok()?;
                let total_minutes = hours * 60 + minutes;
                let dur = Duration::minutes(total_minutes);
                Some(if negative { -dur } else { dur })
            }
            3 => {
                let hours: i64 = parts[0].parse().ok()?;
                let minutes: i64 = parts[1].parse().ok()?;
                let seconds: i64 = parts[2].parse().ok()?;
                let total_seconds = hours * 3600 + minutes * 60 + seconds;
                let dur = Duration::seconds(total_seconds);
                Some(if negative { -dur } else { dur })
            }
            _ => None,
        }
    } else if let Ok(minutes) = offset.parse::<i64>() {
        let dur = Duration::minutes(minutes);
        Some(if negative { -dur } else { dur })
    } else {
        None
    }
}

pub fn apply_time_offset(base: DateTime<Utc>, offset: &str) -> Option<DateTime<Utc>> {
    parse_time_offset(offset).map(|dur| base + dur)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_creation() {
        let project = Project::new("TestProject");
        assert_eq!(project.name, "TestProject");
        assert!(project.color.is_none());
    }

    #[test]
    fn test_entry_duration() {
        let mut entry = Entry::new(1, 1);
        entry.start = Utc::now() - Duration::hours(2);
        entry.end = Some(Utc::now());

        assert!(entry.duration().num_hours() >= 1);
    }

    #[test]
    fn test_time_offset_parsing() {
        assert_eq!(parse_time_offset("-15"), Some(Duration::minutes(-15)));
        assert_eq!(parse_time_offset("-0:30"), Some(Duration::minutes(-30)));
        assert_eq!(parse_time_offset("1:30"), Some(Duration::minutes(90)));
        assert_eq!(
            parse_time_offset("-1:30:00"),
            Some(Duration::seconds(-5400))
        );
    }

    #[test]
    fn test_tracking_state() {
        let mut state = TrackingState::new();
        assert!(!state.is_tracking());

        let entry = Entry::new(1, 1);
        state.start(entry, "Project", "Task");
        assert!(state.is_tracking());

        let finished = state.stop();
        assert!(finished.is_some());
        assert!(!state.is_tracking());
    }
}
