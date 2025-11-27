#!/usr/bin/env bash
set -euo pipefail

VERSION="${VERSION:-latest}"
REPO="ind4skylivey/matteria-track"
BINARY_NAME="materiatrack"
INSTALL_DIR="${INSTALL_DIR:-}"
MAN_DIR="${MAN_DIR:-}"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[INFO]${NC} $*"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }

print_banner() {
    echo -e "${CYAN}"
    cat << 'EOF'
╔══════════════════════════════════════════════════════════════╗
║  💎 MatteriaTrack Installer                                   ║
║     "Equipping your system with Materia"                     ║
╚══════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
}

detect_os() {
    case "$(uname -s)" in
        Linux*)  echo "linux" ;;
        Darwin*) echo "darwin" ;;
        FreeBSD*) echo "freebsd" ;;
        *)       echo "unknown" ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64) echo "x86_64" ;;
        aarch64|arm64) echo "aarch64" ;;
        *)            echo "unknown" ;;
    esac
}

detect_shell() {
    local shell_name
    shell_name=$(basename "${SHELL:-bash}")
    echo "$shell_name"
}

get_install_dir() {
    if [[ -n "$INSTALL_DIR" ]]; then
        echo "$INSTALL_DIR"
    elif [[ $EUID -eq 0 ]]; then
        echo "/usr/local/bin"
    else
        echo "$HOME/.local/bin"
    fi
}

get_man_dir() {
    if [[ -n "$MAN_DIR" ]]; then
        echo "$MAN_DIR"
    elif [[ $EUID -eq 0 ]]; then
        echo "/usr/local/share/man/man1"
    else
        echo "$HOME/.local/share/man/man1"
    fi
}

get_completions_dir() {
    local shell="$1"
    case "$shell" in
        bash)
            if [[ $EUID -eq 0 ]]; then
                echo "/etc/bash_completion.d"
            else
                echo "$HOME/.local/share/bash-completion/completions"
            fi
            ;;
        zsh)
            if [[ -d "$HOME/.oh-my-zsh" ]]; then
                echo "$HOME/.oh-my-zsh/completions"
            elif [[ $EUID -eq 0 ]]; then
                echo "/usr/local/share/zsh/site-functions"
            else
                echo "$HOME/.local/share/zsh/site-functions"
            fi
            ;;
        fish)
            if [[ $EUID -eq 0 ]]; then
                echo "/usr/share/fish/vendor_completions.d"
            else
                echo "$HOME/.config/fish/completions"
            fi
            ;;
        *)
            echo ""
            ;;
    esac
}

get_config_dir() {
    echo "${XDG_CONFIG_HOME:-$HOME/.config}/materiatrack"
}

get_data_dir() {
    echo "${XDG_DATA_HOME:-$HOME/.local/share}/materiatrack"
}

resolve_version() {
    if [[ "$VERSION" != "latest" ]]; then
        echo "$VERSION"
        return
    fi

    local latest
    if latest=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep -m1 '"tag_name"' | cut -d':' -f2 | tr -d '", '); then
        latest="${latest#v}"
        if [[ -n "$latest" ]]; then
            echo "$latest"
            return
        fi
    fi

    log_error "Could not determine latest release version automatically. Set VERSION env var (e.g., VERSION=1.0.1)."
    exit 1
}

download_release() {
    local os="$1"
    local arch="$2"
    local target

    case "${os}-${arch}" in
        linux-x86_64)  target="x86_64-unknown-linux-gnu" ;;
        linux-aarch64) target="aarch64-unknown-linux-gnu" ;;
        darwin-x86_64) target="x86_64-apple-darwin" ;;
        darwin-aarch64) target="aarch64-apple-darwin" ;;
        freebsd-x86_64) target="x86_64-unknown-freebsd" ;;
        *)
            log_error "Unsupported platform: $os-$arch"
            exit 1
            ;;
    esac

    local version
    version=$(resolve_version)

    local url="https://github.com/${REPO}/releases/download/v${version}/materiatrack-${version}-${target}.tar.gz"
    local tmp_dir
    tmp_dir=$(mktemp -d)

    log_info "Downloading MatteriaTrack v${version} for ${target}..."

    if command -v curl &>/dev/null; then
        curl -fsSL "$url" -o "$tmp_dir/materiatrack.tar.gz" || {
            log_error "Download failed. Check if release exists: $url"
            rm -rf "$tmp_dir"
            exit 1
        }
    elif command -v wget &>/dev/null; then
        wget -q "$url" -O "$tmp_dir/materiatrack.tar.gz" || {
            log_error "Download failed. Check if release exists: $url"
            rm -rf "$tmp_dir"
            exit 1
        }
    else
        log_error "Neither curl nor wget found. Please install one."
        exit 1
    fi

    tar -xzf "$tmp_dir/materiatrack.tar.gz" -C "$tmp_dir"
    echo "$tmp_dir"
}

install_binary() {
    local src="$1"
    local dest_dir="$2"

    mkdir -p "$dest_dir"
    cp "$src" "$dest_dir/$BINARY_NAME"
    chmod 755 "$dest_dir/$BINARY_NAME"

    log_success "Binary installed: $dest_dir/$BINARY_NAME"
}

install_man_page() {
    local src_dir="$1"
    local dest_dir="$2"

    if [[ -f "$src_dir/man/materiatrack.1" ]]; then
        mkdir -p "$dest_dir"
        cp "$src_dir/man/materiatrack.1" "$dest_dir/"
        log_success "Man page installed: $dest_dir/materiatrack.1"
    else
        log_warn "Man page not found in release"
    fi
}

install_completions() {
    local src_dir="$1"
    local shell="$2"
    local dest_dir

    dest_dir=$(get_completions_dir "$shell")

    if [[ -z "$dest_dir" ]]; then
        log_warn "Unknown shell: $shell, skipping completions"
        return
    fi

    mkdir -p "$dest_dir"

    case "$shell" in
        bash)
            if [[ -f "$src_dir/completions/materiatrack.bash" ]]; then
                cp "$src_dir/completions/materiatrack.bash" "$dest_dir/materiatrack"
                log_success "Bash completions installed"
            fi
            ;;
        zsh)
            if [[ -f "$src_dir/completions/_materiatrack" ]]; then
                cp "$src_dir/completions/_materiatrack" "$dest_dir/"
                log_success "Zsh completions installed"
            fi
            ;;
        fish)
            if [[ -f "$src_dir/completions/materiatrack.fish" ]]; then
                cp "$src_dir/completions/materiatrack.fish" "$dest_dir/"
                log_success "Fish completions installed"
            fi
            ;;
    esac
}

setup_config() {
    local config_dir
    config_dir=$(get_config_dir)

    if [[ ! -d "$config_dir" ]]; then
        mkdir -p "$config_dir"
        chmod 700 "$config_dir"
        log_success "Config directory created: $config_dir"
    fi

    if [[ ! -f "$config_dir/config.toml" ]]; then
        cat > "$config_dir/config.toml" << 'EOF'
[database]
path = ""

[ui]
theme = "fire"

[tracking]
auto_import_git = false
git_repo_path = ""

[notifications]
enable = false
reminder_interval = 30
daily_summary_hour = 18

[integrations]
obsidian_path = ""

[security]
enable_encryption = false
encryption_key = ""
enable_audit_log = false
EOF
        chmod 600 "$config_dir/config.toml"
        log_success "Default config created: $config_dir/config.toml"
    fi
}

setup_data() {
    local data_dir
    data_dir=$(get_data_dir)

    if [[ ! -d "$data_dir" ]]; then
        mkdir -p "$data_dir"
        chmod 700 "$data_dir"
        log_success "Data directory created: $data_dir"
    fi
}

update_path() {
    local install_dir="$1"
    local shell
    shell=$(detect_shell)

    if [[ ":$PATH:" != *":$install_dir:"* ]]; then
        log_warn "$install_dir is not in your PATH"

        local rc_file=""
        case "$shell" in
            bash) rc_file="$HOME/.bashrc" ;;
            zsh)  rc_file="$HOME/.zshrc" ;;
            fish) rc_file="$HOME/.config/fish/config.fish" ;;
        esac

        if [[ -n "$rc_file" ]]; then
            echo ""
            echo "Add this to your $rc_file:"
            if [[ "$shell" == "fish" ]]; then
                echo -e "${YELLOW}  set -gx PATH $install_dir \$PATH${NC}"
            else
                echo -e "${YELLOW}  export PATH=\"$install_dir:\$PATH\"${NC}"
            fi
        fi
    fi
}

verify_installation() {
    local install_dir="$1"
    local binary="$install_dir/$BINARY_NAME"

    if [[ -x "$binary" ]]; then
        local version
        version=$("$binary" --version 2>/dev/null | head -1) || version="unknown"
        log_success "Verification: $version"
        return 0
    else
        log_error "Binary not executable or not found"
        return 1
    fi
}

print_post_install() {
    echo ""
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  💎 MatteriaTrack installed successfully!                     ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo "Quick start:"
    echo -e "  ${CYAN}mtrack track -p \"Project\" -t \"Task\"${NC}  # Start tracking"
    echo -e "  ${CYAN}mtrack finish${NC}                        # Stop tracking"
    echo -e "  ${CYAN}mtrack list${NC}                          # Show entries"
    echo -e "  ${CYAN}mtrack stats${NC}                         # View statistics"
    echo ""
    echo "Documentation:"
    echo -e "  ${CYAN}man materiatrack${NC}                     # Man page"
    echo -e "  ${CYAN}mtrack --help${NC}                        # Help"
    echo ""
    echo -e "${BLUE}\"Master your time, master your destiny\"${NC}"
}

local_install() {
    log_info "Installing from local build..."

    local script_dir
    script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
    local project_dir
    project_dir="$(dirname "$script_dir")"

    local binary="$project_dir/target/release/materiatrack"

    if [[ ! -f "$binary" ]]; then
        log_error "Binary not found. Run './build/build.sh' first"
        exit 1
    fi

    local install_dir
    install_dir=$(get_install_dir)
    local man_dir
    man_dir=$(get_man_dir)
    local shell
    shell=$(detect_shell)

    install_binary "$binary" "$install_dir"

    if [[ -f "$project_dir/man/materiatrack.1" ]]; then
        mkdir -p "$man_dir"
        cp "$project_dir/man/materiatrack.1" "$man_dir/"
        log_success "Man page installed"
    fi

    install_completions "$project_dir" "$shell"
    setup_config
    setup_data
    update_path "$install_dir"
    verify_installation "$install_dir"
    print_post_install
}

remote_install() {
    local os
    os=$(detect_os)
    local arch
    arch=$(detect_arch)
    local install_dir
    install_dir=$(get_install_dir)
    local man_dir
    man_dir=$(get_man_dir)
    local shell
    shell=$(detect_shell)

    log_info "Detected: $os-$arch"
    log_info "Install directory: $install_dir"

    local tmp_dir
    tmp_dir=$(download_release "$os" "$arch")
    local extract_dir="$tmp_dir/materiatrack-${VERSION}-"*

    for dir in $extract_dir; do
        if [[ -d "$dir" ]]; then
            install_binary "$dir/materiatrack" "$install_dir"
            install_man_page "$dir" "$man_dir"
            install_completions "$dir" "$shell"
            break
        fi
    done

    rm -rf "$tmp_dir"

    setup_config
    setup_data
    update_path "$install_dir"
    verify_installation "$install_dir"
    print_post_install
}

show_help() {
    cat << EOF
MatteriaTrack Installer

USAGE:
    $0 [OPTIONS]

OPTIONS:
    --local         Install from local build (default if in repo)
    --remote        Download and install from GitHub releases
    --version VER   Specify version to install (default: $VERSION)
    --prefix DIR    Install to custom directory
    --help          Show this help

ENVIRONMENT VARIABLES:
    INSTALL_DIR     Custom binary install directory
    MAN_DIR         Custom man page directory
    VERSION         Version to install

EXAMPLES:
    $0                          # Auto-detect and install
    $0 --local                  # Install from local build
    $0 --remote --version 1.0.0 # Install specific version
    INSTALL_DIR=/opt/bin $0     # Custom install directory
EOF
}

main() {
    print_banner

    local mode="auto"

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --local)
                mode="local"
                shift
                ;;
            --remote)
                mode="remote"
                shift
                ;;
            --version)
                VERSION="$2"
                shift 2
                ;;
            --prefix)
                INSTALL_DIR="$2"
                shift 2
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done

    if [[ "$mode" == "auto" ]]; then
        local script_dir
        script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
        if [[ -f "$(dirname "$script_dir")/Cargo.toml" ]]; then
            mode="local"
        else
            mode="remote"
        fi
    fi

    case "$mode" in
        local)
            local_install
            ;;
        remote)
            remote_install
            ;;
    esac
}

main "$@"
