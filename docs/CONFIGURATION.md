# Configuration Guide

## Config File Location

```
~/.config/materiatrack/config.toml
```

Or specify custom path:
```bash
mtrack --config /path/to/config.toml <command>
```

## Full Configuration Reference

```toml
[database]
# Database file path (empty = default location)
path = ""

[ui]
# Theme: fire, ice, lightning, earth, wind, bahamut
theme = "fire"

[tracking]
# Auto-import git commits when tracking in a git repo
auto_import_git = false

# Default git repository path for imports
git_repo_path = ""

[notifications]
# Enable desktop notifications
enable = false

# Reminder interval in minutes (0 = disabled)
reminder_interval = 30

# Hour to send daily summary (0-23)
daily_summary_hour = 18

[integrations]
# Obsidian vault path for daily notes sync
obsidian_path = ""

[security]
# Enable GPG encryption for database
enable_encryption = false

# GPG key ID or email for encryption
encryption_key = ""

# Enable audit logging
enable_audit_log = false

# Custom audit log path (empty = default)
audit_log_path = ""
```

## Section Details

### Database

```toml
[database]
path = "/custom/path/materiatrack.db"
```

Default locations:
- Linux: `~/.local/share/materiatrack/materiatrack.db`
- macOS: `~/Library/Application Support/materiatrack/materiatrack.db`
- Windows: `%APPDATA%\materiatrack\materiatrack.db`

### UI

```toml
[ui]
theme = "bahamut"
```

Available themes:
| Theme | Description | Primary Color |
|-------|-------------|---------------|
| `fire` | Burning passion | Red/Orange |
| `ice` | Cool precision | Blue/Cyan |
| `lightning` | Electric energy | Yellow/Purple |
| `earth` | Grounded strength | Green/Brown |
| `wind` | Ethereal freedom | White/Gray |
| `bahamut` | Ultimate power | Dark Purple/Gold |

### Tracking

```toml
[tracking]
auto_import_git = true
git_repo_path = "~/projects/myrepo"
```

When `auto_import_git = true`:
- Commits made during tracking are auto-imported
- Commit messages become entry notes
- Time ranges are estimated from commit timestamps

### Notifications

```toml
[notifications]
enable = true
reminder_interval = 30
daily_summary_hour = 18
```

Notifications require `notify-send` (Linux) or similar.

**Reminder behavior:**
- Sends notification every `reminder_interval` minutes while tracking
- Set to `0` to disable reminders
- Daily summary sent at `daily_summary_hour` (24-hour format)

### Integrations

```toml
[integrations]
obsidian_path = "~/Documents/Obsidian/MyVault"
```

Obsidian sync:
- Creates daily notes with time entries
- Parses existing time blocks in daily notes
- Bidirectional sync supported

### Security

```toml
[security]
enable_encryption = true
encryption_key = "your.email@example.com"
enable_audit_log = true
audit_log_path = ""
```

**Encryption:**
- Uses GPG for database encryption
- Key must be in your GPG keyring
- Run `gpg --list-keys` to see available keys

**Audit logging:**
- Append-only log with SHA256 checksums
- Tamper-evident design
- Logs: track, finish, edit, delete, export

## Environment Variables

| Variable | Description |
|----------|-------------|
| `MTRACK_CONFIG` | Custom config file path |
| `MTRACK_DB` | Custom database path |
| `EDITOR` | Editor for config editing |

## Command-Line Overrides

```bash
# Custom config
mtrack --config /path/to/config.toml list

# Output format
mtrack --format json list
mtrack --format plain stats

# Verbose output
mtrack --verbose track -p "Project" -t "Task"
```

## Managing Config

```bash
# Show current config
mtrack config show

# Show config file path
mtrack config path

# Edit in default editor
mtrack config edit
```

## Example Configurations

### Minimal

```toml
[ui]
theme = "fire"

[notifications]
enable = false
```

### Power User

```toml
[ui]
theme = "bahamut"

[tracking]
auto_import_git = true
git_repo_path = "~/code"

[notifications]
enable = true
reminder_interval = 25

[integrations]
obsidian_path = "~/Documents/Notes"

[security]
enable_audit_log = true
```

### High Security

```toml
[ui]
theme = "ice"

[security]
enable_encryption = true
encryption_key = "security@example.com"
enable_audit_log = true
audit_log_path = "~/.config/materiatrack/audit.log"
```

---

*"Configure your Materia loadout wisely"*
