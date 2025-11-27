# Integrations Guide

MatteriaTrack integrates with various tools to enhance your workflow.

## Git Integration

Auto-import commits as time entries.

### Setup

```toml
# config.toml
[tracking]
auto_import_git = true
git_repo_path = "~/projects/myrepo"
```

### Usage

```bash
# Track in a git repo
cd ~/projects/myrepo
mtrack track -p "MyProject" -t "Development"

# ... make commits ...

mtrack finish
# Commits during this period are auto-imported
```

### Manual Import

```bash
# Import commits from last 7 days
mtrack import --git --since 7d

# Import from specific repo
mtrack import --git --repo ~/other/repo
```

### How It Works

1. When tracking finishes, MatteriaTrack scans for commits
2. Commits within the tracking window are associated
3. Commit messages become entry notes
4. Author and hash are stored for reference

## Obsidian Integration

Bidirectional sync with Obsidian daily notes.

### Setup

```toml
# config.toml
[integrations]
obsidian_path = "~/Documents/Obsidian/MyVault"
```

### Daily Note Format

MatteriaTrack uses this format in daily notes:

```markdown
## Time Tracking

- 09:00-12:00 Project > Task (3h 0m)
- 14:00-17:30 Another > Work (3h 30m)

**Total: 6h 30m**
```

### Export to Obsidian

```bash
# Sync today's entries to Obsidian
mtrack export --obsidian

# Sync specific date
mtrack export --obsidian --date 2024-01-15
```

### Import from Obsidian

```bash
# Parse time blocks from daily notes
mtrack import --obsidian --date 2024-01-15
```

### Custom Daily Note Path

Default: `Daily Notes/YYYY-MM-DD.md`

Override in config:
```toml
[integrations]
obsidian_path = "~/Vault"
obsidian_daily_format = "Journal/%Y/%m/%Y-%m-%d.md"
```

## Statusbar Integration

Output for DWM, Polybar, i3blocks, Waybar, and more.

### DWM

Add to your statusbar script:

```bash
#!/bin/bash
while true; do
    STATUS=$(mtrack statusbar 2>/dev/null || echo "💎 --:--")
    xsetroot -name "$STATUS"
    sleep 60
done
```

### Polybar

```ini
[module/materiatrack]
type = custom/script
exec = mtrack statusbar --format polybar
interval = 60
label = %output%
```

### i3blocks

```ini
[materiatrack]
command=mtrack statusbar
interval=60
```

### Waybar

```json
{
    "custom/materiatrack": {
        "exec": "mtrack statusbar --format waybar",
        "interval": 60,
        "return-type": "json"
    }
}
```

### Statusbar Output Formats

```bash
# Default (plain)
mtrack statusbar
# Output: 💎 Project > Task 02:30

# Polybar (with colors)
mtrack statusbar --format polybar
# Output: %{F#ff6b6b}💎%{F-} Project > Task %{F#4ecdc4}02:30%{F-}

# Waybar (JSON)
mtrack statusbar --format waybar
# Output: {"text":"💎 02:30","tooltip":"Project > Task","class":"tracking"}

# Lemonbar
mtrack statusbar --format lemonbar

# tmux
mtrack statusbar --format tmux
```

### Standalone Script

```bash
# Use included script
./scripts/dwm-statusbar.sh

# With custom refresh
./scripts/dwm-statusbar.sh --interval 30
```

## Zeit Import

Migrate from Zeit time tracker.

### Full Import

```bash
# Import entire Zeit database
mtrack import --zeit ~/.zeit.db

# Preview without importing
mtrack import --zeit ~/.zeit.db --preview
```

### Selective Import

```bash
# Import only recent entries
mtrack import --zeit ~/.zeit.db --since 2024-01-01

# Import specific project
mtrack import --zeit ~/.zeit.db --project "Work"
```

### Data Mapping

| Zeit | MatteriaTrack |
|------|--------------|
| Project | Project |
| Activity | Task |
| Entry | Entry |
| Notes | Notes |

## Desktop Notifications

Real-time tracking notifications.

### Setup

```toml
[notifications]
enable = true
reminder_interval = 30
daily_summary_hour = 18
```

### Requirements

- Linux: `libnotify` (`notify-send`)
- macOS: Built-in notification center
- Windows: Not yet supported

### Notification Types

1. **Tracking Started** - When you begin tracking
2. **Tracking Finished** - When you stop tracking
3. **Reminders** - Periodic while tracking
4. **Idle Reminder** - When not tracking for a while
5. **Daily Summary** - End of day summary
6. **Achievements** - When unlocked

## Pomodoro Timer

Built-in Pomodoro technique support.

### Usage

```bash
# Start Pomodoro session
mtrack pomodoro start

# Status
mtrack pomodoro status

# Skip break
mtrack pomodoro skip
```

### Configuration

Default: 25 min work, 5 min short break, 15 min long break (every 4 pomodoros)

```toml
[pomodoro]
work_minutes = 25
short_break = 5
long_break = 15
```

## API/Scripting

Use MatteriaTrack in scripts.

### JSON Output

```bash
# Get entries as JSON
mtrack list --format json | jq '.entries'

# Current status
mtrack status --format json

# Stats
mtrack stats --format json --today
```

### Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Error |
| 2 | Config error |
| 3 | Database error |

### Example Script

```bash
#!/bin/bash
# Auto-track based on git activity

if git status &>/dev/null; then
    PROJECT=$(basename $(git rev-parse --show-toplevel))
    BRANCH=$(git branch --show-current)
    
    # Start tracking if not already
    if ! mtrack status --format json | jq -e '.tracking' &>/dev/null; then
        mtrack track -p "$PROJECT" -t "$BRANCH"
    fi
fi
```

---

*"Linking Materia for maximum effect"*
