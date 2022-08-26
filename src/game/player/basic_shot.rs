use std::time::Duration;

use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use crate::game::{shot::Shot, components::{Hitbox, Damage, CollidesEnemy}};

const SHOT_SPEED: f32 = 500.0;
const SHOT_RADIUS: f32 = 20.0;
const SHOT_DAMAGE: f32 = 2.0;
const SHOT_TIME_LENGTH: f32 = 10.0;

#[derive(Component)]
pub struct ShotSpeed(pub f32);

pub fn create_basic_shot(
    commands: &mut Commands,
    position: Vec2,
    angle: f32,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) {
    commands
        .spawn()
        .insert_bundle(MaterialMesh2dBundle {
            transform: Transform {
                translation: Vec3::new(
                    position.x,
                    position.y,
                    1.0
                ),
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3::splat(SHOT_RADIUS)
            },
            mesh: Mesh2dHandle(mesh),
            material,
            ..default()
        })
        .insert(Shot {
            timer: Timer::new(Duration::from_secs_f32(SHOT_TIME_LENGTH), false),
        })
        .insert(CollidesEnemy)
        .insert(ShotSpeed(SHOT_SPEED))
        .insert(Damage(SHOT_DAMAGE))
        .insert(Hitbox(Vec2::new(SHOT_RADIUS * 2.0, SHOT_RADIUS * 2.0)));
}