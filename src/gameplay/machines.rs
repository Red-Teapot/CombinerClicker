use bevy::{math::vec3, prelude::*};

use std::{f32::consts::PI, time::Duration};

use crate::{
    assets::{Fonts, Images},
    gameplay::TILE_SIZE,
};

use super::{
    components::{Balance, Coin, CoinPickup, Currency, Money, NextCoinDepth},
    systems::spawn_coin,
    tile_tracked_entities::{TilePosition, TileTrackedEntities, TileTrackedEntity},
    HALF_TILE_SIZE,
};

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

            let mut spew_coin = |position: Vec2, value: Currency, angle: f32| {
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

pub fn place_machines(
    mut commands: Commands,
    mut requests: EventReader<MachinePlaceRequest>,
    mut balance: ResMut<Balance>,
    tile_tracked_entities: Res<TileTrackedEntities>,
    machines: Query<&PlacedMachine>,
    images: Res<Images>,
    mut update_spots_requests: EventWriter<UpdateSpotsRequest>,
) {
    'requests: for request in requests.iter() {
        let machine = request.machine;
        let machine_cost = machine.cost();

        if machine_cost > balance.coins {
            continue 'requests;
        }

        if let Some(entities) = tile_tracked_entities.get_entities_in_tile(request.position) {
            for tile_entity in entities {
                if machines.get(*tile_entity).is_ok() {
                    continue 'requests;
                }
            }
        }

        balance.coins -= machine_cost;
        let placed_machine = machine.spawn_graphics(&mut commands, &images, true);
        commands
            .entity(placed_machine)
            .insert(Transform::from_translation(
                (request.position.to_world() + Vec2::splat(HALF_TILE_SIZE)).extend(0.0),
            ))
            .insert(PlacedMachine {
                machine,
                action_timer: Timer::new(machine.action_period(), TimerMode::Repeating),
            })
            .insert(TileTrackedEntity);

        let update_positions = vec![
            request.position,
            request.position.offset(-1, 0),
            request.position.offset(1, 0),
            request.position.offset(0, -1),
            request.position.offset(0, 1),
        ];

        for position in update_positions {
            update_spots_requests.send(UpdateSpotsRequest {
                position,
            });
        }
    }
}

pub fn delete_machines(
    mut commands: Commands,
    mut requests: EventReader<MachineDeleteRequest>,
    tile_tracked_entities: Res<TileTrackedEntities>,
    machines: Query<&PlacedMachine>,
    mut update_spots_requests: EventWriter<UpdateSpotsRequest>,
) {
    for request in requests.iter() {
        let mut did_delete = false;

        if let Some(entities) = tile_tracked_entities.get_entities_in_tile(request.position) {
            for tile_entity in entities {
                if machines.get(*tile_entity).is_ok() {
                    commands.entity(*tile_entity).despawn_recursive();
                    did_delete = true;
                }
            }
        }

        if did_delete {
            update_spots_requests.send(UpdateSpotsRequest {
                position: request.position,
            });
        }
    }
}

pub fn update_spots(
    mut requests: EventReader<UpdateSpotsRequest>,
    tile_tracked_entities: Res<TileTrackedEntities>,
    machines: Query<&PlacedMachine>,
    mut spots: Query<&mut Visibility, With<Spot>>,
) {
    for request in requests.iter() {
        let mut has_machine = false;

        if let Some(entities) = tile_tracked_entities.get_entities_in_tile(request.position) {
            for entity in entities {
                has_machine |= machines.contains(*entity);
            }
        }

        if let Some(entities) = tile_tracked_entities.get_entities_in_tile(request.position) {
            for entity in entities {
                if let Ok(mut visibility) = spots.get_mut(*entity) {
                    visibility.is_visible = !has_machine;
                }
            }
        }
    }
}

#[derive(Component, Copy, Clone)]
pub enum Machine {
    Miner,
    Collector,
    ConveyorUp,
    ConveyorDown,
    ConveyorLeft,
    ConveyorRight,
    Adder,
    Multiplier,
}

impl Machine {
    pub fn list() -> &'static [Machine] {
        use Machine::*;

        &[
            Miner,
            Collector,
            ConveyorUp,
            ConveyorDown,
            ConveyorLeft,
            ConveyorRight,
            Adder,
            Multiplier,
        ]
    }

    pub fn cost(&self) -> Currency {
        use Machine::*;

        match self {
            Miner => 20,
            Collector => 200,
            ConveyorUp => 10,
            ConveyorDown => 10,
            ConveyorLeft => 10,
            ConveyorRight => 10,
            Adder => 500,
            Multiplier => 1000,
        }
    }

    pub fn image(&self, assets: &Images) -> Handle<Image> {
        use Machine::*;

        match self {
            Miner => assets.miner.clone(),
            Collector => assets.collector.clone(),
            ConveyorUp => assets.conveyor_up.clone(),
            ConveyorDown => assets.conveyor_down.clone(),
            ConveyorLeft => assets.conveyor_left.clone(),
            ConveyorRight => assets.conveyor_right.clone(),
            Adder => assets.adder.clone(),
            Multiplier => assets.multiplier.clone(),
        }
    }

    pub fn name(&self) -> &str {
        use Machine::*;

        match self {
            Miner => "Miner",
            Collector => "Collector",
            ConveyorUp => "Up Conveyor",
            ConveyorDown => "Down Conveyor",
            ConveyorLeft => "Left Conveyor",
            ConveyorRight => "Right Conveyor",
            Adder => "Adder",
            Multiplier => "Multiplier",
        }
    }

    pub fn action_period(&self) -> Duration {
        use Machine::*;

        match self {
            Miner => Duration::from_secs_f32(1.0),
            Collector => Duration::from_secs_f32(0.1),
            ConveyorUp => Duration::from_secs_f32(0.2),
            ConveyorDown => Duration::from_secs_f32(0.2),
            ConveyorLeft => Duration::from_secs_f32(0.2),
            ConveyorRight => Duration::from_secs_f32(0.2),
            Adder => Duration::from_secs_f32(1.0),
            Multiplier => Duration::from_secs_f32(1.0),
        }
    }

    pub fn spawn_graphics(&self, commands: &mut Commands, images: &Images, is_placed: bool) -> Entity {
        use Machine::*;

        let (spot_up, spot_down, spot_left, spot_right) = match self {
            Miner => (false, true, false, false),
            Collector => (true, false, false, false),
            ConveyorUp => (true, true, false, false),
            ConveyorDown => (true, true, false, false),
            ConveyorLeft => (false, false, true, true),
            ConveyorRight => (false, false, true, true),
            Adder => (false, true, true, true),
            Multiplier => (false, true, true, true),
        };

        commands
            .spawn(SpriteBundle {
                texture: self.image(images),
                ..default()
            })
            .with_children(|machine| {
                if spot_up {
                    let mut spot = machine.spawn(SpriteBundle {
                        texture: images.spot.clone(),
                        transform: Transform::from_translation(vec3(0.0, TILE_SIZE, -0.01)),
                        ..default()
                    });

                    if is_placed {
                        spot.insert(Spot).insert(TileTrackedEntity);
                    }
                }

                if spot_down {
                    let mut spot = machine.spawn(SpriteBundle {
                        texture: images.spot.clone(),
                        transform: Transform::from_translation(vec3(0.0, -TILE_SIZE, -0.01)),
                        ..default()
                    });

                    if is_placed {
                        spot.insert(Spot).insert(TileTrackedEntity);
                    }
                }

                if spot_left {
                    let mut spot = machine.spawn(SpriteBundle {
                        texture: images.spot.clone(),
                        transform: Transform::from_translation(vec3(-TILE_SIZE, 0.0, -0.01)),
                        ..default()
                    });

                    if is_placed {
                        spot.insert(Spot).insert(TileTrackedEntity);
                    }
                }

                if spot_right {
                    let mut spot = machine.spawn(SpriteBundle {
                        texture: images.spot.clone(),
                        transform: Transform::from_translation(vec3(TILE_SIZE, 0.0, -0.01)),
                        ..default()
                    });

                    if is_placed {
                        spot.insert(Spot).insert(TileTrackedEntity);
                    }
                }
            })
            .id()
    }
}

#[derive(Component)]
pub struct PlacedMachine {
    pub machine: Machine,
    pub action_timer: Timer,
}

pub struct MachinePlaceRequest {
    pub machine: Machine,
    pub position: TilePosition,
}

pub struct MachineDeleteRequest {
    pub position: TilePosition,
}

pub struct UpdateSpotsRequest {
    pub position: TilePosition,
}

#[derive(Component)]
pub struct Spot;
