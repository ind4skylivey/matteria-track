# Themes Guide

MatteriaTrack features 6 elemental Materia themes inspired by Final Fantasy.

## Available Themes

### Fire 🔥

*"Burning passion and fierce determination"*

```toml
[ui]
theme = "fire"
```

| Element | Color |
|---------|-------|
| Primary | #FF6B6B (Coral Red) |
| Secondary | #FFA500 (Orange) |
| Accent | #FFD700 (Gold) |
| Background | #1A0A0A (Dark Red) |

Best for: High energy tracking sessions

---

### Ice ❄️

*"Cool precision and crystalline clarity"*

```toml
[ui]
theme = "ice"
```

| Element | Color |
|---------|-------|
| Primary | #4ECDC4 (Teal) |
| Secondary | #45B7D1 (Sky Blue) |
| Accent | #96F2D7 (Mint) |
| Background | #0A1A1A (Dark Teal) |

Best for: Focused deep work

---

### Lightning ⚡

*"Electric energy and swift action"*

```toml
[ui]
theme = "lightning"
```

| Element | Color |
|---------|-------|
| Primary | #FFE66D (Electric Yellow) |
| Secondary | #9B59B6 (Purple) |
| Accent | #F39C12 (Amber) |
| Background | #1A1A0A (Dark Yellow) |

Best for: Quick tasks and sprints

---

### Earth 🌍

*"Grounded strength and natural growth"*

```toml
[ui]
theme = "earth"
```

| Element | Color |
|---------|-------|
| Primary | #27AE60 (Forest Green) |
| Secondary | #8B4513 (Saddle Brown) |
| Accent | #F4D03F (Sunflower) |
| Background | #0A1A0A (Dark Green) |

Best for: Long-term projects

---

### Wind 💨

*"Ethereal freedom and endless possibility"*

```toml
[ui]
theme = "wind"
```

| Element | Color |
|---------|-------|
| Primary | #ECF0F1 (Cloud White) |
| Secondary | #BDC3C7 (Silver) |
| Accent | #95A5A6 (Concrete) |
| Background | #1A1A1A (Dark Gray) |

Best for: Creative brainstorming

---

### Bahamut 🐉

*"Ultimate power and legendary mastery"*

```toml
[ui]
theme = "bahamut"
```

| Element | Color |
|---------|-------|
| Primary | #6C3483 (Royal Purple) |
| Secondary | #1A5276 (Dark Blue) |
| Accent | #F4D03F (Gold) |
| Background | #0A0A1A (Deep Purple) |

Best for: Milestone achievements

---

## Theme Icons

Each theme has unique icons:

| Theme | Materia | Project | Task | Time | Complete |
|-------|---------|---------|------|------|----------|
| Fire | 🔥 | 🏔️ | ⚔️ | ⏰ | ✓ |
| Ice | ❄️ | 🏔️ | 🗡️ | ⏱️ | ✓ |
| Lightning | ⚡ | 🌩️ | ⚔️ | ⏰ | ✓ |
| Earth | 🌍 | 🌲 | 🪓 | 🕐 | ✓ |
| Wind | 💨 | ☁️ | 🪶 | ⏱️ | ✓ |
| Bahamut | 🐉 | 👑 | ⚔️ | ⏰ | ✓ |

## Preview Theme

```bash
# Preview current theme
mtrack theme preview

# Preview specific theme
mtrack theme preview --theme ice
```

Output:
```
╔══════════════════════════════════════╗
║  ❄️ Ice Materia Theme                ║
╠══════════════════════════════════════╣
║  Primary:    #4ECDC4 ████            ║
║  Secondary:  #45B7D1 ████            ║
║  Accent:     #96F2D7 ████            ║
╠══════════════════════════════════════╣
║  🏔️ Project Name                     ║
║    🗡️ Task Name          02:30      ║
║  ✓ Completed                         ║
╚══════════════════════════════════════╝
```

## List Themes

```bash
mtrack theme list
```

## Changing Theme

```bash
# Edit config
mtrack config edit

# Or directly
echo '[ui]
theme = "bahamut"' >> ~/.config/materiatrack/config.toml
```

## Custom Themes (Coming Soon)

Future versions will support custom theme definitions:

```toml
[themes.custom]
name = "Chocobo"
primary = "#FFD700"
secondary = "#FFA500"
accent = "#FFFF00"
background = "#1A1500"
icon_materia = "🐤"
icon_project = "🥕"
```

---

*"Choose your Materia wisely, warrior"*
