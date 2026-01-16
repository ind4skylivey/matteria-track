//! Calendar events and event storage

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CalendarEventType {
    /// Tracking session event
    TrackingSession,
    /// Custom calendar event
    Custom,
    /// Reminder
    Reminder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: String,
    pub title: String,
    pub date: NaiveDate,
    pub time: Option<String>,
    pub event_type: CalendarEventType,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl CalendarEvent {
    pub fn new(title: impl Into<String>, date: NaiveDate) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            date,
            time: None,
            event_type: CalendarEventType::Custom,
            description: None,
            created_at: Utc::now(),
        }
    }

    pub fn with_time(mut self, time: impl Into<String>) -> Self {
        self.time = Some(time.into());
        self
    }

    pub fn with_type(mut self, event_type: CalendarEventType) -> Self {
        self.event_type = event_type;
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn tracking_session(
        title: impl Into<String>,
        date: NaiveDate,
        time: impl Into<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.into(),
            date,
            time: Some(time.into()),
            event_type: CalendarEventType::TrackingSession,
            description: None,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug)]
pub struct EventStore {
    events: Vec<CalendarEvent>,
    file_path: PathBuf,
}

impl EventStore {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            events: Vec::new(),
            file_path,
        }
    }

    pub fn load(&mut self) -> Result<()> {
        if !self.file_path.exists() {
            // Create parent directory if it doesn't exist
            if let Some(parent) = self.file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            // Initialize with empty JSON array
            fs::write(&self.file_path, "[]")?;
            return Ok(());
        }

        let contents = fs::read_to_string(&self.file_path)?;
        self.events = serde_json::from_str(&contents).unwrap_or_default();
        Ok(())
    }

    pub fn save(&self) -> Result<()> {
        let contents = serde_json::to_string_pretty(&self.events)?;
        fs::write(&self.file_path, contents)?;
        Ok(())
    }

    pub fn add_event(&mut self, event: CalendarEvent) -> Result<()> {
        self.events.push(event);
        self.save()
    }

    pub fn remove_event(&mut self, id: &str) -> Result<bool> {
        let before_len = self.events.len();
        self.events.retain(|e| e.id != id);
        let removed = self.events.len() != before_len;

        if removed {
            self.save()?;
        }

        Ok(removed)
    }

    pub fn get_all(&self) -> &[CalendarEvent] {
        &self.events
    }

    pub fn get_by_date(&self, date: NaiveDate) -> Vec<&CalendarEvent> {
        self.events.iter().filter(|e| e.date == date).collect()
    }

    pub fn get_upcoming(&self, from: NaiveDate, limit: usize) -> Vec<&CalendarEvent> {
        let mut upcoming: Vec<&CalendarEvent> =
            self.events.iter().filter(|e| e.date >= from).collect();

        upcoming.sort_by_key(|e| e.date);
        upcoming.into_iter().take(limit).collect()
    }

    pub fn clear(&mut self) -> Result<()> {
        self.events.clear();
        self.save()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_event_creation() {
        let event = CalendarEvent::new("Test Event", NaiveDate::from_ymd_opt(2026, 1, 15).unwrap());
        assert_eq!(event.title, "Test Event");
        assert_eq!(event.event_type, CalendarEventType::Custom);
    }

    #[test]
    fn test_event_store() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path().to_path_buf();

        let mut store = EventStore::new(path.clone());
        store.load()?;

        let event = CalendarEvent::new("Test", NaiveDate::from_ymd_opt(2026, 1, 15).unwrap());
        store.add_event(event)?;

        assert_eq!(store.get_all().len(), 1);

        // Reload from disk
        let mut store2 = EventStore::new(path);
        store2.load()?;
        assert_eq!(store2.get_all().len(), 1);

        Ok(())
    }
}
