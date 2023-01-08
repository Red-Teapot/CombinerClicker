use crate::assets::Images;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::utils::HashMap;
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
        self.spawn_timer.finished() && self.despawn_timer.paused()
    }
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

#[derive(Component)]
pub struct TileTrackedEntity;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

impl TilePosition {
    const TILE_SIZE: f32 = 64.0 * 4.0;

    pub fn new(x: i32, y: i32) -> TilePosition {
        TilePosition { x, y }
    }

    pub fn from_world(position: Vec2) -> TilePosition {
        TilePosition {
            x: (position.x / Self::TILE_SIZE).floor() as i32,
            y: (position.y / Self::TILE_SIZE).floor() as i32,
        }
    }

    pub fn to_world(&self) -> Vec2 {
        vec2(
            (self.x as f32) * Self::TILE_SIZE,
            (self.y as f32) * Self::TILE_SIZE,
        )
    }

    pub fn offset(&self, x: i32, y: i32) -> TilePosition {
        TilePosition {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

#[derive(Resource)]
pub struct TileTrackedEntities {
    map: HashMap<TilePosition, Vec<Entity>>,
}

impl TileTrackedEntities {
    pub fn new() -> TileTrackedEntities {
        TileTrackedEntities {
            map: HashMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn add(&mut self, world_position: Vec2, entity: Entity) {
        let tile_pos = TilePosition::from_world(world_position);

        if let Some(vec) = self.get_entities_in_tile_mut(tile_pos) {
            vec.push(entity);
        } else {
            self.map.insert(tile_pos, vec![entity]);
        }
    }

    pub fn get_entities_in_tile(&self, tile_pos: TilePosition) -> Option<&Vec<Entity>> {
        self.map.get(&tile_pos)
    }

    pub fn get_entities_in_tile_mut(&mut self, tile_pos: TilePosition) -> Option<&mut Vec<Entity>> {
        self.map.get_mut(&tile_pos)
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
