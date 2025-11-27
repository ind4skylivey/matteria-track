# MateriaTrack 💎

[![Release](https://img.shields.io/github/v/release/ind4skylivey/matteria-track?style=for-the-badge)](https://github.com/ind4skylivey/matteria-track/releases)
[![Tests](https://img.shields.io/github/actions/workflow/status/ind4skylivey/matteria-track/ci.yml?style=for-the-badge&label=tests)](https://github.com/ind4skylivey/matteria-track/actions)
[![License](https://img.shields.io/github/license/ind4skylivey/matteria-track?style=for-the-badge)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/materiatrack?style=for-the-badge)](https://crates.io/crates/materiatrack)

> **Mystical Final Fantasy-themed CLI time tracker for power users**

*"Master your time, master your destiny"*

```
╔══════════════════════════════════════════════════════════════╗
║  💎 MateriaTrack - Time Tracking Forged in Mako Energy 💎    ║
║     "Master your time, master your destiny"                  ║
╚══════════════════════════════════════════════════════════════╝
```

## Why you’ll like it
- ⚡ Fast: async, fuzzy finder, zero telemetry.
- 🔐 Safe: optional GPG encryption + tamper-evident audit log.
- 🔗 Connected: Git auto-import, Obsidian sync, statusbar output.
- 🎨 Personal: 6 elemental themes + achievements.
- 🧭 Portable: release binaries for Linux (glibc, musl) and macOS (Intel/ARM).

## Features

🔥 **6 Elemental Themes** - Fire, Ice, Lightning, Earth, Wind, Bahamut

⚔️ **Git Integration** - Auto-import commits as time entries

📓 **Obsidian Sync** - Bidirectional daily notes synchronization

🏆 **20+ Achievements** - Unlock rewards as you track

🔒 **GPG Encryption** - Optional encrypted database

📊 **Statusbar Support** - DWM, Polybar, i3blocks, Waybar

🍅 **Pomodoro Timer** - Built-in focus sessions

🔍 **Fuzzy Finder** - Quick project/task switching

## Quick Start

### Install

```bash
curl -sSL https://raw.githubusercontent.com/ind4skylivey/matteria-track/main/build/install.sh | bash
```

### Or via package manager / binaries

```bash
# Arch Linux (AUR)
yay -S materiatrack

# Homebrew (macOS)
brew install ind4skylivey/tap/materiatrack

# Release binaries (no build)
# Linux glibc (x86_64), Linux musl (x86_64), macOS Intel, macOS Apple Silicon
# See Downloads section below for direct links/checksums
```

### Basic Usage

```bash
# Start tracking
mtrack track -p "MyProject" -t "Development"

# Check status
mtrack status

# Stop tracking
mtrack finish

# View entries
mtrack list

# Statistics
mtrack stats --today
```

## Downloads

Latest release binaries (v1.0.3):

| Target | Asset |
| ------ | ------ |
| Linux x86_64 (glibc) | `materiatrack-1.0.3-x86_64-unknown-linux-gnu.tar.gz` |
| Linux x86_64 (musl/static) | `materiatrack-1.0.3-x86_64-unknown-linux-musl.tar.gz` |
| macOS x86_64 | `materiatrack-1.0.3-x86_64-apple-darwin.tar.gz` |
| macOS ARM64 | `materiatrack-1.0.3-aarch64-apple-darwin.tar.gz` |
| Checksums | `SHA256SUMS` |

All assets live at the GitHub Release page: https://github.com/ind4skylivey/matteria-track/releases/tag/v1.0.3

## Demo

```
$ mtrack track -p "MateriaTrack" -t "README"

💎 MateriaTrack v1.0.0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔥 Started tracking: MateriaTrack > README
   ⏰ 14:30:00

$ mtrack status

💎 Currently Tracking
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🏔️ Project: MateriaTrack
⚔️ Task: README
⏱️ Duration: 00:45:23
   Started: 14:30:00

$ mtrack finish

✓ Tracked: MateriaTrack > README (0h 45m)
```

## Themes

Choose your elemental Materia:

| Theme | Description | Preview |
|-------|-------------|---------|
| 🔥 Fire | Burning passion | Red/Orange |
| ❄️ Ice | Cool precision | Blue/Cyan |
| ⚡ Lightning | Electric energy | Yellow/Purple |
| 🌍 Earth | Grounded strength | Green/Brown |
| 💨 Wind | Ethereal freedom | White/Gray |
| 🐉 Bahamut | Ultimate power | Purple/Gold |

```bash
# Set theme in config
[ui]
theme = "bahamut"
```

## Integrations

### Git

```bash
# Auto-import commits while tracking
[tracking]
auto_import_git = true
```

### Obsidian

```bash
# Sync with daily notes
[integrations]
obsidian_path = "~/Documents/Obsidian/Vault"
```

### Statusbar

```bash
# DWM/dwmblocks
mtrack statusbar

# Polybar
mtrack statusbar --format polybar

# Waybar (JSON)
mtrack statusbar --format waybar
```

## Achievements

Unlock achievements as you master time tracking:

- 💎 **Materia Equipped** - First tracked entry
- ⚡ **Limit Break** - 500 hours tracked
- 📅 **Week Warrior** - 7 day streak
- 🏅 **SOLDIER 1st Class** - 100 day streak
- 🎮 **???** - *Secret achievement*

```bash
mtrack achievements
```

## Documentation

- [Installation Guide](docs/INSTALL.md)
- [Configuration](docs/CONFIGURATION.md)
- [Integrations](docs/INTEGRATIONS.md)
- [Themes](docs/THEMES.md)
- [Achievements](docs/ACHIEVEMENTS.md)
- [Contributing](docs/CONTRIBUTING.md)

## Commands

| Command | Description |
|---------|-------------|
| `track` | Start tracking |
| `finish` | Stop tracking |
| `status` | Current status |
| `list` | Show entries |
| `stats` | Statistics |
| `project` | Manage projects |
| `task` | Manage tasks |
| `export` | Export data |
| `import` | Import data |
| `achievements` | View achievements |
| `dashboard` | Interactive TUI |
| `statusbar` | Statusbar output |

## Configuration

```toml
# ~/.config/materiatrack/config.toml

[ui]
theme = "fire"

[tracking]
auto_import_git = true

[notifications]
enable = true
reminder_interval = 30

[security]
enable_encryption = false
```

## Building from Source

```bash
git clone https://github.com/ind4skylivey/matteria-track.git
cd matteria-track
cargo build --release
./build/install.sh --local
```

## Requirements

- Rust 1.70+ (for building)
- SQLite (bundled)
- GPG (optional, for encryption)
- libnotify (optional, for notifications)

## Contributing

Contributions welcome! See [CONTRIBUTING.md](docs/CONTRIBUTING.md)

## License

MIT License - see [LICENSE](LICENSE)

## Credits

Inspired by:
- [Zeit](https://github.com/mrusme/zeit) - Original time tracker
- Final Fantasy VII - Theme and aesthetics
- The Rust community

---

<p align="center">
  <i>"The Materia awaits your command"</i>
</p>
