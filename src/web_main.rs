use bevy::prelude::*;
use gloo_events::EventListener;
use std::sync::mpsc::{channel, Receiver};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn web_main() {
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.add_startup_system(setup_resize_handler);
    app.add_system(resize_system);

    crate::run(&mut app);
}

struct CanvasResize {
    #[allow(dead_code)]
    listener: EventListener,
    receiver: Receiver<()>,
}

// Wasm is single-threaded anyways
unsafe impl Send for CanvasResize {}
unsafe impl Sync for CanvasResize {}

/// Setups a resize event listener for the browser window
/// to resize the canvas appropriately.
fn setup_resize_handler(mut commands: Commands) {
    let (sender, receiver) = channel();

    // Resize the canvas when starting just in case
    sender.send(()).unwrap_throw();

    let web_window = web_sys::window().unwrap_throw();
    let listener = EventListener::new(&web_window, "resize", move |_event| {
        sender.send(()).unwrap_throw();
    });

    commands.insert_resource(CanvasResize { listener, receiver });
}

/// Resizes the canvas when the browser window gets resized.
fn resize_system(mut windows: ResMut<Windows>, resize: Res<CanvasResize>) {
    let mut update = false;

    while let Ok(_) = resize.receiver.try_recv() {
        // Coalesce multiple events into one update
        update = true;
    }

    if update {
        let web_window = web_sys::window().unwrap_throw();

        let width = web_window
            .inner_width()
            .unwrap_throw()
            .as_f64()
            .unwrap_throw() as f32;
        let height = web_window
            .inner_height()
            .unwrap_throw()
            .as_f64()
            .unwrap_throw() as f32;

        windows
            .get_primary_mut()
            .unwrap_throw()
            .set_resolution(width, height);
    }
}
