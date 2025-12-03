# --- Your existing _sv_git_status function ---
function _sv_git_status
    if not command -v git >/dev/null
        return
    end

    set -l branch (command git symbolic-ref --short HEAD 2>/dev/null)
    if test -z "$branch"
        return
    end

    set -l dirty (command git status --porcelain 2>/dev/null)

    # Git with neon background
    set_color -b ff00ff
    set_color 000000
    echo -n "  $branch "

    if test -n "$dirty"
        set_color ffff00
        echo -n "⚡ "
    end

    set_color -b normal
end

# --- START: MatteriaTrack integration for Iridex Prism ---
# This function fetches and formats the MatteriaTrack status
function __prism_mtrack_status
    # Ensure 'mtrack' command is available before trying to use it
    if type -q mtrack
        # Get the short status from MatteriaTrack, e.g., "Project > Task 00:10"
        # 'command materiatrack' is used to explicitly call the executable.
        # We use the real binary name 'materiatrack' because 'command' ignores aliases like 'mtrack'.
        # '2>/dev/null' redirects any stderr output (e.g., errors if mtrack isn't fully configured)
        # to null, keeping the prompt clean.
        set -l mtrack_output (command materiatrack statusbar --short 2>/dev/null)

        # Only display if the output is not empty AND it does NOT contain "idle"
        # 'string match -qv' means: -q for quiet (no output), -v for inverse match (does NOT contain)
        if test -n "$mtrack_output"; and string match -qv "*idle*" -- "$mtrack_output"
            # --- MATTERIATrack STYLING WITHIN THE PROMPT ---
            # Using a dark magenta background with neon yellow text to match Iridex Prism's style.
            # You can adjust these color codes (e.g., to match your current theme's Materia color)
            set_color -b 8800ff # Background color (e.g., dark magenta)
            set_color ffff00   # Foreground color (neon yellow)
            echo -n " 💎 $mtrack_output " # Materia icon + tracking status
            set_color -b normal # Restore default background to allow subsequent prompt elements to color correctly
        end
    end
end
# --- END: MatteriaTrack integration ---


# --- Your existing fish_prompt function ---
function fish_prompt
    set -l last_status $status

    echo

    # Line 1: Retro Grid with Sunset
    set_color ff00ff
    echo -n "▓▒░ "

    # Sunset emoji on dark bg
    set_color -b 1a0033
    set_color ffff00
    echo -n " 🌆 "
    set_color -b normal

    # User segment (cyan neon)
    set_color -b 00ffff
    set_color 000000
    echo -n " $USER "
    set_color -b normal

    set_color ff00ff
    echo -n " ▸ "

    # Directory (magenta neon)
    set_color -b ff00ff
    set_color 000000
    echo -n " "(prompt_pwd)" "
    set_color -b normal

    # Git status (if present)
    _sv_git_status

    # --- START: Call MatteriaTrack status function ---
    # Call the function to display MatteriaTrack status right after Git status
    __prism_mtrack_status
    # --- END: Call MatteriaTrack status function ---

    # Grid decoration
    set_color ff00ff
    echo -n " ░▒▓"

    echo

    # Line 2: Neon Prompt
    set_color ff00ff
    echo -n "╰─"
    set_color 00ffff
    echo -n "═"
    set_color ffff00
    echo -n "═"
    set_color ff00ff
    echo -n "► "

    # Status indicator based on last command's exit status
    if test $last_status -eq 0
        set_color 00ffff
        echo -n "◆ "
    else
        set_color ff0000
        echo -n "✖ "
    end

    set_color normal
end
