# Changelog

All notable changes to MateriaTrack will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-11-27

### Added

#### Core Features
- Time tracking with project/task hierarchy
- SQLite database with WAL mode for performance
- Time offset parsing for start/end adjustments (`--begin="-0:15"`)
- Multiple output formats: pretty, json, plain, statusbar

#### Integrations
- Git commit auto-import during tracking sessions
- Obsidian daily notes bidirectional sync
- DWM/Polybar/i3blocks/Waybar statusbar integration
- Zeit database import with preview mode

#### Security
- GPG encryption for database
- Append-only audit logging with SHA256 checksums
- Tamper-evident log verification
- Secure file permissions (0600/0700)
- Data sanitization for exports

#### User Experience
- 6 elemental Materia themes (Fire, Ice, Lightning, Earth, Wind, Bahamut)
- 20+ achievements with Final Fantasy references
- Fuzzy finder with frecency scoring
- Desktop notifications via notify-send
- Pomodoro timer (25/5 minute cycles)
- Interactive TUI dashboard

#### Documentation
- Man page with examples
- Shell completions (Bash, Zsh, Fish)
- Comprehensive documentation

#### Packaging
- Cross-platform build system
- Universal installer/uninstaller scripts
- Arch Linux PKGBUILD
- Homebrew formula
- RPM spec
- GitHub Actions CI/CD

### Technical Details
- Built with Rust for performance and safety
- 84 tests with comprehensive coverage
- Binary size: ~6MB (optimized release build)
- Dependencies: rusqlite, clap, chrono, colored, git2, regex, sha2

---

## [Unreleased]

### Planned
- Windows support
- Custom theme definitions
- Time tracking API server mode
- Mobile companion app integration
- Additional integrations (Notion, Toggl import)

---

*"The Materia continues to evolve..."*
