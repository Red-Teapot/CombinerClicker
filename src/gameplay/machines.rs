use bevy::prelude::*;

use std::{f32::consts::PI, time::Duration};

use crate::assets::{Fonts, Images};

use super::{
    components::{Coin, CoinPickup, Money, NextCoinDepth, Spot},
    input::WorldMouseEvent,
    systems::spawn_coin,
    tile_tracked_entities::{TilePosition, TileTrackedEntities},
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

    pub fn cost(&self) -> u128 {
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
}

#[derive(Component)]
pub struct PlacedMachine {
    pub machine: Machine,
    pub action_timer: Timer,
}
