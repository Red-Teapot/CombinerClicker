use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{can_use_mouse, GameState, GameSystemLabel};

use self::{components::*, hud::MachineButtonSelectedEvent};

pub mod components;
pub mod systems;

pub mod input;
pub mod hud;
pub mod machines;
pub mod tile_tracked_entities;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<input::WorldMouseEvent>()
            .add_event::<MachineButtonSelectedEvent>()
            .add_event::<CoinPickup>();

        app.add_system_set(
            ConditionSet::new()
                .label(GameSystemLabel::InputHandling)
                .before(GameSystemLabel::PreUpdate)
                .run_if(can_use_mouse)
                .run_in_state(GameState::Gameplay)
                .with_system(input::handle_bg_input)
                .with_system(input::zoom_camera)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Gameplay)
                .label(GameSystemLabel::PreUpdate)
                .before(GameSystemLabel::Update)
                .with_system(tile_tracked_entities::track_tile_entities)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Gameplay)
                .label(GameSystemLabel::Update)
                .before(GameSystemLabel::PostUpdate)
                .with_system(input::drag_camera)
                .with_system(systems::click_coins)
                .with_system(systems::hover_coins)
                .with_system(systems::update_coins)
                .with_system(systems::move_particles)
                .with_system(hud::update_wallet)
                .with_system(hud::select_machine_button)
                .with_system(systems::drag_ghosts)
                .with_system(systems::place_ghosts)
                .with_system(machines::act_machines)
                .with_system(machines::destroy_machines)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Gameplay)
                .label(GameSystemLabel::PostUpdate)
                .with_system(hud::update_selected_machine_button)
                .into(),
        );
    }
}
