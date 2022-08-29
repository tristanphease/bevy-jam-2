use std::f32::consts::PI;

use bevy::{prelude::*, utils::HashSet};

use crate::game::{components::{Health, CollidesEnemy, Hitbox, Damage}, util::check_collision_line_rectangle};

pub const ZAP_IMAGE_PATH: &str = "images/zap.png";
const ZAP_IMAGE_SIZE: Vec2 = Vec2::new(10.0, 200.0);

const NUM_ZAPS: usize = 10;
const ZAP_TIME_LENGTH: f32 = 0.5;

const ZAP_DAMAGE: f32 = 15.0;

#[derive(Component)]
pub struct ZapSpell {
    timer: Timer,
    start_pos: Vec2,
    end_pos: Vec2,
    zap_ids: [Entity; NUM_ZAPS],
    entities_zapped: HashSet<Entity>,
}

pub fn create_zap_spell(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
    angle: f32,
) {
    let image_handle = asset_server.load(ZAP_IMAGE_PATH);

    let angle = angle - PI/2.0;
    
    let mut zap_entities: [Entity; NUM_ZAPS] = [Entity::from_raw(0); NUM_ZAPS];
    for i in 0..NUM_ZAPS {

        let pos = Vec3::new(
            position.x + (i as f32 + 0.5) * f32::sin(-angle) * ZAP_IMAGE_SIZE.y,
            position.y + (i as f32 + 0.5) * f32::cos(-angle) * ZAP_IMAGE_SIZE.y,
            1.0,
        );

        let zap_id = commands.spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(ZAP_IMAGE_SIZE),
                ..default()
            },
            transform: Transform { 
                translation: pos, 
                rotation: Quat::from_rotation_z(angle),
                ..default()
            },
            texture: image_handle.clone(),
            ..default()
        }).id();

        zap_entities[i] = zap_id;
    }

    commands.spawn()
        .insert(ZapSpell {
            timer: Timer::from_seconds(ZAP_TIME_LENGTH, false),
            start_pos: position, 
            end_pos: Vec2::new(
                position.x + (NUM_ZAPS as f32 - 0.5) * f32::sin(-angle) * ZAP_IMAGE_SIZE.y,
                position.y + (NUM_ZAPS as f32 - 0.5) * f32::cos(-angle) * ZAP_IMAGE_SIZE.y,
            ),
            zap_ids: zap_entities,
            entities_zapped: HashSet::new(),
        })
        .insert(Damage(ZAP_DAMAGE));
    
}

pub fn update_zaps(
    mut commands: Commands,
    mut zap_query: Query<(Entity, &mut ZapSpell, &Damage)>, 
    time: Res<Time>,
    mut enemy_query: Query<(&mut Health, &GlobalTransform, &Hitbox, Entity), With<CollidesEnemy>>,
) {
    for (entity, mut zap, zap_damage) in zap_query.iter_mut() {
        zap.timer.tick(time.delta());

        if zap.timer.finished() {
            for &zap_entity in zap.zap_ids.iter() {
                commands.entity(zap_entity).despawn();
            }
            commands.entity(entity).despawn();
        } else {
            for (mut enemy_health, enemy_trans, enemy_hitbox, enemy_entity) in enemy_query.iter_mut() {
                if !zap.entities_zapped.contains(&enemy_entity) && check_collision_line_rectangle(
                    zap.start_pos,
                    zap.end_pos,
                    enemy_trans.translation(),
                    **enemy_hitbox,
                ) {
                    enemy_health.current -= **zap_damage;
                    zap.entities_zapped.insert(enemy_entity);
                }
            }
        }
    }
}