use bevy::prelude::*;
use crate::assets::GameAssets;

#[derive(Component, Copy, Clone)]
pub enum Machine {
    Miner,
    Collector,
    Adder,
    Multiplicator,
}

impl Machine {
    pub fn list() -> &'static [Machine] {
        use crate::gameplay::components::Machine::*;

        &[Miner, Collector, Adder, Multiplicator]
    }

    pub fn cost(&self) -> usize {
        use crate::gameplay::components::Machine::*;

        match self {
            Miner => 20,
            Collector => 200,
            Adder => 500,
            Multiplicator => 1000,
        }
    }

    pub fn image(&self, assets: &GameAssets) -> Handle<Image> {
        use crate::gameplay::components::Machine::*;

        match self {
            Miner => assets.miner.clone(),
            Collector => assets.collector.clone(),
            Adder => assets.adder.clone(),
            Multiplicator => assets.multiplicator.clone(),
        }
    }

    pub fn name(&self) -> &str {
        use crate::gameplay::components::Machine::*;

        match self {
            Miner => "Miner",
            Collector => "Collector",
            Adder => "Adder",
            Multiplicator => "Multiplier",
        }
    }
}

#[derive(Component)]
pub struct Money(pub usize);

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
    pub damping: f64,
}

#[derive(Copy, Clone)]
pub enum WorldMouseState {
    None,
    Pressed {
        time: f64,
        position_window: Vec2,
        position_world: Vec2,
    },
    Dragging {
        start_position_window: Vec2,
    },
}
