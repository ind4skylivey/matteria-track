# MatteriaTrack User Guide

This guide covers the core workflows and advanced features of MatteriaTrack.

## ⚔️ Core Workflow: Tracking Time

### 1. Start Tracking
Use the `track` command (alias `t`) to begin a session.
```bash
mtrack track -p "ProjectName" -t "TaskName"
```
*   **Projects & Tasks**: If the project or task doesn't exist, MatteriaTrack will prompt you to create it (or create it automatically depending on config).
*   **Notes**: Add descriptions with `-n` or `--notes`.
    ```bash
    mtrack t -p "Matteria" -t "Docs" -n "Writing the user guide"
    ```
*   **Time Adjustment**: Use `--begin` to start in the past.
    ```bash
    mtrack t -p "Matteria" -t "Docs" --begin "-15m"  # Started 15 mins ago
    mtrack t -p "Matteria" -t "Docs" --begin "14:00" # Started at 14:00 today
    ```

### 2. Check Status
See what you are currently working on.
```bash
mtrack status
```
*   Shows project, task, duration, and start time.
*   Displays the relevant elemental icon for your theme.

### 3. Finish Tracking
Stop the current timer.
```bash
mtrack finish
```
*   **Adjust End Time**: Use `--end` if you finished earlier.
    ```bash
    mtrack f --end "-10m"
    ```
*   **Add Closing Notes**:
    ```bash
    mtrack f -n "Completed draft"
    ```

---

## 🏆 Project Management

### Creating Projects
Projects help organize your tasks.
```bash
mtrack project add "ClientWork" --color "#3498db"
```

### Creating Tasks
Tasks belong to projects.
```bash
mtrack task add "Website Redesign" -p "ClientWork"
```

### Git Integration
Link a task to a Git repository to enable auto-importing of commits.
```bash
mtrack task update "Website Redesign" -p "ClientWork" --git-repo "~/projects/client-website"
```

---

## 📊 Reporting & Analysis

### Daily Stats
See where your time went today.
```bash
mtrack stats --today
```

### Weekly Overview
Group by project to see the breakdown.
```bash
mtrack stats --week --by-project
```

### Listing Entries
View raw entries for export or review.
```bash
mtrack list --since "2023-01-01" --limit 100
```

---

## 🖥️ Interactive Dashboard

For a visual overview, launch the TUI (Text User Interface).
```bash
mtrack dashboard
```
*   Navigate with arrow keys.
*   View recent activity, stats, and achievements.
*   Press `q` to exit.

---

## 🔄 Integrations

### Obsidian Sync
Sync your daily summaries to your Obsidian Daily Notes.
1.  Set your vault path in `config.toml`:
    ```toml
    [integrations]
    obsidian_path = "~/Documents/Obsidian/Vault"
    ```
2.  MatteriaTrack will append a summary table to today's note when you finish tasks.

### Statusbar (Polybar/DWM/Waybar)
Display your active task in your system bar.
*   **Polybar**: Add a script module executing `mtrack statusbar --format polybar`.
*   **Waybar**: Add a custom module executing `mtrack statusbar --format waybar`.

---

## 🔐 Security

### GPG Encryption
Protect your sensitive time data.
1.  Ensure you have GPG installed and a key pair generated.
2.  Enable in `config.toml`:
    ```toml
    [security]
    enable_encryption = true
    encryption_key = "YOUR_KEY_ID"
    ```
3.  Your database will be encrypted at rest.

### Audit Log
Changes are logged to `validation_log.md` (if enabled) to ensure data integrity and provide a paper trail for edits.
