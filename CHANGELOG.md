# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog,
and this project follows Semantic Versioning.

---

## [0.2.0-alpha1] – YYYY-MM-DD

### Added
- Initial narrative-driven game loop
- Text-based splash screen with typewriter-style output
- Centralized terminal output system
- Base TMO (Time Machine Organization) introduction
- `help` command displaying the game background lore
- `start` command to initialize the game session
- Mission selection system
- Mission model based on a dedicated enum
- First playable mission briefing (Ancient Egypt)

### Changed
- Refactored UI into dedicated modules:
    - terminal (low-level I/O)
    - splash screen
    - help / lore
    - TMO and mission briefings
- Improved separation between engine logic and UI rendering

### Notes
- This is an early alpha release.
- Gameplay is limited to narrative flow and mission selection.
- No actual time jump or in-mission gameplay is implemented yet.
