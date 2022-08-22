use bevy::prelude::*;

use super::{
    components::{Health, Hitbox, Player},
    health_bar::{get_health_bar, HealthBarHandles},
};

const PLAYER_TEXTURE_SIZE: Vec2 = Vec2::new(106.0, 153.0);
const PLAYER_SIZE: Vec2 = Vec2::new(100.0, 150.0);

const PLAYER_START_POS: Vec2 = Vec2::new(200.0, 0.0);

const PLAYER_HEALTH: f32 = 100.0;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    health_bar_handles: Res<HealthBarHandles>,
) {
    let texture_handler = asset_server.load("images/wizard.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handler, PLAYER_TEXTURE_SIZE, 1, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let health_bar = get_health_bar(&mut commands, health_bar_handles, PLAYER_SIZE.y/2.0);

    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                custom_size: Some(PLAYER_SIZE),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                PLAYER_START_POS.x,
                0.0,
                PLAYER_START_POS.y,
            )),
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .insert(Player)
        .insert(Hitbox(PLAYER_SIZE))
        .insert(Health(PLAYER_HEALTH))
        .add_child(health_bar);
}

pub fn check_player_death(mut commands: Commands, player_query: Query<&Health, With<Player>>) {
    let player_health = player_query.single().0;

    if player_health <= 0.0 {
        //dead, change state here
        println!("player dead");
    }
}
