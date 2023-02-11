use std::time::Duration;

use bevy::{
    prelude::Component,
    time::{Timer, TimerMode},
};

#[derive(Component)]
pub struct DelayedDespawn {
    pub timer: Timer,
    pub recursive: bool,
}

impl DelayedDespawn {
    pub fn with_children(delay: Duration) -> DelayedDespawn {
        DelayedDespawn {
            timer: Timer::new(delay, TimerMode::Once),
            recursive: true,
        }
    }
}
