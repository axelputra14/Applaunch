# Applaunch (Temporary name)

This is a personal project using Tauri + VueJS with SQLite DB and connected to Applaunch_be now as a temporary backend solution. The app is an application launcher similar to Playnite or Windows shortcut but much simpler and more catered for personal use.

## Roadmap

- Finish the frontend.
  - How to design detail page?
  - 3 or 4 per row?
  - Bg sometimes not 100% height.
  - Modal component for error messages similar to swal.
- Figuring out the host for images.
  - Figure out the link stored in db so only image name is saved without it's prefix link of host.
- Convert the backend from Node to Rust (low priority).

## Why this tech stack?

- Tauri: Enabling front-end websites to run like a native OS application, and it's lighter than other similar tech stacks.
- VueJS: Most familiarity compared to other front-end JS.
- NodeJS: Temporary solution for backend due to never learned Rust before.
