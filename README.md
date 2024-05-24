# Applaunch (Temporary name)

This is a personal project using Tauri + VueJS with SQLite DB and connected to Applaunch_be now as a temporary backend solution. The app is an application launcher similar to Playnite or Windows shortcut but much simpler and more catered for personal use. Icons library from HeroIcons.

## Roadmap

- Finish the frontend.
  - Dir input can either type manually or use button to select, then copy the file path.
  - Images try to use relative path from assets to eliminate need of extra http-server.
  - Each image has button selector, same to copy the file name but defaulted to each folder that's static.
- Figuring out the host for images.
- Date when edited has Unix postfixes.
- Convert the backend from Node to Rust (low priority).

## Why this tech stack?

- Tauri: Enabling front-end websites to run like a native OS application, and it's lighter than other similar tech stacks.
- VueJS: Most familiarity compared to other front-end JS.
- SQLite: Light database management engine.
- NodeJS+Sequelize: Temporary solution for backend due to never learned Rust before.
