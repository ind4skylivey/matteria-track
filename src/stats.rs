//! Statistics and reporting for MateriaTrack

use crate::database::Database;
use crate::error::Result;
use crate::models::{Entry, ProjectStats, TaskStats, TimeStats};
use crate::theme::MateriaTheme;
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, TimeZone, Utc};
use colored::Colorize;
use std::collections::HashMap;

type TaskData = (String, i64, usize);
type ProjectData = (String, i64, usize, HashMap<i64, TaskData>);

pub struct StatsEngine {
    db: Database,
    theme: MateriaTheme,
}

impl StatsEngine {
    pub fn new(db: Database, theme: MateriaTheme) -> Self {
        Self { db, theme }
    }

    pub fn calculate_stats(&self, since: Option<DateTime<Utc>>) -> Result<TimeStats> {
        let entries = self.db.list_entries(since)?;
        self.compute_stats(&entries)
    }

    pub fn today_stats(&self) -> Result<TimeStats> {
        let today = Local::now().date_naive().and_hms_opt(0, 0, 0).unwrap();
        let since = Local
            .from_local_datetime(&today)
            .unwrap()
            .with_timezone(&Utc);
        self.calculate_stats(Some(since))
    }

    pub fn week_stats(&self) -> Result<TimeStats> {
        let today = Local::now().date_naive();
        let days_since_monday = today.weekday().num_days_from_monday();
        let monday = today - Duration::days(days_since_monday as i64);
        let monday_midnight = monday.and_hms_opt(0, 0, 0).unwrap();
        let since = Local
            .from_local_datetime(&monday_midnight)
            .unwrap()
            .with_timezone(&Utc);
        self.calculate_stats(Some(since))
    }

    pub fn month_stats(&self) -> Result<TimeStats> {
        let today = Local::now().date_naive();
        let first_of_month = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
        let first_midnight = first_of_month.and_hms_opt(0, 0, 0).unwrap();
        let since = Local
            .from_local_datetime(&first_midnight)
            .unwrap()
            .with_timezone(&Utc);
        self.calculate_stats(Some(since))
    }

    fn compute_stats(&self, entries: &[Entry]) -> Result<TimeStats> {
        let mut total_seconds: i64 = 0;
        let mut project_map: HashMap<i64, ProjectData> = HashMap::new();

        for entry in entries {
            let duration = entry.duration().num_seconds();
            total_seconds += duration;

            let project = self.db.get_project(entry.project_id)?;
            let task = self.db.get_task(entry.task_id)?;

            if let (Some(p), Some(t)) = (project, task) {
                let project_entry = project_map
                    .entry(p.id)
                    .or_insert_with(|| (p.name.clone(), 0, 0, HashMap::new()));

                project_entry.1 += duration;
                project_entry.2 += 1;

                let task_entry = project_entry
                    .3
                    .entry(t.id)
                    .or_insert_with(|| (t.name.clone(), 0, 0));

                task_entry.1 += duration;
                task_entry.2 += 1;
            }
        }

        let projects: Vec<ProjectStats> = project_map
            .into_iter()
            .map(|(id, (name, secs, count, tasks_map))| {
                let tasks: Vec<TaskStats> = tasks_map
                    .into_iter()
                    .map(|(tid, (tname, tsecs, tcount))| TaskStats {
                        task_id: tid,
                        task_name: tname,
                        total_seconds: tsecs,
                        entry_count: tcount,
                    })
                    .collect();

                ProjectStats {
                    project_id: id,
                    project_name: name,
                    total_seconds: secs,
                    entry_count: count,
                    tasks,
                }
            })
            .collect();

        Ok(TimeStats {
            total_seconds,
            entry_count: entries.len(),
            projects,
        })
    }

    pub fn format_stats(&self, stats: &TimeStats, title: &str) -> String {
        let (r, g, b) = self.theme.primary_color();
        let mut output = String::new();

        output.push_str(&format!(
            "\n{} {} {}\n",
            self.theme.icon(),
            title.truecolor(r, g, b).bold(),
            self.theme.materia_icon()
        ));
        output.push_str(&"━".repeat(50));
        output.push('\n');

        output.push_str(&format!(
            "\n⏱️  Total Time: {}\n",
            stats.total_formatted().truecolor(r, g, b).bold()
        ));
        output.push_str(&format!("📊 Total Entries: {}\n\n", stats.entry_count));

        if !stats.projects.is_empty() {
            output.push_str(&format!("{}\n", "Projects:".bold()));

            let mut sorted_projects = stats.projects.clone();
            sorted_projects.sort_by(|a, b| b.total_seconds.cmp(&a.total_seconds));

            for project in &sorted_projects {
                let percentage = project.percentage_of(stats.total_seconds);
                let bar = progress_bar(percentage, 20);

                output.push_str(&format!(
                    "  {} {} {} ({:.1}%)\n",
                    "".truecolor(r, g, b),
                    project.project_name.bold(),
                    project.total_formatted(),
                    percentage
                ));
                output.push_str(&format!("     {}\n", bar.truecolor(r, g, b)));

                let mut sorted_tasks = project.tasks.clone();
                sorted_tasks.sort_by(|a, b| b.total_seconds.cmp(&a.total_seconds));

                for task in &sorted_tasks {
                    output.push_str(&format!(
                        "       {} {} ({})\n",
                        "",
                        task.task_name,
                        task.total_formatted()
                    ));
                }
            }
        }

        output.push('\n');
        output.push_str(&"━".repeat(50));
        output.push('\n');

        output
    }

    pub fn format_stats_json(&self, stats: &TimeStats) -> Result<String> {
        Ok(serde_json::to_string_pretty(stats)?)
    }

    pub fn format_daily_breakdown(&self, since: DateTime<Utc>) -> Result<String> {
        let entries = self.db.list_entries(Some(since))?;
        let (r, g, b) = self.theme.primary_color();

        let mut daily: HashMap<NaiveDate, i64> = HashMap::new();

        for entry in &entries {
            let date = entry.start.with_timezone(&Local).date_naive();
            let duration = entry.duration().num_seconds();
            *daily.entry(date).or_insert(0) += duration;
        }

        let mut dates: Vec<_> = daily.into_iter().collect();
        dates.sort_by(|a, b| b.0.cmp(&a.0));

        let mut output = String::new();
        output.push_str(&format!(
            "\n{} {} Daily Breakdown {}\n",
            self.theme.icon(),
            "📅".truecolor(r, g, b),
            self.theme.materia_icon()
        ));
        output.push_str(&"━".repeat(50));
        output.push('\n');

        let max_seconds = dates.iter().map(|(_, s)| *s).max().unwrap_or(1);

        for (date, seconds) in dates {
            let hours = seconds / 3600;
            let minutes = (seconds % 3600) / 60;
            let bar_width = ((seconds as f64 / max_seconds as f64) * 30.0) as usize;
            let bar = "█".repeat(bar_width);

            output.push_str(&format!(
                "  {} {:>2}h {:>2}m {}\n",
                date.format("%Y-%m-%d"),
                hours,
                minutes,
                bar.truecolor(r, g, b)
            ));
        }

        output.push('\n');
        Ok(output)
    }
}

fn progress_bar(percentage: f64, width: usize) -> String {
    let filled = ((percentage / 100.0) * width as f64) as usize;
    let empty = width.saturating_sub(filled);

    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

pub fn format_time_colored(seconds: i64, theme: MateriaTheme) -> String {
    let (r, g, b) = theme.primary_color();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;

    if hours > 0 {
        format!("{}h {}m", hours, minutes)
            .truecolor(r, g, b)
            .to_string()
    } else {
        format!("{}m", minutes).truecolor(r, g, b).to_string()
    }
}

pub fn format_project_table(
    entries: &[crate::models::EntryWithDetails],
    theme: MateriaTheme,
) -> String {
    let (r, g, b) = theme.primary_color();
    let mut output = String::new();

    output.push_str(&format!(
        "\n{} Recent Entries {}\n",
        theme.icon(),
        theme.materia_icon()
    ));
    output.push_str(&"━".repeat(70));
    output.push('\n');

    output.push_str(&format!(
        "{:<12} {:<15} {:<20} {:<10} {}\n",
        "Date".bold(),
        "Project".bold(),
        "Task".bold(),
        "Duration".bold(),
        "Status".bold()
    ));
    output.push_str(&"─".repeat(70));
    output.push('\n');

    for entry in entries {
        let date = entry.entry.start_local().format("%Y-%m-%d");
        let duration = entry.entry.duration_formatted();
        let status = if entry.entry.is_active() {
            "⚡ Active".truecolor(255, 200, 50).to_string()
        } else {
            "✓ Done".truecolor(100, 255, 100).to_string()
        };

        let project_display = if entry.project_name.len() > 14 {
            format!("{}…", &entry.project_name[..13])
        } else {
            entry.project_name.clone()
        };

        let task_display = if entry.task_name.len() > 19 {
            format!("{}…", &entry.task_name[..18])
        } else {
            entry.task_name.clone()
        };

        output.push_str(&format!(
            "{:<12} {:<15} {:<20} {:<10} {}\n",
            date,
            project_display.truecolor(r, g, b),
            task_display,
            duration,
            status
        ));
    }

    output.push_str(&"━".repeat(70));
    output.push('\n');

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar() {
        let bar = progress_bar(50.0, 10);
        assert_eq!(bar.chars().filter(|c| *c == '█').count(), 5);
        assert_eq!(bar.chars().filter(|c| *c == '░').count(), 5);
    }

    #[test]
    fn test_format_time_colored() {
        let result = format_time_colored(3661, MateriaTheme::Fire);
        assert!(result.contains("1h"));
    }
}
