# MatteriaTrack Calendar TUI

## Overview

The Calendar TUI feature adds an interactive calendar interface to MatteriaTrack, allowing you to visualize your tracking sessions and manage custom events within the terminal.

## Features

- **Month View Calendar**: Navigate through months and weeks with keyboard controls
- **Materia Themes**: Full support for all 6 Materia themes (Fire, Ice, Lightning, Earth, Wind, Bahamut)
- **Event Integration**: Automatically displays tracking sessions from your database
- **Custom Events**: Add custom calendar events with dates
- **Statistics Sidebar**: Shows month statistics, current streak, and total events
- **Upcoming Events**: Quick view of your next scheduled activities

## Usage

### Open Calendar

```bash
# Open calendar with default theme
mtrack calendar

# Open with specific theme
mtrack calendar --theme fire

# Use alias
mtrack cal --theme ice
```

### Add Custom Events

```bash
# Add event for today
mtrack calendar --add "Team Meeting"

# Add event for specific date
mtrack calendar --add "Project Deadline" --date 2026-02-15
```

## Keyboard Navigation

| Key                    | Action              |
| ---------------------- | ------------------- |
| `←` / `→` or `a` / `d` | Previous/Next day   |
| `↑` / `↓` or `w` / `s` | Previous/Next week  |
| `[` / `]`              | Previous/Next month |
| `t` or `H`             | Go to today         |
| `e` or `Enter`         | Add event (Popup)   |
| `x` or `Delete`        | Delete event (List) |
| `h`                    | Toggle help screen  |
| `q`                    | Quit calendar       |

## Event Storage

Events are stored in `~/.matteria-track/events.json` as JSON. Tracking sessions are automatically synced from your database when you open the calendar.

## UI Elements

### Calendar Grid

- **Selected Date**: Highlighted with theme color background
- **Today**: Shown in secondary theme color
- **Days with Events**: Marked with a bullet (•) indicator

### Sidebar

- **Statistics**: Monthly events, current streak, total events
- **Upcoming Events**: Next 5 upcoming events with dates and times

## Themes

All Materia themes are supported:

- 🔥 **Fire** - Vibrant red/orange
- ❄️ **Ice** - Cool cyan/blue
- ⚡ **Lightning** - Bright yellow
- 🌍 **Earth** - Natural brown
- 🌬️ **Wind** - Fresh green
- 🐉 **Bahamut** - Royal purple/gold

## Integration with Tracking

The calendar automatically displays your recent tracking sessions (last 100 entries) as events, showing:

- Project and task name
- Start time
- Event type marker

## Example Workflow

```bash
# Start your day by checking the calendar
mtrack cal

# Add a reminder for later
mtrack cal --add "Code review at 3 PM" --date $(date +%Y-%m-%d)

# Track time on a project
mtrack track -p "Dev" -t "Feature X"

# Check calendar again - your tracking session appears as an event
mtrack cal --theme lightning
```

## Files

- `~/.matteria-track/events.json` - Custom calendar events
- Database - Tracking sessions (automatically synced)
