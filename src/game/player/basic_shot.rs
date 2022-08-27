use std::time::Duration;

use bevy::prelude::*;

use crate::game::{shot::Shot, components::{Hitbox, Damage, CollidesEnemy}};

const SHOT_SPEED: f32 = 500.0;
const SHOT_SIZE: f32 = 40.0;
const SHOT_DAMAGE: f32 = 2.0;
const SHOT_TIME_LENGTH: f32 = 5.0;

pub const BASIC_SHOT_IMAGE_PATH: &str = "images/shot.png";
pub const BASIC_SHOT_WAND_PATH: &str = "images/basic_shot_wand.png";
pub const BASIC_SHOT_TEXTURE_SIZE: Vec2 = Vec2::new(200.0, 200.0);

#[derive(Component)]
pub struct ShotSpeed(pub f32);

pub fn create_basic_shot(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    position: Vec2,
    angle: f32,
) {
    let texture_handler = asset_server.load(BASIC_SHOT_IMAGE_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handler, BASIC_SHOT_TEXTURE_SIZE, 2, 2);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(
                    position.x,
                    position.y,
                    1.0
                ),
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3::new(SHOT_SIZE / BASIC_SHOT_TEXTURE_SIZE.x, SHOT_SIZE / BASIC_SHOT_TEXTURE_SIZE.y, 1.0),
            },
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .insert(Shot {
            timer: Timer::new(Duration::from_secs_f32(SHOT_TIME_LENGTH), false),
        })
        .insert(CollidesEnemy)
        .insert(ShotSpeed(SHOT_SPEED))
        .insert(Damage(SHOT_DAMAGE))
        .insert(Hitbox(Vec2::new(SHOT_SIZE, SHOT_SIZE)));
}