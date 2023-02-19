use std::f32::consts::PI;
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

use super::hud::{BuildingGhost, MachineBuyButton, MachineIcon, MachineName, MoneyDisplay};
use super::input::WorldMouseEvent;
use super::machines::Machine;
use super::tile_tracked_entities::{TilePosition, TileTrackedEntities, TileTrackedEntity};

pub fn startup_gameplay(
    mut commands: Commands,
    fonts: Res<Fonts>,
    images: Res<Images>,
    ninepatches: Res<NinePatches>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    camera.single_mut().scale = vec3(4.0, 4.0, 1.0);

    commands.insert_resource(Balance::default());

    commands.insert_resource(NextCoinDepth {
        depth: 0.1,
        step: 0.00000001,
    });

    commands.insert_resource(super::input::WorldMouse::default());

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
                            .insert(MachineBuyButton::new(*machine));
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

pub fn spawn_coin(
    commands: &mut Commands,
    depth: &mut ResMut<NextCoinDepth>,
    fonts: &Res<Fonts>,
    game_images: &Res<Images>,
    value: Currency,
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
    building_ghosts: Query<&BuildingGhost>,
    fonts: Res<Fonts>,
    game_images: Res<Images>,
    mut depth: ResMut<NextCoinDepth>,
    mut world_mouse_events: EventReader<WorldMouseEvent>,
) {
    if !building_ghosts.is_empty() {
        world_mouse_events.clear();
        return;
    }

    for event in world_mouse_events.iter() {
        match event {
            WorldMouseEvent::Click {
                button: MouseButton::Left,
                position,
            } => {
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
    mut wallet: ResMut<Balance>,
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
                wallet.coins += coin_money.0;
            }

            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn hover_coins(
    coins: Query<(&Transform, &Coin)>,
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
