use std::time::Duration;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::prelude::Val::{Px, Undefined};
use bevy::ui::FocusPolicy;
use bevy_ninepatch::{NinePatchBundle, NinePatchData};
use bevy_tweening::*;
use bevy_tweening::lens::TransformPositionLens;
use crate::assets::GameAssets;
use crate::{BackgroundInteraction, palette};
use crate::gameplay::components::*;

const CLICK_DURATION: f64 = 0.2;
const CLICK_DISTANCE: f32 = 10.0;

pub fn startup_gameplay(mut commands: Commands,
                        assets: Res<GameAssets>,
                        mut camera: Query<&mut Transform, With<Camera2d>>) {
    camera.single_mut().scale = vec3(4.0, 4.0, 1.0);

    commands.insert_resource(Money(0));

    commands.insert_resource(WorldMouseState::None);

    commands.insert_resource(CameraDragStart {
        start_position: Vec2::ZERO,
    });

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Stretch,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        color: Color::NONE.into(),
        ..default()
    }).with_children(|window| {
        window.spawn_bundle(NodeBundle {
            style: Style {
                align_items: AlignItems::FlexStart,
                ..default()
            },
            color: Color::NONE.into(),
            focus_policy: FocusPolicy::Pass,
            ..default()
        }).with_children(|top_panel| {
            top_panel.spawn_bundle(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::all(Px(8.0)),
                    ..default()
                },
                color: Color::NONE.into(),
                ..default()
            }).with_children(|money_display| {
                money_display.spawn_bundle(ImageBundle {
                    image: assets.coin.clone().into(),
                    style: Style {
                        size: Size::new(Px(48.0), Px(48.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }).with_children(|coin| {
                    coin.spawn_bundle(TextBundle {
                        text: Text::from_section("1", TextStyle {
                            font: assets.font.clone(),
                            color: palette::DARK_BLUE,
                            font_size: 40.0,
                        }).with_alignment(TextAlignment::CENTER),
                        ..default()
                    });
                }).insert(Name::new("Icon"));

                money_display.spawn_bundle(TextBundle {
                    text: Text::from_section("0", TextStyle {
                        font: assets.font.clone(),
                        color: palette::DARK_BLUE,
                        font_size: 48.0,
                    }),
                    style: Style {
                        margin: UiRect {
                            left: Px(16.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                }).insert(Name::new("Value")).insert(MoneyDisplay);
            }).insert(Name::new("Money Display"));
        }).insert(Name::new("Top Panel"));

        let bottom_panel_content = window.spawn_bundle(NodeBundle {
            style: Style {
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                min_size: Size {
                    height: Px(192.0),
                    ..default()
                },
                padding: UiRect::all(Px(-8.0)),
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            color: Color::NONE.into(),
            ..default()
        }).with_children(|bottom_panel| {
            for machine in Machine::list() {
                bottom_panel.spawn_bundle(ButtonBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        padding: UiRect::all(Px(8.0)),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                }).with_children(|container| {
                    container.spawn_bundle(TextBundle {
                        text: Text::from_section("???", TextStyle {
                            font: assets.font.clone(),
                            color: palette::LIGHT_BROWN,
                            font_size: 20.0,
                        }),
                        style: Style {
                            margin: UiRect {
                                bottom: Px(4.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    }).insert(MachineName(*machine));

                    container.spawn_bundle(ImageBundle {
                        image: assets.locked.clone().into(),
                        style: Style {
                            size: Size::new(Px(64.0), Px(64.0)),
                            ..default()
                        },
                        ..default()
                    }).insert(MachineIcon(*machine));

                    container.spawn_bundle(TextBundle {
                        text: Text::from_section(machine.cost().to_string(), TextStyle {
                            font: assets.font.clone(),
                            color: palette::DARK_BLUE,
                            font_size: 28.0,
                        }),
                        ..default()
                    });
                }).insert(MachineBuyButton {
                    enabled: false,
                    machine: *machine,
                });
            }
        }).insert(Name::new("Bottom Panel Content")).id();

        window.spawn_bundle(NinePatchBundle {
            nine_patch_data: NinePatchData::with_single_content(
                assets.panel.0.clone(),
                assets.panel.1.clone(),
                bottom_panel_content
            ),
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                position: UiRect {
                    bottom: Px(-84.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        }).insert(Name::new("Bottom Panel"));
    });
}

pub struct CameraDragStart {
    start_position: Vec2,
}

pub fn handle_bg_input(mut commands: Commands,
                       assets: Res<GameAssets>,
                       mut world_mouse_state: ResMut<WorldMouseState>,
                       time: Res<Time>,
                       bg_inter: Query<&Interaction, With<BackgroundInteraction>>,
                       mut camera: Query<(&Camera, &GlobalTransform, &mut Transform)>,
                       buttons: Res<Input<MouseButton>>,
                       windows: Res<Windows>,
                       mut camera_drag_start: ResMut<CameraDragStart>) {
    let (camera, camera_global_transform, mut camera_transform) = camera.single_mut();

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
        let ndc_to_world = camera_global_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        world_pos
    };

    for interaction in bg_inter.iter() {
        match interaction {
            Interaction::Clicked => if buttons.just_pressed(MouseButton::Left) {
                *world_mouse_state = WorldMouseState::Pressed {
                    time: time.seconds_since_startup(),
                    position_window: cursor_position_window,
                    position_world: cursor_position_world,
                };

                camera_drag_start.start_position = camera_transform.translation.truncate();
            }

            Interaction::Hovered => match *world_mouse_state {
                WorldMouseState::None => {
                    //info!("Hovering");
                }

                _ => { }
            }

            _ => { }
        }
    }

    match (buttons.just_released(MouseButton::Left), *world_mouse_state) {
        (true, WorldMouseState::Dragging { .. }) => {
            *world_mouse_state = WorldMouseState::None;
        }

        (false, WorldMouseState::Dragging { start_position_window }) => {
            let cursor_transform = camera_transform.with_translation(Vec3::ZERO);
            let cursor_offset = cursor_transform.mul_vec3((start_position_window - cursor_position_window).extend(0.0));
            camera_transform.translation = camera_drag_start.start_position.extend(camera_transform.translation.z) + cursor_offset;
        }

        (released, WorldMouseState::Pressed { time: press_time, position_world, position_window }) => {
            let press_duration = time.seconds_since_startup() - press_time;
            let press_distance = cursor_position_window.distance(position_window);
            let suitable_for_click = press_duration < CLICK_DURATION && press_distance < CLICK_DISTANCE;

            if released {
                if suitable_for_click {
                    spawn_coin(&mut commands, &assets, 1, position_world, Vec2::ZERO, 1.0);
                } else {
                    let cursor_transform = camera_transform.with_translation(Vec3::ZERO);
                    let cursor_offset = cursor_transform.mul_vec3((position_window - cursor_position_window).extend(0.0));
                    camera_transform.translation = camera_drag_start.start_position.extend(camera_transform.translation.z) + cursor_offset;
                }

                *world_mouse_state = WorldMouseState::None;
            } else if !suitable_for_click {
                let cursor_transform = camera_transform.with_translation(Vec3::ZERO);
                let cursor_offset = cursor_transform.mul_vec3((position_window - cursor_position_window).extend(0.0));
                camera_transform.translation = camera_drag_start.start_position.extend(camera_transform.translation.z) + cursor_offset;

                *world_mouse_state = WorldMouseState::Dragging {
                    start_position_window: position_window,
                };
            }
        }

        _ => { }
    }
}

fn spawn_coin(commands: &mut Commands, assets: &Res<GameAssets>, value: usize, position: Vec2, velocity: Vec2, damping: f64) {
    let font_size = 180.0 / ((value as f32).log10().floor() + 1.0).powf(0.75);

    commands.spawn_bundle(SpriteBundle {
        texture: assets.coin.clone(),
        transform: Transform::from_translation(position.extend(0.0)),
        ..default()
    }).with_children(|coin| {
        coin.spawn_bundle(Text2dBundle {
            text: Text::from_section(value.to_string(), TextStyle {
                font: assets.font.clone(),
                color: palette::DARK_BLUE,
                font_size,
            }).with_alignment(TextAlignment::CENTER),
            transform: Transform::from_xyz(0.0, 0.0, 0.1),
            ..default()
        });
    }).insert(Name::new("Coin"))
        .insert(Particle {
            velocity,
            damping,
        })
        .insert(Money(value));
}

pub fn zoom_camera(mut camera: Query<&mut Transform, With<Camera2d>>,
                   mut scroll_events: EventReader<MouseWheel>) {
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
