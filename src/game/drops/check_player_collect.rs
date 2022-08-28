use bevy::{prelude::*, sprite::collide_aabb};

use crate::game::{components::{Hitbox, Player}, waves::waves::WaveInfo};

use super::ItemDrop;

pub fn check_player_drops(
    mut commands: Commands,
    player_query: Query<(&GlobalTransform, &Hitbox), With<Player>>,
    drops_query: Query<(&GlobalTransform, &Hitbox, Entity), With<ItemDrop>>,
    mut wave_info: ResMut<WaveInfo>,
) {
    let (player_trans, player_hitbox) = player_query.single();

    for (drop_trans, drop_hitbox, drop_entity) in drops_query.iter() {
        if collide_aabb::collide(
            player_trans.translation(),
            **player_hitbox,
            drop_trans.translation(),
            **drop_hitbox,
        ).is_some() {
            wave_info.add_drop();

            commands.entity(drop_entity).despawn();
        }
    }
}