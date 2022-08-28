use bevy::prelude::*;

use crate::game::{components::Hitbox, waves::dragon_wave::DRAGON_COAL_PATH};

use super::ItemDrop;

const DRAGON_COAL_SIZE: Vec2 = Vec2::new(200.0, 100.0);

pub fn create_dragon_coal(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
) {
    let image_handle = asset_server.load(DRAGON_COAL_PATH);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(DRAGON_COAL_SIZE),
            ..default()
        },
        texture: image_handle,
        transform: Transform::from_translation(position),
        ..default()
    })
    .insert(Hitbox(DRAGON_COAL_SIZE))
    .insert(ItemDrop);
}