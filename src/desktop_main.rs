#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    one_clicker::run(&mut app);
}
