use bevy::prelude::*;

#[derive(Component)]
pub struct TitleHint;

#[derive(Resource)]
pub struct TitleFadeOut {
    pub timer: Timer,
}
