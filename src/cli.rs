//! CLI interface for MatteriaTrack

use crate::theme::icons;
use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;

const FF_BANNER: &str = r#"
╔══════════════════════════════════════════════════════════════╗
║  💎 MatteriaTrack - Time Tracking Forged in Mako Energy 💎    ║
║     "Master your time, master your destiny"                   ║
╚══════════════════════════════════════════════════════════════╝
"#;

const AFTER_HELP: &str = r#"
⚔️  MATERIA SYSTEM ⚔️
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

💎 TRACKING MATERIA
  mtrack track -p "project" -t "task"    Start tracking time
  mtrack finish                          Complete current session
  mtrack status                          Show active tracking

✨ LISTING MATERIA
  mtrack list                            Show recent entries
  mtrack list --since "2024-01-01"       Filter by date
  mtrack list --total                    Show time totals

🏆 PROJECT MATERIA
  mtrack project add "name"              Create new project
  mtrack project list                    Show all projects
  mtrack task add "name" -p "project"    Create new task

⭐ STATS MATERIA
  mtrack stats                           Time statistics
  mtrack stats --week                    This week's stats
  mtrack statusbar                       DWM statusbar output

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
"#;

#[derive(Parser, Debug)]
#[command(
    name = "materiatrack",
    author = "ind4skylivey",
    version,
    about = "💎 MatteriaTrack - Final Fantasy-themed time tracking CLI",
    long_about = FF_BANNER,
    after_help = AFTER_HELP,
    propagate_version = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// Output format
    #[arg(short = 'f', long, global = true, default_value = "pretty")]
    pub format: OutputFormat,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Pretty,
    Json,
    Plain,
    Statusbar,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// ⚔️ Start tracking time on a task
    #[command(visible_alias = "t")]
    Track {
        /// Project name
        #[arg(short, long)]
        project: String,

        /// Task name
        #[arg(short, long)]
        task: String,

        /// Start time offset (e.g., -0:15 for 15 minutes ago)
        #[arg(long)]
        begin: Option<String>,

        /// Optional notes for this tracking session
        #[arg(short, long)]
        notes: Option<String>,
    },

    /// ✓ Finish the current tracking session
    #[command(visible_alias = "f")]
    Finish {
        /// Switch to a different task when finishing
        #[arg(short, long)]
        task: Option<String>,

        /// Adjust the start time
        #[arg(long)]
        begin: Option<String>,

        /// Set the end time offset
        #[arg(long)]
        end: Option<String>,

        /// Add notes to the entry
        #[arg(short, long)]
        notes: Option<String>,
    },

    /// 💎 Show current tracking status
    #[command(visible_alias = "s")]
    Status,

    /// ✨ List tracked entries
    #[command(visible_alias = "l")]
    List {
        /// Only show projects and tasks (no entries)
        #[arg(long)]
        only_projects_and_tasks: bool,

        /// Show entries since this datetime (ISO8601 or relative)
        #[arg(long)]
        since: Option<String>,

        /// Show total time in output
        #[arg(long)]
        total: bool,

        /// Maximum number of entries to show
        #[arg(short = 'n', long, default_value = "20")]
        limit: usize,
    },

    /// 🏆 Manage projects
    Project {
        #[command(subcommand)]
        command: ProjectCommands,
    },

    /// ⭐ Manage tasks
    Task {
        #[command(subcommand)]
        command: TaskCommands,
    },

    /// 📊 Show time statistics
    Stats {
        /// Show stats for today only
        #[arg(long)]
        today: bool,

        /// Show stats for this week
        #[arg(long)]
        week: bool,

        /// Show stats for this month
        #[arg(long)]
        month: bool,

        /// Show stats since this date
        #[arg(long)]
        since: Option<String>,

        /// Group stats by project
        #[arg(long)]
        by_project: bool,

        /// Group stats by task
        #[arg(long)]
        by_task: bool,
    },

    /// 🖥️ Output for DWM/i3 statusbar
    Statusbar {
        /// Use short format
        #[arg(short, long)]
        short: bool,

        /// Icon prefix
        #[arg(long)]
        icon: Option<String>,
    },

    /// 🎨 Launch interactive TUI dashboard
    #[command(visible_alias = "ui")]
    Dashboard,

    /// ⚙️ Manage configuration
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// Generate shell completions (bash, zsh, fish)
    Completions {
        /// Target shell
        #[arg(value_enum)]
        shell: Shell,

        /// Output directory (defaults to ./completions)
        #[arg(short, long, default_value = "completions")]
        out_dir: String,
    },

    /// 📤 Import data from Zeit or other trackers
    Import {
        /// Import from Zeit database
        #[arg(long)]
        zeit: Option<String>,

        /// Import from JSON file
        #[arg(long)]
        json: Option<String>,
    },

    /// 📥 Export data to various formats
    Export {
        /// Export format (json, csv)
        #[arg(short = 'F', long = "export-format", default_value = "json")]
        export_format: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,

        /// Export entries since this date
        #[arg(long)]
        since: Option<String>,
    },

    /// 🎨 Manage UI themes
    Theme {
        #[command(subcommand)]
        command: ThemeCommands,
    },

    /// 📅 Open interactive calendar TUI
    #[command(visible_alias = "cal")]
    Calendar {
        /// Theme for calendar view
        #[arg(short, long)]
        theme: Option<String>,

        /// Add a new event
        #[arg(long)]
        add: Option<String>,

        /// Date for the event (YYYY-MM-DD)
        #[arg(long)]
        date: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ThemeCommands {
    /// List available themes
    List,

    /// Preview a specific theme
    Preview {
        /// Theme name to preview (optional, defaults to current)
        #[arg(short, long)]
        theme: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum ProjectCommands {
    /// Add a new project
    Add {
        /// Project name
        name: String,

        /// Project color (hex code)
        #[arg(short = 'C', long)]
        color: Option<String>,
    },

    /// List all projects
    List,

    /// Update a project
    Update {
        /// Project name
        name: String,

        /// New project name
        #[arg(long)]
        new_name: Option<String>,

        /// New color
        #[arg(short = 'C', long)]
        color: Option<String>,
    },

    /// Remove a project
    Remove {
        /// Project name
        name: String,

        /// Force removal without confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum TaskCommands {
    /// Add a new task
    Add {
        /// Task name
        name: String,

        /// Project to add task to
        #[arg(short, long)]
        project: String,

        /// Git repository path for this task
        #[arg(short, long)]
        git_repo: Option<String>,
    },

    /// List all tasks
    List {
        /// Filter by project
        #[arg(short, long)]
        project: Option<String>,
    },

    /// Update a task
    Update {
        /// Task name
        name: String,

        /// Project the task belongs to
        #[arg(short, long)]
        project: String,

        /// New task name
        #[arg(long)]
        new_name: Option<String>,

        /// New git repository path
        #[arg(short, long)]
        git_repo: Option<String>,
    },

    /// Remove a task
    Remove {
        /// Task name
        name: String,

        /// Project the task belongs to
        #[arg(short, long)]
        project: String,

        /// Force removal without confirmation
        #[arg(long)]
        force: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    /// Show current configuration
    Show,

    /// Edit configuration file
    Edit,

    /// Reset configuration to defaults
    Reset {
        /// Force reset without confirmation
        #[arg(long)]
        force: bool,
    },

    /// Set a configuration value
    Set {
        /// Configuration key (e.g., ui.theme)
        key: String,

        /// Value to set
        value: String,
    },

    /// Show configuration file path
    Path,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}

pub fn print_materia_header() {
    println!("{}", FF_BANNER);
}

pub fn print_success(msg: &str) {
    println!("{} {} {}", icons::CHECK, icons::MATERIA, msg);
}

pub fn print_error(msg: &str) {
    eprintln!("{} {} {}", icons::CROSS, icons::MATERIA, msg);
}

pub fn print_info(msg: &str) {
    println!("{} {}", icons::SPARKLE, msg);
}

pub fn print_tracking(project: &str, task: &str, duration: &str) {
    println!(
        "{} {} {} {} {} [{}]",
        icons::CLOCK,
        icons::MATERIA,
        project,
        icons::ARROW_RIGHT,
        task,
        duration
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_cli_parsing() {
        Cli::command().debug_assert();
    }

    #[test]
    fn test_track_command() {
        let cli = Cli::try_parse_from(["mtrack", "track", "-p", "project", "-t", "task"]).unwrap();
        match cli.command {
            Commands::Track { project, task, .. } => {
                assert_eq!(project, "project");
                assert_eq!(task, "task");
            }
            _ => panic!("Expected Track command"),
        }
    }

    #[test]
    fn test_project_add() {
        let cli = Cli::try_parse_from(["mtrack", "project", "add", "MyProject"]).unwrap();
        match cli.command {
            Commands::Project {
                command: ProjectCommands::Add { name, .. },
            } => {
                assert_eq!(name, "MyProject");
            }
            _ => panic!("Expected Project Add command"),
        }
    }
}
