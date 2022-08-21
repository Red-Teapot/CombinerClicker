# KPACUBO - A Bevy Template for Game Jams

> **NOTE:** This project is still WIP. It is usable but far from perfect.

This project aims to provide a simple yet useful template for
making Bevy games targeted at web and desktop, simplifying building the 
release ZIPs and uploading them to [Itch.io](https://itch.io/), 
and somewhat improving the user experience on the web build.

## Features

- Packs the executable and assets in ready to go ZIP files
- Targets Web and Windows
- Wasm target: built-in canvas resize handler
- Wasm target: shows a simple progress bar while loading the Wasm module 
- Supports uploading the builds on Itch.io using [Butler](https://itchio.itch.io/butler) (requires additional setup)
- Somewhat opinionated: configured to use [bevy-kira-audio](https://crates.io/crates/bevy_kira_audio) out of the box.

## Setup & Usage

### Basic

This enables only building the ZIP files.

- Use this repo as a template
- Install [cargo-make](https://sagiegurari.github.io/cargo-make/)
- ???
- PROFIT!

> **NOTE:** The makefile uses [wasm-pack](https://rustwasm.github.io/wasm-pack/) for, 
> well, building the Wasm module. It should be installed automatically when running the first build.

After everything is set up, run `cargo make zip` to build the ZIP archives.
These will be located in the freshly-created `release` folder.

If you want only the Web or Windows build, use `cargo make zip-web` 
or `cargo make zip-windows` correspondingly.

### Itch.io Upload

- [Install and configure Butler](https://itch.io/docs/butler/).
- Copy/rename `butler.env.example` to `butler.env`
- Add necessary values in `butler.env`:
    - `BUTLER_EXE` should point to the Butler executable
    - `ITCH_USER` should be set to your Itch.io username
    - `ITCH_GAME` should be set to the game name as it is written in the URL.
      For example, if the game URL is `https://example.itch.io/example-game`, 
      then `ITCH_NAME` should be `example-game`.

To build and upload your game, run `cargo make publish`. This will build
the game for both Web and Windows and upload it to Itch.io via Butler.
Don't forget to make the game project first in the dashboard.

If you want to upload only the Web or Windows version, use 
`cargo make publish-web` or `cargo make publish-windows` correspondingly.

### Code

Add the code to set up your Bevy application to `src/game.rs`.

### Assets

The Makefile is configured to package files of specific types.
By default, these are `png`, `wav`, `ogg`, and `ttf`.
If you need to package other file types, 
modify the `collect-assets` task in `Makefile.toml`.

## License

KPACUBO (this template) is free and open source! All code in this repository 
is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option. This means you can select the license you prefer! 
This dual-licensing approach is the de-facto standard in the Rust 
ecosystem and there are [very good reasons](https://github.com/bevyengine/bevy/issues/2373) 
to include both.

Unless you explicitly state otherwise, any contribution intentionally 
submitted for inclusion _in the template_ by you, as defined in the Apache-2.0 
license, shall be dual licensed as above, without any additional terms 
or conditions.
