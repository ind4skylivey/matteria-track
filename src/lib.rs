//! MatteriaTrack - Final Fantasy-themed power user time tracking CLI
//!
//! A mystical time tracking system inspired by the Materia system from Final Fantasy.

#![allow(dead_code)]

pub mod achievements;
pub mod cli;
pub mod config;
pub mod database;
pub mod error;
pub mod fuzzy;
pub mod integrations;
pub mod models;
pub mod notifications;
pub mod security;
pub mod stats;
pub mod theme;
pub mod themes;
pub mod tracking;
pub mod ui;

use std::process::ExitCode;

pub use config::Config;
pub use database::Database;
pub use error::{Error, Result};
pub use models::{Entry, Project, Task};
pub use theme::{icons, MateriaTheme};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &str = "mtrack";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitStatus {
    Success = 0,
    Error = 1,
    ConfigError = 2,
    DatabaseError = 3,
}

impl From<ExitStatus> for ExitCode {
    fn from(status: ExitStatus) -> Self {
        ExitCode::from(status as u8)
    }
}

pub fn banner() -> &'static str {
    r#"
╔══════════════════════════════════════════════════════════════╗
║  💎 MateriaTrack - Time Tracking Forged in Mako Energy 💎    ║
║     "Master your time, master your destiny"                   ║
╚══════════════════════════════════════════════════════════════╝
"#
}

pub fn short_banner(theme: MateriaTheme) -> String {
    format!(
        "{} MateriaTrack {} v{}",
        theme.icon(),
        icons::MATERIA,
        VERSION
    )
}
