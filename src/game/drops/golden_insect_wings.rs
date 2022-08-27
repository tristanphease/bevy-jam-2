use bevy::prelude::*;

use crate::game::{waves::insect_wave::GOLDEN_WINGS_PATH, components::Hitbox};

use super::ItemDrop;

const GOLDEN_WINGS_TEXTURE_SIZE: Vec2 = Vec2::new(172.0, 206.0);
const GOLDEN_WINGS_SIZE: Vec2 = Vec2::new(80.0, 100.0);

pub fn create_golden_insect_wings(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
) {
    let image_handle = asset_server.load(GOLDEN_WINGS_PATH);
    commands.spawn_bundle(SpriteBundle {
        texture: image_handle,
        transform: Transform::from_translation(position)
            .with_scale(Vec3::new(GOLDEN_WINGS_SIZE.x / GOLDEN_WINGS_TEXTURE_SIZE.x, GOLDEN_WINGS_SIZE.y / GOLDEN_WINGS_TEXTURE_SIZE.y, 1.0)),
        ..default()
    })
    .insert(Hitbox(GOLDEN_WINGS_SIZE))
    .insert(ItemDrop);
}