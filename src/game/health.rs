use bevy::prelude::*;

use super::components::{Health, Player};

pub fn check_entity_death(
    mut commands: Commands,
    query: Query<(Entity, &Health), Without<Player>>,
) {
    for (entity, health) in query.iter() {
        if health.current <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}