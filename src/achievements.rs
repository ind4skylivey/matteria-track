//! Achievement system for MatteriaTrack
//!
//! Final Fantasy-inspired achievements and milestones.

use crate::database::Database;
use crate::error::Result;
use chrono::{DateTime, Datelike, Local, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub const KONAMI_CODE: &str = "↑↑↓↓←→←→BA";
pub const SECRET_COMMAND: &str = "omnislash";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub points: u32,
    pub secret: bool,
    pub category: AchievementCategory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AchievementCategory {
    Tracking,
    Milestones,
    Dedication,
    Special,
    Secret,
}

impl AchievementCategory {
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Tracking => "⚔️",
            Self::Milestones => "🏆",
            Self::Dedication => "💪",
            Self::Special => "✨",
            Self::Secret => "🔮",
        }
    }
}

pub static ACHIEVEMENTS: &[Achievement] = &[
    // Tracking achievements
    Achievement {
        id: "materia_equipped",
        name: "Materia Equipped",
        description: "Create your first tracked entry",
        icon: "💎",
        points: 10,
        secret: false,
        category: AchievementCategory::Tracking,
    },
    Achievement {
        id: "chocobo_rider",
        name: "Chocobo Rider",
        description: "Track 10 hours total",
        icon: "🐤",
        points: 25,
        secret: false,
        category: AchievementCategory::Milestones,
    },
    Achievement {
        id: "limit_break",
        name: "Limit Break",
        description: "Track 100 hours total",
        icon: "⚡",
        points: 100,
        secret: false,
        category: AchievementCategory::Milestones,
    },
    Achievement {
        id: "summoner",
        name: "Summoner",
        description: "Create 1000 entries",
        icon: "🐉",
        points: 150,
        secret: false,
        category: AchievementCategory::Milestones,
    },
    Achievement {
        id: "week_warrior",
        name: "Week Warrior",
        description: "Track for 7 consecutive days",
        icon: "🗡️",
        points: 50,
        secret: false,
        category: AchievementCategory::Dedication,
    },
    Achievement {
        id: "month_master",
        name: "Month Master",
        description: "Track for 30 consecutive days",
        icon: "👑",
        points: 100,
        secret: false,
        category: AchievementCategory::Dedication,
    },
    Achievement {
        id: "midnight_oil",
        name: "Midnight Oil",
        description: "Start tracking after midnight",
        icon: "🌙",
        points: 15,
        secret: false,
        category: AchievementCategory::Special,
    },
    Achievement {
        id: "early_bird",
        name: "Early Bird",
        description: "Start tracking before 6am",
        icon: "🌅",
        points: 15,
        secret: false,
        category: AchievementCategory::Special,
    },
    Achievement {
        id: "workaholic",
        name: "Workaholic",
        description: "Track 12+ hours in a single day",
        icon: "💼",
        points: 30,
        secret: false,
        category: AchievementCategory::Special,
    },
    Achievement {
        id: "speedrunner",
        name: "Speedrunner",
        description: "Complete 10 tasks in 1 hour",
        icon: "🏃",
        points: 25,
        secret: false,
        category: AchievementCategory::Special,
    },
    Achievement {
        id: "completionist",
        name: "Completionist",
        description: "Use all 6 Materia themes",
        icon: "🎨",
        points: 20,
        secret: false,
        category: AchievementCategory::Special,
    },
    Achievement {
        id: "mako_infused",
        name: "Mako Infused",
        description: "Track 500 hours total",
        icon: "💚",
        points: 200,
        secret: false,
        category: AchievementCategory::Milestones,
    },
    Achievement {
        id: "soldier_first_class",
        name: "SOLDIER First Class",
        description: "Track 1000 hours total",
        icon: "🎖️",
        points: 500,
        secret: false,
        category: AchievementCategory::Milestones,
    },
    Achievement {
        id: "project_master",
        name: "Project Master",
        description: "Create 50 different projects",
        icon: "📁",
        points: 40,
        secret: false,
        category: AchievementCategory::Tracking,
    },
    Achievement {
        id: "task_juggler",
        name: "Task Juggler",
        description: "Create 100 different tasks",
        icon: "🎯",
        points: 50,
        secret: false,
        category: AchievementCategory::Tracking,
    },
    Achievement {
        id: "night_owl",
        name: "Night Owl",
        description: "Track time between 2 AM and 5 AM",
        icon: "🦉",
        points: 20,
        secret: false,
        category: AchievementCategory::Special,
    },
    Achievement {
        id: "weekend_warrior",
        name: "Weekend Warrior",
        description: "Track time on a weekend",
        icon: "🌴",
        points: 15,
        secret: false,
        category: AchievementCategory::Dedication,
    },
    Achievement {
        id: "multitasker",
        name: "Multitasker",
        description: "Switch tasks 10 times in a single day",
        icon: "🤹",
        points: 30,
        secret: false,
        category: AchievementCategory::Tracking,
    },
    Achievement {
        id: "over_9000",
        name: "It's Over 9000!",
        description: "Track over 9000 minutes total",
        icon: "🔥",
        points: 90,
        secret: false,
        category: AchievementCategory::Milestones,
    },
    // Secret achievements
    Achievement {
        id: "secret_passage",
        name: "Secret Passage",
        description: "Find the hidden command",
        icon: "🚪",
        points: 50,
        secret: true,
        category: AchievementCategory::Secret,
    },
    Achievement {
        id: "konami_master",
        name: "Konami Master",
        description: "Enter the legendary code",
        icon: "🎮",
        points: 100,
        secret: true,
        category: AchievementCategory::Secret,
    },
    Achievement {
        id: "omnislash",
        name: "Omnislash",
        description: "Execute the ultimate limit break",
        icon: "⚔️",
        points: 150,
        secret: true,
        category: AchievementCategory::Secret,
    },
    Achievement {
        id: "aerith_lives",
        name: "Aerith Lives",
        description: "Track continuously for 24 hours",
        icon: "🌸",
        points: 200,
        secret: true,
        category: AchievementCategory::Secret,
    },
    Achievement {
        id: "knights_of_round",
        name: "Knights of the Round",
        description: "Complete all other achievements",
        icon: "🏰",
        points: 1000,
        secret: true,
        category: AchievementCategory::Secret,
    },
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementProgress {
    pub unlocked: HashMap<String, DateTime<Utc>>,
    pub themes_used: Vec<String>,
    pub consecutive_days: u32,
    pub last_track_date: Option<DateTime<Utc>>,
    pub total_hours: f64,
    pub total_entries: u64,
    pub total_projects: u64,
    pub total_tasks: u64,
}

impl Default for AchievementProgress {
    fn default() -> Self {
        Self {
            unlocked: HashMap::new(),
            themes_used: Vec::new(),
            consecutive_days: 0,
            last_track_date: None,
            total_hours: 0.0,
            total_entries: 0,
            total_projects: 0,
            total_tasks: 0,
        }
    }
}

impl AchievementProgress {
    pub fn load() -> Result<Self> {
        let path = Self::progress_path()?;
        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let progress: Self = serde_json::from_str(&content)?;
            Ok(progress)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::progress_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    fn progress_path() -> Result<PathBuf> {
        crate::config::Config::config_dir().map(|d| d.join("achievements.json"))
    }

    pub fn is_unlocked(&self, id: &str) -> bool {
        self.unlocked.contains_key(id)
    }

    pub fn unlock(&mut self, id: &str) -> Option<&'static Achievement> {
        if self.is_unlocked(id) {
            return None;
        }

        if let Some(achievement) = ACHIEVEMENTS.iter().find(|a| a.id == id) {
            self.unlocked.insert(id.to_string(), Utc::now());
            Some(achievement)
        } else {
            None
        }
    }

    pub fn total_points(&self) -> u32 {
        self.unlocked
            .keys()
            .filter_map(|id| ACHIEVEMENTS.iter().find(|a| a.id == id))
            .map(|a| a.points)
            .sum()
    }

    pub fn completion_percentage(&self) -> f64 {
        let total = ACHIEVEMENTS.len();
        let unlocked = self.unlocked.len();
        (unlocked as f64 / total as f64) * 100.0
    }

    pub fn add_theme(&mut self, theme: &str) {
        if !self.themes_used.contains(&theme.to_string()) {
            self.themes_used.push(theme.to_string());
        }
    }
}

#[derive(Default)]
pub struct AchievementChecker {
    progress: AchievementProgress,
}

impl AchievementChecker {
    pub fn new() -> Result<Self> {
        let progress = AchievementProgress::load()?;
        Ok(Self { progress })
    }

    pub fn progress(&self) -> &AchievementProgress {
        &self.progress
    }

    pub fn progress_mut(&mut self) -> &mut AchievementProgress {
        &mut self.progress
    }

    pub fn check_and_unlock(&mut self, db: &Database) -> Result<Vec<&'static Achievement>> {
        let mut newly_unlocked = Vec::new();

        // Update stats from database
        self.update_stats(db)?;

        // Check each achievement
        if !self.progress.is_unlocked("materia_equipped") && self.progress.total_entries >= 1 {
            if let Some(a) = self.progress.unlock("materia_equipped") {
                newly_unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("chocobo_rider") && self.progress.total_hours >= 10.0 {
            if let Some(a) = self.progress.unlock("chocobo_rider") {
                newly_unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("limit_break") && self.progress.total_hours >= 100.0 {
            if let Some(a) = self.progress.unlock("limit_break") {
                newly_unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("mako_infused") && self.progress.total_hours >= 500.0 {
            if let Some(a) = self.progress.unlock("mako_infused") {
                newly_unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("soldier_first_class") && self.progress.total_hours >= 1000.0
        {
            if let Some(a) = self.progress.unlock("soldier_first_class") {
                newly_unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("summoner") && self.progress.total_entries >= 1000 {
            if let Some(a) = self.progress.unlock("summoner") {
                newly_unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("week_warrior") && self.progress.consecutive_days >= 7 {
            if let Some(a) = self.progress.unlock("week_warrior") {
                newly_unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("month_master") && self.progress.consecutive_days >= 30 {
            if let Some(a) = self.progress.unlock("month_master") {
                newly_unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("project_master") && self.progress.total_projects >= 50 {
            if let Some(a) = self.progress.unlock("project_master") {
                newly_unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("task_juggler") && self.progress.total_tasks >= 100 {
            if let Some(a) = self.progress.unlock("task_juggler") {
                newly_unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("completionist") && self.progress.themes_used.len() >= 6 {
            if let Some(a) = self.progress.unlock("completionist") {
                newly_unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("over_9000") && self.progress.total_hours * 60.0 >= 9000.0 {
            if let Some(a) = self.progress.unlock("over_9000") {
                newly_unlocked.push(a);
            }
        }

        // Check multitasker (10+ entries today)
        let today = Local::now().date_naive();
        let entries = db.list_entries(None)?;
        let today_count = entries
            .iter()
            .filter(|e| e.start.with_timezone(&Local).date_naive() == today)
            .count();

        if !self.progress.is_unlocked("multitasker") && today_count >= 10 {
            if let Some(a) = self.progress.unlock("multitasker") {
                newly_unlocked.push(a);
            }
        }

        // Check if all non-secret achievements unlocked for Knights of the Round
        let non_secret_count = ACHIEVEMENTS.iter().filter(|a| !a.secret).count();
        let unlocked_non_secret = self
            .progress
            .unlocked
            .keys()
            .filter(|id| {
                ACHIEVEMENTS
                    .iter()
                    .find(|a| a.id == *id)
                    .map(|a| !a.secret)
                    .unwrap_or(false)
            })
            .count();

        if !self.progress.is_unlocked("knights_of_round") && unlocked_non_secret >= non_secret_count
        {
            if let Some(a) = self.progress.unlock("knights_of_round") {
                newly_unlocked.push(a);
            }
        }

        self.progress.save()?;
        Ok(newly_unlocked)
    }

    fn update_stats(&mut self, db: &Database) -> Result<()> {
        let entries = db.list_entries(None)?;
        let projects = db.list_projects()?;
        let tasks = db.list_all_tasks()?;

        self.progress.total_entries = entries.len() as u64;
        self.progress.total_projects = projects.len() as u64;
        self.progress.total_tasks = tasks.len() as u64;

        let total_seconds: i64 = entries.iter().map(|e| e.duration().num_seconds()).sum();
        self.progress.total_hours = total_seconds as f64 / 3600.0;

        // Calculate consecutive days
        self.calculate_consecutive_days(&entries);

        Ok(())
    }

    fn calculate_consecutive_days(&mut self, entries: &[crate::models::Entry]) {
        if entries.is_empty() {
            self.progress.consecutive_days = 0;
            return;
        }

        let mut dates: Vec<_> = entries
            .iter()
            .map(|e| e.start.with_timezone(&Local).date_naive())
            .collect();
        dates.sort();
        dates.dedup();

        let today = Local::now().date_naive();
        let mut streak = 0;
        let mut current = today;

        for date in dates.iter().rev() {
            if *date == current || *date == current - chrono::Duration::days(1) {
                streak += 1;
                current = *date;
            } else {
                break;
            }
        }

        self.progress.consecutive_days = streak;
    }

    pub fn check_time_based(&mut self, start_time: DateTime<Utc>) -> Vec<&'static Achievement> {
        let mut unlocked = Vec::new();
        let local_time = start_time.with_timezone(&Local);
        let hour = local_time.hour();

        if !self.progress.is_unlocked("midnight_oil") && hour < 4 {
            if let Some(a) = self.progress.unlock("midnight_oil") {
                unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("early_bird") && hour < 6 {
            if let Some(a) = self.progress.unlock("early_bird") {
                unlocked.push(a);
            }
        }

        if !self.progress.is_unlocked("night_owl") && (2..5).contains(&hour) {
            if let Some(a) = self.progress.unlock("night_owl") {
                unlocked.push(a);
            }
        }

        let weekday = local_time.weekday();
        if !self.progress.is_unlocked("weekend_warrior")
            && (weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun)
        {
            if let Some(a) = self.progress.unlock("weekend_warrior") {
                unlocked.push(a);
            }
        }

        let _ = self.progress.save();
        unlocked
    }

    pub fn check_secret_code(&mut self, code: &str) -> Option<&'static Achievement> {
        if code == KONAMI_CODE && !self.progress.is_unlocked("konami_master") {
            let a = self.progress.unlock("konami_master");
            let _ = self.progress.save();
            return a;
        }

        if code == SECRET_COMMAND && !self.progress.is_unlocked("omnislash") {
            let a = self.progress.unlock("omnislash");
            let _ = self.progress.save();
            return a;
        }

        if code == "aerith" && !self.progress.is_unlocked("secret_passage") {
            let a = self.progress.unlock("secret_passage");
            let _ = self.progress.save();
            return a;
        }

        None
    }
}

pub fn format_achievement_unlocked(achievement: &Achievement) -> String {
    format!(
        "\n🎉 Achievement Unlocked! 🎉\n\n{} {} - {}\n{}\n+{} points\n",
        achievement.icon,
        achievement.name,
        achievement.category.icon(),
        achievement.description,
        achievement.points
    )
}

pub fn format_achievements_list(progress: &AchievementProgress) -> String {
    let mut output = String::new();

    output.push_str("\n💎 MatteriaTrack Achievements 💎\n");
    output.push_str(&"═".repeat(50));
    output.push_str(&format!(
        "\n\nProgress: {:.1}% | Points: {}\n\n",
        progress.completion_percentage(),
        progress.total_points()
    ));

    for category in [
        AchievementCategory::Tracking,
        AchievementCategory::Milestones,
        AchievementCategory::Dedication,
        AchievementCategory::Special,
    ] {
        let achievements: Vec<_> = ACHIEVEMENTS
            .iter()
            .filter(|a| a.category == category && !a.secret)
            .collect();

        if !achievements.is_empty() {
            output.push_str(&format!("\n{} {:?}\n", category.icon(), category));
            output.push_str(&"─".repeat(40));
            output.push('\n');

            for a in achievements {
                let status = if progress.is_unlocked(a.id) {
                    "✓"
                } else {
                    "○"
                };
                output.push_str(&format!(
                    "{} {} {} ({} pts)\n   {}\n",
                    status, a.icon, a.name, a.points, a.description
                ));
            }
        }
    }

    // Show unlocked secrets
    let unlocked_secrets: Vec<_> = ACHIEVEMENTS
        .iter()
        .filter(|a| a.secret && progress.is_unlocked(a.id))
        .collect();

    if !unlocked_secrets.is_empty() {
        output.push_str(&format!(
            "\n{} Secrets Discovered\n",
            AchievementCategory::Secret.icon()
        ));
        output.push_str(&"─".repeat(40));
        output.push('\n');

        for a in &unlocked_secrets {
            output.push_str(&format!("✓ {} {} ({} pts)\n", a.icon, a.name, a.points));
        }
    }

    let secret_count = ACHIEVEMENTS.iter().filter(|a| a.secret).count();
    let unlocked_secret_count = unlocked_secrets.len();
    output.push_str(&format!(
        "\n🔮 Secrets: {}/{} discovered\n",
        unlocked_secret_count, secret_count
    ));

    output.push('\n');
    output.push_str(&"═".repeat(50));
    output.push('\n');

    output
}

pub fn trigger_easter_egg(code: &str) -> Option<String> {
    match code {
        "↑↑↓↓←→←→BA" => Some(
            r#"
    ╔═══════════════════════════════════════════════════════════╗
    ║                                                           ║
    ║   🎮 KONAMI CODE ACTIVATED! 🎮                            ║
    ║                                                           ║
    ║   "The planet's dyin', Cloud!"                            ║
    ║                                      - Barret Wallace     ║
    ║                                                           ║
    ║   +30 lives... wait, this isn't Contra!                   ║
    ║   But you've unlocked something special...                ║
    ║                                                           ║
    ╚═══════════════════════════════════════════════════════════╝
"#
            .to_string(),
        ),
        "omnislash" => Some(
            r#"
    ╔═══════════════════════════════════════════════════════════╗
    ║                                                           ║
    ║   ⚔️  OMNISLASH! ⚔️                                       ║
    ║                                                           ║
    ║   *Cloud's ultimate Limit Break*                          ║
    ║                                                           ║
    ║   15 consecutive slashes... your productivity enemies     ║
    ║   don't stand a chance!                                   ║
    ║                                                           ║
    ║   "Let's mosey." - Cloud Strife                           ║
    ║                                                           ║
    ╚═══════════════════════════════════════════════════════════╝
"#
            .to_string(),
        ),
        "aerith" => Some(
            r#"
    ╔═══════════════════════════════════════════════════════════╗
    ║                                                           ║
    ║   🌸 Secret Passage Discovered! 🌸                        ║
    ║                                                           ║
    ║   "I'll be going now. I'll come back when it's all over." ║
    ║                                      - Aerith Gainsborough║
    ║                                                           ║
    ║   You found the hidden path. Keep tracking your time,     ║
    ║   and may the Lifestream guide your productivity.         ║
    ║                                                           ║
    ╚═══════════════════════════════════════════════════════════╝
"#
            .to_string(),
        ),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_achievement_count() {
        assert_eq!(ACHIEVEMENTS.len(), 24);
    }

    #[test]
    fn test_secret_achievements() {
        let secrets: Vec<_> = ACHIEVEMENTS.iter().filter(|a| a.secret).collect();
        assert_eq!(secrets.len(), 5);
    }

    #[test]
    fn test_progress_default() {
        let progress = AchievementProgress::default();
        assert!(progress.unlocked.is_empty());
        assert_eq!(progress.total_hours, 0.0);
    }

    #[test]
    fn test_unlock_achievement() {
        let mut progress = AchievementProgress::default();
        let unlocked = progress.unlock("materia_equipped");
        assert!(unlocked.is_some());
        assert!(progress.is_unlocked("materia_equipped"));

        // Can't unlock twice
        let unlocked_again = progress.unlock("materia_equipped");
        assert!(unlocked_again.is_none());
    }

    #[test]
    fn test_total_points() {
        let mut progress = AchievementProgress::default();
        progress.unlock("materia_equipped"); // 10 points
        progress.unlock("chocobo_rider"); // 25 points
        assert_eq!(progress.total_points(), 35);
    }

    #[test]
    fn test_easter_eggs() {
        assert!(trigger_easter_egg("↑↑↓↓←→←→BA").is_some());
        assert!(trigger_easter_egg("omnislash").is_some());
        assert!(trigger_easter_egg("invalid").is_none());
    }

    #[test]
    fn test_theme_tracking() {
        let mut progress = AchievementProgress::default();
        progress.add_theme("fire");
        progress.add_theme("ice");
        progress.add_theme("fire"); // duplicate
        assert_eq!(progress.themes_used.len(), 2);
    }
}
