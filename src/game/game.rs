use bevy::prelude::*;

use crate::{GameResultResource, GameResult};

use super::{components::Hitbox, cauldron::PotionEffect};

const CAULDRON_TEXTURE_SIZE: Vec2 = Vec2::new(353.0, 296.0);
const CAULDRON_SIZE: Vec2 = Vec2::new(350.0, 300.0);

pub const GAME_WIDTH: usize = 2_000;
pub const GAME_HEIGHT: usize = 2_000;

pub const WAVES_TO_COMPLETE: usize = 4;

#[derive(Component)]
pub struct Cauldron;

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

    let cauldron_potion = asset_server.load("images/potion_outline.png");

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(CAULDRON_SIZE),
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Hitbox(CAULDRON_SIZE))
        .insert(Cauldron)
        .with_children(|cauldron| {
            cauldron.spawn_bundle(SpriteBundle {
                sprite: Sprite { 
                    color: Color::WHITE,
                    ..default()
                },
                texture: cauldron_potion,
                transform: Transform::from_translation(Vec3::Z * 5.0),
                ..default()
            })
            .insert(PotionEffect);
        });
}

pub fn cleanup_game(
    mut commands: Commands,
    query: Query<(Entity, Option<&Cauldron>, Option<&PotionEffect>)>,
    game_result: Res<GameResultResource>,
) {
    for (entity, cauldron_option, potion_option) in query.iter() {
        if !(game_result.result == GameResult::Win && (cauldron_option.is_some() || potion_option.is_some())) {
            commands.entity(entity).despawn_recursive();
        }
        
    }
}
