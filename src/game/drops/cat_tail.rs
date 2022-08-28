use bevy::prelude::*;

use crate::game::{components::Hitbox, waves::cat_wave::CAT_DROP_IMAGE_PATH};

use super::ItemDrop;

const CAT_TAIL_TEXTURE_SIZE: Vec2 = Vec2::new(210.0, 220.0);
const CAT_TAIL_SIZE: Vec2 = Vec2::new(100.0, 100.0);

pub fn create_cat_tail(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
) {
    let image_handle = asset_server.load(CAT_DROP_IMAGE_PATH);
    commands.spawn_bundle(SpriteBundle {
        texture: image_handle,
        transform: Transform::from_translation(position)
            .with_scale(Vec3::new(CAT_TAIL_SIZE.x / CAT_TAIL_TEXTURE_SIZE.x, CAT_TAIL_SIZE.y / CAT_TAIL_TEXTURE_SIZE.y, 1.0)),
        ..default()
    })
    .insert(Hitbox(CAT_TAIL_SIZE))
    .insert(ItemDrop);
}