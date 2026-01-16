//! Calendar data model

use chrono::{Datelike, Duration, Local, Month, NaiveDate, Weekday};
use num_traits::FromPrimitive;

use super::events::CalendarEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalendarView {
    Month,
    Week,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    AddingEvent,
    DeletingEvent,
}

#[derive(Debug, Clone)]
pub struct Calendar {
    pub current_date: NaiveDate,
    pub selected_date: NaiveDate,
    pub view: CalendarView,
    pub events: Vec<CalendarEvent>,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub delete_selection_index: usize,
}

impl Calendar {
    pub fn new() -> Self {
        let today = Local::now().date_naive();
        Self {
            current_date: today,
            selected_date: today,
            view: CalendarView::Month,
            events: Vec::new(),
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            delete_selection_index: 0,
        }
    }

    pub fn with_events(mut self, events: Vec<CalendarEvent>) -> Self {
        self.events = events;
        self
    }

    pub fn go_to_today(&mut self) {
        let today = Local::now().naive_local().date();
        self.current_date = today;
        self.selected_date = today;
    }

    pub fn next_month(&mut self) {
        self.current_date = self
            .current_date
            .checked_add_months(chrono::Months::new(1))
            .unwrap_or(self.current_date);
    }

    pub fn prev_month(&mut self) {
        self.current_date = self
            .current_date
            .checked_sub_months(chrono::Months::new(1))
            .unwrap_or(self.current_date);
    }

    pub fn next_week(&mut self) {
        self.selected_date += Duration::weeks(1);
        if self.selected_date.month() != self.current_date.month() {
            self.current_date = self.selected_date;
        }
    }

    pub fn prev_week(&mut self) {
        self.selected_date -= Duration::weeks(1);
        if self.selected_date.month() != self.current_date.month() {
            self.current_date = self.selected_date;
        }
    }

    pub fn next_day(&mut self) {
        self.selected_date += Duration::days(1);
        if self.selected_date.month() != self.current_date.month() {
            self.current_date = self.selected_date;
        }
    }

    pub fn prev_day(&mut self) {
        self.selected_date -= Duration::days(1);
        if self.selected_date.month() != self.current_date.month() {
            self.current_date = self.selected_date;
        }
    }

    pub fn month_name(&self) -> String {
        Month::from_u32(self.current_date.month())
            .map(|m| format!("{:?}", m))
            .unwrap_or_else(|| "Unknown".to_string())
    }

    pub fn year(&self) -> i32 {
        self.current_date.year()
    }

    pub fn get_month_days(&self) -> Vec<Option<NaiveDate>> {
        let first_day =
            NaiveDate::from_ymd_opt(self.current_date.year(), self.current_date.month(), 1)
                .unwrap();

        let weekday_offset = match first_day.weekday() {
            Weekday::Mon => 0,
            Weekday::Tue => 1,
            Weekday::Wed => 2,
            Weekday::Thu => 3,
            Weekday::Fri => 4,
            Weekday::Sat => 5,
            Weekday::Sun => 6,
        };

        let mut days = vec![None; weekday_offset as usize];

        let mut current = first_day;
        while current.month() == self.current_date.month() {
            days.push(Some(current));
            current += Duration::days(1);
        }

        // Pad to complete weeks
        while days.len() % 7 != 0 {
            days.push(None);
        }

        days
    }

    pub fn get_events_for_date(&self, date: NaiveDate) -> Vec<&CalendarEvent> {
        self.events
            .iter()
            .filter(|event| event.date == date)
            .collect()
    }

    pub fn get_upcoming_events(&self, limit: usize) -> Vec<&CalendarEvent> {
        let mut upcoming: Vec<&CalendarEvent> = self
            .events
            .iter()
            .filter(|event| event.date >= self.selected_date)
            .collect();

        upcoming.sort_by_key(|e| e.date);
        upcoming.into_iter().take(limit).collect()
    }

    pub fn total_events_this_month(&self) -> usize {
        self.events
            .iter()
            .filter(|event| {
                event.date.month() == self.current_date.month()
                    && event.date.year() == self.current_date.year()
            })
            .count()
    }

    pub fn total_events(&self) -> usize {
        self.events.len()
    }

    pub fn current_streak(&self) -> i64 {
        let mut streak = 0;
        let mut current = Local::now().naive_local().date();

        loop {
            if self.get_events_for_date(current).is_empty() {
                break;
            }
            streak += 1;
            current -= Duration::days(1);
        }

        streak
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_creation() {
        let cal = Calendar::new();
        let today = Local::now().naive_local().date();
        assert_eq!(cal.current_date, today);
        assert_eq!(cal.selected_date, today);
    }

    #[test]
    fn test_navigation() {
        let mut cal = Calendar::new();
        let initial_month = cal.current_date.month();

        cal.next_month();
        assert_ne!(cal.current_date.month(), initial_month);

        cal.prev_month();
        assert_eq!(cal.current_date.month(), initial_month);
    }

    #[test]
    fn test_get_month_days() {
        let cal = Calendar::new();
        let days = cal.get_month_days();

        // Should have complete weeks
        assert_eq!(days.len() % 7, 0);

        // Should have at least 28 days
        let valid_days: Vec<_> = days.iter().filter(|d| d.is_some()).collect();
        assert!(valid_days.len() >= 28);
    }
}
