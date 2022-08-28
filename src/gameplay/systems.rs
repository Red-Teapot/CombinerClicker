use std::f32::consts::PI;
use std::ops::Add;
use std::time::Duration;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::prelude::Val::{Px, Undefined};
use bevy::ui::FocusPolicy;
use bevy_ninepatch::{NinePatchBundle, NinePatchData};
use bevy_tweening::*;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens, UiPositionLens};
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

    commands.insert_resource(TileTrackedEntities::new());

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
                        focus_policy: FocusPolicy::Pass,
                        ..default()
                    }).insert(MachineName(*machine));

                    container.spawn_bundle(ImageBundle {
                        image: assets.locked.clone().into(),
                        style: Style {
                            size: Size::new(Px(64.0), Px(64.0)),
                            ..default()
                        },
                        focus_policy: FocusPolicy::Pass,
                        ..default()
                    }).insert(MachineIcon(*machine));

                    container.spawn_bundle(TextBundle {
                        text: Text::from_section(machine.cost().to_string(), TextStyle {
                            font: assets.font.clone(),
                            color: palette::DARK_BLUE,
                            font_size: 28.0,
                        }),
                        focus_policy: FocusPolicy::Pass,
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

pub fn track_tile_entities(entities: Query<(Entity, &GlobalTransform), With<TileTrackedEntity>>,
                           mut tracked_entities: ResMut<TileTrackedEntities>) {
    tracked_entities.clear();

    for (entity, transform) in entities.iter() {
        tracked_entities.add(transform.translation().truncate(), entity);
    }
}

pub fn handle_bg_input(mut world_mouse_state: ResMut<WorldMouseState>,
                       time: Res<Time>,
                       bg_inter: Query<&Interaction, With<BackgroundInteraction>>,
                       mut camera: Query<(&Camera, &GlobalTransform)>,
                       buttons: Res<Input<MouseButton>>,
                       windows: Res<Windows>,
                       mut world_mouse_events: EventWriter<WorldMouseEvent>) {
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
            }

            Interaction::Hovered => match *world_mouse_state {
                WorldMouseState::None => {
                    world_mouse_events.send(WorldMouseEvent::Hover {
                        position: cursor_position_world,
                    });
                }

                _ => { }
            }

            _ => { }
        }
    }

    let convert_drag_offset = |offset: Vec2| -> Vec2 {
        let transform = camera_global_transform.compute_transform().with_translation(Vec3::ZERO);

        transform.mul_vec3(offset.extend(0.0)).truncate()
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

        (released, WorldMouseState::Pressed { time: press_time, position_world, position_window }) => {
            let press_duration = time.seconds_since_startup() - press_time;
            let press_distance = cursor_position_window.distance(position_window);
            let suitable_for_click = press_duration < CLICK_DURATION && press_distance < CLICK_DISTANCE;

            if released {
                if suitable_for_click {
                    world_mouse_events.send(WorldMouseEvent::Click {
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

        _ => { }
    }
}

pub fn drag_camera(mut camera: Query<&mut Transform, With<Camera2d>>,
                   mut world_mouse_events: EventReader<WorldMouseEvent>) {
    let mut camera_transform = camera.single_mut();

    for event in world_mouse_events.iter() {
        match event {
            WorldMouseEvent::Drag { offset } =>
                camera_transform.translation -= offset.extend(0.0),

            _ => ()
        }
    }
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

pub fn move_particles(mut particles: Query<(&mut Transform, &mut Particle)>) {
    for (mut transform, mut particle) in particles.iter_mut() {
        transform.translation += particle.velocity.extend(0.0);
        let damping = particle.damping;
        particle.velocity *= damping;
    }
}

fn spawn_coin(commands: &mut Commands,
              assets: &Res<GameAssets>,
              value: usize,
              position: Vec2,
              velocity: Vec2,
              damping: f32) {
    let font_size = 180.0 / ((value as f32).log10().floor() + 1.0).powf(0.75);

    commands.spawn_bundle(SpriteBundle {
        texture: assets.coin.clone(),
        transform: Transform::from_translation(position.extend(0.2))
            .with_scale(Vec3::splat(0.0)),
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
        .insert(Money(value))
        .insert(Animator::new(Tween::new(
            EaseFunction::CubicOut,
            TweeningType::Once,
            Duration::from_secs_f32(0.2),
            TransformScaleLens {
                start: Vec3::splat(0.0),
                end: Vec3::splat(1.0),
            }
        )))
        .insert(Coin {
            spawn_timer: Timer::from_seconds(0.2, false),
            despawn_timer: {
                let mut timer = Timer::from_seconds(0.1, false);
                timer.pause();
                timer
            },
            has_money: true,
        })
        .insert(TileTrackedEntity);
}

pub fn click_coins(mut commands: Commands,
                   mut ghosts: Query<(Entity, &mut Transform, &BuildingGhost)>,
                   assets: Res<GameAssets>,
                   mut world_mouse_events: EventReader<WorldMouseEvent>) {
    if !ghosts.is_empty() {
        world_mouse_events.clear();
        return;
    }

    for event in world_mouse_events.iter() {
        match event {
            WorldMouseEvent::Click { position } => {
                let initial_velocity = Vec2::from_angle(rand::random::<f32>() * 2.0 * PI) * 80.0;
                spawn_coin(&mut commands, &assets, 1, *position, initial_velocity, 0.6);
            }

            _ => ()
        }
    }
}

pub fn update_coins(mut commands: Commands,
                    mut coins: Query<(Entity, &Transform, &mut Coin, &Money)>,
                    time: Res<Time>,
                    mut money: ResMut<Money>,
                    mut coin_pickup_events: EventReader<CoinPickup>) {
    for event in coin_pickup_events.iter() {
        let (_, transform, mut coin, _) = coins.get_mut(event.coin).unwrap();

        const DESPAWN_DURATION: f32 = 0.1;

        coin.despawn_timer.set_duration(Duration::from_secs_f32(DESPAWN_DURATION));
        coin.despawn_timer.unpause();
        coin.has_money = event.add_money;

        commands.entity(event.coin)
            .insert(Animator::new(Tracks::new([
                Tween::new(
                    EaseFunction::CubicIn,
                    TweeningType::Once,
                    Duration::from_secs_f32(DESPAWN_DURATION),
                    TransformScaleLens {
                        start: Vec3::splat(1.0),
                        end: Vec3::splat(0.0),
                    }
                ),
                Tween::new(
                    EaseFunction::CubicIn,
                    TweeningType::Once,
                    Duration::from_secs_f32(DESPAWN_DURATION),
                    TransformPositionLens {
                        start: transform.translation,
                        end: event.target.extend(0.0),
                    }
                ),
            ])));
    }

    for (entity, _, mut coin, coin_money) in coins.iter_mut() {
        coin.spawn_timer.tick(time.delta());
        coin.despawn_timer.tick(time.delta());

        if coin.despawn_timer.just_finished() {
            if coin.has_money {
                money.0 += coin_money.0.pow(2);
            }

            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn hover_coins(mut coins: Query<(&Transform, &Coin), Without<BuildingGhost>>,
                   mut world_mouse_events: EventReader<WorldMouseEvent>,
                   mut tile_tracked_entities: ResMut<TileTrackedEntities>,
                   mut coin_pickup_events: EventWriter<CoinPickup>) {
    for event in world_mouse_events.iter() {
        match event {
            WorldMouseEvent::Hover { position } => {
                let center_tile = TilePosition::from_world(*position);
                let tiles_to_check = [
                    center_tile.offset(-1, -1),
                    center_tile.offset( 0, -1),
                    center_tile.offset( 1, -1),
                    center_tile.offset(-1,  0),
                    center_tile,
                    center_tile.offset( 1,  0),
                    center_tile.offset(-1,  1),
                    center_tile.offset( 0,  1),
                    center_tile.offset( 1,  1),
                ];

                for tile in tiles_to_check {
                    if let Some(entities) = tile_tracked_entities.get_entities_in_tile(tile) {
                        for &entity in entities {
                            if let Ok((transform, coin)) = coins.get(entity) {
                                if coin.pickable() && position.distance(transform.translation.truncate()) <= 192.0 {
                                    coin_pickup_events.send(CoinPickup {
                                        coin: entity,
                                        target: *position,
                                        add_money: true,
                                    });
                                }
                            }
                        }
                    }
                }
            }

            _ => ()
        }
    }
}

pub fn update_money(money_res: Res<Money>,
                    assets: Res<GameAssets>,
                    mut money_display: Query<&mut Text, With<MoneyDisplay>>,
                    mut machine_names: Query<(&mut Text, &MachineName), Without<MoneyDisplay>>,
                    mut machine_icons: Query<(&mut UiImage, &MachineIcon)>,
                    mut machine_buy_buttons: Query<&mut MachineBuyButton>) {
    if !money_res.is_changed() {
        return;
    }

    let mut text = money_display.single_mut();
    text.sections[0].value = money_res.0.to_string();

    for (mut text, MachineName(machine)) in machine_names.iter_mut() {
        if money_res.0 >= machine.cost() {
            text.sections[0].value = machine.name().to_string();
        }
    }

    for (mut image, MachineIcon(machine)) in machine_icons.iter_mut() {
        if money_res.0 >= machine.cost() {
            image.0 = machine.image(&assets);
        } else {
            image.0 = assets.locked.clone();
        }
    }

    for mut button in machine_buy_buttons.iter_mut() {
        button.enabled = money_res.0 >= button.machine.cost();
    }
}

pub fn handle_machine_buy_buttons(mut commands: Commands,
                                  ghosts: Query<&BuildingGhost>,
                                  assets: Res<GameAssets>,
                                  buttons: Query<(Entity, &Interaction, &MachineBuyButton), Changed<Interaction>>,
                                  mut money: ResMut<Money>) {
    for (entity, interaction, button) in buttons.iter() {
        if !button.enabled {
            continue;
        }

        match interaction {
            Interaction::Clicked => {
                if !ghosts.is_empty() {
                    continue;
                }

                money.0 -= button.machine.cost();

                match button.machine {
                    Machine::Miner => {
                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.miner.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Machine(Machine::Miner));

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: 0, offset_y: -1 });
                    }

                    Machine::Collector => {
                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.collector.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Machine(Machine::Collector));

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: 0, offset_y: 1 });
                    }

                    Machine::ConveyorUp => {
                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.conveyor_up.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Machine(Machine::ConveyorUp));

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: 0, offset_y: -1 });

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: 0, offset_y: 1 });
                    }

                    Machine::ConveyorDown => {
                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.conveyor_down.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Machine(Machine::ConveyorDown));

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: 0, offset_y: -1 });

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: 0, offset_y: 1 });
                    }

                    Machine::ConveyorLeft => {
                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.conveyor_left.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Machine(Machine::ConveyorLeft));

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: -1, offset_y: 0 });

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: 1, offset_y: 0 });
                    }

                    Machine::ConveyorRight => {
                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.conveyor_right.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Machine(Machine::ConveyorRight));

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: -1, offset_y: 0 });

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: 1, offset_y: 0 });
                    }

                    Machine::Adder => {
                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.adder.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Machine(Machine::Adder));

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: -1, offset_y: 0 });

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: 1, offset_y: 0 });

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: 0, offset_y: -1 });
                    }

                    Machine::Multiplicator => {
                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.multiplicator.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Machine(Machine::Multiplicator));

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: -1, offset_y: 0 });

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: 1, offset_y: 0 });

                        commands.spawn_bundle(SpriteBundle {
                            texture: assets.spot.clone(),
                            sprite: Sprite {
                                color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..default()
                        }).insert(BuildingGhost::Spot { offset_x: 0, offset_y: -1 });
                    }
                }

                commands.entity(entity)
                    .insert(Animator::new(Tween::new(
                        EaseFunction::CubicOut,
                        TweeningType::Once,
                        Duration::from_secs_f32(0.2),
                        UiPositionLens {
                            start: UiRect::new(Undefined, Undefined, Undefined, Px(8.0)),
                            end: UiRect::new(Undefined, Undefined, Undefined, Px(0.0)),
                        }
                    )));
            }

            Interaction::Hovered => {
                commands.entity(entity)
                    .insert(Animator::new(Tween::new(
                        EaseFunction::CubicOut,
                        TweeningType::Once,
                        Duration::from_secs_f32(0.2),
                        UiPositionLens {
                            start: UiRect::new(Undefined, Undefined, Undefined, Px(0.0)),
                            end: UiRect::new(Undefined, Undefined, Undefined, Px(8.0)),
                        }
                    )));
            }

            Interaction::None => {
                commands.entity(entity)
                    .insert(Animator::new(Tween::new(
                        EaseFunction::CubicOut,
                        TweeningType::Once,
                        Duration::from_secs_f32(0.2),
                        UiPositionLens {
                            start: UiRect::new(Undefined, Undefined, Undefined, Px(8.0)),
                            end: UiRect::new(Undefined, Undefined, Undefined, Px(0.0)),
                        }
                    )));
            }
        }
    }
}

pub fn drag_ghosts(tile_tracked_entities: Res<TileTrackedEntities>,
                   mut ghosts: Query<(Entity, &mut Transform, &BuildingGhost)>,
                   mut world_mouse_events: EventReader<WorldMouseEvent>) {
    let mut hover_position = None;

    for event in world_mouse_events.iter() {
        if let WorldMouseEvent::Hover { position } = event {
            hover_position = Some(*position);
        }
    }

    if hover_position.is_none() {
        return;
    }

    let hover_tile = TilePosition::from_world(hover_position.unwrap());

    for (_, mut transform, ghost) in ghosts.iter_mut() {
        match ghost {
            BuildingGhost::Machine(_) => {
                transform.translation = hover_tile.to_world().add(Vec2::splat(32.0 * 4.0)).extend(0.0);
            }

            BuildingGhost::Spot { offset_x, offset_y } => {
                transform.translation = hover_tile.offset(*offset_x, *offset_y).to_world().add(Vec2::splat(32.0 * 4.0)).extend(0.0);
            }
        }
    }
}

pub fn place_ghosts(mut commands: Commands,
                    tile_tracked_entities: Res<TileTrackedEntities>,
                    mut ghosts: Query<(Entity, &mut Transform, &mut Sprite, &BuildingGhost), Without<PlacedMachine>>,
                    placed_machines: Query<(&Transform, &PlacedMachine), Without<BuildingGhost>>,
                    placed_spots: Query<(&Transform, &Spot), (Without<PlacedMachine>, Without<BuildingGhost>)>,
                    mut world_mouse_events: EventReader<WorldMouseEvent>) {
    let mut clicked = false;

    for event in world_mouse_events.iter() {
        match event {
            WorldMouseEvent::Click { .. } => {
                clicked = true;
            }

            _ => ()
        }
    }

    if !clicked {
        return;
    }

    let mut can_place = true;

    for (_, transform, _, ghost) in ghosts.iter() {
        let tile = TilePosition::from_world(transform.translation.truncate());

        if let Some(tile_entities) = tile_tracked_entities.get_entities_in_tile(tile) {
            match ghost {
                BuildingGhost::Spot { .. } => (),

                BuildingGhost::Machine( .. ) => {
                    for &entity in tile_entities {
                        if let Ok((machine_transform, _)) = placed_machines.get(entity) {
                            let machine_tile = TilePosition::from_world(machine_transform.translation.truncate());

                            if machine_tile == tile {
                                can_place = false;
                            }
                        }
                    }
                }
            }
        }
    }

    if !can_place {
        return;
    }

    for (entity, mut transform, mut sprite, ghost) in ghosts.iter_mut() {
        match ghost {
            BuildingGhost::Spot { .. } => {
                let tile = TilePosition::from_world(transform.translation.truncate());
                let mut despawned = false;

                if let Some(tile_entities) = tile_tracked_entities.get_entities_in_tile(tile) {
                    for &tile_entity in tile_entities {
                        if let Ok((spot_transform, _)) = placed_spots.get(tile_entity) {
                            let spot_tile = TilePosition::from_world(spot_transform.translation.truncate());

                            if spot_tile == tile {
                                commands.entity(entity).despawn_recursive();
                                despawned = true;
                            }
                        }

                        if let Ok((machine_transform, _)) = placed_machines.get(tile_entity) {
                            let machine_tile = TilePosition::from_world(machine_transform.translation.truncate());

                            if machine_tile == tile {
                                commands.entity(entity).despawn_recursive();
                                despawned = true;
                            }
                        }
                    }
                }

                if !despawned {
                    sprite.color = Color::WHITE;
                    commands.entity(entity)
                        .remove::<BuildingGhost>()
                        .insert(Spot)
                        .insert(TileTrackedEntity);
                    transform.translation.z = 0.0;
                }
            }

            BuildingGhost::Machine(machine) => {
                sprite.color = Color::WHITE;
                commands.entity(entity)
                    .remove::<BuildingGhost>()
                    .insert(PlacedMachine {
                        machine: *machine,
                        action_timer: Timer::new(machine.action_period(), true),
                    })
                    .insert(TileTrackedEntity);
                transform.translation.z = 0.0;
            }
        }
    }
}

pub fn act_machines(mut commands: Commands,
                    assets: Res<GameAssets>,
                    mut machines: Query<(&Transform, &mut PlacedMachine)>,
                    mut coins: Query<(&Coin, &Money), Without<PlacedMachine>>,
                    tile_tracked_entities: Res<TileTrackedEntities>,
                    time: Res<Time>,
                    mut coin_pickups: EventWriter<CoinPickup>) {
    let find_coin = |tile_pos: TilePosition| -> Option<(Entity, &Coin, &Money)> {
        if let Some(entities) = tile_tracked_entities.get_entities_in_tile(tile_pos) {
            for &entity in entities {
                if let Ok((coin, money)) = coins.get(entity) {
                    return Some((entity, coin, money));
                }
            }

            return None;
        }

        return None;
    };

    let mut spew_coin = |position: Vec2, value: usize, angle: f32| {
        let spread = PI / 4.0;
        let speed = 80.0 + 30.0 * rand::random::<f32>();
        let velocity = Vec2::from_angle(rand::random::<f32>() * spread - spread / 2.0 + angle) * speed;
        spawn_coin(&mut commands, &assets, value, position, velocity, 0.6);
    };

    for (transform, mut placed_machine) in machines.iter_mut() {
        placed_machine.action_timer.tick(time.delta());

        if placed_machine.action_timer.just_finished() {
            let position = transform.translation.truncate();
            let tile_pos = TilePosition::from_world(position);

            match placed_machine.machine {
                Machine::Miner => {
                    spew_coin(position, 1, -PI / 2.0);
                }

                Machine::Collector => {
                    if let Some((entity, coin, _)) = find_coin(tile_pos.offset(0, 1)) {
                        coin_pickups.send(CoinPickup {
                            coin: entity,
                            target: position,
                            add_money: true,
                        });
                    }
                }

                Machine::Adder => {
                    let mut coin_left = find_coin(tile_pos.offset(-1, 0));
                    let mut coin_right = find_coin(tile_pos.offset(1, 0));

                    match (coin_left, coin_right) {
                        (Some((entity_left, coin_left, money_left)), Some((entity_right, coin_right, money_right))) => {
                            coin_pickups.send(CoinPickup {
                                coin: entity_left,
                                target: position,
                                add_money: false,
                            });
                            coin_pickups.send(CoinPickup {
                                coin: entity_right,
                                target: position,
                                add_money: false,
                            });

                            spew_coin(position, money_left.0 + money_right.0, -PI / 2.0);
                        }

                        _ => ()
                    }
                }

                Machine::ConveyorUp => {
                    let coin_stuff = find_coin(tile_pos).or_else(|| find_coin(tile_pos.offset(0, -1)));

                    if let Some((entity, _, money)) = coin_stuff {
                        coin_pickups.send(CoinPickup {
                            coin: entity,
                            target: position,
                            add_money: false,
                        });
                        spew_coin(position, money.0, PI / 2.0);
                    }
                }

                Machine::ConveyorDown => {
                    let coin_stuff = find_coin(tile_pos).or_else(|| find_coin(tile_pos.offset(0, 1)));

                    if let Some((entity, _, money)) = coin_stuff {
                        coin_pickups.send(CoinPickup {
                            coin: entity,
                            target: position,
                            add_money: false,
                        });
                        spew_coin(position, money.0, -PI / 2.0);
                    }
                }

                Machine::ConveyorLeft => {
                    let coin_stuff = find_coin(tile_pos).or_else(|| find_coin(tile_pos.offset(1, 0)));

                    if let Some((entity, _, money)) = coin_stuff {
                        coin_pickups.send(CoinPickup {
                            coin: entity,
                            target: position,
                            add_money: false,
                        });
                        spew_coin(position, money.0, PI);
                    }
                }

                Machine::ConveyorRight => {
                    let coin_stuff = find_coin(tile_pos).or_else(|| find_coin(tile_pos.offset(-1, 0)));

                    if let Some((entity, _, money)) = coin_stuff {
                        coin_pickups.send(CoinPickup {
                            coin: entity,
                            target: position,
                            add_money: false,
                        });
                        spew_coin(position, money.0, 0.0);
                    }
                }

                Machine::Multiplicator => {
                    let mut coin_left = find_coin(tile_pos.offset(-1, 0));
                    let mut coin_right = find_coin(tile_pos.offset(1, 0));

                    match (coin_left, coin_right) {
                        (Some((entity_left, coin_left, money_left)), Some((entity_right, coin_right, money_right))) => {
                            coin_pickups.send(CoinPickup {
                                coin: entity_left,
                                target: position,
                                add_money: false,
                            });
                            coin_pickups.send(CoinPickup {
                                coin: entity_right,
                                target: position,
                                add_money: false,
                            });

                            spew_coin(position, money_left.0 * money_right.0, -PI / 2.0);
                        }

                        _ => ()
                    }
                }
            }
        }
    }
}
