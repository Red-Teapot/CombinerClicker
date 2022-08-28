use std::time::Duration;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::Inspectable;
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

    pub fn action_period(&self) -> Duration {
        use crate::gameplay::components::Machine::*;

        match self {
            Miner => Duration::from_secs_f32(5.0),
            Collector => Duration::from_secs_f32(1.0),
            Adder => Duration::from_secs_f32(5.0),
            Multiplicator => Duration::from_secs_f32(5.0),
        }
    }
}

#[derive(Component, Inspectable, Default)]
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
    pub damping: f32,
}

#[derive(Component)]
pub struct Coin {
    pub spawn_timer: Timer,
    pub despawn_timer: Timer,
    pub has_money: bool,
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

#[derive(Copy, Clone)]
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
    Click {
        position: Vec2,
    },
    Hover {
        position: Vec2,
    },
    Drag {
        offset: Vec2,
    }
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
        TilePosition {
            x,
            y,
        }
    }

    pub fn from_world(position: Vec2) -> TilePosition {
        TilePosition {
            x: (position.x / Self::TILE_SIZE).floor() as i32,
            y: (position.y / Self::TILE_SIZE).floor() as i32,
        }
    }

    pub fn to_world(&self) -> Vec2 {
        vec2((self.x as f32) * Self::TILE_SIZE, (self.y as f32) * Self::TILE_SIZE)
    }

    pub fn offset(&self, x: i32, y: i32) -> TilePosition {
        TilePosition {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

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
            self.map.insert(tile_pos, vec!(entity));
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
    Spot {
        offset_x: i32,
        offset_y: i32,
    },
    Machine(Machine),
}

#[derive(Component)]
pub struct Spot;

#[derive(Component)]
pub struct PlacedMachine {
    pub machine: Machine,
    pub action_timer: Timer,
}