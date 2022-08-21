use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;

pub fn run(app: &mut App) {
    app.insert_resource(WindowDescriptor {
        title: "KPACUBO".to_string(),
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(AudioPlugin)
    .run();
}
