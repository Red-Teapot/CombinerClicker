use std::f32::consts::PI;
use std::time::Duration;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy_tweening::lens::{TransformPositionLens, TransformScaleLens};
use bevy_tweening::*;

use crate::assets::*;
use crate::gameplay::components::*;
use crate::palette;

use super::hud::ToolGhost;
use super::input::WorldMouseEvent;
use super::tile_tracked_entities::{TilePosition, TileTrackedEntities, TileTrackedEntity};

pub fn startup_gameplay(mut commands: Commands, mut camera: Query<&mut Transform, With<Camera2d>>) {
    camera.single_mut().scale = vec3(4.0, 4.0, 1.0);

    commands.insert_resource(Balance::default());

    commands.insert_resource(NextCoinDepth {
        depth: 0.1,
        step: 0.00000001,
    });

    commands.insert_resource(super::input::WorldMouse::default());

    commands.insert_resource(TileTrackedEntities::new());
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
    building_ghosts: Query<&ToolGhost>,
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
