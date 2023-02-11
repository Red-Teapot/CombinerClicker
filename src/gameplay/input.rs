use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    math::vec3,
    prelude::*,
};

use crate::BackgroundInteraction;

const CLICK_DURATION: f64 = 0.2;
const CLICK_DISTANCE: f32 = 10.0;

pub fn handle_bg_input(
    mut world_mouse_state: ResMut<WorldMouseState>,
    time: Res<Time>,
    bg_inter: Query<&Interaction, With<BackgroundInteraction>>,
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

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (cursor_position_window / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world =
            camera_global_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        world_pos
    };

    for interaction in bg_inter.iter() {
        match interaction {
            Interaction::Clicked => {
                if buttons.just_pressed(MouseButton::Left) {
                    *world_mouse_state = WorldMouseState::Pressed {
                        time: time.elapsed_seconds_f64(),
                        position_window: cursor_position_window,
                        position_world: cursor_position_world,
                    };
                }
            }

            Interaction::Hovered => {
                if buttons.just_pressed(MouseButton::Right) {
                    world_mouse_events.send(WorldMouseEvent::RightClick {
                        position: cursor_position_world,
                    });
                }

                match *world_mouse_state {
                    WorldMouseState::None => {
                        world_mouse_events.send(WorldMouseEvent::Hover {
                            position: cursor_position_world,
                        });
                    }

                    _ => {}
                }
            }

            _ => {}
        }
    }

    let convert_drag_offset = |offset: Vec2| -> Vec2 {
        let transform = camera_global_transform
            .compute_transform()
            .with_translation(Vec3::ZERO);

        transform.transform_point(offset.extend(0.0)).truncate()
    };

    match (buttons.just_released(MouseButton::Left), *world_mouse_state) {
        (true, WorldMouseState::Dragging { last_position }) => {
            world_mouse_events.send(WorldMouseEvent::Drag {
                offset: convert_drag_offset(cursor_position_window - last_position),
            });

            *world_mouse_state = WorldMouseState::None;
        }

        (false, WorldMouseState::Dragging { last_position }) => {
            world_mouse_events.send(WorldMouseEvent::Drag {
                offset: convert_drag_offset(cursor_position_window - last_position),
            });

            *world_mouse_state = WorldMouseState::Dragging {
                last_position: cursor_position_window,
            };
        }

        (
            released,
            WorldMouseState::Pressed {
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
                    world_mouse_events.send(WorldMouseEvent::LeftClick {
                        position: position_world,
                    });
                } else {
                    world_mouse_events.send(WorldMouseEvent::Drag {
                        offset: convert_drag_offset(cursor_position_window - position_window),
                    });
                }

                *world_mouse_state = WorldMouseState::None;
            } else if !suitable_for_click {
                world_mouse_events.send(WorldMouseEvent::Drag {
                    offset: convert_drag_offset(cursor_position_window - position_window),
                });

                *world_mouse_state = WorldMouseState::Dragging {
                    last_position: cursor_position_window,
                };
            }
        }

        _ => {}
    }
}

pub fn drag_camera(
    mut camera: Query<&mut Transform, With<Camera2d>>,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
) {
    let mut camera_transform = camera.single_mut();

    for event in world_mouse_events.iter() {
        match event {
            WorldMouseEvent::Drag { offset } => camera_transform.translation -= offset.extend(0.0),

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

#[derive(Copy, Clone, Resource)]
pub enum WorldMouseState {
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

pub enum WorldMouseEvent {
    LeftClick { position: Vec2 },
    RightClick { position: Vec2 },
    Hover { position: Vec2 },
    Drag { offset: Vec2 },
}
