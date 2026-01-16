# 🚀 Calendar TUI Installation Guide

## ✅ Installation Completed

MatteriaTrack v1.0.6 with Calendar TUI has been successfully installed on your system!

### 📍 Binary Location

```
~/.local/bin/mtrack (symlink to matteriatrack)
```

### 📦 Data Files

- **Database**: `~/.local/share/materiatrack/materiatrack.db`
- **Configuration**: `~/.config/materiatrack/config.toml`
- **Calendar Events**: `~/.config/materiatrack/events.json`

---

## 🎯 Quick Start Guide

### 1. Open Calendar

```bash
# With default theme (configured in config.toml)
mtrack calendar

# Or use the alias
mtrack cal

# With a specific theme
mtrack cal --theme fire
mtrack cal --theme ice
mtrack cal --theme lightning
mtrack cal --theme earth
mtrack cal --theme wind
mtrack cal --theme bahamut
```

### 2. Add Events

```bash
# Event for today
mtrack cal --add "Team Meeting"

# Event for a specific date
mtrack cal --add "Project Presentation" --date "2026-01-25"

# Event with theme
mtrack cal --add "Code Review" --date "$(date +%Y-%m-%d)" --theme lightning
```

### 3. Keyboard Controls (inside calendar)

```
←/→  or  a/d     →  Navigate days
↑/↓  or  w/s     →  Navigate weeks
[  /  ]          →  Change months
t   or   H       →  Go to today
e   or   Enter   →  Add event (Interactive)
x   or   Delete  →  Delete event (Interactive)
h                →  Toggle help
q                →  Quit
```

---

## 🎨 Available Themes

| Theme         | Icon | Primary Color | Description           |
| ------------- | ---- | ------------- | --------------------- |
| **fire**      | 🔥   | Red/Orange    | Energy and passion    |
| **ice**       | ❄️   | Blue/Cyan     | Tranquility and focus |
| **lightning** | ⚡   | Bright Yellow | Speed and dynamism    |
| **earth**     | 🌍   | Brown/Green   | Stability and growth  |
| **wind**      | 🌬️   | Light Green   | Freedom and agility   |
| **bahamut**   | 🐉   | Purple/Gold   | Epic power            |

---

## 💡 Full Usage Examples

### Daily Workflow

```bash
# 1. Check daily calendar
mtrack cal

# 2. Add a reminder
mtrack cal --add "Client Call 3PM" --date "$(date +%Y-%m-%d)"

# 3. Start tracking work
mtrack track -p "Development" -t "Feature Calendar"

# 4. Check calendar again - you will see your tracking session as an event
mtrack cal --theme bahamut
```

### View Added Events

```bash
# Check the events JSON file
cat ~/.config/materiatrack/events.json | jq
```

### Tracking Integration

The calendar automatically displays:

- ✅ Your last 100 tracking sessions
- ✅ Custom events you add
- ✅ Monthly statistics
- ✅ Current streak of activity days
- ✅ Next 5 upcoming events

---

## 🔧 Default Theme Configuration

Edit `~/.config/materiatrack/config.toml`:

```toml
[ui]
theme = "bahamut"  # Change this to your favorite theme
```

**Available themes**: fire, ice, lightning, earth, wind, bahamut

---

## 📊 Calendar TUI Features

### Main View

- ✨ Full monthly calendar
- 📅 Visual indicator for days with events (•)
- 🎯 Selected day highlighting
- 🌟 Special marker for "today"

### Sidebar

- 📈 **Month Stats**: Total events
- 🔥 **Current Streak**: Consecutive days with activity
- 📊 **Grand Total**: All recorded events
- 📋 **Upcoming Events**: List of next 5 events

### Help Screen

Press `h` inside the calendar to see all available controls.

---

## 🎓 Advanced Tips

### 1. Create Quick Reminders

```bash
# Script for tomorrow's event
tomorrow=$(date -d "tomorrow" +%Y-%m-%d)
mtrack cal --add "Daily Standup 9AM" --date "$tomorrow"
```

### 2. Combine with Tracking

```bash
# Start tracking and log event
mtrack track -p "Project X" -t "Sprint Planning"
mtrack cal --add "Sprint Planning completed" --date "$(date +%Y-%m-%d)"
```

### 3. View Calendar with Different Themes

```bash
# Test all themes
for theme in fire ice lightning earth wind bahamut; do
    echo "Theme: $theme"
    mtrack cal --theme $theme
done
```

---

## 🆘 Troubleshooting

### Command not found

```bash
# Check location
which mtrack

# Ensure ~/.local/bin is in your PATH
echo $PATH | grep ".local/bin"

# If not, add it to your ~/.bashrc or ~/.zshrc
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Calendar not showing tracking sessions

```bash
# Check if you have data in the database
mtrack list

# If you have data, open the calendar and they should appear
mtrack cal
```

### Configuration Error

```bash
# Check your config file
cat ~/.config/materiatrack/config.toml

# Ensure string values have quotes
# ✅ Correct:   theme = "fire"
# ❌ Incorrect: theme = fire
```

---

## 📚 Additional Documentation

- **Original RFC**: `docs/RFC-calendar-tui.md`
- **Full Documentation**: `docs/calendar-tui.md`
- **General Help**: `mtrack --help`
- **Calendar Help**: `mtrack calendar --help`

---

## 🎉 Enjoy your new Calendar TUI!

The calendar is fully integrated with your tracking system. Every time you work on a project, it will automatically appear in the calendar. Additionally, you can add custom events to have a complete view of your day/week/month.

**Master your time, master your destiny!** 💎✨
