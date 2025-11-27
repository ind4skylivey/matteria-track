//! Error handling for MateriaTrack

use std::fmt;
use std::io;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Config(ConfigError),
    Database(DatabaseError),
    Io(io::Error),
    Git(String),
    Parse(String),
    Tracking(TrackingError),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug)]
pub enum ConfigError {
    NotFound(String),
    ParseError(String),
    InvalidPath(String),
    MissingField(String),
    EncryptionError(String),
}

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionFailed(String),
    MigrationFailed(String),
    QueryFailed(String),
    NotFound(String),
    IntegrityError(String),
}

#[derive(Debug)]
pub enum TrackingError {
    AlreadyTracking(String),
    NotTracking,
    InvalidTimeRange,
    ProjectNotFound(String),
    TaskNotFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Config(e) => write!(f, "Config error: {}", e),
            Self::Database(e) => write!(f, "Database error: {}", e),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Git(msg) => write!(f, "Git error: {}", msg),
            Self::Parse(msg) => write!(f, "Parse error: {}", msg),
            Self::Tracking(e) => write!(f, "Tracking error: {}", e),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(path) => write!(f, "Config file not found: {}", path),
            Self::ParseError(msg) => write!(f, "Failed to parse config: {}", msg),
            Self::InvalidPath(path) => write!(f, "Invalid path in config: {}", path),
            Self::MissingField(field) => write!(f, "Missing required field: {}", field),
            Self::EncryptionError(msg) => write!(f, "Encryption error: {}", msg),
        }
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionFailed(msg) => write!(f, "Database connection failed: {}", msg),
            Self::MigrationFailed(msg) => write!(f, "Database migration failed: {}", msg),
            Self::QueryFailed(msg) => write!(f, "Database query failed: {}", msg),
            Self::NotFound(msg) => write!(f, "Record not found: {}", msg),
            Self::IntegrityError(msg) => write!(f, "Database integrity error: {}", msg),
        }
    }
}

impl fmt::Display for TrackingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyTracking(entry) => write!(f, "Already tracking: {}", entry),
            Self::NotTracking => write!(f, "No active tracking session"),
            Self::InvalidTimeRange => write!(f, "Invalid time range"),
            Self::ProjectNotFound(name) => write!(f, "Project not found: {}", name),
            Self::TaskNotFound(name) => write!(f, "Task not found: {}", name),
        }
    }
}

impl std::error::Error for Error {}
impl std::error::Error for ConfigError {}
impl std::error::Error for DatabaseError {}
impl std::error::Error for TrackingError {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ConfigError> for Error {
    fn from(e: ConfigError) -> Self {
        Self::Config(e)
    }
}

impl From<DatabaseError> for Error {
    fn from(e: DatabaseError) -> Self {
        Self::Database(e)
    }
}

impl From<TrackingError> for Error {
    fn from(e: TrackingError) -> Self {
        Self::Tracking(e)
    }
}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Self::Database(DatabaseError::QueryFailed(e.to_string()))
    }
}

impl From<git2::Error> for Error {
    fn from(e: git2::Error) -> Self {
        Self::Git(e.message().to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Parse(e.to_string())
    }
}

impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Self {
        Self::Parse(e.to_string())
    }
}
