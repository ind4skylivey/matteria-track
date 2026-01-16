//! Calendar TUI rendering with ratatui

use crate::calendar::events::{CalendarEvent, CalendarEventType, EventStore};
use crate::calendar::model::{Calendar, InputMode};
use crate::error::Result;
use crate::theme::MateriaTheme;
use chrono::{Datelike, Local};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::io;

pub struct CalendarTui {
    calendar: Calendar,
    event_store: Option<EventStore>,
    theme: MateriaTheme,
    show_help: bool,
}

impl CalendarTui {
    pub fn new(calendar: Calendar, theme: MateriaTheme, event_store: Option<EventStore>) -> Self {
        Self {
            calendar,
            event_store,
            theme,
            show_help: false,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        terminal.clear()?;

        loop {
            terminal.draw(|f| self.render(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match self.calendar.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('h') => self.show_help = !self.show_help,
                            KeyCode::Char('t') | KeyCode::Char('H') => self.calendar.go_to_today(),
                            KeyCode::Left | KeyCode::Char('a') => self.calendar.prev_day(),
                            KeyCode::Right | KeyCode::Char('d') => self.calendar.next_day(),
                            KeyCode::Up | KeyCode::Char('w') => self.calendar.prev_week(),
                            KeyCode::Down | KeyCode::Char('s') => self.calendar.next_week(),
                            KeyCode::Char('[') => self.calendar.prev_month(),
                            KeyCode::Char(']') => self.calendar.next_month(),
                            KeyCode::Char('e') | KeyCode::Enter => {
                                // 'e' or Enter to add event
                                self.calendar.input_mode = InputMode::AddingEvent;
                                self.calendar.input_buffer.clear();
                            }
                            KeyCode::Char('x') | KeyCode::Delete => {
                                // 'x' or Delete to remove event
                                let date = self.calendar.selected_date;
                                let events: Vec<&CalendarEvent> = self
                                    .calendar
                                    .events
                                    .iter()
                                    .filter(|e| {
                                        e.date == date && e.event_type == CalendarEventType::Custom
                                    })
                                    .collect();

                                if !events.is_empty() {
                                    self.calendar.input_mode = InputMode::DeletingEvent;
                                    self.calendar.delete_selection_index = 0;
                                }
                            }
                            _ => {}
                        },
                        InputMode::AddingEvent => match key.code {
                            KeyCode::Enter => {
                                if !self.calendar.input_buffer.is_empty() {
                                    let title = self.calendar.input_buffer.clone();
                                    let date = self.calendar.selected_date;

                                    let event = CalendarEvent::new(title, date)
                                        .with_type(CalendarEventType::Custom);

                                    if let Some(store) = &mut self.event_store {
                                        store.add_event(event.clone())?;
                                    }

                                    // Add to local calendar view immediately
                                    self.calendar.events.push(event);
                                }
                                self.calendar.input_mode = InputMode::Normal;
                                self.calendar.input_buffer.clear();
                            }
                            KeyCode::Esc => {
                                self.calendar.input_mode = InputMode::Normal;
                                self.calendar.input_buffer.clear();
                            }
                            KeyCode::Char(c) => {
                                self.calendar.input_buffer.push(c);
                            }
                            KeyCode::Backspace => {
                                self.calendar.input_buffer.pop();
                            }
                            _ => {}
                        },
                        InputMode::DeletingEvent => match key.code {
                            KeyCode::Esc => {
                                self.calendar.input_mode = InputMode::Normal;
                            }
                            KeyCode::Up | KeyCode::Char('w') => {
                                if self.calendar.delete_selection_index > 0 {
                                    self.calendar.delete_selection_index -= 1;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('s') => {
                                let date = self.calendar.selected_date;
                                let count = self
                                    .calendar
                                    .events
                                    .iter()
                                    .filter(|e| {
                                        e.date == date && e.event_type == CalendarEventType::Custom
                                    })
                                    .count();
                                if count > 0 && self.calendar.delete_selection_index < count - 1 {
                                    self.calendar.delete_selection_index += 1;
                                }
                            }
                            KeyCode::Enter => {
                                let date = self.calendar.selected_date;
                                let events_to_delete: Vec<CalendarEvent> = self
                                    .calendar
                                    .events
                                    .iter()
                                    .filter(|e| {
                                        e.date == date && e.event_type == CalendarEventType::Custom
                                    })
                                    .cloned()
                                    .collect();

                                if let Some(event_to_delete) =
                                    events_to_delete.get(self.calendar.delete_selection_index)
                                {
                                    // Remove from store
                                    if let Some(store) = &mut self.event_store {
                                        store.remove_event(&event_to_delete.id)?;
                                    }

                                    // Remove from local view
                                    if let Some(pos) = self
                                        .calendar
                                        .events
                                        .iter()
                                        .position(|e| e.id == event_to_delete.id)
                                    {
                                        self.calendar.events.remove(pos);
                                    }
                                }

                                self.calendar.input_mode = InputMode::Normal;
                            }
                            _ => {}
                        },
                    }
                }
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        let size = frame.area();

        // Main layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content
                Constraint::Length(2), // Footer
            ])
            .split(size);

        // Render header
        self.render_header(frame, chunks[0]);

        // Render content
        if self.show_help {
            self.render_help(frame, chunks[1]);
        } else {
            let content_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(70), // Calendar view
                    Constraint::Percentage(30), // Sidebar
                ])
                .split(chunks[1]);

            self.render_calendar(frame, content_chunks[0]);
            self.render_sidebar(frame, content_chunks[1]);
        }

        // Render footer
        self.render_footer(frame, chunks[2]);

        // Render input popup if needed
        if self.calendar.input_mode == InputMode::AddingEvent {
            self.render_input_popup(frame);
        } else if self.calendar.input_mode == InputMode::DeletingEvent {
            self.render_delete_popup(frame);
        }
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let theme_icon = self.theme.icon();
        let title = format!(
            " {} Materia Calendar - {} {} ",
            theme_icon,
            self.calendar.month_name(),
            self.calendar.year()
        );

        let (r, g, b) = self.theme.primary_color();
        let header = Paragraph::new(title)
            .style(
                Style::default()
                    .fg(Color::Rgb(r, g, b))
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(r, g, b))),
            );

        frame.render_widget(header, area);
    }

    fn render_calendar(&self, frame: &mut Frame, area: Rect) {
        let (r, g, b) = self.theme.primary_color();
        let (sr, sg, sb) = self.theme.secondary_color();

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Calendar ")
            .border_style(Style::default().fg(Color::Rgb(r, g, b)));

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Calendar grid
        let days = self.calendar.get_month_days();
        let weeks = days.chunks(7);

        let constraints: Vec<Constraint> = std::iter::repeat(Constraint::Length(3))
            .take(weeks.len() + 1) // +1 for header
            .collect();

        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner);

        // Weekday header
        let weekdays = ["L", "M", "X", "J", "V", "S", "D"];
        let header_spans: Vec<Span> = weekdays
            .iter()
            .map(|d| {
                Span::styled(
                    format!(" {:^3} ", d),
                    Style::default()
                        .fg(Color::Rgb(r, g, b))
                        .add_modifier(Modifier::BOLD),
                )
            })
            .collect();

        let header_line = Line::from(header_spans);
        let header_para = Paragraph::new(header_line);
        frame.render_widget(header_para, rows[0]);

        // Calendar days
        let today = Local::now().naive_local().date();

        for (week_idx, week) in weeks.enumerate() {
            let day_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(14); 7])
                .split(rows[week_idx + 1]);

            for (day_idx, day_opt) in week.iter().enumerate() {
                let content = if let Some(day) = day_opt {
                    let day_num = day.day();
                    let events = self.calendar.get_events_for_date(*day);
                    let has_events = !events.is_empty();

                    let mut style = Style::default();

                    // Highlight selected date
                    if *day == self.calendar.selected_date {
                        style = style
                            .bg(Color::Rgb(r, g, b))
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD);
                    }
                    // Highlight today
                    else if *day == today {
                        style = style
                            .fg(Color::Rgb(sr, sg, sb))
                            .add_modifier(Modifier::BOLD);
                    }
                    // Show events indicator
                    else if has_events {
                        style = style.fg(Color::Rgb(r, g, b));
                    }

                    let day_str = if has_events {
                        format!("{:>2}•", day_num)
                    } else {
                        format!("{:>2} ", day_num)
                    };

                    Paragraph::new(day_str)
                        .style(style)
                        .alignment(Alignment::Center)
                } else {
                    Paragraph::new("   ")
                };

                frame.render_widget(content, day_chunks[day_idx]);
            }
        }
    }

    fn render_sidebar(&self, frame: &mut Frame, area: Rect) {
        let (r, g, b) = self.theme.primary_color();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(8), // Stats
                Constraint::Min(0),    // Upcoming events
            ])
            .split(area);

        // Stats block
        let stats_content = vec![
            Line::from(vec![
                Span::styled("Mes: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!(
                    "{} eventos",
                    self.calendar.total_events_this_month()
                )),
            ]),
            Line::from(vec![
                Span::styled("Racha: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{} días", self.calendar.current_streak())),
            ]),
            Line::from(vec![
                Span::styled("Total: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", self.calendar.total_events())),
            ]),
        ];

        let stats = Paragraph::new(stats_content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Estadísticas ")
                    .border_style(Style::default().fg(Color::Rgb(r, g, b))),
            )
            .alignment(Alignment::Left);

        frame.render_widget(stats, chunks[0]);

        // Upcoming events
        let upcoming = self.calendar.get_upcoming_events(5);
        let event_items: Vec<ListItem> = upcoming
            .iter()
            .map(|event| {
                let time_str = event.time.as_deref().unwrap_or("--:--");
                let date_str = if event.date == Local::now().naive_local().date() {
                    "Hoy".to_string()
                } else {
                    event.date.format("%d/%m").to_string()
                };
                let content = format!("{} {} - {}", date_str, time_str, event.title);
                ListItem::new(content)
            })
            .collect();

        let events_list = List::new(event_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Próximos eventos ")
                .border_style(Style::default().fg(Color::Rgb(r, g, b))),
        );

        frame.render_widget(events_list, chunks[1]);
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        let (r, g, b) = self.theme.primary_color();

        let help_text = vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "Navegación:",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Rgb(r, g, b)),
            )]),
            Line::from("  ←/→ o a/d    - Día anterior/siguiente"),
            Line::from("  ↑/↓ o w/s    - Semana anterior/siguiente"),
            Line::from("  [ / ]        - Mes anterior/siguiente"),
            Line::from("  t o H        - Ir a hoy"),
            Line::from("  e / Enter    - Agregar evento"),
            Line::from("  x / Delete   - Eliminar evento"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "General:",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Rgb(r, g, b)),
            )]),
            Line::from("  h            - Mostrar/ocultar ayuda"),
            Line::from("  q            - Salir"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Leyenda:",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Rgb(r, g, b)),
            )]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    "##",
                    Style::default().bg(Color::Rgb(r, g, b)).fg(Color::Black),
                ),
                Span::raw(" - Día seleccionado"),
            ]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled("##", Style::default().fg(Color::Rgb(r, g, b))),
                Span::raw(" - Día con eventos"),
            ]),
        ];

        let help = Paragraph::new(help_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Ayuda ")
                    .border_style(Style::default().fg(Color::Rgb(r, g, b))),
            )
            .alignment(Alignment::Left);

        frame.render_widget(help, area);
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let (r, g, b) = self.theme.primary_color();

        let footer_text = " ◄─ Navegación: Flechas │ h = Ayuda │ q = Salir ";
        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::Rgb(r, g, b)))
            .alignment(Alignment::Center);

        frame.render_widget(footer, area);
    }

    fn render_input_popup(&self, frame: &mut Frame) {
        let area = frame.area();

        // Center the popup
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Length(3),
                Constraint::Percentage(40),
            ])
            .split(area);

        let popup_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(popup_layout[1])[1];

        // Clear the area under the popup
        frame.render_widget(Clear, popup_area);

        let (r, g, b) = self.theme.primary_color();

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Add Event ")
            .border_style(Style::default().fg(Color::Rgb(r, g, b)));

        let input = Paragraph::new(self.calendar.input_buffer.as_str())
            .style(Style::default().fg(Color::White))
            .block(block);

        frame.render_widget(input, popup_area);

        // Set cursor position
        frame.set_cursor_position(Position::new(
            popup_area.x + self.calendar.input_buffer.len() as u16 + 1,
            popup_area.y + 1,
        ));
    }

    fn render_delete_popup(&self, frame: &mut Frame) {
        let area = frame.area();

        // Center the popup
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(area);

        let popup_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ])
            .split(popup_layout[1])[1];

        // Clear the area under the popup
        frame.render_widget(Clear, popup_area);

        let (r, g, b) = self.theme.primary_color();

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Delete Event ")
            .border_style(Style::default().fg(Color::Rgb(r, g, b)));

        let date = self.calendar.selected_date;
        let events: Vec<&CalendarEvent> = self
            .calendar
            .events
            .iter()
            .filter(|e| e.date == date && e.event_type == CalendarEventType::Custom)
            .collect();

        let items: Vec<ListItem> = events
            .iter()
            .enumerate()
            .map(|(i, e)| {
                let content = format!(
                    "{} {}",
                    if i == self.calendar.delete_selection_index {
                        ">"
                    } else {
                        " "
                    },
                    e.title
                );
                let style = if i == self.calendar.delete_selection_index {
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(content).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD));

        frame.render_widget(list, popup_area);
    }
}
