use bevy::prelude::*;

use crate::game::{components::Hitbox, waves::digger_wave::DIGGER_EYES_PATH};

use super::ItemDrop;

const DIGGER_EYES_SIZE: Vec2 = Vec2::new(60.0, 50.0);

pub fn create_digger_eyes(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
) {
    let image_handle = asset_server.load(DIGGER_EYES_PATH);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(DIGGER_EYES_SIZE),
            ..default()
        },
        texture: image_handle,
        transform: Transform::from_translation(position),
        ..default()
    })
    .insert(Hitbox(DIGGER_EYES_SIZE))
    .insert(ItemDrop);
}