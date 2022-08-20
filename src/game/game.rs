use bevy::prelude::*;

use super::components::Hitbox;

const CAULDRON_TEXTURE_SIZE: Vec2 = Vec2::new(353.0, 296.0);
const CAULDRON_SIZE: Vec2 = Vec2::new(350.0, 300.0);

/// Sets up objects in world
pub fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    //set up cauldron
    let texture_handler = asset_server.load("images/cauldron.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handler, CAULDRON_TEXTURE_SIZE, 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(CAULDRON_SIZE),
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .insert(Hitbox(CAULDRON_SIZE));
}
