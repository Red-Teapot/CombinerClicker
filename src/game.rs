use bevy::math::vec3;
use bevy::prelude::*;
use bevy::prelude::Val::Percent;
use bevy_kira_audio::AudioPlugin;
use bevy_ninepatch::NinePatchPlugin;
use bevy_tweening::TweeningPlugin;
use iyes_loopless::prelude::*;
use crate::assets::GameAssets;
use crate::{assets, gameplay, palette, title};
use crate::gameplay::components::{CoinPickup, WorldMouseEvent};

pub fn run(app: &mut App) {
    app.insert_resource(WindowDescriptor {
        title: "Combiner Clicker".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(AudioPlugin)
    .add_plugin(NinePatchPlugin::<()>::default())
    .add_plugin(TweeningPlugin)
    .add_plugin(GamePlugin);

    #[cfg(all(target_os = "windows", debug_assertions))]
    {
        app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());
    };

    app.run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(palette::OFF_WHITE))
            .add_startup_system(assets::load_assets)
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
                .into());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[derive(SystemLabel)]
enum GameSystemLabel {
    BeforeGeneral,
    General,
}

pub fn startup_game(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        ..default()
    });

    commands.spawn_bundle(ButtonBundle {
        style: Style {
            position_type: PositionType::Absolute,
            size: Size::new(Percent(100.0), Percent(100.0)),
            ..default()
        },
        color: Color::NONE.into(),
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
