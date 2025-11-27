# Achievements Guide

Unlock achievements as you master your time tracking journey.

## Viewing Achievements

```bash
mtrack achievements
```

## Achievement List

### Getting Started

| Achievement | Description | Points | Condition |
|-------------|-------------|--------|-----------|
| 💎 **Materia Equipped** | Create your first tracked entry | 10 | Track 1 entry |
| 🏆 **Project Pioneer** | Create your first project | 10 | Create 1 project |
| ⚔️ **Task Tactician** | Create your first task | 10 | Create 1 task |

### Time Milestones

| Achievement | Description | Points | Condition |
|-------------|-------------|--------|-----------|
| ⏰ **Time Keeper** | Track 10 hours total | 25 | 10 hours tracked |
| 🕐 **Century Club** | Track 100 hours total | 50 | 100 hours tracked |
| ⚡ **Limit Break** | Track 500 hours total | 100 | 500 hours tracked |
| 🌟 **Meteor Master** | Track 1000 hours total | 200 | 1000 hours tracked |

### Consistency

| Achievement | Description | Points | Condition |
|-------------|-------------|--------|-----------|
| 📅 **Week Warrior** | Track for 7 consecutive days | 50 | 7 day streak |
| 📆 **Month Master** | Track for 30 consecutive days | 100 | 30 day streak |
| 🏅 **SOLDIER 1st Class** | Track for 100 consecutive days | 200 | 100 day streak |

### Productivity

| Achievement | Description | Points | Condition |
|-------------|-------------|--------|-----------|
| 🎯 **Focus Fighter** | Track 8+ hours in a single day | 50 | 8 hour day |
| 💪 **Marathon Runner** | Track a single session for 4+ hours | 75 | 4 hour session |
| 🌅 **Early Bird** | Start tracking before 6 AM | 25 | Track before 6:00 |
| 🌙 **Midnight Oil** | Track after midnight | 25 | Track 00:00-04:00 |

### Mastery

| Achievement | Description | Points | Condition |
|-------------|-------------|--------|-----------|
| 📊 **Data Miner** | Export your tracking data | 25 | Use export command |
| 🔒 **Fort Condor** | Enable encryption for your data | 50 | Enable GPG encryption |
| 🎨 **Summoner** | Try all 6 Materia themes | 50 | Use each theme once |

### Secret Achievements

*Hidden until unlocked!*

| Achievement | Hint | Points |
|-------------|------|--------|
| 🎮 **???** | *Classic input...* | 100 |
| ⚔️ **???** | *The ultimate technique...* | 150 |
| 🌸 **???** | *Remember her...* | 50 |
| 🐤 **???** | *Kweh!* | 25 |
| 🎵 **???** | *Victory fanfare!* | 25 |

## Easter Eggs

MatteriaTrack includes several hidden easter eggs:

### Konami Code

Try entering a classic sequence...

```bash
# Hint: ↑ ↑ ↓ ↓ ← → ← → B A
```

### Special Commands

```bash
mtrack omnislash    # ???
mtrack aerith       # ???
mtrack chocobo      # ???
```

## Progress Tracking

Your achievement progress is saved in:
```
~/.config/materiatrack/achievements.json
```

### Progress Data

```json
{
  "unlocked": ["materia_equipped", "project_pioneer"],
  "total_points": 20,
  "stats": {
    "total_hours": 45.5,
    "longest_streak": 12,
    "current_streak": 5,
    "total_entries": 156
  }
}
```

## Points Leaderboard

| Rank | Points | Title |
|------|--------|-------|
| Bronze | 0-99 | Apprentice |
| Silver | 100-299 | Tracker |
| Gold | 300-599 | Time Mage |
| Platinum | 600-999 | SOLDIER |
| Diamond | 1000+ | Legendary |

## Notifications

Enable achievement notifications:

```toml
[notifications]
enable = true
```

You'll receive a notification when you unlock an achievement!

## Tips for Unlocking

### Week Warrior (7 day streak)

- Track at least one entry every day
- Use reminders to stay consistent
- Even a 5-minute entry counts!

### Limit Break (500 hours)

- Set daily tracking goals
- Use auto-import for git commits
- Consistent daily tracking adds up

### SOLDIER 1st Class (100 day streak)

- Make tracking a habit
- Set a daily reminder
- Track even on weekends/holidays

### Summoner (All themes)

```bash
# Quick way to try all themes
for theme in fire ice lightning earth wind bahamut; do
    mtrack config set ui.theme $theme
    mtrack status
done
```

## Achievement Ideas (Planned)

- **Chocobo Farmer** - Track 50 entries with "break" in task name
- **Gil Getter** - Track billable time worth 10,000+ (with hourly rate)
- **Phoenix Down** - Recover from a 7+ day tracking gap
- **Knights of the Round** - Track 12 different projects

---

*"Master your time, unlock your potential"*
