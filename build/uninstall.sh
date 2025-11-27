#!/usr/bin/env bash
set -euo pipefail

BINARY_NAME="materiatrack"

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
║  💎 MateriaTrack Uninstaller                                 ║
║     "Returning Materia to the Lifestream"                    ║
╚══════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
}

get_config_dir() {
    echo "${XDG_CONFIG_HOME:-$HOME/.config}/materiatrack"
}

get_data_dir() {
    echo "${XDG_DATA_HOME:-$HOME/.local/share}/materiatrack"
}

find_binary() {
    local locations=(
        "/usr/local/bin/$BINARY_NAME"
        "$HOME/.local/bin/$BINARY_NAME"
        "$HOME/.cargo/bin/$BINARY_NAME"
    )

    for loc in "${locations[@]}"; do
        if [[ -f "$loc" ]]; then
            echo "$loc"
            return 0
        fi
    done

    local found
    found=$(command -v "$BINARY_NAME" 2>/dev/null) || true
    if [[ -n "$found" ]]; then
        echo "$found"
        return 0
    fi

    return 1
}

find_man_page() {
    local locations=(
        "/usr/local/share/man/man1/materiatrack.1"
        "/usr/share/man/man1/materiatrack.1"
        "$HOME/.local/share/man/man1/materiatrack.1"
    )

    for loc in "${locations[@]}"; do
        if [[ -f "$loc" ]]; then
            echo "$loc"
            return 0
        fi
    done

    return 1
}

find_completions() {
    local files=()

    local bash_locations=(
        "/etc/bash_completion.d/materiatrack"
        "$HOME/.local/share/bash-completion/completions/materiatrack"
    )

    local zsh_locations=(
        "/usr/local/share/zsh/site-functions/_materiatrack"
        "$HOME/.local/share/zsh/site-functions/_materiatrack"
        "$HOME/.oh-my-zsh/completions/_materiatrack"
    )

    local fish_locations=(
        "/usr/share/fish/vendor_completions.d/materiatrack.fish"
        "$HOME/.config/fish/completions/materiatrack.fish"
    )

    for loc in "${bash_locations[@]}" "${zsh_locations[@]}" "${fish_locations[@]}"; do
        if [[ -f "$loc" ]]; then
            files+=("$loc")
        fi
    done

    printf '%s\n' "${files[@]}"
}

backup_data() {
    local config_dir
    config_dir=$(get_config_dir)
    local data_dir
    data_dir=$(get_data_dir)

    local backup_dir="$HOME/materiatrack-backup-$(date +%Y%m%d-%H%M%S)"

    if [[ -d "$config_dir" ]] || [[ -d "$data_dir" ]]; then
        mkdir -p "$backup_dir"

        if [[ -d "$config_dir" ]]; then
            cp -r "$config_dir" "$backup_dir/config"
            log_info "Config backed up to: $backup_dir/config"
        fi

        if [[ -d "$data_dir" ]]; then
            cp -r "$data_dir" "$backup_dir/data"
            log_info "Data backed up to: $backup_dir/data"
        fi

        log_success "Backup created: $backup_dir"
    fi
}

remove_binary() {
    local binary
    if binary=$(find_binary); then
        rm -f "$binary"
        log_success "Removed binary: $binary"

        local symlinks=("mtrack" "mt")
        local dir
        dir=$(dirname "$binary")
        for link in "${symlinks[@]}"; do
            if [[ -L "$dir/$link" ]]; then
                rm -f "$dir/$link"
                log_success "Removed symlink: $dir/$link"
            fi
        done
    else
        log_warn "Binary not found"
    fi
}

remove_man_page() {
    local man_page
    if man_page=$(find_man_page); then
        rm -f "$man_page"
        log_success "Removed man page: $man_page"
    else
        log_info "Man page not found (already removed or never installed)"
    fi
}

remove_completions() {
    local completions
    completions=$(find_completions)

    if [[ -n "$completions" ]]; then
        while IFS= read -r file; do
            rm -f "$file"
            log_success "Removed completion: $file"
        done <<< "$completions"
    else
        log_info "Completions not found"
    fi
}

remove_config() {
    local config_dir
    config_dir=$(get_config_dir)

    if [[ -d "$config_dir" ]]; then
        rm -rf "$config_dir"
        log_success "Removed config: $config_dir"
    fi
}

remove_data() {
    local data_dir
    data_dir=$(get_data_dir)

    if [[ -d "$data_dir" ]]; then
        rm -rf "$data_dir"
        log_success "Removed data: $data_dir"
    fi
}

print_post_uninstall() {
    echo ""
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║  💎 MateriaTrack uninstalled successfully!                   ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BLUE}\"The Materia has returned to the Planet...\"${NC}"
    echo ""
    echo "Thank you for using MateriaTrack!"
    echo "Feedback: https://github.com/ind4skylivey/matteria-track/issues"
}

show_help() {
    cat << EOF
MateriaTrack Uninstaller

USAGE:
    $0 [OPTIONS]

OPTIONS:
    --all           Remove everything including config and data
    --keep-data     Remove binary but keep config and data (default)
    --backup        Create backup before removing data
    --dry-run       Show what would be removed without removing
    --help          Show this help

EXAMPLES:
    $0                  # Remove binary, keep data
    $0 --all            # Remove everything
    $0 --backup --all   # Backup then remove everything
    $0 --dry-run        # Preview what would be removed
EOF
}

main() {
    print_banner

    local remove_data_flag=false
    local backup_flag=false
    local dry_run=false

    while [[ $# -gt 0 ]]; do
        case "$1" in
            --all)
                remove_data_flag=true
                shift
                ;;
            --keep-data)
                remove_data_flag=false
                shift
                ;;
            --backup)
                backup_flag=true
                shift
                ;;
            --dry-run)
                dry_run=true
                shift
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

    if $dry_run; then
        log_info "Dry run - showing what would be removed:"
        echo ""

        local binary
        if binary=$(find_binary); then
            echo "  Binary: $binary"
        fi

        local man_page
        if man_page=$(find_man_page); then
            echo "  Man page: $man_page"
        fi

        local completions
        completions=$(find_completions)
        if [[ -n "$completions" ]]; then
            while IFS= read -r file; do
                echo "  Completion: $file"
            done <<< "$completions"
        fi

        if $remove_data_flag; then
            local config_dir
            config_dir=$(get_config_dir)
            local data_dir
            data_dir=$(get_data_dir)

            if [[ -d "$config_dir" ]]; then
                echo "  Config: $config_dir"
            fi
            if [[ -d "$data_dir" ]]; then
                echo "  Data: $data_dir"
            fi
        fi

        echo ""
        log_info "Run without --dry-run to remove"
        exit 0
    fi

    if $backup_flag && $remove_data_flag; then
        backup_data
    fi

    remove_binary
    remove_man_page
    remove_completions

    if $remove_data_flag; then
        remove_config
        remove_data
    else
        log_info "Config and data preserved (use --all to remove)"
    fi

    print_post_uninstall
}

main "$@"
