use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bevy_ninepatch::NinePatchPlugin;
use bevy_tweening::TweeningPlugin;
use iyes_loopless::prelude::*;
use crate::assets::GameAssets;
use crate::{assets, palette, title};

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
            .add_loopless_state(GameState::Title)
            .add_enter_system(GameState::Title, title::startup_title)
            .add_system(title::handle_title_click.run_in_state(GameState::Title));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Title,
    Gameplay,
}