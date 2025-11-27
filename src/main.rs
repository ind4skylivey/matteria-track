//! MateriaTrack - Final Fantasy-themed time tracking CLI

#![allow(dead_code)]

mod achievements;
mod cli;
mod config;
mod database;
mod error;
mod fuzzy;
mod integrations;
mod models;
mod notifications;
mod security;
mod stats;
mod theme;
mod themes;
mod tracking;
mod ui;

use chrono::{DateTime, Local, NaiveDate, TimeZone, Utc};
use cli::{Cli, Commands, ConfigCommands, OutputFormat, ProjectCommands, TaskCommands};
use colored::Colorize;
use config::Config;
use database::Database;
use error::Result;
use models::Project;
use stats::StatsEngine;
use tracking::TrackingEngine;

use crate::cli::{print_error, print_info, print_success, print_tracking};

#[tokio::main]
async fn main() {
    let exit_code = match run().await {
        Ok(_) => 0,
        Err(e) => {
            print_error(&e.to_string());
            match e {
                error::Error::Config(_) => 2,
                error::Error::Database(_) => 3,
                _ => 1,
            }
        }
    };

    std::process::exit(exit_code);
}

async fn run() -> Result<()> {
    let cli = Cli::parse_args();

    let config = if let Some(ref path) = cli.config {
        Config::load_from_path(path)?
    } else {
        Config::load()?
    };

    let db_path = config.db_path()?;
    let db = Database::open(&db_path)?;
    let theme = config.theme();
    let engine = TrackingEngine::new(db, config.clone());

    match cli.command {
        Commands::Track {
            project,
            task,
            begin,
            notes,
        } => {
            let (entry, proj, tsk) =
                engine.start_tracking(&project, &task, begin.as_deref(), notes.as_deref())?;

            match cli.format {
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&entry)?);
                }
                OutputFormat::Statusbar => {
                    println!(
                        "{}",
                        tracking::statusbar_output(
                            &proj.name,
                            &tsk.name,
                            0,
                            true,
                            Some(theme.icon())
                        )
                    );
                }
                _ => {
                    print_success(&format!(
                        "Started tracking {} {} {} at {}",
                        proj.name.bold(),
                        "→".truecolor(100, 100, 100),
                        tsk.name,
                        entry.start_local().format("%H:%M:%S")
                    ));
                }
            }
        }

        Commands::Finish {
            task,
            begin,
            end,
            notes,
        } => {
            let (entry, proj, tsk) = engine.finish_tracking(
                task.as_deref(),
                begin.as_deref(),
                end.as_deref(),
                notes.as_deref(),
            )?;

            match cli.format {
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&entry)?);
                }
                _ => {
                    print_success(&format!(
                        "Finished tracking {} {} {} ({})",
                        proj.name.bold(),
                        "→".truecolor(100, 100, 100),
                        tsk.name,
                        entry.duration_formatted()
                    ));

                    if !entry.git_commits.is_empty() {
                        print_info(&format!(
                            "Git commits captured: {}",
                            entry.git_commits.len()
                        ));
                    }
                }
            }
        }

        Commands::Status => {
            if let Some((entry, proj, tsk)) = engine.get_status()? {
                match cli.format {
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&entry)?);
                    }
                    OutputFormat::Statusbar => {
                        let secs = entry.duration().num_seconds();
                        println!(
                            "{}",
                            tracking::statusbar_output(
                                &proj.name,
                                &tsk.name,
                                secs,
                                true,
                                Some(theme.icon())
                            )
                        );
                    }
                    _ => {
                        let (r, g, b) = theme.primary_color();
                        println!(
                            "\n{} {} Active Tracking {}\n",
                            theme.icon(),
                            "💎".truecolor(r, g, b),
                            theme.icon()
                        );
                        print_tracking(&proj.name, &tsk.name, &entry.duration_formatted());
                        println!(
                            "  Started: {}",
                            entry.start_local().format("%Y-%m-%d %H:%M:%S")
                        );
                        if let Some(ref notes) = entry.notes {
                            println!("  Notes: {}", notes);
                        }
                    }
                }
            } else {
                match cli.format {
                    OutputFormat::Json => println!("null"),
                    OutputFormat::Statusbar => println!("{} idle", theme.icon()),
                    _ => print_info("No active tracking session"),
                }
            }
        }

        Commands::List {
            only_projects_and_tasks,
            since,
            total,
            limit,
        } => {
            let since_dt = since.as_ref().and_then(|s| parse_datetime(s));

            if only_projects_and_tasks {
                let projects = engine.db().list_projects()?;
                match cli.format {
                    OutputFormat::Json => {
                        let mut output = Vec::new();
                        for p in &projects {
                            let tasks = engine.db().list_tasks(p.id)?;
                            output.push(serde_json::json!({
                                "project": p,
                                "tasks": tasks
                            }));
                        }
                        println!("{}", serde_json::to_string_pretty(&output)?);
                    }
                    _ => {
                        let (r, g, b) = theme.primary_color();
                        println!("\n{} Projects & Tasks\n", theme.icon());
                        for p in &projects {
                            println!("  {} {}", "".truecolor(r, g, b), p.name.bold());
                            let tasks = engine.db().list_tasks(p.id)?;
                            for t in &tasks {
                                println!("       {}", t.name);
                            }
                        }
                    }
                }
            } else {
                let entries = engine.db().list_entries_with_details(since_dt)?;
                let entries: Vec<_> = entries.into_iter().take(limit).collect();

                match cli.format {
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&entries)?);
                    }
                    _ => {
                        let output = stats::format_project_table(&entries, theme);
                        println!("{}", output);

                        if total {
                            let total_secs: i64 = entries
                                .iter()
                                .map(|e| e.entry.duration().num_seconds())
                                .sum();
                            let formatted = tracking::format_duration_long(total_secs);
                            println!("Total: {}", formatted.bold());
                        }
                    }
                }
            }
        }

        Commands::Project { command } => match command {
            ProjectCommands::Add { name, color } => {
                let mut project = Project::new(&name);
                if let Some(c) = color {
                    project = project.with_color(c);
                }
                engine.db().create_project(&mut project)?;
                print_success(&format!("Created project: {}", name.bold()));
            }

            ProjectCommands::List => {
                let projects = engine.db().list_projects()?;
                match cli.format {
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&projects)?);
                    }
                    _ => {
                        let (r, g, b) = theme.primary_color();
                        println!("\n{} Projects\n", theme.icon());
                        for p in &projects {
                            let color_dot = p.color.as_ref().map_or("○", |_| "●");
                            println!("  {} {}", color_dot.truecolor(r, g, b), p.name);
                        }
                        println!();
                    }
                }
            }

            ProjectCommands::Update {
                name,
                new_name,
                color,
            } => {
                if let Some(mut project) = engine.db().get_project_by_name(&name)? {
                    if let Some(ref nn) = new_name {
                        project.name = nn.clone();
                    }
                    if let Some(ref c) = color {
                        project.color = Some(c.clone());
                    }
                    engine.db().update_project(&project)?;
                    print_success(&format!("Updated project: {}", name));
                } else {
                    return Err(error::Error::NotFound(format!("Project: {}", name)));
                }
            }

            ProjectCommands::Remove { name, force: _ } => {
                if let Some(project) = engine.db().get_project_by_name(&name)? {
                    engine.db().delete_project(project.id)?;
                    print_success(&format!("Removed project: {}", name));
                } else {
                    return Err(error::Error::NotFound(format!("Project: {}", name)));
                }
            }
        },

        Commands::Task { command } => match command {
            TaskCommands::Add {
                name,
                project,
                git_repo,
            } => {
                let proj = engine.db().get_or_create_project(&project)?;
                let mut task = models::Task::new(proj.id, &name);
                if let Some(repo) = git_repo {
                    task = task.with_git_repo(repo);
                }
                engine.db().create_task(&mut task)?;
                print_success(&format!("Created task: {} (in {})", name.bold(), project));
            }

            TaskCommands::List { project } => {
                let tasks = if let Some(ref proj_name) = project {
                    if let Some(p) = engine.db().get_project_by_name(proj_name)? {
                        engine.db().list_tasks(p.id)?
                    } else {
                        Vec::new()
                    }
                } else {
                    engine.db().list_all_tasks()?
                };

                match cli.format {
                    OutputFormat::Json => {
                        println!("{}", serde_json::to_string_pretty(&tasks)?);
                    }
                    _ => {
                        let (r, g, b) = theme.primary_color();
                        println!("\n{} Tasks\n", theme.icon());
                        for t in &tasks {
                            let git_indicator = if t.git_repo.is_some() { " " } else { "" };
                            println!("  {} {}{}", "".truecolor(r, g, b), t.name, git_indicator);
                        }
                        println!();
                    }
                }
            }

            TaskCommands::Update {
                name,
                project,
                new_name,
                git_repo,
            } => {
                if let Some(proj) = engine.db().get_project_by_name(&project)? {
                    if let Some(mut task) = engine.db().get_task_by_name(proj.id, &name)? {
                        if let Some(ref nn) = new_name {
                            task.name = nn.clone();
                        }
                        if let Some(ref repo) = git_repo {
                            task.git_repo = Some(repo.clone());
                        }
                        engine.db().update_task(&task)?;
                        print_success(&format!("Updated task: {}", name));
                    } else {
                        return Err(error::Error::NotFound(format!("Task: {}", name)));
                    }
                } else {
                    return Err(error::Error::NotFound(format!("Project: {}", project)));
                }
            }

            TaskCommands::Remove {
                name,
                project,
                force: _,
            } => {
                if let Some(proj) = engine.db().get_project_by_name(&project)? {
                    if let Some(task) = engine.db().get_task_by_name(proj.id, &name)? {
                        engine.db().delete_task(task.id)?;
                        print_success(&format!("Removed task: {}", name));
                    } else {
                        return Err(error::Error::NotFound(format!("Task: {}", name)));
                    }
                } else {
                    return Err(error::Error::NotFound(format!("Project: {}", project)));
                }
            }
        },

        Commands::Stats {
            today,
            week,
            month,
            since,
            by_project: _,
            by_task: _,
        } => {
            let db2 = Database::open(&db_path)?;
            let stats_engine = StatsEngine::new(db2, theme);

            let (stats, title) = if today {
                (stats_engine.today_stats()?, "Today's Stats")
            } else if week {
                (stats_engine.week_stats()?, "This Week's Stats")
            } else if month {
                (stats_engine.month_stats()?, "This Month's Stats")
            } else if let Some(ref s) = since {
                let dt = parse_datetime(s);
                (stats_engine.calculate_stats(dt)?, "Stats")
            } else {
                (stats_engine.week_stats()?, "This Week's Stats")
            };

            match cli.format {
                OutputFormat::Json => {
                    println!("{}", stats_engine.format_stats_json(&stats)?);
                }
                _ => {
                    println!("{}", stats_engine.format_stats(&stats, title));
                }
            }
        }

        Commands::Statusbar { short, icon } => {
            if let Some((entry, proj, tsk)) = engine.get_status()? {
                let secs = entry.duration().num_seconds();
                println!(
                    "{}",
                    tracking::statusbar_output(&proj.name, &tsk.name, secs, short, icon.as_deref())
                );
            } else {
                let icon = icon.as_deref().unwrap_or("💎");
                println!("{} idle", icon);
            }
        }

        Commands::Dashboard => {
            let db2 = Database::open(&db_path)?;
            ui::run_dashboard(db2, theme)?;
        }

        Commands::Config { command } => match command {
            ConfigCommands::Show => match cli.format {
                OutputFormat::Json => {
                    println!("{}", serde_json::to_string_pretty(&config)?);
                }
                _ => {
                    println!("\n{} Configuration\n", theme.icon());
                    println!("Database: {}", config.database.path);
                    println!("Theme: {}", config.ui.theme);
                    println!("Auto-import Git: {}", config.tracking.auto_import_git);
                    println!("Encryption: {}", config.security.enable_encryption);
                }
            },

            ConfigCommands::Path => {
                println!("{}", Config::config_path()?.display());
            }

            ConfigCommands::Edit => {
                let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
                let config_path = Config::config_path()?;
                std::process::Command::new(&editor)
                    .arg(&config_path)
                    .status()?;
            }

            ConfigCommands::Reset { force: _ } => {
                let default_config = Config::default();
                default_config.save()?;
                print_success("Configuration reset to defaults");
            }

            ConfigCommands::Set { key, value } => {
                print_info(&format!(
                    "Setting {} = {} (not yet implemented)",
                    key, value
                ));
            }
        },

        Commands::Import { zeit, json } => {
            if let Some(path) = zeit {
                let importer = integrations::zeit::ZeitImporter::new()
                    .with_db_path(std::path::PathBuf::from(&path));

                match importer.preview() {
                    Ok(preview) => {
                        print_info(&format!("Preview:\n{}", preview.display()));
                        let result = importer.import_to_database(engine.db())?;
                        print_success(&result.summary());
                    }
                    Err(e) => {
                        print_error(&format!("Failed to preview Zeit database: {}", e));
                    }
                }
            } else if let Some(path) = json {
                let result = integrations::zeit::import_from_json(&path, engine.db())?;
                print_success(&result.summary());
            } else {
                if let Some(default_path) = integrations::zeit::ZeitImporter::default_zeit_path() {
                    if default_path.exists() {
                        let importer = integrations::zeit::ZeitImporter::new();
                        let preview = importer.preview()?;
                        print_info(&format!("Found Zeit database:\n{}", preview.display()));
                        print_info("Use --zeit flag to import");
                    } else {
                        print_info("No Zeit database found. Use --zeit <path> or --json <path>");
                    }
                } else {
                    print_info("Specify --zeit <path> or --json <path> to import");
                }
            }
        }

        Commands::Export {
            export_format,
            output,
            since,
        } => {
            let since_dt = since.as_ref().and_then(|s| parse_datetime(s));
            let entries = engine.db().list_entries_with_details(since_dt)?;

            let content = match export_format.as_str() {
                "csv" => {
                    let mut csv =
                        String::from("date,project,task,start,end,duration_seconds,notes\n");
                    for e in &entries {
                        csv.push_str(&format!(
                            "{},{},{},{},{},{},{}\n",
                            e.entry.start_local().format("%Y-%m-%d"),
                            e.project_name,
                            e.task_name,
                            e.entry.start.to_rfc3339(),
                            e.entry.end.map_or(String::new(), |t| t.to_rfc3339()),
                            e.entry.duration().num_seconds(),
                            e.entry.notes.as_deref().unwrap_or("")
                        ));
                    }
                    csv
                }
                _ => serde_json::to_string_pretty(&entries)?,
            };

            if let Some(path) = output {
                std::fs::write(&path, &content)?;
                print_success(&format!("Exported to: {}", path));
            } else {
                println!("{}", content);
            }
        }
    }

    Ok(())
}

fn parse_datetime(s: &str) -> Option<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }

    if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        let datetime = date.and_hms_opt(0, 0, 0)?;
        return Local
            .from_local_datetime(&datetime)
            .latest()
            .map(|dt| dt.with_timezone(&Utc));
    }

    match s.to_lowercase().as_str() {
        "today" => {
            let today = Local::now().date_naive().and_hms_opt(0, 0, 0)?;
            Local
                .from_local_datetime(&today)
                .latest()
                .map(|dt| dt.with_timezone(&Utc))
        }
        "yesterday" => {
            let yesterday = (Local::now() - chrono::Duration::days(1))
                .date_naive()
                .and_hms_opt(0, 0, 0)?;
            Local
                .from_local_datetime(&yesterday)
                .latest()
                .map(|dt| dt.with_timezone(&Utc))
        }
        "week" => {
            let week_ago = (Local::now() - chrono::Duration::weeks(1))
                .date_naive()
                .and_hms_opt(0, 0, 0)?;
            Local
                .from_local_datetime(&week_ago)
                .latest()
                .map(|dt| dt.with_timezone(&Utc))
        }
        "month" => {
            let month_ago = (Local::now() - chrono::Duration::days(30))
                .date_naive()
                .and_hms_opt(0, 0, 0)?;
            Local
                .from_local_datetime(&month_ago)
                .latest()
                .map(|dt| dt.with_timezone(&Utc))
        }
        _ => None,
    }
}
