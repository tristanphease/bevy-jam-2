use bevy::prelude::*;

use crate::game::{components::{Hitbox, Health, ShotType, CollidesShot}, health_bar::{generate_health_bar, HealthBarMaterial, WithHealthBar}};

use super::insect_ai::InsectAI;

const INSECT_IMAGE_PATH: &str = "images/insect.png";
const INSECT_TEXTURE_SIZE: Vec2 = Vec2::new(336.0, 442.0);
const INSECT_SIZE: Vec2 = Vec2::new(100.0, 140.0);
const INSECT_HEALTH: f32 = 10.0;

#[derive(Component)]
pub struct Insect;

pub fn spawn_insect(
    commands: &mut Commands, 
    asset_server: &mut Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    meshes: &mut ResMut<Assets<Mesh>>, 
    materials: &mut ResMut<Assets<HealthBarMaterial>>,
    x: f32, 
    y: f32,
) {
    let texture_handler = asset_server.load(INSECT_IMAGE_PATH);
    let texture_atlas = TextureAtlas::from_grid(texture_handler, INSECT_TEXTURE_SIZE, 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let health_bar = generate_health_bar(commands, meshes, materials, INSECT_SIZE.y/2.0);

    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            custom_size: Some(INSECT_SIZE),
            ..default()
        },
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
        ..default()
    })
    .insert(Hitbox(INSECT_SIZE))
    .insert(Health::new(INSECT_HEALTH))
    .insert(CollidesShot(ShotType::Enemy))
    .insert(Insect)
    .insert(InsectAI { target_position: Vec2::new(x, y) })
    .insert(WithHealthBar(health_bar));
}

