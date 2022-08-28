use bevy::prelude::*;

use crate::game::{components::Hitbox, waves::cat_wave::CAT_DROP_IMAGE_PATH};

use super::ItemDrop;

const CAT_TAIL_SIZE: Vec2 = Vec2::new(30.0, 30.0);

pub fn create_cat_tail(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
) {
    let image_handle = asset_server.load(CAT_DROP_IMAGE_PATH);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(CAT_TAIL_SIZE),
            ..default()
        },
        texture: image_handle,
        transform: Transform::from_translation(position),
        ..default()
    })
    .insert(Hitbox(CAT_TAIL_SIZE))
    .insert(ItemDrop);
}