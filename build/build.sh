#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
VERSION="${VERSION:-$(grep '^version' "$PROJECT_DIR/Cargo.toml" | head -1 | cut -d'"' -f2)}"
RELEASE_DIR="$PROJECT_DIR/release"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-unknown-linux-musl"
    "aarch64-unknown-linux-gnu"
)

MACOS_TARGETS=(
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
)

BSD_TARGETS=(
    "x86_64-unknown-freebsd"
)

log_info() { echo -e "${BLUE}[INFO]${NC} $*"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }

print_banner() {
    echo -e "${BLUE}"
    cat << 'EOF'
╔══════════════════════════════════════════════════════════════╗
║  💎 MateriaTrack Build System                                ║
║     "Forging binaries in Mako Energy"                        ║
╚══════════════════════════════════════════════════════════════╝
EOF
    echo -e "${NC}"
}

check_dependencies() {
    log_info "Checking build dependencies..."

    local missing=()

    if ! command -v cargo &>/dev/null; then
        missing+=("cargo (Rust toolchain)")
    fi

    if ! command -v strip &>/dev/null; then
        missing+=("strip (binutils)")
    fi

    if ! command -v sha256sum &>/dev/null && ! command -v shasum &>/dev/null; then
        missing+=("sha256sum or shasum")
    fi

    if ! command -v tar &>/dev/null; then
        missing+=("tar")
    fi

    if [[ ${#missing[@]} -gt 0 ]]; then
        log_error "Missing dependencies:"
        for dep in "${missing[@]}"; do
            echo "  - $dep"
        done
        exit 1
    fi

    log_success "All dependencies available"
}

setup_release_dir() {
    log_info "Setting up release directory..."
    rm -rf "$RELEASE_DIR"
    mkdir -p "$RELEASE_DIR"
}

install_target() {
    local target="$1"
    if ! rustup target list --installed | grep -q "^$target$"; then
        log_info "Installing target: $target"
        rustup target add "$target" || {
            log_warn "Could not install target $target (may require cross-compilation setup)"
            return 1
        }
    fi
    return 0
}

build_target() {
    local target="$1"
    local output_name="materiatrack"

    log_info "Building for $target..."

    if ! install_target "$target"; then
        log_warn "Skipping $target (target not available)"
        return 1
    fi

    cd "$PROJECT_DIR"

    if cargo build --release --target "$target" 2>/dev/null; then
        local binary_path="$PROJECT_DIR/target/$target/release/$output_name"

        if [[ -f "$binary_path" ]]; then
            strip "$binary_path" 2>/dev/null || true

            local archive_name="materiatrack-${VERSION}-${target}"
            local archive_dir="$RELEASE_DIR/$archive_name"

            mkdir -p "$archive_dir"
            cp "$binary_path" "$archive_dir/materiatrack"
            cp "$PROJECT_DIR/README.md" "$archive_dir/" 2>/dev/null || true
            cp "$PROJECT_DIR/LICENSE" "$archive_dir/" 2>/dev/null || true
            cp -r "$PROJECT_DIR/man" "$archive_dir/" 2>/dev/null || true
            cp -r "$PROJECT_DIR/completions" "$archive_dir/" 2>/dev/null || true

            cd "$RELEASE_DIR"
            tar -czvf "${archive_name}.tar.gz" "$archive_name"
            rm -rf "$archive_dir"

            log_success "Built $target -> ${archive_name}.tar.gz"
            return 0
        fi
    fi

    log_warn "Failed to build for $target"
    return 1
}

generate_checksums() {
    log_info "Generating checksums..."

    cd "$RELEASE_DIR"

    if command -v sha256sum &>/dev/null; then
        sha256sum *.tar.gz > SHA256SUMS
    elif command -v shasum &>/dev/null; then
        shasum -a 256 *.tar.gz > SHA256SUMS
    fi

    log_success "Checksums generated: SHA256SUMS"
}

build_native() {
    log_info "Building native release..."

    cd "$PROJECT_DIR"
    cargo build --release

    local binary="$PROJECT_DIR/target/release/materiatrack"
    if [[ -f "$binary" ]]; then
        strip "$binary" 2>/dev/null || true

        local size=$(du -h "$binary" | cut -f1)
        log_success "Native build complete: $binary ($size)"
    fi
}

build_all() {
    local successful=0
    local failed=0

    for target in "${TARGETS[@]}"; do
        if build_target "$target"; then
            ((successful++))
        else
            ((failed++))
        fi
    done

    if [[ "$(uname)" == "Darwin" ]]; then
        for target in "${MACOS_TARGETS[@]}"; do
            if build_target "$target"; then
                ((successful++))
            else
                ((failed++))
            fi
        done
    fi

    echo ""
    log_info "Build summary: $successful successful, $failed failed"
}

show_help() {
    cat << EOF
MateriaTrack Build System

USAGE:
    $0 [COMMAND] [OPTIONS]

COMMANDS:
    native      Build for current platform only (default)
    all         Build for all supported targets
    linux       Build for Linux targets only
    target      Build for specific target
    clean       Clean build artifacts
    help        Show this help

OPTIONS:
    --version VERSION    Override version string

EXAMPLES:
    $0                   # Native build
    $0 all               # Cross-compile all targets
    $0 target x86_64-unknown-linux-musl
    $0 --version 1.0.0 all

SUPPORTED TARGETS:
    Linux:
        x86_64-unknown-linux-gnu
        x86_64-unknown-linux-musl
        aarch64-unknown-linux-gnu

    macOS (requires macOS host):
        x86_64-apple-darwin
        aarch64-apple-darwin

    BSD:
        x86_64-unknown-freebsd
EOF
}

main() {
    print_banner

    local command="${1:-native}"

    case "$command" in
        native)
            check_dependencies
            build_native
            ;;
        all)
            check_dependencies
            setup_release_dir
            build_all
            generate_checksums
            echo ""
            log_success "Release artifacts in: $RELEASE_DIR"
            ls -lh "$RELEASE_DIR"
            ;;
        linux)
            check_dependencies
            setup_release_dir
            for target in "${TARGETS[@]}"; do
                build_target "$target" || true
            done
            generate_checksums
            ;;
        target)
            if [[ -z "${2:-}" ]]; then
                log_error "Please specify a target"
                exit 1
            fi
            check_dependencies
            setup_release_dir
            build_target "$2"
            generate_checksums
            ;;
        clean)
            log_info "Cleaning build artifacts..."
            cd "$PROJECT_DIR"
            cargo clean
            rm -rf "$RELEASE_DIR"
            log_success "Clean complete"
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            log_error "Unknown command: $command"
            show_help
            exit 1
            ;;
    esac
}

main "$@"
