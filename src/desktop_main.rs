#![windows_subsystem="windows"]

use bevy::prelude::App;

fn main() {
    let mut app = App::new();
    one_clicker::run(&mut app);
}
