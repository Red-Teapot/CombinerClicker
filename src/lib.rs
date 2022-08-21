#[cfg(target_arch = "wasm32")]
mod web_main;

mod game;

pub use game::*;
