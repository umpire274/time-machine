# Time Machine

**Time Machine** is a narrative-driven text adventure written in Rust, inspired by classic interactive fiction and
modern time-travel storytelling.

The player assumes the role of a Temporal Operative working for the **Time Machine Organization (TMO)**, a private
research institution dedicated to recovering fragments of a lost temporal artifact scattered across human history.

---

## Project Status

🚧 **Early Alpha**  
Current version: **0.2.0-alpha1**

This project is under active development.  
Gameplay mechanics are still being implemented.

---

## Features

- Text-based interface with typewriter-style output
- Atmospheric splash screen inspired by classic terminal software
- Modular game engine architecture
- Narrative-driven game flow
- Base TMO introduction and briefing
- Mission selection system
- Structured mission model for future expansion

---

## Gameplay Overview

1. The game starts with a textual splash screen.
2. The player is introduced to the Time Machine Organization.
3. Using text commands, the player can:
    - read the background lore
    - start the game session
    - select an available temporal mission
4. Each mission represents a different historical period.

Currently, only mission briefings are implemented.

---

## Available Commands

| Command       | Description                          |
|---------------|--------------------------------------|
| `help`        | Display the game background and lore |
| `start`       | Initialize the TMO session           |
| `mission`     | List available temporal missions     |
| `mission <n>` | Select a mission by number           |
| `quit`        | Exit the application                 |

---

## Architecture Overview

The project is structured to clearly separate responsibilities:

```text
src/
├─ engine/     // game state and core loop
├─ mission/    // mission definitions and models
├─ player/     // player data
├─ world/      // epochs and world concepts
└─ ui/         // terminal output and narrative UI
```

This structure is designed to scale as the game evolves.

---

## Roadmap

- Core engine skeleton
- Narrative splash screen 
- Base TMO introduction 
- Mission selection 
- Multi-language support 
- Externalized text resources 
- Time jump and in-mission gameplay 
- Save / load system

---

## License

MIT License