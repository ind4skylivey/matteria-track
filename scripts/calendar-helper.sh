#!/bin/bash
# MatteriaTrack Calendar Helper Script
# Quick shortcuts for common calendar operations

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored message
print_msg() {
    local color=$1
    shift
    echo -e "${color}$@${NC}"
}

# Function to show usage
show_usage() {
    cat << EOF
${PURPLE}
╔══════════════════════════════════════════════════════════════╗
║  💎 MatteriaTrack Calendar Helper - Quick Commands 💎        ║
╚══════════════════════════════════════════════════════════════╝
${NC}

${CYAN}Usage:${NC}
  $0 [command] [options]

${CYAN}Commands:${NC}
  ${GREEN}open [theme]${NC}              - Open calendar (optional theme)
  ${GREEN}add <title> [date]${NC}        - Add event (date format: YYYY-MM-DD)
  ${GREEN}today${NC}                     - Add event for today
  ${GREEN}tomorrow${NC}                  - Add event for tomorrow
  ${GREEN}list${NC}                      - List all events
  ${GREEN}themes${NC}                    - Show available themes
  ${GREEN}help${NC}                      - Show this help

${CYAN}Examples:${NC}
  $0 open fire                    # Open calendar with fire theme
  $0 add "Team Meeting" 2026-01-20   # Add event for specific date
  $0 today "Daily Standup"           # Add event for today
  $0 tomorrow "Deploy to prod"       # Add event for tomorrow
  $0 list                           # List all events

${CYAN}Themes:${NC}
  🔥 fire      ❄️  ice      ⚡ lightning
  🌍 earth     🌬️  wind     🐉 bahamut

EOF
}

# Function to open calendar
open_calendar() {
    local theme=${1:-}
    if [ -n "$theme" ]; then
        print_msg "$PURPLE" "🎨 Opening calendar with $theme theme..."
        mtrack calendar --theme "$theme"
    else
        print_msg "$PURPLE" "📅 Opening calendar..."
        mtrack calendar
    fi
}

# Function to add event
add_event() {
    local title="$1"
    local date="${2:-$(date +%Y-%m-%d)}"

    if [ -z "$title" ]; then
        print_msg "$RED" "❌ Error: Event title is required"
        exit 1
    fi

    print_msg "$BLUE" "➕ Adding event: $title"
    mtrack calendar --add "$title" --date "$date"
}

# Function to add event for today
add_today() {
    local title="$1"
    if [ -z "$title" ]; then
        print_msg "$RED" "❌ Error: Event title is required"
        exit 1
    fi

    local today=$(date +%Y-%m-%d)
    print_msg "$GREEN" "📅 Adding event for today ($today)..."
    mtrack calendar --add "$title" --date "$today"
}

# Function to add event for tomorrow
add_tomorrow() {
    local title="$1"
    if [ -z "$title" ]; then
        print_msg "$RED" "❌ Error: Event title is required"
        exit 1
    fi

    local tomorrow=$(date -d "tomorrow" +%Y-%m-%d 2>/dev/null || date -v+1d +%Y-%m-%d)
    print_msg "$GREEN" "📅 Adding event for tomorrow ($tomorrow)..."
    mtrack calendar --add "$title" --date "$tomorrow"
}

# Function to list events
list_events() {
    local events_file="$HOME/.config/materiatrack/events.json"

    if [ ! -f "$events_file" ]; then
        print_msg "$YELLOW" "⚠️  No events file found"
        return
    fi

    print_msg "$CYAN" "📋 Calendar Events:"
    echo ""

    if command -v jq &> /dev/null; then
        jq -r '.[] | "  📅 \(.date)  │  \(.title) \(if .time then "(\(.time))" else "" end)"' "$events_file" | sort
    else
        cat "$events_file"
    fi

    echo ""
    local count=$(jq 'length' "$events_file" 2>/dev/null || echo "?")
    print_msg "$PURPLE" "Total: $count events"
}

# Function to show themes
show_themes() {
    cat << EOF
${PURPLE}
╔══════════════════════════════════════════════════════════════╗
║                🎨 Materia Themes Available 🎨                 ║
╚══════════════════════════════════════════════════════════════╝
${NC}

${RED}  🔥 fire       ${NC}- Vibrant red/orange energy
${CYAN}  ❄️  ice        ${NC}- Cool cyan tranquility
${YELLOW}  ⚡ lightning  ${NC}- Bright yellow speed
  🌍 earth      - Natural brown stability
${GREEN}  🌬️  wind       ${NC}- Fresh green freedom
${PURPLE}  🐉 bahamut    ${NC}- Royal purple power

${CYAN}Usage:${NC}
  mtrack calendar --theme <theme_name>

${CYAN}Example:${NC}
  mtrack calendar --theme bahamut

EOF
}

# Main script logic
case "${1:-help}" in
    open)
        open_calendar "${2:-}"
        ;;
    add)
        add_event "$2" "${3:-}"
        ;;
    today)
        add_today "$2"
        ;;
    tomorrow)
        add_tomorrow "$2"
        ;;
    list)
        list_events
        ;;
    themes)
        show_themes
        ;;
    help|-h|--help|"")
        show_usage
        ;;
    *)
        print_msg "$RED" "❌ Unknown command: $1"
        echo ""
        show_usage
        exit 1
        ;;
esac
