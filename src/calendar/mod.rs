//! Calendar module for MatteriaTrack
//!
//! Provides TUI calendar interface with Materia theme support

pub mod events;
pub mod model;
pub mod tui;

pub use events::{CalendarEvent, CalendarEventType, EventStore};
pub use model::{Calendar, CalendarView};
pub use tui::CalendarTui;
