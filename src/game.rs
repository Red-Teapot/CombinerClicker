use bevy::math::vec3;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_ninepatch::NinePatchPlugin;
use bevy_tweening::TweeningPlugin;
use iyes_loopless::prelude::*;

use crate::assets::*;
use crate::gameplay::components::{CoinPickup, Money, WorldMouseEvent};

#[cfg(target_arch = "wasm32")]
mod web_main;

pub mod palette;
pub mod assets;
pub mod title;
pub mod gameplay;

pub fn run(app: &mut App) {
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "One Clicker".to_string(),
            ..default()
        },
        ..default()
    }).build().add_before::<AssetPlugin, _>(EmbeddedAssetPlugin))
    .add_plugin(AudioPlugin)
    .add_plugin(NinePatchPlugin::<()>::default())
    .add_plugin(TweeningPlugin)
    .add_plugin(GamePlugin);

    #[cfg(all(target_os = "windows", debug_assertions))]
    {
        app.add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin);
    };

    app.init_collection::<Images>();
    app.init_collection::<Fonts>();
    app.init_resource::<NinePatches>();
    app.run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(palette::OFF_WHITE))
            .add_startup_system(startup_game)
            .add_loopless_state(GameState::Title)
            .add_enter_system(GameState::Title, title::startup_title)
            .add_system(title::handle_title_click.run_in_state(GameState::Title))
            .add_enter_system(GameState::Gameplay, gameplay::systems::startup_gameplay)
            .add_event::<WorldMouseEvent>()
            .add_event::<CoinPickup>()
            .add_system_set(ConditionSet::new()
                .run_in_state(GameState::Gameplay)
                .label(GameSystemLabel::BeforeGeneral)
                .before(GameSystemLabel::General)
                .with_system(gameplay::systems::track_tile_entities)
                .with_system(gameplay::systems::handle_bg_input)
                .into())
            .add_system_set(ConditionSet::new()
                .run_in_state(GameState::Gameplay)
                .label(GameSystemLabel::General)
                .with_system(gameplay::systems::zoom_camera)
                .with_system(gameplay::systems::drag_camera)
                .with_system(gameplay::systems::click_coins)
                .with_system(gameplay::systems::hover_coins)
                .with_system(gameplay::systems::update_coins)
                .with_system(gameplay::systems::move_particles)
                .with_system(gameplay::systems::update_money)
                .with_system(gameplay::systems::handle_machine_buy_buttons)
                .with_system(gameplay::systems::drag_ghosts)
                .with_system(gameplay::systems::place_ghosts)
                .with_system(gameplay::systems::act_machines)
                .with_system(gameplay::systems::destroy_machines)
                .into())
            .add_system_set(ConditionSet::new()
                .run_in_state(GameState::Gameplay)
                .label(GameSystemLabel::AfterGeneral)
                .after(GameSystemLabel::General)
                .into());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
enum GameSystemLabel {
    BeforeGeneral,
    General,
    AfterGeneral,
}

pub fn startup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        ..default()
    });

    commands.spawn(ButtonBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        background_color: Color::NONE.into(),
        ..default()
    }).insert(BackgroundInteraction);
}

#[derive(Component)]
pub struct BackgroundInteraction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Title,
    Gameplay,
}
