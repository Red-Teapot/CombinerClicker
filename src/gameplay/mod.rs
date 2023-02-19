use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{can_use_mouse, GameState, GameSystemLabel};

use self::{
    components::*,
    hud::ToolbarButtonSelectedEvent,
    machines::{MachinePlaceRequest, UpdateSpotsRequest, MachineDeleteRequest},
};

pub mod components;
pub mod systems;

pub mod hud;
pub mod input;
pub mod machines;
pub mod tile_tracked_entities;

pub const TILE_SIZE: f32 = 64.0 * 4.0;
pub const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Gameplay, systems::startup_gameplay)
            .add_enter_system(GameState::Gameplay, hud::setup_hud);

        app.add_event::<input::WorldMouseEvent>()
            .add_event::<ToolbarButtonSelectedEvent>()
            .add_event::<MachinePlaceRequest>()
            .add_event::<MachineDeleteRequest>()
            .add_event::<UpdateSpotsRequest>()
            .add_event::<CoinPickup>();

        app.add_system_set(
            ConditionSet::new()
                .label(GameSystemLabel::InputHandling)
                .before(GameSystemLabel::PreUpdate)
                .run_if(can_use_mouse)
                .run_in_state(GameState::Gameplay)
                .with_system(input::handle_bg_input)
                .with_system(input::zoom_camera)
                .with_system(hud::hide_building_ghost_on_right_click)
                .with_system(hud::ghost_place_machine)
                .with_system(hud::ghost_delete_machine)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Gameplay)
                .label(GameSystemLabel::PreUpdate)
                .before(GameSystemLabel::Update)
                .with_system(tile_tracked_entities::track_tile_entities)
                .with_system(machines::update_spots)
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
                .with_system(hud::update_balance_display)
                .with_system(hud::select_toolbar_button)
                .with_system(hud::drag_building_ghost)
                .with_system(machines::act_machines)
                .with_system(machines::place_machines)
                .with_system(machines::delete_machines)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Gameplay)
                .label(GameSystemLabel::PostUpdate)
                .with_system(hud::update_selected_machine_button)
                .with_system(hud::show_hide_building_ghost)
                .into(),
        );
    }
}
