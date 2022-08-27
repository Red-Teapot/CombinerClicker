#[cfg(target_arch = "wasm32")]
mod web_main;

mod game;
pub mod palette;
pub mod assets;
pub mod title;

pub use game::*;
