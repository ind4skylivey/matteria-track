# MateriaTrack Documentation

Welcome to MateriaTrack - a mystical, Final Fantasy-themed CLI time tracker.

## Table of Contents

- [Installation](INSTALL.md) - Setup guides for all platforms
- [Configuration](CONFIGURATION.md) - Config file reference
- [Integrations](INTEGRATIONS.md) - Git, Obsidian, DWM setup
- [Themes](THEMES.md) - Theme gallery and customization
- [Achievements](ACHIEVEMENTS.md) - Unlock conditions and rewards
- [Contributing](CONTRIBUTING.md) - How to contribute

## Quick Start

```bash
# Install
curl -sSL https://raw.githubusercontent.com/ind4skylivey/matteria-track/main/build/install.sh | bash

# Start tracking
mtrack track -p "Project" -t "Task"

# Stop tracking
mtrack finish

# View entries
mtrack list

# Statistics
mtrack stats
```

## Key Features

| Feature | Description |
|---------|-------------|
| Time Tracking | Track time with project/task hierarchy |
| Git Integration | Auto-import commits as time entries |
| Obsidian Sync | Bidirectional daily notes sync |
| Themes | 6 elemental Materia themes |
| Achievements | 20+ unlockable achievements |
| Security | GPG encryption, audit logging |
| Statusbar | DWM/Polybar/i3blocks output |

## Command Reference

### Tracking

```bash
mtrack track -p PROJECT -t TASK     # Start tracking
mtrack track -p PROJECT -t TASK --begin="-0:15"  # Start 15 min ago
mtrack finish                       # Stop tracking
mtrack finish --end="-0:10"         # End 10 min ago
mtrack status                       # Current status
```

### Listing

```bash
mtrack list                         # Recent entries
mtrack list --since 2024-01-01      # Since date
mtrack list --total                 # With totals
mtrack list --project "Project"     # Filter by project
```

### Statistics

```bash
mtrack stats                        # All time stats
mtrack stats --today                # Today only
mtrack stats --week                 # This week
mtrack stats --month                # This month
```

### Projects & Tasks

```bash
mtrack project add "Name"           # Create project
mtrack project list                 # List projects
mtrack task add "Name" -p PROJECT   # Create task
mtrack task list                    # List tasks
```

### Export/Import

```bash
mtrack export --export-format json  # Export JSON
mtrack export --export-format csv   # Export CSV
mtrack import --zeit ~/.zeit.db     # Import from Zeit
```

## Support

- Issues: https://github.com/ind4skylivey/matteria-track/issues
- Discussions: https://github.com/ind4skylivey/matteria-track/discussions

---

*"Master your time, master your destiny"*
