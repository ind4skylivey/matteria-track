# Installation Guide

## Quick Install (Recommended)

### One-liner

```bash
curl -sSL https://raw.githubusercontent.com/ind4skylivey/matteria-track/main/build/install.sh | bash
```

This automatically:
- Detects your OS and architecture
- Downloads the appropriate binary (Linux glibc, Linux musl, macOS x86_64/arm64) or falls back to local build if none available
- Installs to `~/.local/bin` (or `/usr/local/bin` with sudo)
- Sets up shell completions
- Creates config directory

## Package Managers

### Arch Linux (AUR)

```bash
# Using yay
yay -S materiatrack

# Using paru
paru -S materiatrack

# Manual
git clone https://aur.archlinux.org/materiatrack.git
cd materiatrack
makepkg -si
```

### Homebrew (macOS)

```bash
brew tap ind4skylivey/tap
brew install materiatrack
```

## Manual Installation

### From Source

```bash
# Clone repository
git clone https://github.com/ind4skylivey/matteria-track.git
cd matteria-track

# Build release
cargo build --release

# Install
./build/install.sh --local

# Or manually
sudo cp target/release/materiatrack /usr/local/bin/
sudo cp man/materiatrack.1 /usr/share/man/man1/
```

### From Release Binary

```bash
# Download latest release
VERSION="1.0.3"
ARCH="x86_64-unknown-linux-musl" # or x86_64-unknown-linux-gnu, x86_64-apple-darwin, aarch64-apple-darwin
wget "https://github.com/ind4skylivey/matteria-track/releases/download/v${VERSION}/materiatrack-${VERSION}-${ARCH}.tar.gz"

# Extract
tar -xzf "materiatrack-${VERSION}-${ARCH}.tar.gz"

# Install
sudo cp materiatrack /usr/local/bin/
sudo cp man/materiatrack.1 /usr/share/man/man1/
```

Available artifacts (v1.0.3):
- materiatrack-1.0.3-x86_64-unknown-linux-gnu.tar.gz (glibc)
- materiatrack-1.0.3-x86_64-unknown-linux-musl.tar.gz (static/musl)
- materiatrack-1.0.3-x86_64-apple-darwin.tar.gz (macOS Intel)
- materiatrack-1.0.3-aarch64-apple-darwin.tar.gz (macOS Apple Silicon)
- SHA256SUMS

All at: https://github.com/ind4skylivey/matteria-track/releases/tag/v1.0.3

## Platform-Specific Notes

### Linux

**Dependencies:**
- `sqlite3` - Database (bundled in binary)
- `gpg` - Optional, for encrypted database
- `libnotify` - Optional, for desktop notifications

```bash
# Debian/Ubuntu
sudo apt install sqlite3 gnupg libnotify-bin

# Fedora/RHEL
sudo dnf install sqlite gnupg2 libnotify

# Arch
sudo pacman -S sqlite gnupg libnotify
```

### macOS

```bash
# Install Xcode command line tools if needed
xcode-select --install

# Dependencies via Homebrew
brew install sqlite gnupg
```

### FreeBSD

```bash
pkg install sqlite3 gnupg
```

## Shell Completions

### Bash

```bash
# System-wide
sudo cp completions/materiatrack.bash /etc/bash_completion.d/materiatrack

# User-only
mkdir -p ~/.local/share/bash-completion/completions
cp completions/materiatrack.bash ~/.local/share/bash-completion/completions/materiatrack
```

### Zsh

```bash
# Oh My Zsh
cp completions/_materiatrack ~/.oh-my-zsh/completions/

# System-wide
sudo cp completions/_materiatrack /usr/local/share/zsh/site-functions/

# User-only
mkdir -p ~/.local/share/zsh/site-functions
cp completions/_materiatrack ~/.local/share/zsh/site-functions/
```

### Fish

```bash
cp completions/materiatrack.fish ~/.config/fish/completions/
```

## Verification

```bash
# Check installation
materiatrack --version

# Or use alias
mtrack --version

# Run smoke test
mtrack project add "Test"
mtrack task add "TestTask" -p "Test"
mtrack track -p "Test" -t "TestTask"
mtrack status
mtrack finish
mtrack list
```

## Uninstallation

```bash
# Using uninstall script
./build/uninstall.sh

# Keep data
./build/uninstall.sh --keep-data

# Remove everything including config and database
./build/uninstall.sh --all

# Backup before removing
./build/uninstall.sh --backup --all
```

## Upgrading

```bash
# Using installer (auto-detects and upgrades)
curl -sSL https://raw.githubusercontent.com/ind4skylivey/matteria-track/main/build/install.sh | bash

# Using cargo
cargo install materiatrack --force

# Using AUR
yay -Syu materiatrack
```

## Troubleshooting

### Binary not found after install

Add `~/.local/bin` to your PATH:

```bash
# Bash (~/.bashrc)
export PATH="$HOME/.local/bin:$PATH"

# Zsh (~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"

# Fish (~/.config/fish/config.fish)
set -gx PATH $HOME/.local/bin $PATH
```

### Permission denied

```bash
# Make binary executable
chmod +x ~/.local/bin/materiatrack

# Or install with sudo
sudo ./build/install.sh
```

### Database locked

```bash
# Check for running instances
pgrep materiatrack

# If stuck, restart
killall materiatrack
```

---

*"The Materia has been equipped successfully"*
