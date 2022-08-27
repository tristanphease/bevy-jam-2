use bevy::prelude::*;

use crate::game::{components::Hitbox, waves::digger_wave::DIGGER_EYES_PATH};

use super::ItemDrop;

const DIGGER_EYES_TEXTURE_SIZE: Vec2 = Vec2::new(95.0, 73.0);
const DIGGER_EYES_SIZE: Vec2 = Vec2::new(60.0, 50.0);

pub fn create_digger_eyes(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
) {
    let image_handle = asset_server.load(DIGGER_EYES_PATH);
    commands.spawn_bundle(SpriteBundle {
        texture: image_handle,
        transform: Transform::from_translation(position)
            .with_scale(Vec3::new(DIGGER_EYES_SIZE.x / DIGGER_EYES_TEXTURE_SIZE.x, DIGGER_EYES_SIZE.y / DIGGER_EYES_TEXTURE_SIZE.y, 1.0)),
        ..default()
    })
    .insert(Hitbox(DIGGER_EYES_SIZE))
    .insert(ItemDrop);
}