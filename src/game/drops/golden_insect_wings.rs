use bevy::prelude::*;

use crate::game::{waves::insect_wave::GOLDEN_WINGS_PATH, components::Hitbox};

use super::ItemDrop;

const GOLDEN_WINGS_SIZE: Vec2 = Vec2::new(80.0, 100.0);

pub fn create_golden_insect_wings(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
) {
    let image_handle = asset_server.load(GOLDEN_WINGS_PATH);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(GOLDEN_WINGS_SIZE),
            ..default()
        },
        texture: image_handle,
        transform: Transform::from_translation(position),
        ..default()
    })
    .insert(Hitbox(GOLDEN_WINGS_SIZE))
    .insert(ItemDrop);
}