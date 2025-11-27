#!/bin/bash
# ============================================================================
# MateriaTrack DWM Statusbar Script
# ============================================================================
# Displays current tracking status in DWM statusbar using xsetroot
# 
# Usage: ./dwm-statusbar.sh [options]
# Options:
#   -i, --interval SECONDS   Update interval (default: 1)
#   -s, --short              Use short format
#   -t, --theme THEME        Materia theme (fire|ice|lightning|earth|wind)
#   -c, --config PATH        Config file path
#   -o, --once               Run once and exit
#   -h, --help               Show this help
#
# Installation:
#   1. Copy to ~/.local/bin/ or /usr/local/bin/
#   2. Make executable: chmod +x dwm-statusbar.sh
#   3. Add to .xinitrc or autostart: ~/scripts/dwm-statusbar.sh &
# ============================================================================

set -euo pipefail

MTRACK_BIN="${MTRACK_BIN:-materiatrack}"
INTERVAL=1
SHORT_FORMAT=""
THEME=""
CONFIG=""
ONCE=false
IDLE_MSG="💎 idle"

show_help() {
    head -25 "$0" | tail -20 | sed 's/^# //' | sed 's/^#//'
    exit 0
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            -i|--interval)
                INTERVAL="$2"
                shift 2
                ;;
            -s|--short)
                SHORT_FORMAT="--short"
                shift
                ;;
            -t|--theme)
                THEME="$2"
                shift 2
                ;;
            -c|--config)
                CONFIG="$2"
                shift 2
                ;;
            -o|--once)
                ONCE=true
                shift
                ;;
            -h|--help)
                show_help
                ;;
            *)
                echo "Unknown option: $1" >&2
                exit 1
                ;;
        esac
    done
}

check_dependencies() {
    if ! command -v "$MTRACK_BIN" &>/dev/null; then
        echo "Error: $MTRACK_BIN not found in PATH" >&2
        echo "Install MateriaTrack or set MTRACK_BIN environment variable" >&2
        exit 1
    fi

    if ! command -v xsetroot &>/dev/null; then
        echo "Warning: xsetroot not found, output will be printed to stdout" >&2
        USE_XSETROOT=false
    else
        USE_XSETROOT=true
    fi
}

build_command() {
    local cmd="$MTRACK_BIN statusbar"
    
    [[ -n "$SHORT_FORMAT" ]] && cmd="$cmd $SHORT_FORMAT"
    [[ -n "$CONFIG" ]] && cmd="$cmd --config '$CONFIG'"
    
    echo "$cmd"
}

get_status() {
    local cmd
    cmd=$(build_command)
    
    local status
    status=$(eval "$cmd" 2>/dev/null) || status=""
    
    if [[ -z "$status" ]]; then
        echo "$IDLE_MSG"
    else
        echo "$status"
    fi
}

update_statusbar() {
    local status
    status=$(get_status)
    
    if [[ "$USE_XSETROOT" == true ]]; then
        xsetroot -name "$status"
    else
        echo "$status"
    fi
}

cleanup() {
    xsetroot -name "" 2>/dev/null || true
    exit 0
}

main() {
    parse_args "$@"
    check_dependencies
    
    trap cleanup SIGINT SIGTERM
    
    if [[ "$ONCE" == true ]]; then
        update_statusbar
        exit 0
    fi
    
    while true; do
        update_statusbar
        sleep "$INTERVAL"
    done
}

main "$@"
