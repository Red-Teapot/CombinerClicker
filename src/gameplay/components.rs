use bevy::prelude::*;

use super::machines::Machine;

pub type Currency = u128;

#[derive(Resource)]
pub struct Balance {
    pub money: Currency,
}

impl Default for Balance {
    fn default() -> Self {
        Self { 
            money: 0,
        }
    }
}

#[derive(Component)]
pub struct Money(pub Currency);

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
