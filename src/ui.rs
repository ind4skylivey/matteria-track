//! Terminal UI dashboard for MatteriaTrack

use crate::database::Database;
use crate::error::Result;
use crate::models::{Entry, EntryWithDetails, Project, Task, TimeStats};
use crate::stats::StatsEngine;
use crate::theme::MateriaTheme;
use chrono::Local;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Gauge, List, ListItem, Paragraph, Row, Table, Tabs},
    Frame, Terminal,
};
use std::io;
use std::time::Duration as StdDuration;

pub struct App {
    db: Database,
    theme: MateriaTheme,
    selected_tab: usize,
    entries: Vec<EntryWithDetails>,
    projects: Vec<Project>,
    #[allow(dead_code)]
    stats: Option<TimeStats>,
    active_entry: Option<(Entry, Project, Task)>,
    should_quit: bool,
    scroll_offset: usize,
}

impl App {
    pub fn new(db: Database, theme: MateriaTheme) -> Result<Self> {
        let entries = db.list_entries_with_details(None)?;
        let projects = db.list_projects()?;

        // Stats engine placeholder - we don't need a separate DB for stats
        let _stats_engine: Option<StatsEngine> = None;

        let active = db.get_active_tracking().ok().flatten().and_then(|entry| {
            let project = db.get_project(entry.project_id).ok()??;
            let task = db.get_task(entry.task_id).ok()??;
            Some((entry, project, task))
        });

        Ok(Self {
            db,
            theme,
            selected_tab: 0,
            entries,
            projects,
            stats: None,
            active_entry: active,
            should_quit: false,
            scroll_offset: 0,
        })
    }

    pub fn refresh_data(&mut self) -> Result<()> {
        self.entries = self.db.list_entries_with_details(None)?;
        self.projects = self.db.list_projects()?;
        self.active_entry = self.db.get_active_tracking()?.and_then(|entry| {
            let project = self.db.get_project(entry.project_id).ok()??;
            let task = self.db.get_task(entry.task_id).ok()??;
            Some((entry, project, task))
        });
        Ok(())
    }

    fn theme_colors(&self) -> (Color, Color) {
        let (r, g, b) = self.theme.primary_color();
        let (sr, sg, sb) = self.theme.secondary_color();
        (Color::Rgb(r, g, b), Color::Rgb(sr, sg, sb))
    }
}

pub fn run_dashboard(db: Database, theme: MateriaTheme) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(db, theme)?;

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(StdDuration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                        KeyCode::Tab | KeyCode::Right => {
                            app.selected_tab = (app.selected_tab + 1) % 4;
                        }
                        KeyCode::BackTab | KeyCode::Left => {
                            app.selected_tab = if app.selected_tab == 0 {
                                3
                            } else {
                                app.selected_tab - 1
                            };
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.scroll_offset = app.scroll_offset.saturating_add(1);
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.scroll_offset = app.scroll_offset.saturating_sub(1);
                        }
                        KeyCode::Char('r') => {
                            let _ = app.refresh_data();
                        }
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }

        let _ = app.refresh_data();
    }
}

fn ui(f: &mut Frame, app: &App) {
    let (primary, secondary) = app.theme_colors();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    draw_header(f, app, chunks[0], primary);
    draw_status(f, app, chunks[1], primary, secondary);

    match app.selected_tab {
        0 => draw_entries(f, app, chunks[2], primary),
        1 => draw_projects(f, app, chunks[2], primary),
        2 => draw_stats(f, app, chunks[2], primary),
        3 => draw_help(f, app, chunks[2], primary),
        _ => {}
    }

    draw_footer(f, app, chunks[3], primary);
}

fn draw_header(f: &mut Frame, app: &App, area: Rect, primary: Color) {
    let titles = vec!["📋 Entries", "🏆 Projects", "📊 Stats", "❓ Help"];
    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" 💎 MatteriaTrack {} ", app.theme.icon())),
        )
        .select(app.selected_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(primary).add_modifier(Modifier::BOLD));

    f.render_widget(tabs, area);
}

fn draw_status(f: &mut Frame, app: &App, area: Rect, primary: Color, secondary: Color) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" ⚔️ Active Tracking ");

    let inner = block.inner(area);
    f.render_widget(block, area);

    if let Some((ref entry, ref project, ref task)) = app.active_entry {
        let duration = entry.duration();
        let hours = duration.num_hours();
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;

        let progress = (duration.num_seconds() as f64 / 28800.0 * 100.0).min(100.0);

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(inner);

        let text = vec![
            Line::from(vec![
                Span::styled("Project: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    &project.name,
                    Style::default().fg(primary).add_modifier(Modifier::BOLD),
                ),
                Span::raw("  →  "),
                Span::styled("Task: ", Style::default().fg(Color::Gray)),
                Span::styled(&task.name, Style::default().fg(secondary)),
            ]),
            Line::from(vec![
                Span::styled("Duration: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{:02}:{:02}:{:02}", hours, minutes, seconds),
                    Style::default().fg(primary).add_modifier(Modifier::BOLD),
                ),
            ]),
        ];

        let paragraph = Paragraph::new(text);
        f.render_widget(paragraph, chunks[0]);

        let gauge = Gauge::default()
            .block(Block::default().title(" Daily Progress "))
            .gauge_style(Style::default().fg(primary))
            .percent(progress as u16)
            .label(format!("{:.1}%", progress));

        f.render_widget(gauge, chunks[1]);
    } else {
        let text = Paragraph::new(vec![Line::from(vec![
            Span::styled("✨ ", Style::default()),
            Span::styled(
                "No active tracking - Use 'mtrack track' to start",
                Style::default().fg(Color::DarkGray),
            ),
        ])]);
        f.render_widget(text, inner);
    }
}

fn draw_entries(f: &mut Frame, app: &App, area: Rect, primary: Color) {
    let header_cells = ["Date", "Project", "Task", "Duration", "Status"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(primary).add_modifier(Modifier::BOLD)));

    let header = Row::new(header_cells).height(1).bottom_margin(1);

    let rows: Vec<Row> = app
        .entries
        .iter()
        .skip(app.scroll_offset)
        .take(area.height.saturating_sub(4) as usize)
        .map(|entry| {
            let date = entry.entry.start_local().format("%m-%d %H:%M").to_string();
            let duration = entry.entry.duration_formatted();
            let status = if entry.entry.is_active() {
                "⚡ Active"
            } else {
                "✓ Done"
            };

            let status_style = if entry.entry.is_active() {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Green)
            };

            Row::new(vec![
                Cell::from(date),
                Cell::from(entry.project_name.clone()).style(Style::default().fg(primary)),
                Cell::from(entry.task_name.clone()),
                Cell::from(duration),
                Cell::from(status).style(status_style),
            ])
        })
        .collect();

    let widths = [
        Constraint::Length(12),
        Constraint::Length(15),
        Constraint::Length(20),
        Constraint::Length(12),
        Constraint::Length(10),
    ];

    let table = Table::new(rows, widths).header(header).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" 📋 Recent Entries ({}) ", app.entries.len())),
    );

    f.render_widget(table, area);
}

fn draw_projects(f: &mut Frame, app: &App, area: Rect, primary: Color) {
    let items: Vec<ListItem> = app
        .projects
        .iter()
        .map(|p| {
            let color_indicator = p.color.as_ref().map_or("", |_| "●");
            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{} ", color_indicator),
                    Style::default().fg(primary),
                ),
                Span::styled(
                    &p.name,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" 🏆 Projects ({}) ", app.projects.len())),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

    f.render_widget(list, area);
}

fn draw_stats(f: &mut Frame, app: &App, area: Rect, primary: Color) {
    let text = vec![
        Line::from(vec![Span::styled(
            "📊 Statistics Panel",
            Style::default().fg(primary).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::raw("Total Entries: "),
            Span::styled(app.entries.len().to_string(), Style::default().fg(primary)),
        ]),
        Line::from(vec![
            Span::raw("Total Projects: "),
            Span::styled(app.projects.len().to_string(), Style::default().fg(primary)),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Use 'mtrack stats' for detailed statistics",
            Style::default().fg(Color::DarkGray),
        )]),
    ];

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" 📊 Statistics "),
    );

    f.render_widget(paragraph, area);
}

fn draw_help(f: &mut Frame, _app: &App, area: Rect, primary: Color) {
    let help_text = vec![
        Line::from(vec![Span::styled(
            "⌨️  Keyboard Shortcuts",
            Style::default().fg(primary).add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Tab/→  ", Style::default().fg(primary)),
            Span::raw("Next tab"),
        ]),
        Line::from(vec![
            Span::styled("BackTab/←  ", Style::default().fg(primary)),
            Span::raw("Previous tab"),
        ]),
        Line::from(vec![
            Span::styled("j/↓  ", Style::default().fg(primary)),
            Span::raw("Scroll down"),
        ]),
        Line::from(vec![
            Span::styled("k/↑  ", Style::default().fg(primary)),
            Span::raw("Scroll up"),
        ]),
        Line::from(vec![
            Span::styled("r  ", Style::default().fg(primary)),
            Span::raw("Refresh data"),
        ]),
        Line::from(vec![
            Span::styled("q/Esc  ", Style::default().fg(primary)),
            Span::raw("Quit"),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "💎 \"Master your time, master your destiny\"",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )]),
    ];

    let paragraph =
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title(" ❓ Help "));

    f.render_widget(paragraph, area);
}

fn draw_footer(f: &mut Frame, app: &App, area: Rect, primary: Color) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" 💎 ", Style::default()),
        Span::styled(
            format!("MatteriaTrack {} ", app.theme.icon()),
            Style::default().fg(primary),
        ),
        Span::raw("│ "),
        Span::raw(now),
        Span::raw(" │ Press "),
        Span::styled("q", Style::default().fg(primary)),
        Span::raw(" to quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, area);
}
