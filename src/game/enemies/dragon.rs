use bevy::prelude::*;

use crate::game::{health_bar::{HealthBarMaterial, generate_health_bar, WithHealthBar}, components::{Health, Hitbox, CollidesEnemy}};

use super::dragon_ai::{DragonState, Dragon};

const DRAGON_IMAGE_PATH: &str = "images/dragon.png";
const DRAGON_TEXTURE_SIZE: Vec2 = Vec2::new(243.0, 251.0);
const DRAGON_SIZE: Vec2 = Vec2::new(200.0, 200.0);

const DRAGON_HEALTH: f32 = 70.0;

pub fn create_dragon(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<HealthBarMaterial>>,
    position: Vec2,
) {
    let texture_handler = asset_server.load(DRAGON_IMAGE_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handler, DRAGON_TEXTURE_SIZE, 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let translation = Vec3::new(position.x, position.y, 0.0);

    let health_bar = generate_health_bar(commands, meshes, materials, translation, DRAGON_SIZE.y/2.0);

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite { 
            custom_size: Some(DRAGON_SIZE),
            ..default()
        },
        transform: Transform::from_translation(translation),
        ..default()
    })
    .insert(Health::new(DRAGON_HEALTH))
    .insert(Hitbox(DRAGON_SIZE))
    .insert(WithHealthBar(health_bar))
    .insert(Dragon {
        state: DragonState::Moving,
        timer: Timer::from_seconds(2.0, true),
    })
    .insert(CollidesEnemy);
}