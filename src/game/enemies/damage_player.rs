use bevy::{prelude::*, sprite::collide_aabb};

use crate::game::{components::{Health, Player, DamageCooldown, Damage, Hitbox, CollidesPlayer}, health_bar::WithHealthBar};

//for system that damages the player

pub fn damage_player(
    mut commands: Commands,
    mut player_query: Query<(&mut Health, &GlobalTransform, &Hitbox), With<Player>>,
    mut query: Query<(Option<&mut DamageCooldown>, Option<&WithHealthBar>, &Damage, &GlobalTransform, &Hitbox, Entity), (With<CollidesPlayer>, Without<Player>)>,
) {
    let (mut player_health, player_trans, player_hitbox) = player_query.single_mut();
    for (damage_cooldown_op, health_bar_op, damage, enemy_trans, enemy_hitbox, entity) in query.iter_mut() {
        if collide_aabb::collide(player_trans.translation(), player_hitbox.0, enemy_trans.translation(), enemy_hitbox.0).is_some() {
            
            if let Some(mut damage_cooldown) = damage_cooldown_op {
                if damage_cooldown.timer.finished() {
                    player_health.current -= damage.0;
                    damage_cooldown.timer.reset();
                }
            } else {
                player_health.current -= damage.0;
                commands.entity(entity).despawn();

                if let Some(health_bar) = health_bar_op {
                    commands.entity(**health_bar).despawn();
                }
            }
        }
    }
}

pub fn update_damage_cooldowns(
    mut query: Query<&mut DamageCooldown>,
    time: Res<Time>,
) {
    for mut cooldown in query.iter_mut() {
        cooldown.timer.tick(time.delta());
    }
}

