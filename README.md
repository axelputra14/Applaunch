# Applaunch

This is a personal project using Tauri + VueJS with SQLite DB and connected to Applaunch_be now as a temporary backend solution. The app is an application launcher similar to Playnite or Windows shortcut but much simpler than Playnite and more catered for personal use. Icons library from HeroIcons. See it as simpler than Playnite but more convoluted than Flow Launcher or Microsft Powertools.

## Tech stack

- Tauri: Enabling front-end websites to run like a native OS application, and it's lighter than other similar tech stacks.
- VueJS: Most familiarity compared to other front-end JS.
- SQLite: Light database management engine.
- NodeJS+Sequelize+ExpressJS: Backend API

## The quirk

Needs separate backend at port 16850 and http-server at port 25850 to function since I failed to implement Rust backend and I don't want to sideload node into Tauri.
