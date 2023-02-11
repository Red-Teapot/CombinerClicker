use bevy::prelude::*;

use super::components::DelayedDespawn;

pub fn update_delayed_despawn(
    mut entities: Query<(Entity, &mut DelayedDespawn)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut delayed_despawn) in entities.iter_mut() {
        delayed_despawn.timer.tick(time.delta());

        if delayed_despawn.timer.just_finished() {
            if delayed_despawn.recursive {
                commands.entity(entity).despawn_recursive();
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}
