use crate::assets::Images;
use bevy::prelude::*;
use std::time::Duration;

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
        use crate::gameplay::components::Machine::*;

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
        use crate::gameplay::components::Machine::*;

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
        use crate::gameplay::components::Machine::*;

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
        use crate::gameplay::components::Machine::*;

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
        use crate::gameplay::components::Machine::*;

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

#[derive(Component, Resource)]
pub struct Money(pub u128);

#[derive(Component)]
pub struct MoneyDisplay;

#[derive(Component)]
pub struct MachineIcon(pub Machine);

#[derive(Component)]
pub struct MachineName(pub Machine);

#[derive(Component)]
pub struct MachineBuyButton {
    pub enabled: bool,
    pub machine: Machine,
}

#[derive(Component)]
pub struct Particle {
    pub velocity: Vec2,
    pub damping: f32,
}

#[derive(Component)]
pub struct Coin {
    pub spawn_timer: Timer,
    pub despawn_timer: Timer,
    pub has_money: bool,
    pub alive: bool,
}

#[derive(Resource)]
pub struct NextCoinDepth {
    pub depth: f32,
    pub step: f32,
}

pub struct CoinPickup {
    pub coin: Entity,
    pub target: Vec2,
    pub add_money: bool,
}

impl Coin {
    pub fn pickable(&self) -> bool {
        self.alive && self.spawn_timer.finished() && self.despawn_timer.paused()
    }
}

#[derive(Component)]
pub enum BuildingGhost {
    Spot { offset_x: i32, offset_y: i32 },
    Machine(Machine),
}

#[derive(Component)]
pub struct Spot;

#[derive(Component)]
pub struct PlacedMachine {
    pub machine: Machine,
    pub action_timer: Timer,
}
