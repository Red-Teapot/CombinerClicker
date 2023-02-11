use std::f32::consts::PI;
use std::ops::Add;
use std::time::Duration;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;
use bevy_ninepatch::{NinePatchBundle, NinePatchData};
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens, UiPositionLens};
use bevy_tweening::*;

use crate::assets::*;
use crate::gameplay::components::*;
use crate::palette;

use super::input::WorldMouseEvent;
use super::tile_tracked_entities::{TileTrackedEntity, TileTrackedEntities, TilePosition};

pub fn startup_gameplay(
    mut commands: Commands,
    fonts: Res<Fonts>,
    images: Res<Images>,
    ninepatches: Res<NinePatches>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    camera.single_mut().scale = vec3(4.0, 4.0, 1.0);

    commands.insert_resource(Money(0));

    commands.insert_resource(NextCoinDepth {
        depth: 0.1,
        step: 0.00000001,
    });

    commands.insert_resource(super::input::WorldMouseState::None);

    commands.insert_resource(TileTrackedEntities::new());

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            focus_policy: FocusPolicy::Pass,
            ..default()
        })
        .with_children(|window| {
            window
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                })
                .with_children(|top_panel| {
                    top_panel
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                padding: UiRect::all(Val::Px(8.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|money_display| {
                            money_display
                                .spawn(ImageBundle {
                                    image: images.coin.clone().into(),
                                    style: Style {
                                        size: Size::new(Val::Px(48.0), Val::Px(48.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|coin| {
                                    coin.spawn(TextBundle {
                                        text: Text::from_section(
                                            "1",
                                            TextStyle {
                                                font: fonts.varela.clone(),
                                                color: palette::DARK_BLUE,
                                                font_size: 40.0,
                                            },
                                        )
                                        .with_alignment(TextAlignment::CENTER),
                                        ..default()
                                    });
                                })
                                .insert(Name::new("Icon"));

                            money_display
                                .spawn(TextBundle {
                                    text: Text::from_section(
                                        "0",
                                        TextStyle {
                                            font: fonts.varela.clone(),
                                            color: palette::DARK_BLUE,
                                            font_size: 48.0,
                                        },
                                    ),
                                    style: Style {
                                        margin: UiRect {
                                            left: Val::Px(16.0),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(Name::new("Value"))
                                .insert(MoneyDisplay);
                        })
                        .insert(Name::new("Money Display"));
                })
                .insert(Name::new("Top Panel"))
                .insert(Animator::new(Tween::new(
                    EaseFunction::QuadraticOut,
                    Duration::from_secs_f32(0.2),
                    UiPositionLens {
                        start: UiRect::top(Val::Px(-64.0)),
                        end: UiRect::top(Val::Px(0.0)),
                    },
                )));

            let bottom_panel_content = window
                .spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::FlexEnd,
                        justify_content: JustifyContent::Center,
                        min_size: Size {
                            height: Val::Px(208.0),
                            ..default()
                        },
                        padding: UiRect::new(
                            Val::Px(-8.0),
                            Val::Px(-8.0),
                            Val::Px(-8.0),
                            Val::Px(64.0),
                        ),
                        ..default()
                    },
                    focus_policy: FocusPolicy::Pass,
                    ..default()
                })
                .with_children(|bottom_panel| {
                    for machine in Machine::list() {
                        bottom_panel
                            .spawn(ButtonBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    padding: UiRect::all(Val::Px(8.0)),
                                    align_items: AlignItems::Center,
                                    size: Size {
                                        width: Val::Px(90.0),
                                        height: Val::Undefined,
                                    },
                                    ..default()
                                },
                                background_color: Color::NONE.into(),
                                ..default()
                            })
                            .with_children(|container| {
                                container
                                    .spawn(TextBundle {
                                        text: Text::from_section(
                                            "???",
                                            TextStyle {
                                                font: fonts.varela.clone(),
                                                color: palette::LIGHT_BROWN,
                                                font_size: 20.0,
                                            },
                                        )
                                        .with_alignment(TextAlignment::BOTTOM_CENTER),
                                        style: Style {
                                            margin: UiRect {
                                                bottom: Val::Px(4.0),
                                                ..default()
                                            },
                                            max_size: Size {
                                                width: Val::Px(90.0),
                                                height: default(),
                                            },
                                            ..default()
                                        },
                                        focus_policy: FocusPolicy::Pass,
                                        ..default()
                                    })
                                    .insert(MachineName(*machine));

                                container
                                    .spawn(ImageBundle {
                                        image: images.locked.clone().into(),
                                        style: Style {
                                            size: Size::new(Val::Px(64.0), Val::Px(64.0)),
                                            ..default()
                                        },
                                        focus_policy: FocusPolicy::Pass,
                                        ..default()
                                    })
                                    .insert(MachineIcon(*machine));

                                container.spawn(TextBundle {
                                    text: Text::from_section(
                                        machine.cost().to_string(),
                                        TextStyle {
                                            font: fonts.varela.clone(),
                                            color: palette::DARK_BLUE,
                                            font_size: 28.0,
                                        },
                                    ),
                                    focus_policy: FocusPolicy::Pass,
                                    ..default()
                                });
                            })
                            .insert(MachineBuyButton {
                                enabled: false,
                                machine: *machine,
                            });
                    }
                })
                .insert(Name::new("Bottom Panel Content"))
                .id();

            window
                .spawn(NinePatchBundle {
                    nine_patch_data: NinePatchData::with_single_content(
                        images.panel.clone(),
                        ninepatches.panel.clone(),
                        bottom_panel_content,
                    ),
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        align_self: AlignSelf::Center,
                        position: UiRect {
                            bottom: Val::Px(-84.0 - 144.0),
                            ..default()
                        },
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Bottom Panel"))
                .insert(Animator::new(Tween::new(
                    EaseFunction::QuadraticOut,
                    Duration::from_secs_f32(0.2),
                    UiPositionLens {
                        start: UiRect::bottom(Val::Px(-84.0 - 144.0)),
                        end: UiRect::bottom(Val::Px(-84.0)),
                    },
                )));
        });
}

pub fn move_particles(mut particles: Query<(&mut Transform, &mut Particle)>) {
    for (mut transform, mut particle) in particles.iter_mut() {
        transform.translation += particle.velocity.extend(0.0);
        let damping = particle.damping;
        particle.velocity *= damping;
    }
}

fn spawn_coin(
    commands: &mut Commands,
    depth: &mut ResMut<NextCoinDepth>,
    fonts: &Res<Fonts>,
    game_images: &Res<Images>,
    value: u128,
    position: Vec2,
    velocity: Vec2,
    damping: f32,
) {
    let font_size = 180.0 / ((value as f32).log10().floor() + 1.0).powf(0.75);

    commands
        .spawn(SpriteBundle {
            texture: game_images.coin.clone(),
            transform: Transform::from_translation(position.extend(depth.depth))
                .with_scale(Vec3::splat(0.0)),
            ..default()
        })
        .with_children(|coin| {
            coin.spawn(Text2dBundle {
                text: Text::from_section(
                    value.to_string(),
                    TextStyle {
                        font: fonts.varela.clone(),
                        color: palette::DARK_BLUE,
                        font_size,
                    },
                )
                .with_alignment(TextAlignment::CENTER),
                transform: Transform::from_xyz(0.0, 0.0, depth.step * 0.5),
                ..default()
            });
        })
        .insert(Name::new("Coin"))
        .insert(Particle { velocity, damping })
        .insert(Money(value))
        .insert(Animator::new(Tween::new(
            EaseFunction::CubicOut,
            Duration::from_secs_f32(0.2),
            TransformScaleLens {
                start: Vec3::splat(0.0),
                end: Vec3::splat(1.0),
            },
        )))
        .insert(Coin {
            spawn_timer: Timer::from_seconds(0.2, TimerMode::Once),
            despawn_timer: {
                let mut timer = Timer::from_seconds(0.1, TimerMode::Once);
                timer.pause();
                timer
            },
            has_money: true,
            alive: true,
        })
        .insert(TileTrackedEntity);

    depth.depth += depth.step;
    if depth.depth >= 0.2 {
        depth.depth = 0.1;
    }
}

pub fn click_coins(
    mut commands: Commands,
    ghosts: Query<(Entity, &mut Transform, &BuildingGhost)>,
    fonts: Res<Fonts>,
    game_images: Res<Images>,
    mut depth: ResMut<NextCoinDepth>,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
) {
    if !ghosts.is_empty() {
        world_mouse_events.clear();
        return;
    }

    for event in world_mouse_events.iter() {
        match event {
            WorldMouseEvent::LeftClick { position } => {
                let initial_velocity = Vec2::from_angle(rand::random::<f32>() * 2.0 * PI) * 80.0;
                spawn_coin(
                    &mut commands,
                    &mut depth,
                    &fonts,
                    &game_images,
                    1,
                    *position,
                    initial_velocity,
                    0.6,
                );
            }

            _ => (),
        }
    }
}

pub fn update_coins(
    mut commands: Commands,
    mut coins: Query<(Entity, &Transform, &mut Coin, &Money)>,
    time: Res<Time>,
    mut money: ResMut<Money>,
    mut coin_pickup_events: EventReader<CoinPickup>,
) {
    for event in coin_pickup_events.iter() {
        let coin = coins.get_mut(event.coin);

        if coin.is_err() {
            continue;
        }

        let (_, transform, mut coin, _) = coin.unwrap();

        const DESPAWN_DURATION: f32 = 0.1;

        coin.despawn_timer
            .set_duration(Duration::from_secs_f32(DESPAWN_DURATION));
        coin.despawn_timer.unpause();
        coin.has_money = event.add_money;

        commands
            .entity(event.coin)
            .insert(Animator::new(Tracks::new([
                Tween::new(
                    EaseFunction::CubicIn,
                    Duration::from_secs_f32(DESPAWN_DURATION),
                    TransformScaleLens {
                        start: Vec3::splat(1.0),
                        end: Vec3::splat(0.0),
                    },
                ),
                Tween::new(
                    EaseFunction::CubicIn,
                    Duration::from_secs_f32(DESPAWN_DURATION),
                    TransformPositionLens {
                        start: transform.translation,
                        end: event.target.extend(0.0),
                    },
                ),
            ])));
    }

    for (entity, _, mut coin, coin_money) in coins.iter_mut() {
        coin.spawn_timer.tick(time.delta());
        coin.despawn_timer.tick(time.delta());

        if coin.despawn_timer.just_finished() {
            if coin.has_money {
                money.0 += coin_money.0;
            }

            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn hover_coins(
    coins: Query<(&Transform, &Coin), Without<BuildingGhost>>,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
    tile_tracked_entities: ResMut<TileTrackedEntities>,
    mut coin_pickup_events: EventWriter<CoinPickup>,
) {
    for event in world_mouse_events.iter() {
        match event {
            WorldMouseEvent::Hover { position } => {
                let center_tile = TilePosition::from_world(*position);
                let tiles_to_check = [
                    center_tile.offset(-1, -1),
                    center_tile.offset(0, -1),
                    center_tile.offset(1, -1),
                    center_tile.offset(-1, 0),
                    center_tile,
                    center_tile.offset(1, 0),
                    center_tile.offset(-1, 1),
                    center_tile.offset(0, 1),
                    center_tile.offset(1, 1),
                ];

                for tile in tiles_to_check {
                    if let Some(entities) = tile_tracked_entities.get_entities_in_tile(tile) {
                        for &entity in entities {
                            if let Ok((transform, coin)) = coins.get(entity) {
                                if coin.pickable()
                                    && position.distance(transform.translation.truncate()) <= 192.0
                                {
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

            _ => (),
        }
    }
}

pub fn update_money(
    money_res: Res<Money>,
    ui_images: Res<Images>,
    game_images: Res<Images>,
    mut money_display: Query<&mut Text, With<MoneyDisplay>>,
    mut machine_names: Query<(&mut Text, &MachineName), Without<MoneyDisplay>>,
    mut machine_icons: Query<(&mut UiImage, &MachineIcon)>,
    mut machine_buy_buttons: Query<&mut MachineBuyButton>,
) {
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
            image.0 = machine.image(&game_images);
        } else {
            image.0 = ui_images.locked.clone();
        }
    }

    for mut button in machine_buy_buttons.iter_mut() {
        button.enabled = money_res.0 >= button.machine.cost();
    }
}

pub fn handle_machine_buy_buttons(
    mut commands: Commands,
    ghosts: Query<&BuildingGhost>,
    images: Res<Images>,
    buttons: Query<(Entity, &Interaction, &MachineBuyButton), Changed<Interaction>>,
    mut money: ResMut<Money>,
) {
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
                        commands
                            .spawn(SpriteBundle {
                                texture: images.miner.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::Miner));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: -1,
                            });
                    }

                    Machine::Collector => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.collector.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::Collector));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: 1,
                            });
                    }

                    Machine::ConveyorUp => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.conveyor_up.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::ConveyorUp));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: -1,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: 1,
                            });
                    }

                    Machine::ConveyorDown => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.conveyor_down.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::ConveyorDown));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: -1,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: 1,
                            });
                    }

                    Machine::ConveyorLeft => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.conveyor_left.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::ConveyorLeft));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: -1,
                                offset_y: 0,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 1,
                                offset_y: 0,
                            });
                    }

                    Machine::ConveyorRight => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.conveyor_right.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::ConveyorRight));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: -1,
                                offset_y: 0,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 1,
                                offset_y: 0,
                            });
                    }

                    Machine::Adder => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.adder.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::Adder));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: -1,
                                offset_y: 0,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 1,
                                offset_y: 0,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: -1,
                            });
                    }

                    Machine::Multiplier => {
                        commands
                            .spawn(SpriteBundle {
                                texture: images.multiplier.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Machine(Machine::Multiplier));

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: -1,
                                offset_y: 0,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 1,
                                offset_y: 0,
                            });

                        commands
                            .spawn(SpriteBundle {
                                texture: images.spot.clone(),
                                sprite: Sprite {
                                    color: Color::rgba(1.0, 1.0, 1.0, 0.5),
                                    ..default()
                                },
                                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                                ..default()
                            })
                            .insert(BuildingGhost::Spot {
                                offset_x: 0,
                                offset_y: -1,
                            });
                    }
                }

                commands.entity(entity).insert(Animator::new(Tween::new(
                    EaseFunction::CubicOut,
                    Duration::from_secs_f32(0.2),
                    UiPositionLens {
                        start: UiRect::bottom(Val::Px(8.0)),
                        end: UiRect::bottom(Val::Px(0.0)),
                    },
                )));
            }

            Interaction::Hovered => {
                commands.entity(entity).insert(Animator::new(Tween::new(
                    EaseFunction::CubicOut,
                    Duration::from_secs_f32(0.2),
                    UiPositionLens {
                        start: UiRect::bottom(Val::Px(0.0)),
                        end: UiRect::bottom(Val::Px(8.0)),
                    },
                )));
            }

            Interaction::None => {
                commands.entity(entity).insert(Animator::new(Tween::new(
                    EaseFunction::CubicOut,
                    Duration::from_secs_f32(0.2),
                    UiPositionLens {
                        start: UiRect::bottom(Val::Px(8.0)),
                        end: UiRect::bottom(Val::Px(0.0)),
                    },
                )));
            }
        }
    }
}

pub fn drag_ghosts(
    mut ghosts: Query<(Entity, &mut Transform, &BuildingGhost)>,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
) {
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
                transform.translation = hover_tile
                    .to_world()
                    .add(Vec2::splat(32.0 * 4.0))
                    .extend(0.0);
            }

            BuildingGhost::Spot { offset_x, offset_y } => {
                transform.translation = hover_tile
                    .offset(*offset_x, *offset_y)
                    .to_world()
                    .add(Vec2::splat(32.0 * 4.0))
                    .extend(0.0);
            }
        }
    }
}

pub fn place_ghosts(
    mut commands: Commands,
    tile_tracked_entities: Res<TileTrackedEntities>,
    mut ghosts: Query<
        (Entity, &mut Transform, &mut Sprite, &BuildingGhost),
        Without<PlacedMachine>,
    >,
    placed_machines: Query<(&Transform, &PlacedMachine), Without<BuildingGhost>>,
    placed_spots: Query<(&Transform, &Spot), (Without<PlacedMachine>, Without<BuildingGhost>)>,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
) {
    let mut clicked = false;

    for event in world_mouse_events.iter() {
        match event {
            WorldMouseEvent::LeftClick { .. } => {
                clicked = true;
            }

            _ => (),
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

                BuildingGhost::Machine(..) => {
                    for &entity in tile_entities {
                        if let Ok((machine_transform, _)) = placed_machines.get(entity) {
                            let machine_tile =
                                TilePosition::from_world(machine_transform.translation.truncate());

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
                        if let Ok((machine_transform, _)) = placed_machines.get(tile_entity) {
                            let machine_tile =
                                TilePosition::from_world(machine_transform.translation.truncate());

                            if machine_tile == tile {
                                commands.entity(entity).despawn_recursive();
                                despawned = true;
                            }
                        }
                    }
                }

                if !despawned {
                    sprite.color = Color::WHITE;
                    commands
                        .entity(entity)
                        .remove::<BuildingGhost>()
                        .insert(Spot)
                        .insert(TileTrackedEntity);
                    transform.translation.z = 0.0;
                }
            }

            BuildingGhost::Machine(machine) => {
                let tile = TilePosition::from_world(transform.translation.truncate());

                if let Some(tile_entities) = tile_tracked_entities.get_entities_in_tile(tile) {
                    for &tile_entity in tile_entities {
                        if let Ok((spot_transform, _)) = placed_spots.get(tile_entity) {
                            let spot_tile =
                                TilePosition::from_world(spot_transform.translation.truncate());

                            if spot_tile == tile {
                                commands.entity(tile_entity).despawn_recursive();
                            }
                        }
                    }
                }

                sprite.color = Color::WHITE;
                commands
                    .entity(entity)
                    .remove::<BuildingGhost>()
                    .insert(PlacedMachine {
                        machine: *machine,
                        action_timer: Timer::new(machine.action_period(), TimerMode::Repeating),
                    })
                    .insert(TileTrackedEntity);
                transform.translation.z = 0.0;
            }
        }
    }
}

pub fn act_machines(
    mut commands: Commands,
    fonts: Res<Fonts>,
    images: Res<Images>,
    mut depth: ResMut<NextCoinDepth>,
    mut machines: Query<(&Transform, &mut PlacedMachine)>,
    mut coins: Query<(&mut Coin, &Money), Without<PlacedMachine>>,
    tile_tracked_entities: Res<TileTrackedEntities>,
    time: Res<Time>,
    mut coin_pickups: EventWriter<CoinPickup>,
) {
    let mut consumed_coins = Vec::new();
    for (transform, mut placed_machine) in machines.iter_mut() {
        consumed_coins.clear();
        placed_machine.action_timer.tick(time.delta());

        if placed_machine.action_timer.just_finished() {
            let find_coin = |tile_pos: TilePosition| -> Option<(Entity, &Coin, &Money)> {
                if let Some(entities) = tile_tracked_entities.get_entities_in_tile(tile_pos) {
                    for &entity in entities {
                        if let Ok((coin, money)) = coins.get(entity) {
                            if coin.pickable() {
                                return Some((entity, &coin, money));
                            }
                        }
                    }

                    return None;
                }

                return None;
            };

            let mut spew_coin = |position: Vec2, value: u128, angle: f32| {
                let spread = PI / 4.0;
                let speed = 80.0 + 30.0 * rand::random::<f32>();
                let velocity =
                    Vec2::from_angle(rand::random::<f32>() * spread - spread / 2.0 + angle) * speed;
                spawn_coin(
                    &mut commands,
                    &mut depth,
                    &fonts,
                    &images,
                    value,
                    position,
                    velocity,
                    0.6,
                );
            };

            let position = transform.translation.truncate();
            let tile_pos = TilePosition::from_world(position);

            match placed_machine.machine {
                Machine::Miner => {
                    spew_coin(position, 1, -PI / 2.0);
                }

                Machine::Collector => {
                    if let Some((entity, _coin, _)) = find_coin(tile_pos.offset(0, 1)) {
                        consumed_coins.push(entity);
                        coin_pickups.send(CoinPickup {
                            coin: entity,
                            target: position,
                            add_money: true,
                        });
                    }
                }

                Machine::Adder => {
                    let coin_left = find_coin(tile_pos.offset(-1, 0));
                    let coin_right = find_coin(tile_pos.offset(1, 0));

                    match (coin_left, coin_right) {
                        (
                            Some((entity_left, _coin_left, money_left)),
                            Some((entity_right, _coin_right, money_right)),
                        ) => {
                            consumed_coins.push(entity_left);
                            coin_pickups.send(CoinPickup {
                                coin: entity_left,
                                target: position,
                                add_money: false,
                            });
                            consumed_coins.push(entity_right);
                            coin_pickups.send(CoinPickup {
                                coin: entity_right,
                                target: position,
                                add_money: false,
                            });

                            spew_coin(position, money_left.0 + money_right.0, -PI / 2.0);
                        }

                        _ => (),
                    }
                }

                Machine::ConveyorUp => {
                    let coin_stuff =
                        find_coin(tile_pos).or_else(|| find_coin(tile_pos.offset(0, -1)));

                    if let Some((entity, _, money)) = coin_stuff {
                        consumed_coins.push(entity);
                        coin_pickups.send(CoinPickup {
                            coin: entity,
                            target: position,
                            add_money: false,
                        });
                        spew_coin(position, money.0, PI / 2.0);
                    }
                }

                Machine::ConveyorDown => {
                    let coin_stuff =
                        find_coin(tile_pos).or_else(|| find_coin(tile_pos.offset(0, 1)));

                    if let Some((entity, _, money)) = coin_stuff {
                        consumed_coins.push(entity);
                        coin_pickups.send(CoinPickup {
                            coin: entity,
                            target: position,
                            add_money: false,
                        });
                        spew_coin(position, money.0, -PI / 2.0);
                    }
                }

                Machine::ConveyorLeft => {
                    let coin_stuff =
                        find_coin(tile_pos).or_else(|| find_coin(tile_pos.offset(1, 0)));

                    if let Some((entity, _, money)) = coin_stuff {
                        consumed_coins.push(entity);
                        coin_pickups.send(CoinPickup {
                            coin: entity,
                            target: position,
                            add_money: false,
                        });
                        spew_coin(position, money.0, PI);
                    }
                }

                Machine::ConveyorRight => {
                    let coin_stuff =
                        find_coin(tile_pos).or_else(|| find_coin(tile_pos.offset(-1, 0)));

                    if let Some((entity, _, money)) = coin_stuff {
                        consumed_coins.push(entity);
                        coin_pickups.send(CoinPickup {
                            coin: entity,
                            target: position,
                            add_money: false,
                        });
                        spew_coin(position, money.0, 0.0);
                    }
                }

                Machine::Multiplier => {
                    let coin_left = find_coin(tile_pos.offset(-1, 0));
                    let coin_right = find_coin(tile_pos.offset(1, 0));

                    match (coin_left, coin_right) {
                        (
                            Some((entity_left, _coin_left, money_left)),
                            Some((entity_right, _coin_right, money_right)),
                        ) => {
                            consumed_coins.push(entity_left);
                            coin_pickups.send(CoinPickup {
                                coin: entity_left,
                                target: position,
                                add_money: false,
                            });
                            consumed_coins.push(entity_right);
                            coin_pickups.send(CoinPickup {
                                coin: entity_right,
                                target: position,
                                add_money: false,
                            });

                            spew_coin(position, money_left.0 * money_right.0, -PI / 2.0);
                        }

                        _ => (),
                    }
                }
            }
        }

        for coin_entity in consumed_coins.iter() {
            let (mut coin, _) = coins.get_mut(*coin_entity).unwrap();
            coin.alive = false;
        }
    }
}

pub fn destroy_machines(
    mut commands: Commands,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
    machines: Query<&PlacedMachine, Without<Spot>>,
    spots: Query<&Spot, Without<PlacedMachine>>,
    tile_tracked_entities: Res<TileTrackedEntities>,
) {
    let try_despawn_spot = |tile_pos: TilePosition, commands: &mut Commands| {
        if let Some(entities) = tile_tracked_entities.get_entities_in_tile(tile_pos) {
            for &entity in entities {
                if let Ok(_) = spots.get(entity) {
                    commands.entity(entity).despawn_recursive();
                    break;
                }
            }
        }
    };

    for event in world_mouse_events.iter() {
        match event {
            WorldMouseEvent::RightClick { position } => {
                let tile_pos = TilePosition::from_world(*position);

                if let Some(entities) = tile_tracked_entities.get_entities_in_tile(tile_pos) {
                    for &machine_entity in entities {
                        if let Ok(machine) = machines.get(machine_entity) {
                            commands.entity(machine_entity).despawn_recursive();

                            match machine.machine {
                                Machine::Miner => {
                                    try_despawn_spot(tile_pos.offset(0, -1), &mut commands)
                                }
                                Machine::Collector => {
                                    try_despawn_spot(tile_pos.offset(0, 1), &mut commands)
                                }
                                Machine::ConveyorUp => {
                                    try_despawn_spot(tile_pos.offset(0, 1), &mut commands);
                                    try_despawn_spot(tile_pos.offset(0, -1), &mut commands);
                                }
                                Machine::ConveyorDown => {
                                    try_despawn_spot(tile_pos.offset(0, 1), &mut commands);
                                    try_despawn_spot(tile_pos.offset(0, -1), &mut commands);
                                }
                                Machine::ConveyorLeft => {
                                    try_despawn_spot(tile_pos.offset(1, 0), &mut commands);
                                    try_despawn_spot(tile_pos.offset(-1, 0), &mut commands);
                                }
                                Machine::ConveyorRight => {
                                    try_despawn_spot(tile_pos.offset(1, 0), &mut commands);
                                    try_despawn_spot(tile_pos.offset(-1, 0), &mut commands);
                                }
                                Machine::Adder => {
                                    try_despawn_spot(tile_pos.offset(1, 0), &mut commands);
                                    try_despawn_spot(tile_pos.offset(-1, 0), &mut commands);
                                    try_despawn_spot(tile_pos.offset(0, -1), &mut commands);
                                }
                                Machine::Multiplier => {
                                    try_despawn_spot(tile_pos.offset(1, 0), &mut commands);
                                    try_despawn_spot(tile_pos.offset(-1, 0), &mut commands);
                                    try_despawn_spot(tile_pos.offset(0, -1), &mut commands);
                                }
                            }
                        }
                    }
                }
            }

            _ => (),
        }
    }
}
