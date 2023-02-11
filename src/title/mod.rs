use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{can_use_mouse, GameState, GameSystemLabel};

pub mod components;
pub mod systems;

const FADE_OUT_TIME: f32 = 0.3;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Title, systems::startup_title)
            .add_system_set(
                ConditionSet::new()
                    .label(GameSystemLabel::InputHandling)
                    .before(GameSystemLabel::PreUpdate)
                    .run_in_state(GameState::Title)
                    .run_if(can_use_mouse)
                    .with_system(systems::handle_title_click)
                    .into(),
            );
    }
}
