use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    math::vec3,
    prelude::*,
};

use crate::WorldInteraction;

const CLICK_DURATION: f64 = 0.2;
const CLICK_DISTANCE: f32 = 10.0;

pub fn handle_bg_input(
    mut world_mouse: ResMut<WorldMouse>,
    time: Res<Time>,
    bg_inter: Query<&Interaction, With<WorldInteraction>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut world_mouse_events: EventWriter<WorldMouseEvent>,
) {
    let (camera, camera_global_transform) = camera.single();

    let window = if let Some(window) = windows.get_primary() {
        window
    } else {
        return;
    };

    let cursor_position_window = if let Some(position) = window.cursor_position() {
        position
    } else {
        return;
    };

    let cursor_position_world = {
        let window_size = Vec2::new(window.width() as f32, window.height() as f32);

        let ndc = (cursor_position_window / window_size) * 2.0 - Vec2::ONE;

        let ndc_to_world =
            camera_global_transform.compute_matrix() * camera.projection_matrix().inverse();

        ndc_to_world.project_point3(ndc.extend(-1.0)).truncate()
    };

    world_mouse.position_world = cursor_position_world;

    for interaction in bg_inter.iter() {
        match interaction {
            Interaction::Hovered | Interaction::Clicked => {
                let mouse_state = MouseButtonState::Pressed {
                    time: time.elapsed_seconds_f64(),
                    position_window: cursor_position_window,
                    position_world: cursor_position_world,
                };

                if buttons.just_pressed(MouseButton::Left) {
                    world_mouse.button_state_left = mouse_state;
                }

                if buttons.just_pressed(MouseButton::Middle) {
                    world_mouse.button_state_middle = mouse_state;
                }

                if buttons.just_pressed(MouseButton::Right) {
                    world_mouse.button_state_right = mouse_state;
                }

                match world_mouse.state {
                    MouseState::None | MouseState::Hovering => {
                        world_mouse_events.send(WorldMouseEvent::Hover {
                            position: cursor_position_world,
                        });

                        world_mouse.state = MouseState::Hovering;
                    }

                    _ => {}
                }
            }

            Interaction::None => {
                world_mouse.state = MouseState::None;
                world_mouse.button_state_left = MouseButtonState::None;
                world_mouse.button_state_middle = MouseButtonState::None;
                world_mouse.button_state_right = MouseButtonState::None;
            }
        }
    }

    let mouse_states = vec![
        update_mouse_button_state(
            MouseButton::Left,
            &mut world_mouse.button_state_left,
            cursor_position_window,
            camera_global_transform,
            &time,
            &buttons,
            &mut world_mouse_events,
        ),
        update_mouse_button_state(
            MouseButton::Middle,
            &mut world_mouse.button_state_middle,
            cursor_position_window,
            camera_global_transform,
            &time,
            &buttons,
            &mut world_mouse_events,
        ),
        update_mouse_button_state(
            MouseButton::Right,
            &mut world_mouse.button_state_right,
            cursor_position_window,
            camera_global_transform,
            &time,
            &buttons,
            &mut world_mouse_events,
        ),
    ];

    world_mouse.state = *mouse_states.iter().max_by_key(|s| s.priority()).unwrap();
}

fn update_mouse_button_state(
    button: MouseButton,
    button_state: &mut MouseButtonState,
    cursor_position_window: Vec2,
    camera_global_transform: &GlobalTransform,
    time: &Res<Time>,
    buttons: &Input<MouseButton>,
    world_mouse_events: &mut EventWriter<WorldMouseEvent>,
) -> MouseState {
    let camera_transform = camera_global_transform
        .compute_transform()
        .with_translation(Vec3::ZERO);

    let convert_drag_offset = |offset: Vec2| -> Vec2 {
        camera_transform
            .transform_point(offset.extend(0.0))
            .truncate()
    };

    match (buttons.just_released(button), *button_state) {
        (true, MouseButtonState::Dragging { last_position }) => {
            world_mouse_events.send(WorldMouseEvent::Drag {
                button,
                offset: convert_drag_offset(cursor_position_window - last_position),
            });

            *button_state = MouseButtonState::None;

            MouseState::None
        }

        (false, MouseButtonState::Dragging { last_position }) => {
            world_mouse_events.send(WorldMouseEvent::Drag {
                button,
                offset: convert_drag_offset(cursor_position_window - last_position),
            });

            *button_state = MouseButtonState::Dragging {
                last_position: cursor_position_window,
            };

            MouseState::Dragging
        }

        (
            released,
            MouseButtonState::Pressed {
                time: press_time,
                position_world,
                position_window,
            },
        ) => {
            let press_duration = time.elapsed_seconds_f64() - press_time;
            let press_distance = cursor_position_window.distance(position_window);
            let suitable_for_click =
                press_duration < CLICK_DURATION && press_distance < CLICK_DISTANCE;

            if released {
                if suitable_for_click {
                    world_mouse_events.send(WorldMouseEvent::Click {
                        button,
                        position: position_world,
                    });
                } else {
                    world_mouse_events.send(WorldMouseEvent::Drag {
                        button,
                        offset: convert_drag_offset(cursor_position_window - position_window),
                    });
                }

                *button_state = MouseButtonState::None;

                MouseState::None
            } else if !suitable_for_click {
                world_mouse_events.send(WorldMouseEvent::Drag {
                    button,
                    offset: convert_drag_offset(cursor_position_window - position_window),
                });

                *button_state = MouseButtonState::Dragging {
                    last_position: cursor_position_window,
                };

                MouseState::Dragging
            } else {
                MouseState::None
            }
        }

        _ => MouseState::None,
    }
}

pub fn drag_camera(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
) {
    let mut camera_transform = camera.single_mut();

    for event in world_mouse_events.iter() {
        match event {
            WorldMouseEvent::Drag {
                button: MouseButton::Middle,
                offset,
            } => camera_transform.translation -= offset.extend(0.0),

            _ => (),
        }
    }
}

pub fn zoom_camera(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut scroll_events: EventReader<MouseWheel>,
) {
    let mut camera_transform = camera.single_mut();

    let mut scroll = 1.0f32;

    for event in scroll_events.iter() {
        match event.unit {
            MouseScrollUnit::Line => scroll -= event.y * 0.2,
            MouseScrollUnit::Pixel => scroll -= (event.y / 100.0) * 0.2,
        }
    }

    let scale_multiplier = vec3(scroll, scroll, 1.0);
    camera_transform.scale = (camera_transform.scale * scale_multiplier)
        .clamp(vec3(1.0, 1.0, 1.0), vec3(20.0, 20.0, 1.0));
}

#[derive(Resource, Copy, Clone, Debug)]
pub struct WorldMouse {
    pub position_world: Vec2,
    pub state: MouseState,
    pub button_state_left: MouseButtonState,
    pub button_state_middle: MouseButtonState,
    pub button_state_right: MouseButtonState,
}

#[derive(Copy, Clone, Debug)]
pub enum MouseState {
    None,
    Hovering,
    Dragging,
}

impl MouseState {
    pub fn priority(&self) -> u32 {
        use MouseState::*;

        match self {
            None => 0,
            Hovering => 1,
            Dragging => 2,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum MouseButtonState {
    None,
    Pressed {
        time: f64,
        position_window: Vec2,
        position_world: Vec2,
    },
    Dragging {
        last_position: Vec2,
    },
}

impl Default for WorldMouse {
    fn default() -> Self {
        WorldMouse {
            position_world: Vec2::ZERO,
            state: MouseState::None,
            button_state_left: MouseButtonState::None,
            button_state_middle: MouseButtonState::None,
            button_state_right: MouseButtonState::None,
        }
    }
}

#[derive(Debug)]
pub enum WorldMouseEvent {
    Click { button: MouseButton, position: Vec2 },

    Hover { position: Vec2 },

    Drag { button: MouseButton, offset: Vec2 },
}
