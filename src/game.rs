use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_ninepatch::NinePatchPlugin;
use bevy_tweening::TweeningPlugin;
use iyes_loopless::prelude::*;

use crate::assets::*;

#[cfg(target_arch = "wasm32")]
mod web_main;

pub mod assets;
pub mod common;
pub mod gameplay;
pub mod palette;
pub mod title;

pub fn run(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "One Clicker".to_string(),
                    ..default()
                },
                ..default()
            })
            .build()
            .add_before::<AssetPlugin, _>(EmbeddedAssetPlugin),
    )
    .add_plugin(AudioPlugin)
    .add_plugin(NinePatchPlugin::<()>::default())
    .add_plugin(TweeningPlugin)
    .insert_resource(InputHandlingBehavior {
        can_use_mouse: true,
        can_use_keyboard: true,
    });

    #[cfg(all(not(target_arch = "wasm32"), debug_assertions))]
    {
        use bevy_egui::EguiContext;

        app.add_plugin(bevy_inspector_egui::quick::WorldInspectorPlugin)
            .add_system_set(
                SystemSet::new()
                    .before(GameSystemLabel::InputHandling)
                    .with_system(
                        |mut egui_context: ResMut<EguiContext>,
                         mut behavior: ResMut<InputHandlingBehavior>| {
                            let ctx = egui_context.ctx_mut();
                            behavior.can_use_mouse =
                                !(ctx.is_pointer_over_area() || ctx.wants_pointer_input());
                            behavior.can_use_keyboard = !ctx.wants_keyboard_input();
                        },
                    ),
            );
    };

    app.init_collection::<Images>()
        .init_collection::<Fonts>()
        .init_resource::<NinePatches>()
        .add_plugin(GamePlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(palette::OFF_WHITE))
            .add_startup_system(startup_game)
            .add_loopless_state(GameState::Title)
            .add_enter_system(GameState::Gameplay, gameplay::systems::startup_gameplay);

        app.add_system(common::systems::update_delayed_despawn);

        app.add_plugin(title::TitlePlugin)
            .add_plugin(gameplay::GameplayPlugin);
    }
}

pub fn startup_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(WorldInteraction);
}

pub fn can_use_mouse(behavior: Res<InputHandlingBehavior>) -> bool {
    behavior.can_use_mouse
}

pub fn should_use_keyboard(behavior: Res<InputHandlingBehavior>) -> bool {
    behavior.can_use_keyboard
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Title,
    Gameplay,
}

#[derive(SystemLabel, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSystemLabel {
    InputHandling,
    PreUpdate,
    Update,
    PostUpdate,
}

#[derive(Resource)]
pub struct InputHandlingBehavior {
    pub can_use_mouse: bool,
    pub can_use_keyboard: bool,
}

#[derive(Component)]
pub struct WorldInteraction;
