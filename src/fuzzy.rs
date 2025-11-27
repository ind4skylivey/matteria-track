//! Fuzzy finder for MateriaTrack
//!
//! Provides fuzzy matching for projects and tasks with frecency scoring.

use crate::database::Database;
use crate::error::Result;
use crate::models::{Project, Task};
use chrono::{DateTime, Duration, Utc};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::collections::HashMap;
use std::io::{self, BufRead};

pub struct FuzzyFinder {
    matcher: SkimMatcherV2,
    frecency: FrecencyScorer,
}

impl FuzzyFinder {
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
            frecency: FrecencyScorer::new(),
        }
    }

    pub fn with_frecency(mut self, frecency: FrecencyScorer) -> Self {
        self.frecency = frecency;
        self
    }

    pub fn match_score(&self, pattern: &str, text: &str) -> Option<i64> {
        self.matcher.fuzzy_match(text, pattern)
    }

    pub fn find_projects<'a>(&self, pattern: &str, projects: &'a [Project]) -> Vec<ScoredItem<&'a Project>> {
        let mut results: Vec<_> = projects
            .iter()
            .filter_map(|p| {
                let fuzzy_score = self.matcher.fuzzy_match(&p.name, pattern)?;
                let frecency_score = self.frecency.score(&p.name);
                let total_score = fuzzy_score + (frecency_score as i64 * 10);

                Some(ScoredItem {
                    item: p,
                    score: total_score,
                    fuzzy_score,
                    frecency_score,
                })
            })
            .collect();

        results.sort_by(|a, b| b.score.cmp(&a.score));
        results
    }

    pub fn find_tasks<'a>(&self, pattern: &str, tasks: &'a [Task]) -> Vec<ScoredItem<&'a Task>> {
        let mut results: Vec<_> = tasks
            .iter()
            .filter_map(|t| {
                let fuzzy_score = self.matcher.fuzzy_match(&t.name, pattern)?;
                let frecency_score = self.frecency.score(&t.name);
                let total_score = fuzzy_score + (frecency_score as i64 * 10);

                Some(ScoredItem {
                    item: t,
                    score: total_score,
                    fuzzy_score,
                    frecency_score,
                })
            })
            .collect();

        results.sort_by(|a, b| b.score.cmp(&a.score));
        results
    }

    pub fn find_best_project<'a>(&self, pattern: &str, projects: &'a [Project]) -> Option<&'a Project> {
        self.find_projects(pattern, projects)
            .into_iter()
            .next()
            .map(|s| s.item)
    }

    pub fn find_best_task<'a>(&self, pattern: &str, tasks: &'a [Task]) -> Option<&'a Task> {
        self.find_tasks(pattern, tasks).into_iter().next().map(|s| s.item)
    }
}

impl Default for FuzzyFinder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ScoredItem<T> {
    pub item: T,
    pub score: i64,
    pub fuzzy_score: i64,
    pub frecency_score: f64,
}

#[derive(Debug, Clone, Default)]
pub struct FrecencyScorer {
    access_times: HashMap<String, Vec<DateTime<Utc>>>,
    max_history: usize,
}

impl FrecencyScorer {
    pub fn new() -> Self {
        Self {
            access_times: HashMap::new(),
            max_history: 100,
        }
    }

    pub fn record_access(&mut self, item: &str) {
        let times = self.access_times.entry(item.to_string()).or_default();
        times.push(Utc::now());

        if times.len() > self.max_history {
            times.remove(0);
        }
    }

    pub fn score(&self, item: &str) -> f64 {
        let times = match self.access_times.get(item) {
            Some(t) => t,
            None => return 0.0,
        };

        let now = Utc::now();
        let mut score = 0.0;

        for time in times {
            let age = now - *time;
            let weight = Self::time_weight(age);
            score += weight;
        }

        score
    }

    fn time_weight(age: Duration) -> f64 {
        let hours = age.num_hours() as f64;

        if hours < 1.0 {
            100.0
        } else if hours < 24.0 {
            80.0
        } else if hours < 24.0 * 7.0 {
            60.0
        } else if hours < 24.0 * 30.0 {
            40.0
        } else {
            20.0
        }
    }

    pub fn load() -> Result<Self> {
        let path = Self::frecency_path()?;
        if path.exists() {
            let content = std::fs::read_to_string(&path)?;
            let data: HashMap<String, Vec<DateTime<Utc>>> = serde_json::from_str(&content)?;
            Ok(Self {
                access_times: data,
                max_history: 100,
            })
        } else {
            Ok(Self::new())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::frecency_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string(&self.access_times)?;
        std::fs::write(&path, content)?;
        Ok(())
    }

    fn frecency_path() -> Result<std::path::PathBuf> {
        crate::config::Config::config_dir().map(|d| d.join("frecency.json"))
    }
}

pub struct InteractivePicker {
    items: Vec<PickerItem>,
    selected: usize,
    filter: String,
    filtered_indices: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct PickerItem {
    pub id: i64,
    pub display: String,
    pub project: String,
    pub task: Option<String>,
    pub last_used: Option<DateTime<Utc>>,
}

impl InteractivePicker {
    pub fn new(items: Vec<PickerItem>) -> Self {
        let filtered_indices: Vec<usize> = (0..items.len()).collect();
        Self {
            items,
            selected: 0,
            filter: String::new(),
            filtered_indices,
        }
    }

    pub fn from_database(db: &Database) -> Result<Self> {
        let projects = db.list_projects()?;
        let mut items = Vec::new();

        for project in projects {
            let tasks = db.list_tasks(project.id)?;

            if tasks.is_empty() {
                items.push(PickerItem {
                    id: project.id,
                    display: format!("💎 {} │ (no tasks)", project.name),
                    project: project.name.clone(),
                    task: None,
                    last_used: None,
                });
            } else {
                for task in tasks {
                    items.push(PickerItem {
                        id: task.id,
                        display: format!("💎 {} │ ⚔️ {}", project.name, task.name),
                        project: project.name.clone(),
                        task: Some(task.name),
                        last_used: None,
                    });
                }
            }
        }

        Ok(Self::new(items))
    }

    pub fn update_filter(&mut self, filter: &str) {
        self.filter = filter.to_string();
        let matcher = SkimMatcherV2::default();

        self.filtered_indices = self
            .items
            .iter()
            .enumerate()
            .filter_map(|(i, item)| {
                if filter.is_empty() {
                    Some(i)
                } else {
                    matcher.fuzzy_match(&item.display, filter).map(|_| i)
                }
            })
            .collect();

        if self.selected >= self.filtered_indices.len() {
            self.selected = self.filtered_indices.len().saturating_sub(1);
        }
    }

    pub fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected < self.filtered_indices.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    pub fn selected_item(&self) -> Option<&PickerItem> {
        self.filtered_indices
            .get(self.selected)
            .and_then(|&i| self.items.get(i))
    }

    pub fn render(&self, max_items: usize) -> String {
        let mut output = String::new();

        output.push_str(&format!("\n💎 Quick Switch ({})\n", self.items.len()));
        output.push_str(&"─".repeat(50));
        output.push('\n');

        output.push_str(&format!("Filter: {}_\n\n", self.filter));

        let start = self.selected.saturating_sub(max_items / 2);
        let end = (start + max_items).min(self.filtered_indices.len());

        for (display_idx, &item_idx) in self.filtered_indices[start..end].iter().enumerate() {
            let item = &self.items[item_idx];
            let actual_idx = start + display_idx;

            let prefix = if actual_idx == self.selected {
                "▶ "
            } else {
                "  "
            };

            output.push_str(&format!("{}{}\n", prefix, item.display));
        }

        if self.filtered_indices.is_empty() {
            output.push_str("  (no matches)\n");
        }

        output.push_str(&"\n");
        output.push_str(&"─".repeat(50));
        output.push_str("\n[↑/↓] Navigate  [Enter] Select  [Esc] Cancel\n");

        output
    }

    pub fn run_interactive(&mut self) -> Option<PickerItem> {
        // Simple fallback for non-interactive mode
        println!("{}", self.render(10));
        println!("\nEnter selection number (1-{}): ", self.filtered_indices.len());

        let stdin = io::stdin();
        let mut input = String::new();

        if stdin.lock().read_line(&mut input).is_ok() {
            if let Ok(num) = input.trim().parse::<usize>() {
                if num > 0 && num <= self.filtered_indices.len() {
                    self.selected = num - 1;
                    return self.selected_item().cloned();
                }
            }
        }

        None
    }
}

pub fn quick_switch(db: &Database) -> Result<Option<(String, String)>> {
    let mut picker = InteractivePicker::from_database(db)?;

    if let Some(item) = picker.run_interactive() {
        let task = item.task.unwrap_or_else(|| "default".to_string());
        Ok(Some((item.project, task)))
    } else {
        Ok(None)
    }
}

pub fn format_recent_items(items: &[PickerItem], limit: usize) -> String {
    let mut output = String::new();

    output.push_str("\n💎 Recent Projects & Tasks\n");
    output.push_str(&"─".repeat(50));
    output.push('\n');

    for (i, item) in items.iter().take(limit).enumerate() {
        output.push_str(&format!("{:>2}. {}\n", i + 1, item.display));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_finder() {
        let finder = FuzzyFinder::new();

        let projects = vec![
            Project::new("MateriaTrack"),
            Project::new("Another Project"),
            Project::new("Test"),
        ];

        let results = finder.find_projects("mat", &projects);
        assert!(!results.is_empty());
        assert_eq!(results[0].item.name, "MateriaTrack");
    }

    #[test]
    fn test_frecency_scorer() {
        let mut scorer = FrecencyScorer::new();

        scorer.record_access("test");
        let score = scorer.score("test");
        assert!(score > 0.0);

        let empty_score = scorer.score("nonexistent");
        assert_eq!(empty_score, 0.0);
    }

    #[test]
    fn test_picker_item() {
        let item = PickerItem {
            id: 1,
            display: "💎 Project │ ⚔️ Task".to_string(),
            project: "Project".to_string(),
            task: Some("Task".to_string()),
            last_used: None,
        };

        assert_eq!(item.project, "Project");
        assert_eq!(item.task, Some("Task".to_string()));
    }

    #[test]
    fn test_interactive_picker_filter() {
        let items = vec![
            PickerItem {
                id: 1,
                display: "Project A".to_string(),
                project: "A".to_string(),
                task: None,
                last_used: None,
            },
            PickerItem {
                id: 2,
                display: "Project B".to_string(),
                project: "B".to_string(),
                task: None,
                last_used: None,
            },
        ];

        let mut picker = InteractivePicker::new(items);
        picker.update_filter("A");

        assert_eq!(picker.filtered_indices.len(), 1);
    }

    #[test]
    fn test_picker_navigation() {
        let items = vec![
            PickerItem {
                id: 1,
                display: "A".to_string(),
                project: "A".to_string(),
                task: None,
                last_used: None,
            },
            PickerItem {
                id: 2,
                display: "B".to_string(),
                project: "B".to_string(),
                task: None,
                last_used: None,
            },
        ];

        let mut picker = InteractivePicker::new(items);
        assert_eq!(picker.selected, 0);

        picker.move_down();
        assert_eq!(picker.selected, 1);

        picker.move_up();
        assert_eq!(picker.selected, 0);
    }
}
