use bevy::prelude::*;

use super::{components::{Health, Player}, health_bar::WithHealthBar};

pub fn check_entity_death(
    mut commands: Commands,
    query: Query<(Entity, &Health, Option<&WithHealthBar>), Without<Player>>,
) {
    for (entity, health, health_bar_option) in query.iter() {
        if health.current <= 0.0 {
            commands.entity(entity).despawn();

            if let Some(health_bar) = health_bar_option {
                commands.entity(health_bar.0).despawn();
            }
        }
    }
}