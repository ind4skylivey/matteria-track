//! Desktop notifications for MateriaTrack
//!
//! Provides tracking reminders, daily summaries, and Pomodoro timer.

use crate::config::Config;
use crate::error::Result;
use crate::models::Entry;
use chrono::{DateTime, Duration, Local, Utc};
use std::process::Command;

const DEFAULT_REMINDER_INTERVAL: u64 = 30;
const POMODORO_WORK_MINUTES: u64 = 25;
const POMODORO_BREAK_MINUTES: u64 = 5;

pub struct NotificationManager {
    enabled: bool,
    reminder_interval: Duration,
    use_system_notify: bool,
}

impl NotificationManager {
    pub fn new(config: &Config) -> Self {
        Self {
            enabled: config.notifications.enable,
            reminder_interval: Duration::minutes(
                config
                    .notifications
                    .reminder_interval
                    .unwrap_or(DEFAULT_REMINDER_INTERVAL) as i64,
            ),
            use_system_notify: check_notify_send(),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn send(&self, title: &str, body: &str, urgency: Urgency) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        if self.use_system_notify {
            send_system_notification(title, body, urgency)?;
        } else {
            println!("\n🔔 {} - {}\n", title, body);
        }

        Ok(())
    }

    pub fn send_tracking_started(&self, project: &str, task: &str) -> Result<()> {
        self.send(
            "💎 MateriaTrack",
            &format!("Started tracking: {} > {}", project, task),
            Urgency::Normal,
        )
    }

    pub fn send_tracking_finished(&self, project: &str, task: &str, duration: &str) -> Result<()> {
        self.send(
            "✓ Tracking Complete",
            &format!("{} > {} ({})", project, task, duration),
            Urgency::Normal,
        )
    }

    pub fn send_reminder(&self, project: &str, task: &str, duration: &str) -> Result<()> {
        self.send(
            "⏰ Still Tracking",
            &format!("{} > {} - {}", project, task, duration),
            Urgency::Low,
        )
    }

    pub fn send_idle_reminder(&self) -> Result<()> {
        self.send(
            "💎 MateriaTrack",
            "You're not tracking anything. Start a session?",
            Urgency::Low,
        )
    }

    pub fn send_daily_summary(&self, total_hours: f64, entry_count: usize) -> Result<()> {
        self.send(
            "📊 Daily Summary",
            &format!(
                "Today: {:.1} hours tracked across {} entries",
                total_hours, entry_count
            ),
            Urgency::Normal,
        )
    }

    pub fn send_achievement(&self, name: &str, description: &str) -> Result<()> {
        self.send(
            "🎉 Achievement Unlocked!",
            &format!("{} - {}", name, description),
            Urgency::Normal,
        )
    }

    pub fn send_pomodoro_work_complete(&self) -> Result<()> {
        self.send(
            "🍅 Pomodoro Complete!",
            &format!("Time for a {} minute break", POMODORO_BREAK_MINUTES),
            Urgency::Critical,
        )
    }

    pub fn send_pomodoro_break_complete(&self) -> Result<()> {
        self.send(
            "🍅 Break Over!",
            "Ready to start another Pomodoro?",
            Urgency::Critical,
        )
    }

    pub fn reminder_interval(&self) -> Duration {
        self.reminder_interval
    }

    pub fn should_remind(&self, entry: &Entry) -> bool {
        if !self.enabled {
            return false;
        }

        let elapsed = Utc::now() - entry.start;
        let intervals = elapsed.num_minutes() / self.reminder_interval.num_minutes();

        intervals > 0 && elapsed.num_seconds() % (self.reminder_interval.num_seconds()) < 60
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self {
            enabled: false,
            reminder_interval: Duration::minutes(DEFAULT_REMINDER_INTERVAL as i64),
            use_system_notify: check_notify_send(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

impl Urgency {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Normal => "normal",
            Self::Critical => "critical",
        }
    }
}

fn check_notify_send() -> bool {
    Command::new("notify-send")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn send_system_notification(title: &str, body: &str, urgency: Urgency) -> Result<()> {
    let result = Command::new("notify-send")
        .args([
            "--app-name=MateriaTrack",
            "--icon=appointment-soon",
            &format!("--urgency={}", urgency.as_str()),
            title,
            body,
        ])
        .output();

    match result {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Notification warning: {}", stderr);
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to send notification: {}", e);
            Ok(())
        }
    }
}

pub struct PomodoroTimer {
    work_duration: Duration,
    break_duration: Duration,
    current_session: Option<PomodoroSession>,
    completed_pomodoros: u32,
}

#[derive(Debug, Clone)]
pub struct PomodoroSession {
    pub session_type: SessionType,
    pub started_at: DateTime<Utc>,
    pub duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionType {
    Work,
    ShortBreak,
    LongBreak,
}

impl PomodoroTimer {
    pub fn new() -> Self {
        Self {
            work_duration: Duration::minutes(POMODORO_WORK_MINUTES as i64),
            break_duration: Duration::minutes(POMODORO_BREAK_MINUTES as i64),
            current_session: None,
            completed_pomodoros: 0,
        }
    }

    pub fn with_durations(work_minutes: u64, break_minutes: u64) -> Self {
        Self {
            work_duration: Duration::minutes(work_minutes as i64),
            break_duration: Duration::minutes(break_minutes as i64),
            current_session: None,
            completed_pomodoros: 0,
        }
    }

    pub fn start_work(&mut self) -> &PomodoroSession {
        self.current_session = Some(PomodoroSession {
            session_type: SessionType::Work,
            started_at: Utc::now(),
            duration: self.work_duration,
        });
        self.current_session.as_ref().unwrap()
    }

    pub fn start_break(&mut self) -> &PomodoroSession {
        let session_type = if self.completed_pomodoros > 0 && self.completed_pomodoros % 4 == 0 {
            SessionType::LongBreak
        } else {
            SessionType::ShortBreak
        };

        let duration = if session_type == SessionType::LongBreak {
            self.break_duration * 3
        } else {
            self.break_duration
        };

        self.current_session = Some(PomodoroSession {
            session_type,
            started_at: Utc::now(),
            duration,
        });
        self.current_session.as_ref().unwrap()
    }

    pub fn complete_session(&mut self) {
        if let Some(ref session) = self.current_session {
            if session.session_type == SessionType::Work {
                self.completed_pomodoros += 1;
            }
        }
        self.current_session = None;
    }

    pub fn current_session(&self) -> Option<&PomodoroSession> {
        self.current_session.as_ref()
    }

    pub fn remaining_time(&self) -> Option<Duration> {
        self.current_session.as_ref().map(|s| {
            let elapsed = Utc::now() - s.started_at;
            s.duration - elapsed
        })
    }

    pub fn is_complete(&self) -> bool {
        self.remaining_time()
            .map(|r| r <= Duration::zero())
            .unwrap_or(false)
    }

    pub fn completed_count(&self) -> u32 {
        self.completed_pomodoros
    }

    pub fn status(&self) -> String {
        match &self.current_session {
            Some(session) => {
                let remaining = self.remaining_time().unwrap_or(Duration::zero());
                let mins = remaining.num_minutes();
                let secs = remaining.num_seconds() % 60;

                let type_str = match session.session_type {
                    SessionType::Work => "🍅 Working",
                    SessionType::ShortBreak => "☕ Short Break",
                    SessionType::LongBreak => "🌴 Long Break",
                };

                format!("{} - {:02}:{:02} remaining", type_str, mins, secs)
            }
            None => format!("🍅 Pomodoros completed: {}", self.completed_pomodoros),
        }
    }
}

impl Default for PomodoroTimer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DailySummary {
    pub date: DateTime<Local>,
    pub total_seconds: i64,
    pub entry_count: usize,
    pub projects: Vec<String>,
    pub most_tracked_project: Option<String>,
}

impl DailySummary {
    pub fn total_hours(&self) -> f64 {
        self.total_seconds as f64 / 3600.0
    }

    pub fn format(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "\n📊 Daily Summary - {}\n",
            self.date.format("%A, %B %d")
        ));
        output.push_str(&"─".repeat(40));
        output.push('\n');

        output.push_str(&format!("⏱️  Total: {:.1} hours\n", self.total_hours()));
        output.push_str(&format!("📝 Entries: {}\n", self.entry_count));
        output.push_str(&format!("📁 Projects: {}\n", self.projects.len()));

        if let Some(ref top) = self.most_tracked_project {
            output.push_str(&format!("🏆 Most tracked: {}\n", top));
        }

        output
    }
}

pub fn should_send_daily_summary(hour: u32, minute: u32, target_hour: u32) -> bool {
    hour == target_hour && minute == 0
}

pub fn format_reminder_message(project: &str, task: &str, elapsed: Duration) -> String {
    let hours = elapsed.num_hours();
    let minutes = elapsed.num_minutes() % 60;

    if hours > 0 {
        format!(
            "Still tracking {} > {} ({}h {}m)",
            project, task, hours, minutes
        )
    } else {
        format!("Still tracking {} > {} ({}m)", project, task, minutes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_manager_default() {
        let nm = NotificationManager::default();
        assert!(!nm.is_enabled());
    }

    #[test]
    fn test_urgency() {
        assert_eq!(Urgency::Low.as_str(), "low");
        assert_eq!(Urgency::Normal.as_str(), "normal");
        assert_eq!(Urgency::Critical.as_str(), "critical");
    }

    #[test]
    fn test_pomodoro_timer() {
        let mut timer = PomodoroTimer::new();
        assert_eq!(timer.completed_count(), 0);

        timer.start_work();
        assert!(timer.current_session().is_some());
        assert_eq!(
            timer.current_session().unwrap().session_type,
            SessionType::Work
        );
    }

    #[test]
    fn test_pomodoro_break_after_work() {
        let mut timer = PomodoroTimer::new();

        timer.start_work();
        timer.complete_session();
        assert_eq!(timer.completed_count(), 1);

        timer.start_break();
        assert_eq!(
            timer.current_session().unwrap().session_type,
            SessionType::ShortBreak
        );
    }

    #[test]
    fn test_pomodoro_long_break() {
        let mut timer = PomodoroTimer::new();

        for _ in 0..4 {
            timer.start_work();
            timer.complete_session();
        }

        timer.start_break();
        assert_eq!(
            timer.current_session().unwrap().session_type,
            SessionType::LongBreak
        );
    }

    #[test]
    fn test_format_reminder() {
        let msg = format_reminder_message("Project", "Task", Duration::minutes(45));
        assert!(msg.contains("45m"));

        let msg2 = format_reminder_message("Project", "Task", Duration::hours(2));
        assert!(msg2.contains("2h"));
    }

    #[test]
    fn test_daily_summary_format() {
        let summary = DailySummary {
            date: Local::now(),
            total_seconds: 7200,
            entry_count: 5,
            projects: vec!["A".into(), "B".into()],
            most_tracked_project: Some("A".into()),
        };

        let formatted = summary.format();
        assert!(formatted.contains("2.0 hours"));
        assert!(formatted.contains("5"));
    }
}
