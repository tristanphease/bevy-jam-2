use bevy::{prelude::*, sprite::collide_aabb};

use crate::game::components::{Health, Player, DamageCooldown, DamagesPlayer, Hitbox};

//for system that damages the player

pub fn damage_player(
    mut commands: Commands,
    mut player_query: Query<(&mut Health, &Transform, &Hitbox), With<Player>>,
    mut query: Query<(Option<&mut DamageCooldown>, &DamagesPlayer, &Transform, &Hitbox, Entity), Without<Player>>,
) {
    let (mut player_health, player_trans, player_hitbox) = player_query.single_mut();
    for (damage_cooldown_op, damage, enemy_trans, enemy_hitbox, entity) in query.iter_mut() {
        if collide_aabb::collide(player_trans.translation, player_hitbox.0, enemy_trans.translation, enemy_hitbox.0).is_some() {
            
            if let Some(mut damage_cooldown) = damage_cooldown_op {
                if damage_cooldown.cooldown_timer.finished() {
                    player_health.current -= damage.damage;
                    damage_cooldown.cooldown_timer.reset();
                }
            } else {
                player_health.current -= damage.damage;
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn update_damage_cooldowns(
    mut query: Query<&mut DamageCooldown>,
    time: Res<Time>,
) {
    for mut cooldown in query.iter_mut() {
        cooldown.cooldown_timer.tick(time.delta());
    }
}

